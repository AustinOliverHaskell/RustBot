use std::fs;

pub mod parser { pub mod gcode; }
use parser::gcode::*;

fn main() {
    let rawdata: String = fs::read_to_string("C:\\Users\\austi\\Desktop\\Rust Projects\\SVG_Art_Generator\\RustArt\\Robot\\Driver\\example-gcode\\test001.gcode")
        .expect("Unable to read the file");
    let commands = GCode::deserialize(&rawdata);

    for command in commands {
        println!("{:?}", command);
    }
}
