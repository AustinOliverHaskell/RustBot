use crate::parser::parser_defs;
use crate::parser::svg_util;

pub fn parse_svg_rect(element: &quick_xml::events::BytesStart) -> Result<parser_defs::SVGShape, String> {
    let attribute_list = element.attributes();
    let color: parser_defs::SVGColor = parser_defs::SVGColor {r: 0, g: 0, b: 0, a: 0};

    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut width: f32  = 0.0;
    let mut height: f32 = 0.0;

    for attribute in attribute_list {
        let att = attribute.unwrap();
        let data = &String::from_utf8(att.value.to_vec()).unwrap();
        match att.key {
            b"x" => {
                x = svg_util::parse_possible_float(data);
            },
            b"y" => {
                y = svg_util::parse_possible_float(data);
            },
            b"width" => {
                width = svg_util::parse_possible_float(data);
            },
            b"height" => {
                height = svg_util::parse_possible_float(data);
            },
            _ => {}
        }
    }

    let mut points: Vec<parser_defs::SVGPoint> = Vec::new();
    points.push(parser_defs::SVGPoint {
        x: x,
        y: y
    });
    points.push(parser_defs::SVGPoint {
        x: x + width,
        y: y
    });
    points.push(parser_defs::SVGPoint {
        x: x + width,
        y: y + height
    });
    points.push(parser_defs::SVGPoint {
        x: x,
        y: y + height
    });
    points.push(parser_defs::SVGPoint {
        x: x,
        y: y
    });

    Ok(parser_defs::SVGShape {
        points: points,
        position: parser_defs::SVGPoint {
            x: x,
            y: y
        },
        shape_type: String::from("rectangle"),
        color: color
    })
}