// This is used to translate the GCode to an agnostic internal format. The reason for this
//  is so we can control the robot manually or through some other application without having
//  to generate GCode for those actions                                     - Austin Haskell

use crate::robot::Command;
use crate::parser::gcode::GCode;

// Todo: Optomize this and maybe the calling code. Could do some loop unrolling to make sure there's not a cache miss - Austin Haskell
// I'm worried this is going to be really slow with big files, maybe this translation should happen on a laptop rather than the raspberry pi zero
pub fn translate_to_internal_command_list(gcode_list: &Vec<GCode>, print_area: (f32, f32)) -> Vec<Command> {

    struct OrderedGCode {
        pub command: GCode,
        pub order: u32,         // We have to preserve order while splitting - Austin Haskell
        pub quadrant: (u16, u16)
    }

    // After some thought this will need to be able to split a gcode command across boundries.
    //  The robot is going to move to a "Quadrant" and then draw everything in that square. It
    //  will then move to the next section and continue. ( Note that corrections to position are
    //  going to be accomplished by Computer Vision, this is because we're using omniwheels to
    //  move the bot ). 
    // 
    // Ex: Draw everything in A and then move to B and draw everything there. 
    //  ---------- ----------
    // |          |          |
    // |     A    |     B    |
    // |          |          |
    //  ----------+----------
    // |          |          |
    // |     D    |     C    |
    // |          |          |
    //  ---------- ----------
    // Since the GCode commands are essentially just how much to move in a certain axis and if we
    //  know the size of the space we can draw without moving the bot then we should be able to break
    //  the commands into sections by splitting a long-moving command into two. -Austin Haskell

    let mut commands_that_cross_boundaries: Vec<OrderedGCode> = Vec::new();
    let mut commands_that_dont_cross: Vec<OrderedGCode> = Vec::new();

    let mut prev_code: Option<&GCode> = None;
    let mut order_num: u32 = 0;
    for code in gcode_list {
        
        // First iteration
        // Todo: This is going to skip adding the first command to the list
        if prev_code.is_none() {
            prev_code = Some(code);
            continue; 
        }

        if does_boundary_cross_occour((prev_code.unwrap(), code), print_area) {
            // Note: We're going to have to add in some pen lifts here, imagine if we're drawing a triangle
            //  and one of the tips of the triangle goes into another quadrant. A pen lift needs to be added
            //  so the triangle doesnt end up with a flat line at the top. - Austin Haskell
            for item in split_gcode_at_quadrant_line((prev_code.unwrap(), code), print_area) {
                commands_that_cross_boundaries.push(
                    OrderedGCode {
                        command: item.clone(),
                        order: order_num,
                        quadrant: calc_quadrant(&item, print_area)
                    }
                );
                order_num += 1;
            }

            commands_that_cross_boundaries.push(
                OrderedGCode {
                    command: code.clone(),
                    order: order_num,
                    quadrant: calc_quadrant(code, print_area)
                }
            );
        } else {
            commands_that_dont_cross.push (
                OrderedGCode {
                    command: code.clone(),
                    order: order_num,
                    quadrant: calc_quadrant(code, print_area)
                }
            );
        }

        order_num += 1;
        prev_code = Some(code);
    }

    let final_command_list: Vec<Command> = Vec::new();
    final_command_list
}

fn does_boundary_cross_occour(commands: (&GCode, &GCode), print_area: (f32, f32)) -> bool {
    calc_quadrant(commands.0, print_area) != calc_quadrant(commands.1, print_area)
}

// Note: This will get wierd with really small print areas - Austin Haskell
fn calc_quadrant(command: &GCode, print_area: (f32, f32)) -> (u16, u16) {
    ((command.x as f32 / print_area.0) as u16, (command.y as f32 / print_area.1) as u16)
}

fn split_gcode_at_quadrant_line(commands: (&GCode, &GCode), print_area: (f32, f32)) -> Vec<GCode> {
    
    let start_quad = calc_quadrant(commands.0, print_area);
    let end_quad = calc_quadrant(commands.1, print_area);

    let quad_distance = (diff((start_quad.0, end_quad.0)), diff((start_quad.1, end_quad.1)));

    if quad_distance == (0, 0) {
        return Vec::new(); // Nothing to do here - Austin Haskell
    }

    // This represents the lower left and upper right coordanate of each square quadrant - Austin Haskell
    let mut quad_bounds: Vec<((f32, f32), (f32, f32))> = Vec::new();

    let mut x_bound: u16 = 0;
    let mut y_bound: u16 = 0;
    while y_bound <= quad_distance.1 {
        while x_bound <= quad_distance.0 {

            quad_bounds.push((
                (print_area.0 * x_bound as f32, print_area.1 * y_bound as f32), 
                (print_area.0, 0.0)));

            x_bound += 1;
        }

        y_bound += 1;
    }
    

    //  ----- ----- -----
    // |  A  |  B  |  C  |
    // |     |     |     |
    //  -----+-----+-----
    // |  D  |  E  |  F  |
    // |     |     |     |
    //  -----+-----+-----
    // |  G  |  H  |  I  |
    // |     |     |     |
    //  ----- ----- -----
    // This calculation has to account for cases where we go from A to F, which would mean that we might
    //  need a split for A,B,E and F.                                                   - Austin Haskell

    let codes: Vec<GCode> = Vec::new();



    codes
}

// Nothing in the std has anything to do this for unsinged types ;( - Austin Haskell
fn diff(values: (u16, u16)) -> u16 {
    if values.0 > values.1 {
        return values.0 - values.1;
    }
    values.1 - values.0
}


// ----- Unit tests ----- 
#[test]
fn calc_quadrant_calculates() {
    let code = GCode {
        command: String::from("G1"),
        x: 100.0,
        y: 50.0,
        z: 0.0
    };

    assert_eq!((9, 4), calc_quadrant(&code, (10.5, 10.5)))
}

#[test]
fn boundary_cross_occours() {

    let code1 = GCode {
        command: String::from("G1"),
        x: 100.0,
        y: 50.0,
        z: 0.0
    };

    let code2 = GCode {
        command: String::from("G1"),
        x: 100.0,
        y: 55.0,
        z: 0.0
    };

    assert_eq!(true, does_boundary_cross_occour((&code1, &code2), (10.5, 10.5)))
}

#[test]
fn boundary_cross_does_not_occour() {

    let code1 = GCode {
        command: String::from("G1"),
        x: 100.0,
        y: 50.0,
        z: 0.0
    };

    let code2 = GCode {
        command: String::from("G1"),
        x: 100.0,
        y: 45.0,
        z: 0.0
    };

    assert_eq!(false, does_boundary_cross_occour((&code1, &code2), (10.5, 10.5)))
}

#[test]
fn split_gcode_across_multiple_boundaries() {
    assert_eq!(true, false)
}

#[test]
fn split_gcode_across_boundary() {
    assert_eq!(true, false)
}

