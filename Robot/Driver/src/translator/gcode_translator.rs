// This is used to translate the GCode to an agnostic internal format. The reason for this
//  is so we can control the robot manually or through some other application without having
//  to generate GCode for those actions                                     - Austin Haskell

use crate::robot::Command;
use crate::parser::gcode::GCode;

pub fn translate_to_internal_command_list(gcode_list: &Vec<GCode>, print_area: (f32, f32)) -> Vec<Command> {

    struct OrderedGCode {
        command: GCode,
        order: u32         // We have to preserve order while splitting - Austin Haskell
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

        if prev_code.is_none() {
            prev_code = Some(code);
            continue; // First iteration
        }

        if does_boundary_cross_occour((prev_code.unwrap(), code), print_area) {
            commands_that_cross_boundaries.push(OrderedGCode {
                command: code.clone(),
                order: order_num
            });
        } else {
            commands_that_dont_cross.push (OrderedGCode {
                command: code.clone(),
                order: order_num
            });
        }

        order_num += 1;
        prev_code = Some(code);
    }

    let final_command_list: Vec<Command> = Vec::new();
    final_command_list
}

// Todo: Optomize this and maybe the calling code. Could do some loop unrolling to make sure there's not a cache miss - Austin Haskell
fn does_boundary_cross_occour(commands: (&GCode, &GCode), print_area: (f32, f32)) -> bool {
    calc_quadrant(commands.0, print_area) != calc_quadrant(commands.1, print_area)
}

// Note: This will get wierd with really small print areas - Austin Haskell
fn calc_quadrant(command: &GCode, print_area: (f32, f32)) -> (u16, u16) {
    ((command.x as f32 / print_area.0) as u16, (command.y as f32 / print_area.1) as u16)
}



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