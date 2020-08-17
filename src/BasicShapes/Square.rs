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

pub fn make_square_with_infill(width: i32, height: i32, x: i32, y: i32, density: i32) -> Square {
    let mut shape = Square {
        height: height,
        width: width,
        x: x,
        y: y,
        traits: Shape {
            depth: 0,
            infill: 0.0,
            shapeData: Data::new(),
            color: String::from("black")
        }
    };

    #[derive(Debug)]
    struct RawShape {
        pub points: Vec<(i32, i32)>
    }

    let mut pointCloud = RawShape {
        points: Vec::new()
    };

    let segments = segment_line((0,0), (width, 0), 10);

    println!("Segmenting a line with a width of {:?}", width);
    for item in segments {
        pointCloud.points.push(item);
        println!("{:?}", item);
    }

    shape.traits.shapeData = Data::new()
        .move_to((x, y));

    for p in pointCloud.points {
        shape.traits.shapeData = shape.traits.shapeData
            .line_by(p);
    }

    shape
}

fn midpoint(p1: (i32, i32), p2: (i32, i32)) -> (i32, i32) {
    ((p1.0 + p2.0) / 2, (p1.1 + p2.1) / 2)
}

fn segment_line(p1: (i32, i32), p2: (i32, i32), segment_count: i32) -> Vec<(i32, i32)> {
    let mut vec: Vec<(i32, i32)> = Vec::new();
    let mut count: i32 = segment_count;
        
    vec.push(p1);
    vec.push(midpoint(p1, p2));
    vec.push(p2);
    count-=2;

    if segment_count <= 1 {
        return vec
    }

    let subsegment1 = segment_line(vec[0], vec[1], count);
    let subsegment2 = segment_line(vec[1], vec[2], count);
    
    let mut joined_vec: Vec<(i32, i32)> = Vec::new();
    joined_vec.push(vec[0]);
    for item in subsegment1 {
        joined_vec.push(item);
    }
    joined_vec.push(vec[1]);
    for item in subsegment2 {
        joined_vec.push(item);
    }
    joined_vec.push(vec[2]);
    // Reduce duplicate elements
    // Scalling is incorrect, need to account for the width of the segments since
    //  lineby draws the point from the point of view of the previous - Austin Haskell
    joined_vec
}