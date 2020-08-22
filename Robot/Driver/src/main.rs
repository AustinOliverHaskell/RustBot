use std::fs;

pub mod parser { pub mod gcode; }
pub mod translator { pub mod gcode_translator; }
pub mod robot;

use parser::gcode::*;
use translator::gcode_translator::*;
use robot::*;

fn main() {
    let rawdata: String = fs::read_to_string("/home/austinhaskell/Documents/Rust/RustSVGArt/Robot/Driver/example-gcode/test001.gcode")
        .expect("Unable to read the file");
    let raw_commands = GCode::deserialize(&rawdata);

    for command in &raw_commands {
        println!("{:?}", command);
    }

    let robot = Robot::new(0.0, 100.0, 100.0);

    let commands = translator::gcode_translator::translate_to_internal_command_list(&raw_commands);
}
