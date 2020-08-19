use crate::Shape::*;

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

pub struct Square {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub traits: Shape
}

impl ShapeContract for Square {
    fn make(width: u32, height: u32, x: i32, y: i32) -> Square {
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
                    .line_by((0, -(height as i32)/2))
                    .line_by((width/2, 0))
                    .line_by((0, height/2))
                    .close(),
                color: String::from("black")
            }
        };
    
        shape
    }

    fn make_with_infill(width: u32, height: u32, x: i32, y: i32, density: i32) -> Square {
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
    
        #[derive(Debug, PartialEq)]
        struct RawShape {
            pub points: Vec<(u32, u32)>
        }
    
        let mut pointCloud = RawShape {
            points: Vec::new()
        };
    
        let segments = segment_line((0,0), (width, 0), density);
        let segments2 = segment_line((0,0), (0, height), density);
    
        let zippedLine = zip_line(segments, segments2);
    
        println!("Segmenting a line with a width of {:?}", width);
        for item in zippedLine {
            pointCloud.points.push(item);
        }
    
        shape.traits.shapeData = Data::new()
            .move_to((x, y));
    
        for p in pointCloud.points {
            shape.traits.shapeData = shape.traits.shapeData
                .line_by(p);
        }
    
        shape
    }
}

fn midpoint(p1: (u32, u32), p2: (u32, u32)) -> (u32, u32) {
    ((p1.0 + p2.0) / 2, (p1.1 + p2.1) / 2)
}

fn segment_line(p1: (u32, u32), p2: (u32, u32), segment_count: i32) -> Vec<(u32, u32)> {
    let mut vec: Vec<(u32, u32)> = Vec::new();
    let mut count: i32 = segment_count;
    
    vec.push(p1);
    vec.push(midpoint(p1, p2));
    vec.push(p2);
    count-=3;

    if count <= 1 {
        return vec
    }

    let subsegment1 = segment_line(vec[0], vec[1], count);
    let subsegment2 = segment_line(vec[1], vec[2], count);
    
    let mut joined_vec: Vec<(u32, u32)> = Vec::new();
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
    vec.clear();

    for item in joined_vec {
        if !vec.contains(&item) {
            vec.push(item);
        }
    }

    vec
}

fn zip_line(a: Vec<(u32, u32)>, b: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut line_zipped: Vec<(u32, u32)> = Vec::new();
    
    if b.len() != a.len() {
        return line_zipped;
    }

    // WIP, doesnt work correct atm - Austin Haskell
    let mut ping_pong: usize = 0;
    for point in a {
        print!("Pushing point: {:?}", point);
        line_zipped.push(point);
        ping_pong = ping_pong+1;
        if ping_pong % 3 == 0 {
            if ping_pong < b.len() {
                print!("Pushing point: {:?}", b[ping_pong]);
                line_zipped.push(b[ping_pong])
            }
            else {
                break;
            }
        }
    }

    line_zipped
}

#[test]
fn midpoint_calculation_is_equal() {
    assert_eq!(midpoint((10, 10), (20, 20)), (15, 15));
}

#[test]
fn segment_line_under_test() {
    let segmentedLine: Vec<(u32, u32)> = segment_line((0, 0), (20, 0), 10);
    let expectedLine: Vec<(u32, u32)> = vec![(0,0), (5,0), (7, 0), (10,0), (15, 0), (20,0)];

    assert_eq!(segmentedLine, expectedLine);
}




