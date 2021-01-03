use crate::parser::parser_defs;
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
                // [a-zA-Z]([ ,]*[\-0-9]*[ ,]*)*
                let regex = Regex::new(r"[a-zA-Z]([ ,]*[\-0-9]*[ ,]*)*").unwrap();
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

    let delimiter = Regex::new(",* *").unwrap();
    let mut items: Vec<&str> = Vec::new();
    for i in delimiter.find_iter(data) {
        items.push(&data[i.start()..i.end()]);
    }

    println!("Parsing position for path: {:?}", data);
    let mut point: Option<parser_defs::SVGPoint> = None;
    for item in items {
        println!("Item: {:?}", item);
        if item.chars().nth(0).unwrap() == 'M' {
            
            let mut positions: Vec<&str> = Vec::new();
            for i in delimiter.find_iter(item) {
                positions.push(&item[i.start()..i.end()]);
            }
            if positions.len() < 2 {
                println!("Expected atleast 2 points, instead found {:?}", positions.len());
                return Err(String::from("Error: Malformed positional data for path. "));
            }

            point = Some(
                parser_defs::SVGPoint {
                    x: positions[0].parse().unwrap(),
                    y: positions[1].parse().unwrap()
                }
            );
        }
    }
    if point.is_none() {
        return Err(String::from("Error: No position data found (M), svg file is considered malformed. "));
    }

    Ok(point.unwrap())
}

fn parse_points(data: &Vec<&str>, position: parser_defs::SVGPoint) -> Result<Vec<parser_defs::SVGPoint>, String> {

    let mut point_list: Vec<parser_defs::SVGPoint> = Vec::new();

    let mut last_point: parser_defs::SVGPoint = position;
    point_list.push(last_point);
    for item in data {
        let mut coordanates: Vec<&str> = (&item[1..]).split(", ").collect();
        
        match item.chars().nth(0).unwrap() {
            'l' => {
                let coordanates: Vec<&str> = (&item[1..]).split(',').collect();

                let x: f32 = coordanates[0].parse().unwrap();
                let y: f32 = coordanates[1].parse().unwrap();
    
                let point = parser_defs::SVGPoint {
                    x: last_point.x + x,
                    y: last_point.y + y
                };
    
                point_list.push(point);
                last_point = point;
            },
            'L' => {
                let coordanates: Vec<&str> = (&item[1..]).split(',').collect();
                let x: f32 = coordanates[0].parse().unwrap();
                let y: f32 = coordanates[1].parse().unwrap();
    
                let point = parser_defs::SVGPoint {
                    x: x,
                    y: y
                };
    
                point_list.push(point);
                last_point = point;
            },
            'h' => {
                let coordanates: Vec<&str> = (&item[1..]).split(',').collect();
                let x: f32 = coordanates[0].parse().unwrap();
    
                let point = parser_defs::SVGPoint {
                    x: x + last_point.x,
                    y: last_point.y
                };
    
                point_list.push(point);
                last_point = point;
            },
            'H' => {
                let coordanates: Vec<&str> = (&item[1..]).split(',').collect();
                let x: f32 = coordanates[0].parse().unwrap();
    
                let point = parser_defs::SVGPoint {
                    x: x,
                    y: last_point.y
                };
    
                point_list.push(point);
                last_point = point;
            },
            'v' => {
                let coordanates: Vec<&str> = (&item[1..]).split(',').collect();
                let y: f32 = coordanates[0].parse().unwrap();
    
                let point = parser_defs::SVGPoint {
                    x: last_point.x,
                    y: y + last_point.y
                };
    
                point_list.push(point);
                last_point = point;
            },
            'V' => {
                let coordanates: Vec<&str> = (&item[1..]).split(',').collect();
                let y: f32 = coordanates[0].parse().unwrap();
    
                let point = parser_defs::SVGPoint {
                    x: last_point.x,
                    y: y
                };
    
                point_list.push(point);
                last_point = point;
            }
            'C' => println!("Got C item"),
            'c' => println!("Got c item"),
            'Z' => println!("Got Z item"),
            'M' => continue,
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