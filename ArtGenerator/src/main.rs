// TODO: fix all names and unused - Austin Haskell
#![allow(non_snake_case)] 
#![allow(unused_imports)]

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

pub mod Shape;
pub mod BasicShapes { pub mod Rectangle; }

use Shape::ShapeContract;
use BasicShapes::Rectangle::*;

fn main() {

    let pageWidth: i32 = 200;
    let pageHeight: i32 = 100;

    let mut document = Document::new()
        .set("viewBox", (0, 0, pageWidth, pageHeight));

    let mut vec: Vec<BasicShapes::Rectangle::Rectangle> = Vec::new();
    for _ in 0..10 {
        let size = rand::random::<u32>() % 20 + 10;
        vec.push( 
            Rectangle::make_with_infill(
                size + 10, 
                size, 
                rand::random::<u32>() as i32 % pageWidth + 10, 
                rand::random::<u32>() as i32 % pageHeight + 10,
                1
        ));
    }

    for s in vec {
        document = document
            .add( Path::new()
            .set("fill", "none")
            .set("stroke", pick_color())
            .set("stroke-width", 0.5)
            .set("d", s.traits.shapeData));
    }

    svg::save("image.svg", &document).unwrap();
}

fn pick_color() -> String {
    let colorCode = rand::random::<u8>() % 7;
    match colorCode {
        0 => return String::from("blue"),
        1 => return String::from("red"),
        2 => return String::from("green"),
        3 => return String::from("orange"),
        4 => return String::from("purple"),
        5 => return String::from("yellow"),
        _ => return String::from("black")
    }
}