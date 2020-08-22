// This is used to translate the GCode to an agnostic internal format. The reason for this
//  is so we can control the robot manually or through some other application without having
//  to generate GCode for those actions                                     - Austin Haskell

use crate::robot::Command;
use crate::parser::gcode::GCode;

pub fn translate_to_internal_command_list(gcode_list: &Vec<GCode>) -> Vec<Command> {

    let mut command_list: Vec<Command> = Vec::new();

    // This is super basic right now and unnessasary but in the future should we need to
    //  support more complicated actions this will come in handy        - Austin Haskell
    for code in gcode_list {
        let command: Command = Command {
            x: code.x,
            y: code.y,
            pen_lifted: code.z < 0.0
        };

        command_list.push(command);
    }

    command_list
}