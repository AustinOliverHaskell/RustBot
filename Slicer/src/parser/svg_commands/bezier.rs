use crate::parser::parser_defs;
use crate::parser::svg_util;

// Note: Offset is used for the relative cubic bezier. - Austin Haskell
// Adapted from: https://www.geeksforgeeks.org/cubic-bezier-curve-implementation-in-c/
pub fn calculate_cubic_bezier(
    start_point: (f32, f32),
    control_point_1: (f32, f32), 
    control_point_2: (f32, f32), 
    end_point: (f32, f32), 
    t: f32) -> parser_defs::SVGPoint {
    
    let mut x = (1.0 - t).powf(3.0) * start_point.0;
    x += 3.0 * t * (1.0 - t).powf(2.0) * control_point_1.0;
    x += 3.0 * (1.0 - t) * t.powf(2.0) * control_point_2.0;
    x += t.powf(3.0) * end_point.0;
    
    let mut y = (1.0 - t).powf(3.0) * start_point.1;
    y += 3.0 * t * (1.0 - t).powf(2.0) * control_point_1.1;
    y += 3.0 * (1.0 - t) * t.powf(2.0) * control_point_2.1;
    y += t.powf(3.0) * end_point.1;
    
    parser_defs::SVGPoint {
        x: x,
        y: y
    }
}

pub fn calculate_quadratic_bezier(
    start_point: (f32, f32),
    control_point: (f32, f32), 
    end_point: (f32, f32), 
    t: f32) -> parser_defs::SVGPoint {

    let mut x = (1.0 - t).powf(2.0) * start_point.0;
    x += 2.0 * (1.0 - t) * t * control_point.0;
    x += t.powf(2.0) * end_point.0;
        
    let mut y = (1.0 - t).powf(2.0) * start_point.1;
    y += 2.0 * (1.0 - t) * t * control_point.1;
    y += t.powf(2.0) * end_point.1;
        
    parser_defs::SVGPoint {
        x: x,
        y: y
    }
}

pub fn calculate_reflected_control_point(prev_control_point: (f32, f32), last_point: (f32, f32)) -> (f32, f32) {
    let x_diff = (last_point.0 - prev_control_point.0).abs();
    let y_diff = (last_point.1 - prev_control_point.1).abs();

    let x: f32;
    if last_point.0 < prev_control_point.0 {
        x = last_point.0 - x_diff;
    } else {
        x = last_point.0 + x_diff;
    }

    let y: f32;
    if last_point.1 < prev_control_point.1 {
        y = last_point.1 - y_diff;
    } else {
        y = last_point.1 + y_diff;
    }

    (x, y)
}