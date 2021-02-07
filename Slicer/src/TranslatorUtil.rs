extern crate geo;
extern crate line_intersection;

use line_intersection::{LineInterval, LineRelation};
use geo::{Coordinate, Line, Point};

use crate::GCode;

// TODO: Figure out a better spot for this - Austin Haskell
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub quad_x: i32,
    pub quad_y: i32,
    pub x: f32, 
    pub y: f32,
    pub height: f32, 
    pub width: f32
}

// Explanation found here: https://en.wikipedia.org/wiki/Cohen%E2%80%93Sutherland_algorithm
//  Note that the code below does not
pub fn find_intersection_points_for_rectangle(line: ((f32, f32),(f32, f32)), rectangle: Rectangle) -> Vec<(f32, f32)> {

    let mut expanded_rectangle: Vec<((f32, f32), (f32, f32))> = Vec::new();
    expanded_rectangle.push(((rectangle.x, rectangle.y),                    (rectangle.x, rectangle.y + rectangle.height)));
    expanded_rectangle.push(((rectangle.x, rectangle.y),                    (rectangle.x + rectangle.width, rectangle.y))); 
    expanded_rectangle.push(((rectangle.x + rectangle.width, rectangle.y),  (rectangle.x + rectangle.width, rectangle.y + rectangle.height)));
    expanded_rectangle.push(((rectangle.x, rectangle.y + rectangle.height), (rectangle.x + rectangle.width, rectangle.y + rectangle.height)));

    let mut intersection_points: Vec<(f32, f32)> = Vec::new();

    for p in expanded_rectangle {
        let intersection = find_intersection_point_of_lines(line, (p.0, p.1));

        if intersection.is_none() {
            continue;
        }

        intersection_points.push(intersection.unwrap());
    }

    intersection_points
}

pub fn find_intersection_point_of_lines(line1: ((f32, f32), (f32, f32)), line2: ((f32, f32), (f32, f32))) -> Option<(f32, f32)> {
    let line_segment_1 = LineInterval::line_segment( Line {
        start: ((line1.0).0, (line1.0).1).into(),
        end: ((line1.1).0, (line1.1).1).into()
    });

    let line_segment_2 = LineInterval::line_segment( Line {
        start: ((line2.0).0, (line2.0).1).into(),
        end: ((line2.1).0, (line2.1).1).into()
    });

    let line_relationships = line_segment_1.relate(&line_segment_2);

    if line_relationships == LineRelation::DivergentDisjoint ||
       line_relationships == LineRelation::Parallel ||
       line_relationships == LineRelation::Collinear{
        return None;
    }

    let intersection_point = line_relationships.unique_intersection().unwrap();

    Some((intersection_point.x(), intersection_point.y()))
}

pub fn is_quadrant_adjacent(quadrant_base: (i32, i32), quadrant_to_check: (i32, i32)) -> bool {
    let diff = (quadrant_base.0 - quadrant_to_check.0, 
                quadrant_base.1 - quadrant_to_check.1);

    if diff.0 > 1 || diff.0 < -1{
        return false;
    }

    if diff.1 > 1 || diff.1 < -1 {
        return false;
    }

    true
}

pub fn build_two_parameter_command(word: GCode::Word, x: f32, y: f32) -> GCode::GCode {

    GCode::GCode {
        command: word,
        x: x,
        y: y,
        z: 0.0
    }
}

pub fn point_to_move_cmd(point: (f32, f32)) -> GCode::GCode {
    build_two_parameter_command(GCode::Word {
        letter: 'G',
        value: 1
    }, point.0, point.1)
}

pub fn point_to_move_quadrant_cmd(quadrant: (i32, i32)) -> GCode::GCode {
    build_two_parameter_command(GCode::Word {
        letter: 'Q',
        value: 1
    }, quadrant.0 as f32, quadrant.1 as f32)
}



#[test]
pub fn does_line_cross_rectangle_crosses() { 
    let rect = Rectangle {
        height: 16.0,
        width: 16.0,
        x: 0.0,
        y: 0.0,
        quad_x: 0,
        quad_y: 0
    };

    let point1: (f32, f32) = (5.0, 5.0);
    let point2: (f32, f32) = (18.0, 5.0);

    let actual = find_intersection_points_for_rectangle((point1, point2), rect);
    assert!(actual.len() != 0);
}

#[test]
pub fn does_line_cross_rectangle_doesnt_cross() {
    let rect = Rectangle {
        height: 16.0,
        width: 16.0,
        x: 0.0,
        y: 0.0,
        quad_x: 0,
        quad_y: 0
    };

    let point1: (f32, f32) = (16.5, 16.5);
    let point2: (f32, f32) = (18.0, 18.0);

    let actual = find_intersection_points_for_rectangle((point1, point2), rect);
    assert!(actual.len() == 0);
}

#[test]
pub fn find_intersection_point_of_lines_does_not_intersect() {
    
    let line1: ((f32, f32), (f32, f32)) = ((0.0, 0.0), (10.0, 10.0));
    let line2: ((f32, f32), (f32, f32)) = ((12.0, 12.0), (20.0, 20.0));

    assert!(find_intersection_point_of_lines(line1, line2).is_none());
}

#[test]
pub fn find_intersection_point_of_lines_does_intersect() {
    
    let line1: ((f32, f32), (f32, f32)) = ((0.0, 0.0), (10.0, 10.0));
    let line2: ((f32, f32), (f32, f32)) = ((0.0, 6.0), (2.0, 0.0));

    assert!(find_intersection_point_of_lines(line1, line2).is_some());
}

#[test]
fn IsQuadrantAdjacent_IsAdjacent() {
    let q1: (i32, i32) = (0,  0);
    let q2: (i32, i32) = (-1, 0);
    let q3: (i32, i32) = (0, -1);
    let q4: (i32, i32) = (0,  1);
    let q5: (i32, i32) = (1,  0);

    assert!(is_quadrant_adjacent(q1, q2));
    assert!(is_quadrant_adjacent(q1, q3));
    assert!(is_quadrant_adjacent(q1, q4));
    assert!(is_quadrant_adjacent(q1, q5));
}


#[test]
fn IsQuadrantAdjacent_IsNotAdjacent() {
    let q1: (i32, i32) = (0,  0);
    let q2: (i32, i32) = (-2, 0);
    let q3: (i32, i32) = (0, -5);
    let q4: (i32, i32) = (0,  10);
    let q5: (i32, i32) = (10,  0);

    assert!(!is_quadrant_adjacent(q1, q2));
    assert!(!is_quadrant_adjacent(q1, q3));
    assert!(!is_quadrant_adjacent(q1, q4));
    assert!(!is_quadrant_adjacent(q1, q5));
}