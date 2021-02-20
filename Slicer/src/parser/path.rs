use crate::parser::parser_defs;
use crate::parser::svg_util;
use crate::regex_defs;
use crate::parser::svg_commands::bezier;
use crate::parser::svg_commands::line;
use crate::parser::path_defs;

use regex::*;

const BEZIER_RESOLUTION: f32 = 0.01;

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

                // TODO: Functions like transform, scale, etc that show up after data points. - Austin Haskell
                //  Note: I think some refactoring is going to be needed to do the above todo. 
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

    let mut current_position = position;
    let mut last_control_point: Option<(f32, f32)> = None;
    let mut close_path: bool = false;
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

        let mut point: Option<parser_defs::SVGPoint> = None;

        match item.chars().nth(0).unwrap() {
            path_defs::MOVE_ABSOLUTE => {
                let x: f32 = coordanates[0];
                let y: f32 = coordanates[1];

                current_position.x = x;
                current_position.y = y;
            },
            path_defs::MOVE_RELATIVE => {
                let x: f32 = coordanates[0];
                let y: f32 = coordanates[1];

                current_position.x += x;
                current_position.y += y;
            }
            path_defs::LINE_RELATIVE => {

                let x: f32 = coordanates[0];
                let y: f32 = coordanates[1];
    
                point = Some(line::line_relative((x, y), (last_point.x, last_point.y)));
            },
            path_defs::LINE_ABSOLUTE => {
                let x: f32 = coordanates[0];
                let y: f32 = coordanates[1];
    
                point = Some(line::line_absolute((x, y)));
            },
            path_defs::HORIZONTAL_RELATIVE => {
                let x: f32 = coordanates[0];
    
                point = Some(line::horizontal_line_relative(x, (last_point.x, last_point.y)));
            },
            path_defs::HORIZONTAL_ABSOLUTE => {
                let x: f32 = coordanates[0];
    
                point = Some(line::horizontal_line_absolute((x, last_point.y)));
            },
            path_defs::VERTICAL_RELATIVE => {
                let y: f32 = coordanates[0];
    
                point = Some(line::vertical_line_relative(y, (last_point.x, last_point.y)));
            },
            path_defs::VERTICAL_ABSOLUTE => {
                let y: f32 = coordanates[0];
    
                point = Some(line::vertical_line_absolute((last_point.x, y)));

            }
            // TODO: Refactor the bezier functions -Austin Haskell
            path_defs::CUBIC_BEZIER_ABSOLUTE => {
                if coordanates.len() % 2 != 0 {
                    return Err(String::from("Error: Absolute Bezier curve has an insufficiant point count"));
                }

                let points = svg_util::create_xy_point_list(&coordanates);

                let repeat_command_groups: i32 = (points.len() as i32) / path_defs::CUBIC_POINTS_PER_GROUP;
                for group in 1..=repeat_command_groups {
                    let group_offset: usize = ((path_defs::CUBIC_POINTS_PER_GROUP * group) - path_defs::CUBIC_POINTS_PER_GROUP) as usize;

                    let mut t: f32 = 0.0;
                    while t < 1.0 {
                        point_list.push(
                            bezier::calculate_cubic_bezier(
                                (current_position.x, current_position.y), 
                                points[0 + group_offset], 
                                points[1 + group_offset], 
                                points[2 + group_offset], t));
                        t += BEZIER_RESOLUTION;
                    }

                    last_control_point = Some(points[1 + group_offset]);
                    current_position.x = points[2 + group_offset].0;
                    current_position.y = points[2 + group_offset].1;

                    point_list.push(parser_defs::SVGPoint {
                            x: current_position.x,
                            y: current_position.y
                        }
                    );
                }
            },
            path_defs::CUBIC_BEZIER_RELATIVE => {
                if coordanates.len() % 2 != 0 {
                    return Err(String::from("Error: Relative Bezier curve has an insufficiant point count"));
                }

                let points = svg_util::create_xy_point_list(&coordanates);

                let repeat_command_groups: i32 = (points.len() as i32) / path_defs::CUBIC_POINTS_PER_GROUP;
                for group in 1..=repeat_command_groups {
                    let group_offset: usize = ((path_defs::CUBIC_POINTS_PER_GROUP * group) - path_defs::CUBIC_POINTS_PER_GROUP) as usize;
                    let mut t: f32 = 0.0;
                    while t < 1.0 {
                        point_list.push(
                            bezier::calculate_cubic_bezier(
                                (current_position.x, current_position.y), 
                                (points[0 + group_offset].0 + current_position.x, points[0 + group_offset].1 + current_position.y), 
                                (points[1 + group_offset].0 + current_position.x, points[1 + group_offset].1 + current_position.y), 
                                (points[2 + group_offset].0 + current_position.x, points[2 + group_offset].1 + current_position.y), t));
                        t += BEZIER_RESOLUTION;
                    }

                    current_position.x += points[2 + group_offset].0;
                    current_position.y += points[2 + group_offset].1;
                    last_control_point = Some((points[1 + group_offset].0 + current_position.x, points[1 + group_offset].1 + current_position.y));

                    point_list.push(parser_defs::SVGPoint {
                        x: current_position.x,
                        y: current_position.y
                    });
                }
            },
            path_defs::SHORTHAND_CUBIC_BEZIER_ABSOLUTE => {
                if last_control_point.is_none() {
                    return Err(String::from("Error: Shorthand abolute bezier curve was used before a previous control point could be established. "));
                }

                if coordanates.len() % 2 != 0 {
                    return Err(String::from("Error: Shorthand absolute Bezier curve has an insufficiant point count"));
                }

                let points = svg_util::create_xy_point_list(&coordanates);

                let repeat_command_groups: i32 = (points.len() as i32) / path_defs::SHORTHAND_CUBIC_POINTS_PER_GROUP as i32;
                for group in 1..=repeat_command_groups {
                    let group_offset: usize = ((path_defs::SHORTHAND_CUBIC_POINTS_PER_GROUP * group) - path_defs::SHORTHAND_CUBIC_POINTS_PER_GROUP) as usize;
                    let mut t: f32 = 0.0;
                    while t < 1.0 {
                        point_list.push(
                            bezier::calculate_cubic_bezier(
                                (current_position.x, current_position.y), 
                                bezier::calculate_reflected_control_point(last_control_point.unwrap(), (current_position.x, current_position.y)), 
                                (points[0 + group_offset].0, points[0 + group_offset].1), 
                                (points[1 + group_offset].0, points[1 + group_offset].1), t));
                        t += BEZIER_RESOLUTION;
                    }

                    current_position.x = points[1 + group_offset].0;
                    current_position.y = points[1 + group_offset].1;
                    last_control_point = Some(points[0 + group_offset]);

                    point_list.push(parser_defs::SVGPoint {
                        x: current_position.x,
                        y: current_position.y
                    });
                }

            },
            path_defs::SHORTHAND_CUBIC_BEZIER_RELATIVE => {
                if last_control_point.is_none() {
                    return Err(String::from("Error: Shorthand reelative bezier curve was used before a previous control point could be established. "));
                }

                if coordanates.len() % 2 != 0 {
                    return Err(String::from("Error: Shorthand relative Bezier curve has an insufficiant point count"));
                }

                let points = svg_util::create_xy_point_list(&coordanates);

                let repeat_command_groups: i32 = (points.len() as i32) / path_defs::SHORTHAND_CUBIC_POINTS_PER_GROUP as i32;
                for group in 1..=repeat_command_groups {
                    let group_offset: usize = ((path_defs::SHORTHAND_CUBIC_POINTS_PER_GROUP * group) - path_defs::SHORTHAND_CUBIC_POINTS_PER_GROUP) as usize;
                    let mut t: f32 = 0.0;
                    while t < 1.0 {
                        point_list.push(
                            bezier::calculate_cubic_bezier(
                                (current_position.x, current_position.y), 
                                bezier::calculate_reflected_control_point(last_control_point.unwrap(), (current_position.x, current_position.y)), 
                                (points[0 + group_offset].0 + current_position.x, points[0 + group_offset].1 + current_position.y), 
                                (points[1 + group_offset].0 + current_position.x, points[1 + group_offset].1 + current_position.y), t));
                        t += BEZIER_RESOLUTION;
                    }

                    last_control_point = Some((points[0 + group_offset].0 + current_position.x, points[0 + group_offset].1 + current_position.y));

                    current_position.x += points[1 + group_offset].0;
                    current_position.y += points[1 + group_offset].1;

                    point_list.push(parser_defs::SVGPoint {
                        x: current_position.x,
                        y: current_position.y
                    });
                }
            },
            path_defs::QUADRATIC_BEZIER_ABSOLUTE => {
                println!("No implementation currently exists for quadratic bezier (Q)");
            },
            path_defs::QUADRATIC_BEZIER_RELATIVE => {
                println!("No implementation currently exists for quadratic bezier (q)");

                /*
                if coordanates.len() % 2 != 0 {
                    return Err(String::from("Error: Absolute Bezier curve has an insufficiant point count"));
                }

                let points = svg_util::create_xy_point_list(&coordanates);

                let repeat_command_groups: i32 = (points.len() as i32) / path_defs::QUADRATIC_POINTS_PER_GROUP as i32;
                for group in 1..=repeat_command_groups {
                    let group_offset: usize = ((path_defs::QUADRATIC_POINTS_PER_GROUP * group) - path_defs::QUADRATIC_POINTS_PER_GROUP) as usize;
                    let mut t: f32 = 0.0;
                    while t < 1.0 {
                        point_list.push(
                            bezier::calculate_quadratic_bezier(
                                (current_position.x, current_position.y), 
                                (points[0 + group_offset].0 + current_position.x, points[0 + group_offset].1 + current_position.y),
                                (points[1 + group_offset].0 + current_position.x, points[1 + group_offset].1 + current_position.y), 
                                t));
                        t += BEZIER_RESOLUTION;
                    }

                    current_position.x = points[1 + group_offset].0 + current_position.x;
                    current_position.y = points[1 + group_offset].1 + current_position.y;
                    last_control_point = Some((points[0 + group_offset].0 + current_position.x, points[0 + group_offset].1 + current_position.y));

                    point_list.push(parser_defs::SVGPoint {
                            x: current_position.x,
                            y: current_position.y
                        }
                    );
                }*/
            },
            path_defs::SHORTHAND_QUADRATIC_BEZIER_ABSOLUTE => {
                println!("No implementation currently exists for shorthand quadratic bezier (T)");
            },
            path_defs::SHORTHAND_QUADRATIC_BEZIER_RELATIVE => {
                println!("No implementation currently exists for shorthand quadratic bezier (t)");
            },
            path_defs::ELIPTICAL_ARC_ABSOLUTE => {
                println!("No implementation currently exists for eliptical arcs (A)");
            },
            path_defs::ELIPTICAL_ARC_RELATIVE => {
                println!("No implementation currently exists for eliptical arcs (a)");

                if coordanates.len() % 5 != 0 {
                    return Err(String::from("Error: Eliptical Arc relative has an insufficiant point count. Needed 5 data points"));
                }
            },
            path_defs::FINISH_PATH_LOWER => close_path = true,
            path_defs::FINISH_PATH_UPPER => close_path = true,
            _ => println!("")
        };

        if point.is_some() {
            point_list.push(point.unwrap());
            last_point = point.unwrap();
        }
    }

    if close_path {
        point_list.push(position);
    }

    if point_list.is_empty() {
        return Err(String::from("Error: No point data found (l), svg file is considered malformed. "));
    }

    Ok(point_list)
}

