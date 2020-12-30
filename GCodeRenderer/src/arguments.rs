use clap::{Arg, App};

#[derive(Clone, Debug)]
pub struct ProgramArgs {
    pub input_file: String,
    pub output_file: String,
    pub pixel_size: u32,
    pub thread_count: u8,
    pub create_joined: bool 
}

impl ProgramArgs {
    pub fn new() -> Self {
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
            .get_matches();

        let input_file: &str    = run_arguments.value_of("file").unwrap_or("input.gcode");
        let output_base: &str   = run_arguments.value_of("output").unwrap_or("quadrant");
        let pixel_scalar: u32   = run_arguments.value_of("pixels_per_mm").unwrap_or("1").parse().unwrap_or(1);
        let thread_count: u8    = run_arguments.value_of("thread_count").unwrap_or("4").parse().unwrap_or(4);
        let create_joined: bool = run_arguments.is_present("create_joined_image");

        ProgramArgs {
            input_file: String::from(input_file),
            output_file: String::from(output_base),
            pixel_size: pixel_scalar,
            thread_count: thread_count,
            create_joined: create_joined
        }
    }
}