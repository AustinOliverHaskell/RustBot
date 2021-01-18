use crate::parser::parser_defs::*;

struct MinMax {
    x_min: f32,
    y_min: f32,
    x_max: f32,
    y_max: f32
}

pub fn scale_points(shape_list: Vec<SVGShape>, x_scaling: f32, y_scaling: f32, preserve_aspect: bool) -> Vec<Vec<(f32, f32)>>{
    let mut total_list: Vec<Vec<(f32, f32)>> = Vec::new();

    for shape in shape_list {
        let mut point_list: Vec<(f32, f32)> = Vec::new();

        for point in shape.points {
            point_list.push((point.x, point.y));
        }
    
        total_list.push(point_list);
    }
    let mut minmax = find_min_and_max(&total_list);

    scale(&mut total_list, &minmax, x_scaling, y_scaling, preserve_aspect);

    minmax = find_min_and_max(&total_list);

    remove_negatives_and_shift(&total_list, &minmax)
}

fn scale(points: &mut Vec<Vec<(f32, f32)>>, minmax: &MinMax, x_scaling: f32, y_scaling: f32, preserve_aspect: bool) {

    // Big Note: We interpret a point of 1, 1 in the svg to mean that point is 1mm x 1mm away from the origin. - Austin Haskell
    let x_distance = minmax.x_max - minmax.x_min;
    let y_distance = minmax.y_max - minmax.y_min;

    let x_scale_factor = x_scaling * 1000.0 / x_distance;
    let y_scale_factor = y_scaling * 1000.0 / y_distance;

    println!("Using scale factors of {:?}, {:?} respectivly", x_scale_factor, y_scale_factor);

    for list in points {
        for point in list {
            point.0 = point.0 * x_scale_factor;
            if preserve_aspect {
                point.1 = point.1 * x_scale_factor;
            } else {
                point.1 = point.1 * y_scale_factor;
            }
        }
    }
}

fn remove_negatives_and_shift(points: &Vec<Vec<(f32, f32)>>, minmax: &MinMax) -> Vec<Vec<(f32, f32)>> {
    let y_min = minmax.y_min.abs();
    let x_min = minmax.x_min.abs();
    println!("Shifting by {:?} in the x direction and {:?} in the y direction to make sure that all points are positive", x_min, y_min);

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

fn find_min_and_max(points: &Vec<Vec<(f32, f32)>>) -> MinMax {
    let mut y_min = 0.0;
    let mut x_min = 0.0;
    let mut y_max = 0.0;
    let mut x_max = 0.0;

    for list in points {
        for point in list {
            if point.0 < x_min {
                x_min = point.0
            }
            if point.1 < y_min {
                y_min = point.1;
            }

            if point.0 > x_max {
                x_max = point.0;
            }
            if point.1 > y_max {
                y_max = point.1;
            }
        }
    }

    MinMax {
        x_min: x_min,
        x_max: x_max,
        y_min: y_min,
        y_max: y_max
    }
}