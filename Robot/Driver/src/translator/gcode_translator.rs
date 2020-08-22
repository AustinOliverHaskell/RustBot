// This is used to translate the GCode to an agnostic internal format. The reason for this
//  is so we can control the robot manually or through some other application without having
//  to generate GCode for those actions                                     - Austin Haskell

use crate::robot::Command;
use crate::parser::gcode::GCode;

fn translate_to_internal_command_list(gcode_list: Vec<GCode>) -> Vec<Command> {

    let mut command_list: Vec<Command> = Vec::new();

    for code in gcode_list {
        let command: Command = Command {
            x: code.x,
            y: code.y,
            pen_lifted: code.z < 0.0
        };

        command_list = command_list.push(command);
    }

    command_list
}