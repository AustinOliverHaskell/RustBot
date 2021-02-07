use crate::GCode;

pub fn float_mod(a: f32, b: f32) -> f32 {

    // TODO: Refactor this, it could be made cleaner / more efficiant - Austin Haskell

    if b == 0.0 || a == 0.0 {
        return 0.0;
    }

    if a < b {
        return a / b;
    }

    let mut isNegative = false;
    let mut divisor = b;
    if divisor < 0.0 {
        isNegative = true;
        divisor *= -1.0;
    }

    let mut value: f32 = a;
    if value < 0.0 {
        value *= -1.0;
        isNegative = !isNegative;
    }

    let mut divisions: i32 = 0;
    while value > divisor {
        value -= divisor;
        divisions += 1;
    }

    /*let remainder;
    if a > 0.0 {
        remainder = ((a * value) as i32) as f32 / 10.0;
    } else {
        remainder = (((a * -1.0) * value) as i32) as f32 / 10.0;
    }*/
    let mut result = divisions as f32;// + remainder;

    if isNegative {
        result *= -1.0;
    }

    result
}

pub fn compare_gcode_line_vectors(list_a: Vec<GCode::GCode>, list_b: Vec<GCode::GCode>) -> bool {
    if list_a.len() != list_b.len() {
        return false;
    }

    for i in 0..list_a.len() {
        if list_a[i] != list_b[i] {
            return false;
        }
    }

    true
}


#[test]
fn FloatMod_Calculates() {

    let expected: f32 = 1.0;
    let actual: f32 = float_mod(59.8, 46.5);

    assert_eq!(expected, actual)
}

// Note: No test for negatives because the slicer should never get fed negative points 
//  from the pre-processor - Austin Haskell

#[test]
fn compare_gcode_line_vectors_Are_not_equal() {
    let list_a: Vec<GCode::GCode> = 
        vec![
            GCode::GCode {
                command : GCode::Word {
                    letter: 'Q',
                    value: 1
                },
                x : 14.5,
                y : 26.78,
                z : 0.0
            }, 
            GCode::GCode {
                command : GCode::Word {
                    letter: 'Q',
                    value: 1
                },
                x : 67.5,
                y : 29.78,
                z : 10.0
            }];

    let list_b: Vec<GCode::GCode> = 
        vec![
            GCode::GCode {
                command : GCode::Word {
                    letter: 'Q',
                    value: 1
                },
                x : 7.5,
                y : 26.78,
                z : 0.0
            }, 
            GCode::GCode {
                command : GCode::Word {
                    letter: 'G',
                    value: 1
                },
                x : 67.5,
                y : 29.78,
                z : 10.0
            }];

    assert_eq!(false, compare_gcode_line_vectors(list_a, list_b));
}

#[test]
fn compare_gcode_line_vectors_Are_equal() {
    let list_a: Vec<GCode::GCode> = 
        vec![
            GCode::GCode {
                command : GCode::Word {
                    letter: 'Q',
                    value: 1
                },
                x : 14.5,
                y : 26.78,
                z : 0.0
            }, 
            GCode::GCode {
                command : GCode::Word {
                    letter: 'Q',
                    value: 1
                },
                x : 67.5,
                y : 29.78,
                z : 10.0
            }];

    let list_b: Vec<GCode::GCode> = 
        vec![
            GCode::GCode {
                command : GCode::Word {
                    letter: 'Q',
                    value: 1
                },
                x : 14.5,
                y : 26.78,
                z : 0.0
            }, 
            GCode::GCode {
                command : GCode::Word {
                    letter: 'Q',
                    value: 1
                },
                x : 67.5,
                y : 29.78,
                z : 10.0
            }];

    assert_eq!(true, compare_gcode_line_vectors(list_a, list_b));
}