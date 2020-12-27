#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SVGColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SVGPoint {
    pub x: f32,
    pub y: f32
}

#[derive(Debug, PartialEq)]
pub struct SVGShape {
    pub points: Vec<SVGPoint>,
    pub color: SVGColor,
    pub shape_type: String,
    pub position: SVGPoint
}

pub fn map_color(color: &str) -> SVGColor {

    let parsed_color: SVGColor;

    match color {
        "blue"   => parsed_color = SVGColor { r: 0,   g: 0,   b: 255, a: 0},
        "red"    => parsed_color = SVGColor { r: 255, g: 0,   b: 0,   a: 0},
        "green"  => parsed_color = SVGColor { r: 0,   g: 255, b: 0,   a: 0},
        "yellow" => parsed_color = SVGColor { r: 255, g: 255, b: 0,   a: 0},
        "purple" => parsed_color = SVGColor { r: 255, g: 0,   b: 255, a: 0},
        "white"  => parsed_color = SVGColor { r: 255, g: 255, b: 255, a: 0},
        "black"  => parsed_color = SVGColor { r: 0  , g: 0,   b: 0  , a: 0},
        _ => parsed_color = SVGColor { r: 0  , g: 0,   b: 0  , a: 0}
    }
    
    parsed_color
}