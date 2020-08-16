use crate::Shape::*;

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

pub struct Square {
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub traits: Shape
}

impl ShapeContract for Square {
    fn fillShapeData(mut self: Self) {

    }
}

pub fn make_square(width: i32, height: i32, x: i32, y: i32) -> Square {
    let shape = Square {
        height: height,
        width: width,
        x: x,
        y: y,
        traits: Shape {
            depth: 0,
            infill: 0.0,
            shapeData: Data::new()
                .move_to((x, y))
                .line_by((0, -height/2))
                .line_by((width/2, 0))
                .line_by((0, height/2))
                .close(),
            color: String::from("black")
        }
    };

    shape
}

pub fn add_infill(mut square: Square) {

}
