use std::env;
use std::fs::*;
use clap::*;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgramArgs {
    pub input_file: String,
    pub output_file: String,
    pub pixel_size: u32,
    pub thread_count: u8,
    pub create_joined: bool,
    pub outline_quadrants: bool,
    pub render_all_points: bool
}

impl ProgramArgs {
    pub fn new() -> Option<Self> {

        if env::args().len() == 1 {
            let raw_config = read_to_string("config");
            let config: ProgramArgs; 
            match raw_config {
                Ok(val) => config = serde_json::from_str(&val).unwrap(),
                _ => return None
            }

            return Some(config);

        } 
        else {
            let run_arguments = App::new("GCode Rendering")
                .version("0.1.0")
                .author("Austin Haskell")
                .about("Debugging tool to view what a given gcode snippet will look like")
                .arg(Arg::with_name("file")
                    .short("f")
                    .long("file")
                    .takes_value(true)
                    .required(true)
                    .help("File to generate images from"))
                .arg(Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .takes_value(true)
                    .required(true)
                    .help("Base output filename, this filename will have the quadrant rendered appended"))
                .arg(Arg::with_name("pixels_per_mm")
                    .short("p")
                    .long("pixel")
                    .takes_value(true)
                    .required(true)
                    .help("Number of pixels to use per mm, default is 1"))
                .arg(Arg::with_name("thread_count")
                    .short("t")
                    .long("threads")
                    .takes_value(true)
                    .required(true)
                    .help("Number of threads to use to speed up rendering"))
                .arg(Arg::with_name("create_joined_image")
                    .short("j")
                    .long("joined")
                    .takes_value(false)
                    .help("Additionally create joined image that is combined result of all quadrant images"))
                .arg(Arg::with_name("outline_quadrants_in_master")
                    .short("l")
                    .long("outline")
                    .takes_value(false)
                    .help("Used in conjunction with the -j command, outlines each quadrant in red. Ignored if -j is not defined. "))
                .arg(Arg::with_name("always_render_points")
                    .short("r")
                    .long("points")
                    .takes_value(false)
                    .help("If present, will render points that have no connections. "))
                .get_matches();

            let input_file: &str        = run_arguments.value_of("file").unwrap_or("input.gcode");
            let output_base: &str       = run_arguments.value_of("output").unwrap_or("quadrant");
            let pixel_scalar: u32       = run_arguments.value_of("pixels_per_mm").unwrap_or("1").parse().unwrap_or(1);
            let thread_count: u8        = run_arguments.value_of("thread_count").unwrap_or("4").parse().unwrap_or(4);
            let create_joined: bool     = run_arguments.is_present("create_joined_image");
            let outline_quadrants: bool = run_arguments.is_present("outline_quadrants_in_master");
            let render_all_points: bool = run_arguments.is_present("always_render_points");

            Some(
                ProgramArgs {
                    input_file: String::from(input_file),
                    output_file: String::from(output_base),
                    pixel_size: pixel_scalar,
                    thread_count: thread_count,
                    create_joined: create_joined,
                    outline_quadrants: outline_quadrants,
                    render_all_points: render_all_points
                }
            )
        }
    }

    pub fn dump(&self, path: &str) {

        let arg_dump = serde_json::to_string(&self);
        match arg_dump {
            Err(e) => {
                println!("Error: Failed to dump arguments to file. {:?}", e);
                return;
            },
            _ => { }
        }

        let raw_arg_file = File::create(path);
        let mut arg_file;
        match raw_arg_file {
            Err(e) => {
                println!("Failed to create/open argument file: {:?}:{:?}", path, e);
                return;
            },
            Ok(val) => arg_file = val
        }
        arg_file.write_all(arg_dump.unwrap().as_bytes()).unwrap();
    }
}