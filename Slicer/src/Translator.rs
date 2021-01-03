use string_builder::Builder;

use crate::Util;
use crate::GCode;
use crate::TranslatorUtil;

pub struct Translator {
    pub printbed_width: f32,
    pub printbed_height: f32
}

#[derive(Clone)]
pub struct QuadrantBlock {
    pub quadrant: (i32, i32),
    pub gcode: Vec<GCode::GCode>
}

// Note: We use absolute positioning, that allows us to segment them into quadrants. - Austin Haskell
impl Translator {
    pub fn Line(self: &Self, points: Vec<(f32, f32)>) -> Vec<QuadrantBlock> {
        if points.len() <= 0 {
            return vec![]; 
        }

        let mut block_list: Vec<QuadrantBlock> = Vec::new();
        let mut last_point: (f32, f32) = points[0];

        for point in points {
            if last_point == point {
                // Skip first iteration. 
                continue;
            }

            let mut start_quadrant = self.CalcQuadrantForPoint(last_point);
            let mut end_quadrant   = self.CalcQuadrantForPoint(point);

            // We want to make sure that the start point is the leftmost point, this is to help
            //  with the iteration later on. - Austin Haskell
            if start_quadrant.0 > end_quadrant.0 {
                let temp = start_quadrant;
                start_quadrant = end_quadrant;
                end_quadrant = temp;
            } else if start_quadrant.0 == end_quadrant.0 && start_quadrant.1 > end_quadrant.1 {
                // Note: This could probs be joined with the if above, though it might be unreadable. 
                let temp = start_quadrant;
                start_quadrant = end_quadrant;
                end_quadrant = temp;
            }

            if start_quadrant == end_quadrant {
                let mut block = QuadrantBlock {
                    quadrant: start_quadrant,
                    gcode: Vec::new()
                };

                block.gcode.push(
                    TranslatorUtil::point_to_move_cmd(
                        self.normalize_point_to_printbed(last_point, self.CalcQuadrantForPoint(last_point))));

                block.gcode.push(
                    TranslatorUtil::point_to_move_cmd(
                        self.normalize_point_to_printbed(point, self.CalcQuadrantForPoint(point))));

                block_list.push(block);
            } 
            else {
                // TODO: This is where that QuadrantBlock needs to come into play, we dont want to have to
                //  parse the resulting string list a second time - Austin Haskell

                let mut quadrant_list: Vec<TranslatorUtil::Rectangle> = Vec::new();
                if start_quadrant.1 == end_quadrant.1 {
                    for x in start_quadrant.0..=end_quadrant.0 {
                        quadrant_list.push( TranslatorUtil::Rectangle {
                            quad_x: x,
                            quad_y: start_quadrant.1,
                            x: x as f32 * self.printbed_width,
                            y: start_quadrant.1 as f32 * self.printbed_height,
                            width: self.printbed_width,
                            height: self.printbed_height
                        });
                    }
                }
                else if start_quadrant.0 == end_quadrant.0 {
                    for y in start_quadrant.1..=end_quadrant.1 {
                        quadrant_list.push(TranslatorUtil::Rectangle {
                            quad_x: start_quadrant.0,
                            quad_y: y,
                            x: start_quadrant.0 as f32 * self.printbed_width,
                            y: y as f32 * self.printbed_height,
                            width: self.printbed_width,
                            height: self.printbed_height
                        });
                    }
                }
                else {
                    for x in start_quadrant.0..=end_quadrant.0 {
                        for y in start_quadrant.1..=end_quadrant.1 {
                            quadrant_list.push(TranslatorUtil::Rectangle {
                                quad_x: x,
                                quad_y: y,
                                x: x as f32 * self.printbed_width,
                                y: y as f32 * self.printbed_height,
                                width: self.printbed_width,
                                height: self.printbed_height
                            });
                        }
                    }
                }

                
                for quadrant in quadrant_list {
                    let intersection_points = TranslatorUtil::find_intersection_points_for_rectangle(
                        (last_point, point), 
                        quadrant);

                    let mut block = QuadrantBlock {
                        quadrant: (quadrant.quad_x, quadrant.quad_y),
                        gcode: Vec::new()
                    };

                    if intersection_points.len() == 1 {
                        // This case will get hit when we're on the ends of a line. This 
                        //  ______ ______ ______
                        // |      |      |      |
                        // |  X---|------|---X  |
                        // |______|______|______|

                        println!(
                            "Only one intersection point found for quadrant [{:?},{:?}]. Intersection at {:?}. S:{:?} E:{:?}. Point normalized to {:?}", 
                            quadrant.quad_x, 
                            quadrant.quad_y, 
                            intersection_points[0],
                            last_point, point,
                            self.normalize_point_to_printbed(intersection_points[0], (quadrant.quad_x,quadrant.quad_y)));

                        block.gcode.push(
                            TranslatorUtil::point_to_move_cmd(
                                self.normalize_point_to_printbed(intersection_points[0], (quadrant.quad_x,quadrant.quad_y))));

                        let mut p = last_point;
                        if (quadrant.quad_x, quadrant.quad_y) == start_quadrant && self.CalcQuadrantForPoint(point) == start_quadrant {
                            p = point;
                        } else if (quadrant.quad_x, quadrant.quad_y) == end_quadrant && self.CalcQuadrantForPoint(point) == end_quadrant {
                            p = point
                        }

                        block.gcode.push(
                            TranslatorUtil::point_to_move_cmd(
                                self.normalize_point_to_printbed(p, self.CalcQuadrantForPoint(p))));

                    }
                    else if intersection_points.len() > 0 {                        
                        for intersection_point in intersection_points {
                            block.gcode.push(TranslatorUtil::point_to_move_cmd(
                                self.normalize_point_to_printbed(intersection_point, (quadrant.quad_x,quadrant.quad_y))));
                        }
                    }
                    block_list.push(block);
                } 
            }

            last_point = point;
        }
        
        block_list
    }

    pub fn Polygon(self: &Self, points: Vec<(f32, f32)>) -> Vec<GCode::GCode> {

        if points.len() < 3 {
            vec![String::from("")];
        }
        // TODO: Infill - Austin Haskell

        let mut gcode: Vec<GCode::GCode> = Vec::new();
        let mut last_point: (f32, f32) = points[0];
        for point in points {
            if last_point == point {
                continue;
            }

            let mut line: Vec<(f32, f32)> = Vec::new();
            line.push(last_point);
            line.push(point);

            //gcode.append(&mut Translator::Line(&self, line));

            last_point = point;
        }

        vec![]
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

    pub fn Polyline(self: &Self) -> Vec<String> {
        vec![String::from("")]
        // TODO: Implementation - Austin Haskell
    }

    pub fn Arc(self: &Self) -> Vec<String> {
        vec![String::from("")]
        // TODO: Implementation - Austin Haskell
    }

    pub fn CalcQuadrantForPoint(self: &Self, point: (f32, f32)) -> (i32, i32) {
        (Util::float_mod(point.0, self.printbed_width) as i32, 
         Util::float_mod(point.1, self.printbed_height) as i32)
    }

    fn normalize_point_to_printbed(self: &Self, point: (f32, f32), quadrant: (i32, i32)) -> (f32, f32){
        let mut normalized_point = 
            (point.0 - (point.0 / self.printbed_width).floor() * self.printbed_width,
            point.1 - (point.1 / self.printbed_height).floor() * self.printbed_height);

        // If 0 we might need to make it = printbed_width 
        if normalized_point.0 == 0.0 && point.0 == quadrant.0 as f32 * self.printbed_width + self.printbed_width{
            println!("Correcting x normalization for point {:?} in quadrant {:?}", point, quadrant);
            normalized_point.0 = self.printbed_width;
        }
        if normalized_point.1 == 0.0 && point.1 == quadrant.1 as f32 * self.printbed_height + self.printbed_height {
            println!("Correcting y normalization for point {:?} in quadrant {:?}", point, quadrant);
            normalized_point.1 = self.printbed_height;
        }

        normalized_point
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

    let actual = machine.Line(points);

    let mut expected: Vec<GCode::GCode> = Vec::new();

    // TODO: Fix this unit test - Austin Haskell

    /*expected.push("Q 0 0".to_string());
    expected.push("X 0".to_string());
    expected.push("Y 0".to_string());
    expected.push("X 10.5".to_string());
    expected.push("Y 10".to_string());*/

    //assert!(Util::compare_gcode_line_vectors(expected, actual));
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

    let actual = machine.Line(points);

    // TODO: Fix this test - Austin Haskell
    let mut expected: Vec<GCode::GCode> = Vec::new();
    /*expected.push("Q 0 0".to_string());
    expected.push("X 0".to_string());
    expected.push("Y 0".to_string());
    expected.push("X 10.5".to_string());
    expected.push("Y 10".to_string());*/

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

    let actual = machine.Line(points);

    // Plotted this on a graphing calculator. intersection points are
    // (16, 8.788), (29.131, 16), (32, 17.576)
    // and passes through quadrants 
    // (0,0) (1,0) (1,1) (2,1) 

    // TODO: Finish this test - Austin Haskell
    let expected: Vec<GCode::GCode> = Vec::new();

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