use crate::Shape::*;

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub traits: Shape
}

impl ShapeContract for Rectangle {
    fn make(width: u32, height: u32, x: i32, y: i32) -> Rectangle {
        let shape = Rectangle {
            height: height,
            width: width,
            x: x,
            y: y,
            traits: Shape {
                depth: 0,
                infill: 0.0,
                shapeData: Data::new()
                    .move_to((x, y))
                    .line_by((0, -(height as i32)/2))
                    .line_by((width/2, 0))
                    .line_by((0, height/2))
                    .close(),
                color: String::from("black")
            }
        };
    
        shape
    }

    fn make_with_infill(width: u32, height: u32, x: i32, y: i32, density: i32) -> Rectangle {
        let mut square = Rectangle {
            height: height,
            width: width,
            x: x,
            y: y,
            traits: Shape {
                depth: 0,
                infill: 0.0,
                shapeData: Data::new()
                    .move_to((x, y))
                    /*Draw the initial square with a diaganal */
                    .line_by((0, -(height as i32)))
                    .line_by((width, 0))
                    .line_by((0, height))
                    .line_by((-(width as i32), 0))
                    .line_by(diaganal(width as i32, height as i32, 0).0)
                    .line_by(diaganal(width as i32, height as i32, 0).1),
                color: String::from("black")
            }
        };

        let mut backtrace: i32 = 0;
        let mut iteration: i32 = 1;
        loop {
            let diaganal = diaganal(width as i32, height as i32, density * iteration);

            if diaganal == ((0, 0), (0, 0)) {
                break;
            }
            square.traits.shapeData = square.traits.shapeData
                .line_by((density, 0))
                .line_by(diaganal.0)
                .line_by(diaganal.1);

            backtrace += density;
            iteration += 1;
        }
        square.traits.shapeData = square.traits.shapeData.line_by((-backtrace, 0));

        backtrace = 0;
        iteration = 1;
        loop {
            let diaganal = diaganal(width as i32, height as i32, density * iteration);

            if diaganal == ((0, 0), (0, 0)) {
                break;
            }
            square.traits.shapeData = square.traits.shapeData
                .line_by((0, -density))
                .line_by(diaganal.0)
                .line_by(diaganal.1);

            backtrace += density;
            iteration += 1;
        }

        square
    }
}

fn diaganal(width: i32, height: i32, increment: i32) -> ((i32, i32), (i32, i32)) {

    if width < increment || height < increment {
        return ((0,0), (0,0));
    }

    let component = (width - increment, -(height - increment));
    let inverse = (-component.0, -component.1);

    (component, inverse)
}


#[test]
fn diaganal_under_test() {
    assert_eq!(diaganal(10, 10, 5), ((5, -5),(-5, 5)));
    assert_eq!(diaganal(10, 10, 10), ((0, 0),(0, 0)));
}


