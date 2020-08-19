use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

pub struct Shape {
    pub depth: i32,
    pub infill: f32,
    pub shapeData: Data,
    pub color: String
}

pub trait ShapeContract {
    fn make(width: u32, height: u32, x: i32, y: i32) -> Self;
    fn make_with_infill(width: u32, height: u32, x: i32, y: i32, density: i32) -> Self;
}
