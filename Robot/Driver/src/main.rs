// Todo: Remove this eventually and fix these warnings, for now it's just annoying since this is just getting started. - Austin Haskell
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fs;

pub mod parser { pub mod gcode; }
pub mod translator { pub mod gcode_translator; }
pub mod robot;

use parser::gcode::*;
use translator::gcode_translator::*;
use robot::*;

fn main() {
    let print_area: (f32, f32) = (16.0, 16.0);
    let rawdata: String = fs::read_to_string("C:\\Users\\austi\\Desktop\\Rust Projects\\SVG_Art_Generator\\RustArt\\Robot\\Driver\\example-gcode\\test001.gcode")
        .expect("Unable to read the file");
    let raw_commands = GCode::deserialize(&rawdata);

    for command in &raw_commands {
        println!("{:?}", command);
    }

    let robot = Robot {
        acceleration: 1.0,
        current_quadrant: (0, 0),
        print_area: print_area
    };

    let commands = translator::gcode_translator::translate_to_internal_command_list(&raw_commands, print_area);
}
