use string_builder::Builder;

use crate::Util;
use crate::TranslatorUtil;

pub struct Translator {
    pub printbed_width: f32,
    pub printbed_height: f32
} 

pub struct QuadrantBlock {
    quadrant: (i32, i32),
    gcode: Vec<String>
}

// Note: We use absolute positioning, that allows us to segment them into quadrants. - Austin Haskell

// TODO: Use Scalar value - Austin Haskell
impl Translator {
    pub fn Line(self: &Self, points: Vec<(f32, f32)>, scaler: f32) -> Vec<String> {
        if points.len() <= 0 {
            return vec![String::from("")]; 
        }

        let mut gcode: Vec<String> = Vec::new();
        let mut last_point: (f32, f32) = points[0];

        let starting_quadrant: (i32, i32) = self.CalcQuadrantForPoint(last_point);

        for point in points {
            if last_point == point {
                // Skip first iteration. 
                continue;
            }

            let mut start_quadrant = self.CalcQuadrantForPoint(last_point);
            let mut end_quadrant   = self.CalcQuadrantForPoint(point);

            println!("Start quadrant [{:?}]", start_quadrant);
            println!("End quadrant [{:?}]", end_quadrant);


            // We want to make sure that the start point is the leftmost point, this is to help
            //  with the iteration later on. - Austin Haskell
            if start_quadrant.0 > end_quadrant.0 {
                let temp = start_quadrant;
                start_quadrant = end_quadrant;
                end_quadrant = temp;
            }

            if start_quadrant == end_quadrant {
                gcode.push(TranslatorUtil::point_to_move_quadrant_cmd(starting_quadrant));
                gcode.push(TranslatorUtil::point_to_move_cmd(point));
            } else {
                // TODO: This is where that QuadrantBlock needs to come into play, we dont want to have to
                //  parse the resulting string list a second time - Austin Haskell

                if start_quadrant.1 == end_quadrant.1 { // Quadrant move is only horizontal
                    for x in start_quadrant.0..=end_quadrant.0 {
                        let quadrant = TranslatorUtil::Rectangle {
                            x: x as f32,
                            y: start_quadrant.0 as f32,
                            width: self.printbed_width,
                            height: self.printbed_height
                        };

                        let intersection_points = TranslatorUtil::find_intersection_points_for_rectangle(
                            (last_point, point), 
                            quadrant);

                        if intersection_points.len() > 0 {
                            gcode.push(TranslatorUtil::point_to_move_quadrant_cmd((x, start_quadrant.1)));

                            for intersection_point in intersection_points {
                                gcode.push(TranslatorUtil::point_to_move_cmd(
                                    (intersection_point.0 - (x as f32 * self.printbed_width), 
                                    intersection_point.1)));
                            }
                        } else {
                            println!("No intersection found. ");
                        }
                    }
                }
                else if start_quadrant.0 == end_quadrant.0 { // Quadrant move is only vertical
                    for y in start_quadrant.1..=end_quadrant.1 {
                        let quadrant = TranslatorUtil::Rectangle {
                            x: start_quadrant.0 as f32,
                            y: y as f32,
                            width: self.printbed_width,
                            height: self.printbed_height
                        };

                        let intersection_points = TranslatorUtil::find_intersection_points_for_rectangle(
                            (last_point, point), 
                            quadrant);

                        if intersection_points.len() > 0 {
                            gcode.push(TranslatorUtil::point_to_move_quadrant_cmd((start_quadrant.0, y)));

                            for intersection_point in intersection_points {
                                gcode.push(TranslatorUtil::point_to_move_cmd(
                                    (intersection_point.0, 
                                    (intersection_point.1 - (y as f32 * self.printbed_height)))));
                            }
                        } else {
                            println!("No intersection found. ");
                        }
                    }
                }
                else {
                    for x in start_quadrant.0..=end_quadrant.0 {
                        for y in start_quadrant.1..=end_quadrant.1 {
                            let quadrant = TranslatorUtil::Rectangle {
                                x: x as f32,
                                y: y as f32,
                                width: self.printbed_width,
                                height: self.printbed_height
                            };

                            let intersection_points = TranslatorUtil::find_intersection_points_for_rectangle(
                                (last_point, point), 
                                quadrant);

                            if intersection_points.len() > 0 {
                                gcode.push(TranslatorUtil::point_to_move_quadrant_cmd((x, y)));

                                for intersection_point in intersection_points {
                                    gcode.push(TranslatorUtil::point_to_move_cmd(
                                        (intersection_point.0 - (x as f32 * self.printbed_width), 
                                        (intersection_point.1 - (y as f32 * self.printbed_height)))));
                                }
                            } else {
                                println!("No intersection found. ");
                            }
                        }
                    }
                }
            }

            last_point = point;
        }
        
        gcode
    }

    pub fn Polyline(self: &Self) -> Vec<String> {
        vec![String::from("")]
        // TODO: Implementation - Austin Haskell
    }

    pub fn Arc(self: &Self) -> Vec<String> {
        vec![String::from("")]
        // TODO: Implementation - Austin Haskell
    }

    pub fn Polygon(self: &Self) -> Vec<String> {
        vec![String::from("")]
        // TODO: Implementation - Austin Haskell
    }

    pub fn Rectangle(self: &Self) -> Vec<String> {
        vec![String::from("")]
        // TODO: Implementation - Austin Haskell
    }

    pub fn Circle(self: &Self) -> Vec<String> {
        vec![String::from("")]
        // TODO: Implementation - Austin Haskell
    }

    pub fn Elipse(self: &Self) -> Vec<String> {
        vec![String::from("")]
        // TODO: Implementation - Austin Haskell
    }

    pub fn CalcQuadrantForPoint(self: &Self, point: (f32, f32)) -> (i32, i32) {
        (Util::float_mod(point.0, self.printbed_width) as i32, 
         Util::float_mod(point.1, self.printbed_height) as i32)
    }

    pub fn SplitLineAtBoundary(self: Self, point: (i32, i32)) -> ((f32, f32), (f32, f32)) {
        ((0.0, 0.0), (0.0, 0.0))
    }

    pub fn DoesLineCrossBoundary(self: &Self, start: (f32, f32), end: (f32, f32)) -> bool {
        let point_one_quadrant = Translator::CalcQuadrantForPoint(&self, start); 
        let point_two_quadrant = Translator::CalcQuadrantForPoint(&self, end);

        return point_one_quadrant != point_two_quadrant;
    }

    pub fn CalcBoundariesCrossedByLine(self: Self, point_start: (f32, f32), point_end: (f32, f32)) -> Vec<(i32, i32)> {
        // TODO: Implement this. Should probs be a cobination of Cohen-Sutherland algorithm and vector dot products - Austin Haskell        
        vec![]
    }
}

#[test]
fn Line_DoesNotCrossBoundary_Translates() {
    let machine = Translator {
        printbed_width: 16.0,
        printbed_height: 16.0
    };

    let mut points: Vec<(f32, f32)> = Vec::new();
    points.push((0.0, 0.0));
    points.push((10.5, 10.0));

    let actual = machine.Line(points, 1.0);

    let mut expected: Vec<String> = Vec::new();
    expected.push("Q 0 0".to_string());
    expected.push("X 0".to_string());
    expected.push("Y 0".to_string());
    expected.push("X 10.5".to_string());
    expected.push("Y 10".to_string());

    assert!(Util::compare_gcode_line_vectors(expected, actual));
}

#[test]
fn Line_CrossesSingularBondary_Splits() {
    let machine = Translator {
        printbed_width: 16.0,
        printbed_height: 16.0
    };

    let mut points: Vec<(f32, f32)> = Vec::new();
    points.push((0.0, 0.0));
    points.push((26.4, 14.5));

    let actual = machine.Line(points, 1.0);

    let mut expected: Vec<String> = Vec::new();
    expected.push("Q 0 0".to_string());
    expected.push("X 0".to_string());
    expected.push("Y 0".to_string());
    expected.push("X 10.5".to_string());
    expected.push("Y 10".to_string());

    assert_eq!(expected, actual);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Line_CrossesMultipleBoundaries_Splits() {

    let mut points: Vec<(f32, f32)> = Vec::new();
    points.push((0.0, 0.0));
    points.push((45.03, 19.24));

    let machine = Translator {
        printbed_width: 16.0,
        printbed_height: 16.0
    };

    let actual = machine.Line(points, 1.0);

    // Plotted this on a graphing calculator. intersection points are
    // (16, 8.788), (29.131, 16), (32, 17.576)
    // and passes through quadrants 
    // (0,0) (1,0) (1,1) (2,1) 
    let mut expected: Vec<String> = Vec::new();

    assert_eq!(expected, actual);
}

#[test]
fn Polyline_CrossesBoundary_Splits() {
    assert!(false)
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Polyline_DoesNotCrossBoundary_Translates() {
    assert!(false)
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Arc_CrossesBondary_Splits() {
    assert!(false)
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Arc_DoesNotCrossBondary_Translates() {
    assert!(false)
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Polygon_CrossesBoundary_Splits() {
    assert!(false)
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Polygon_DoesNotCrossBoundary_Translates() {
    assert!(false)
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Rectangle_CrossesBoundary_Splits() {
    assert!(false)
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Rectangle_DoesNotCrossBoundary_Translates() {
    assert!(false)
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Circle_CrossesBoundary_Splits() {
    assert!(false)
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Circle_DoesNotCrossBoundary_Translates() {
    assert!(false)
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Elipse_CrossesBondary_Splits() {
    assert!(false)
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Elipse_DoesNotCrossBondary_Translates() {
    assert!(false)
    // TODO: Implementation - Austin Haskell
}

#[test]
fn CalcQuadrantForPoint_PointOnBoarder_Calculates() {
    assert!(false)
    // TODO: Implementation - Austin Haskell
}

#[test]
fn CalcQuadrantForPoint_Calculates() {

    let expected: (i32, i32) = (2, -1);

    let machine = Translator {
        printbed_width: 16.0,
        printbed_height: 16.0
    };

    let actual = machine.CalcQuadrantForPoint((38.5, -17.2));

    assert_eq!(expected, actual);
}

#[test]
fn DoesLineCrossBoundary_Crosses() {
    let machine = Translator {
        printbed_width: 16.0,
        printbed_height: 16.0
    };

    assert_eq!(true, machine.DoesLineCrossBoundary((0.0, 0.0), (17.0, -17.0)));
}

#[test]
fn DoesLineCrossBoundary_DoesNotCross() {
    let machine = Translator {
        printbed_width: 16.0,
        printbed_height: 16.0
    };

    assert_eq!(false, machine.DoesLineCrossBoundary((0.0, 0.0), (1.0, -1.0)));

}