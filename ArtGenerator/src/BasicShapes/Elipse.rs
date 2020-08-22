use crate::Shape::*;

pub struct Circle {
    pub radius: u32,
    pub x: i32,
    pub y: i32,
    pub traits: Shape
}

impl ShapeContract for Circle {

}