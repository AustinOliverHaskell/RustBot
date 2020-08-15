use Shape;

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

pub struct Square {
    width: f32;
    height: f32;
    traits: Shape;
}

impl Shape for Square {
    fn Draw(self: &Self) {}
}
