use crate::parser::parser_defs;
use crate::parser::svg_util;
use crate::regex_defs;

use regex::*;

pub fn parse_svg_polygon(element: &quick_xml::events::BytesStart) -> Result<parser_defs::SVGShape, String> {
    let mut points: Vec<parser_defs::SVGPoint> = Vec::new();
    let attribute_list = element.attributes();
    let color: parser_defs::SVGColor = parser_defs::SVGColor {r: 0, g: 0, b: 0, a: 0};
    
    for attribute in attribute_list {
        let unwrapped_attribute = attribute.unwrap();
        let attribute_data = &String::from_utf8(unwrapped_attribute.value.to_vec()).unwrap();
        match unwrapped_attribute.key {
            b"points" => {
                println!("Polygon data: {:?}", attribute_data);

                let regex = Regex::new(regex_defs::COMMA_OR_SPACE).unwrap();
                let mut raw_points: Vec<f32> = Vec::new();
                let mut pos = 0;
                for i in regex.find_iter(attribute_data) {
                    let item_data: &str = &attribute_data[pos..i.start()];
                    if item_data.trim().is_empty(){
                        continue;
                    }   
                    raw_points.push(svg_util::parse_possible_float(item_data));
                    pos = i.end();
                }
                raw_points.push(svg_util::parse_possible_float(&attribute_data[pos..]));

                let collected_points = svg_util::create_xy_point_list(&raw_points);
                for p in collected_points {
                    points.push(parser_defs::SVGPoint {
                        x: p.0,
                        y: p.1
                    })
                }
            }
            _ => {}
        }
    }
    points.push(points[0].clone());

    println!("points for polygon: {:?}", points);

    let position = points[0];
    Ok(parser_defs::SVGShape {
        points: points,
        position: position,
        shape_type: String::from("polygon"),
        color: color
    })
}

//SVGPoint { x: 100.0, y: 10.0 }, 
//SVGPoint { x: 40.0, y: 198.0 }, 
//SVGPoint { x: 190.0, y: 78.0 }, 
//SVGPoint { x: 10.0, y: 78.0 }, 
//SVGPoint { x: 160.0, y: 198.0 }, 
//SVGPoint { x: 100.0, y: 10.0 }]


// 100,10 40,198 190,78 10,78 160,198