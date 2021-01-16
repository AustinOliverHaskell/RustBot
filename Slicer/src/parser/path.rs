use crate::parser::parser_defs;
use crate::parser::svg_util;
use crate::regex_defs;

use regex::*;

pub fn parse_svg_path(element: &quick_xml::events::BytesStart) -> Result<parser_defs::SVGShape, String> {
    let attribute_list = element.attributes();

    let mut color: parser_defs::SVGColor = parser_defs::SVGColor {r: 0, g: 0, b: 0, a: 0};
    let mut position: Option<parser_defs::SVGPoint> = None;
    let mut points: Vec<parser_defs::SVGPoint> = Vec::new(); 
    for attribute in attribute_list {
        let att = attribute.unwrap();
        match att.key {
            b"d" => {
                let data = &String::from_utf8(att.value.to_vec()).unwrap();
                let mut parsed_items: Vec<&str> = Vec::new();

                // TODO: Lazy Statics so this doesnt get compiled everytime this is called. - Austin Haskell
                let regex = Regex::new(regex_defs::SVG_COMMAND_GROUPING).unwrap();
                for i in regex.find_iter(data) {
                     parsed_items.push(&data[i.start()..i.end()]);
                }

                let possible_position = parse_position(parsed_items[0]);
                match possible_position {
                    Err(e) => return Err(e),
                    Ok(val) => position = Some(val)
                }

                let possible_points = parse_points(&parsed_items, position.unwrap());
                match possible_points {
                    Err(e) => return Err(e),
                    Ok(val) => points = val 
                }
            },
            b"stroke" => color = parser_defs::map_color(&String::from_utf8(att.value.to_vec()).unwrap()),
            _ => print!("")
        }
    }

    Ok(parser_defs::SVGShape {
        points: points,
        position: position.unwrap(),
        shape_type: String::from("polyline"),
        color: color
    })
}

// Note: It is very inefficiant to search through this twice, this could be parsed at the same
//  time as the point data to avoid the extra loop. - Austin Haskell
fn parse_position(data: &str) -> Result<parser_defs::SVGPoint, String> {

    println!("Data: {:?}", data);
    let delimiter = Regex::new(regex_defs::COMMA_OR_SPACE).unwrap();
    let mut items: Vec<&str> = Vec::new();
    let mut pos = 1;
    for i in delimiter.find_iter(data) {
        items.push(&data[pos..i.end()-1]);
        pos = i.start() + 1;
    }

    if items.len() < 2 {
        // Maybe need to go to eol and the line doenst have a space or letter at the end. 
        items.push(&data[pos..]);
    }

    
    if items.len() == 2 {
        let x: f32 = svg_util::parse_possible_float(items[0]);
        let y: f32 = svg_util::parse_possible_float(items[1]);
        
        return Ok(parser_defs::SVGPoint {
            x: x,
            y: y
        })
    }

    Err(String::from("Error: No position data found (M), svg file is considered malformed. "))

}

fn parse_points(data: &Vec<&str>, position: parser_defs::SVGPoint) -> Result<Vec<parser_defs::SVGPoint>, String> {

    let mut point_list: Vec<parser_defs::SVGPoint> = Vec::new();
    let regex = Regex::new(regex_defs::SEPERATE_TWO_NUMBERS).unwrap();

    let mut last_point: parser_defs::SVGPoint = position;
    point_list.push(last_point);

    for item in data {
        // TODO: Clean this up a bit - Austin Haskell
        let mut str_coordanates: Vec<&str> = Vec::new();
        for i in regex.find_iter(item) {
            let item_data: &str = &item[i.start()..i.end()];
            if item_data == "" {
                continue;
            }   
            str_coordanates.push(item_data);
        }

        let mut coordanates: Vec<f32> = Vec::new();
        for coord in str_coordanates {
            coordanates.push(svg_util::parse_possible_float(coord));
        }
        
        match item.chars().nth(0).unwrap() {
            'M' => {
                // TODO: Implement positional move - Austin Haskell
                println!("Got a new position. Ignoring now because of lack of implementation");
            },
            'l' => {

                let x: f32 = coordanates[0];
                let y: f32 = coordanates[1];
    
                let point = parser_defs::SVGPoint {
                    x: last_point.x + x,
                    y: last_point.y + y
                };
    
                point_list.push(point);
                last_point = point;
            },
            'L' => {
                let x: f32 = coordanates[0];
                let y: f32 = coordanates[1];
    
                let point = parser_defs::SVGPoint {
                    x: x,
                    y: y
                };
    
                point_list.push(point);
                last_point = point;
            },
            'h' => {
                let x: f32 = coordanates[0];
    
                let point = parser_defs::SVGPoint {
                    x: x + last_point.x,
                    y: last_point.y
                };
    
                point_list.push(point);
                last_point = point;
            },
            'H' => {
                let x: f32 = coordanates[0];
    
                let point = parser_defs::SVGPoint {
                    x: x,
                    y: last_point.y
                };
    
                point_list.push(point);
                last_point = point;
            },
            'v' => {
                let y: f32 = coordanates[0];
    
                let point = parser_defs::SVGPoint {
                    x: last_point.x,
                    y: y + last_point.y
                };
    
                point_list.push(point);
                last_point = point;
            },
            'V' => {
                let y: f32 = coordanates[0];
    
                let point = parser_defs::SVGPoint {
                    x: last_point.x,
                    y: y
                };
    
                point_list.push(point);
                last_point = point;
            }
            'C' => {
                if coordanates.len() % 2 != 0 {
                    return Err(String::from("Error: Absolute Bezier curve has an insufficiant point count"));
                }
                let bezier_points = create_bezier_from_points(&coordanates, position);
                for point in bezier_points {
                    point_list.push(point);
                }
            },
            'c' => {
                if coordanates.len() % 2 != 0 {
                    return Err(String::from("Error: Relative Bezier curve has an insufficiant point count"));
                }
                let bezier_points = create_bezier_from_points(&coordanates, position);
                for point in bezier_points {
                    point_list.push(point);
                }
            },
            'Z' => println!("Got Z item"),
            _ => println!("")
        };
    }

    // TODO: Figure out if that always holds true. I dont think it does. That's what the Z command is for - Austin Haskell
    // For a path, it's always closed back to the first point. - Austin Haskell
    point_list.push(position);

    if point_list.is_empty() {
        return Err(String::from("Error: No point data found (l), svg file is considered malformed. "));
    }

    Ok(point_list)
}

// TODO: Make a data type for this - Austin Haskell
// Adapted from: https://stackoverflow.com/questions/31167663/how-to-code-an-nth-order-bezier-curve/31169371
fn create_bezier_from_points(list: &Vec<f32>, position: parser_defs::SVGPoint) -> Vec<parser_defs::SVGPoint> {

    let point_list = svg_util::create_xy_point_list(list);
    let order = point_list.len()- 1;
    println!("Creating an order {:?} bezier curve. ", order);

    let mut bezier_points: Vec<parser_defs::SVGPoint> = Vec::new();
    let mut t: f32 = 0.0;
    while t < 1.0 {
        let point = calculate_point_on_bezier(t, &point_list, order as i32);
        t += 0.01;
        if point.is_err() {
            continue;
        }

        bezier_points.push(point.unwrap());
    }

    bezier_points
}

// Adapted from: https://stackoverflow.com/questions/31167663/how-to-code-an-nth-order-bezier-curve/31169371
fn calculate_coefficiant(max_order: i32, order: i32) -> f32 {
    let mut coefficiant: f32 = 1.0;

    for i in (max_order - order + 1)..=max_order {
        coefficiant = coefficiant * i as f32;
    }

    for i in 1..=order {
        coefficiant = coefficiant / i as f32;
    }

    coefficiant
}

fn calculate_point_on_bezier(t: f32, point_list: &Vec<(f32, f32)>, order: i32) -> Result<parser_defs::SVGPoint, String> {

    let mut x = 0.0;
    let mut y = 0.0;

    for i in 0..=order {
        x = x + (calculate_coefficiant(order, i) * (1.0 - t).powf((order - i) as f32) * t.powf(i as f32) * point_list[i as usize].0);
        y = y + (calculate_coefficiant(order, i) * (1.0 - t).powf((order - i) as f32) * t.powf(i as f32) * point_list[i as usize].1);
    }

    if x.is_infinite() {
        println!("Got an infinite value for x. point_list: {:?}", point_list);
        return Err(String::from("Err: Infinite value. "));
    }
    if y.is_infinite() {
        println!("Got an infinite value for x. point_list: {:?}", point_list);
        return Err(String::from("Err: Infinite value. "));
    }

    Ok(parser_defs::SVGPoint {
        x: x,
        y: y
    })
}