use crate::parser::parser_defs;

pub fn vertical_line_relative(v: f32, position: (f32, f32)) -> parser_defs::SVGPoint {
    parser_defs::SVGPoint {
        x: position.0,
        y: position.1 + v
    }
}

pub fn horizontal_line_relative(h: f32, position: (f32, f32)) -> parser_defs::SVGPoint {
    parser_defs::SVGPoint {
        x: position.0 + h,
        y: position.1
    }
}

pub fn line_relative(point: (f32, f32), position: (f32, f32)) -> parser_defs::SVGPoint {
    parser_defs::SVGPoint {
        x: position.0 + point.0,
        y: position.1 + point.1
    }
}

pub fn line_absolute(point: (f32, f32)) -> parser_defs::SVGPoint {
    parser_defs::SVGPoint {
        x: point.0,
        y: point.1
    }
}

pub fn vertical_line_absolute(point: (f32, f32)) -> parser_defs::SVGPoint {
    line_absolute(point)
}

pub fn horizontal_line_absolute(point: (f32, f32)) -> parser_defs::SVGPoint {
    line_absolute(point)
}