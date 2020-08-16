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
    fn fillShapeData(self: Self);
}
