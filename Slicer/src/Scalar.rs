use crate::parser::parser_defs::*;

pub fn scale_points(shape_list: Vec<SVGShape>, x_scaling: f32, y_scaling: f32) -> Vec<Vec<(f32, f32)>>{
    let mut total_list: Vec<Vec<(f32, f32)>> = Vec::new();

    for shape in shape_list {
        let mut point_list: Vec<(f32, f32)> = Vec::new();

        for point in shape.points {
            point_list.push((point.x, point.y));
        }
    
        total_list.push(point_list);
    }

    remove_negatives_and_shift(&total_list)
}

pub fn remove_negatives_and_shift(points: &Vec<Vec<(f32, f32)>>) -> Vec<Vec<(f32, f32)>> {

    let mut y_min = 0.0;
    let mut x_min = 0.0;

    for list in points {
        for point in list {
            if point.0 < x_min {
                x_min = point.0
            }
            if point.1 < y_min {
                y_min = point.1;
            }
        }
    }

    y_min = y_min.abs();
    x_min = x_min.abs();

    let mut shifted_points_list: Vec<Vec<(f32, f32)>> = Vec::new();
    for list in points {
        let mut shifted_points: Vec<(f32, f32)> = Vec::new();
        for point in list {
            shifted_points.push((point.0 + x_min, point.1 + y_min));
        }
        shifted_points_list.push(shifted_points);
    }

    shifted_points_list
}