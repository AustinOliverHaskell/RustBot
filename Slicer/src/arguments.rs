use clap::{Arg, App};

#[derive(Clone, Debug)]
pub struct ProgramArgs {
    pub input_file: String,
    pub output_file: String,
    pub printbed_width: f32,
    pub printbed_height: f32,
    pub scaling_x: f32,
    pub scaling_y: f32,
    pub preserve_aspect: bool
}

impl ProgramArgs {
    pub fn new() -> Self {
    let arguments = App::new("SVG Slicer")
            .version("0.1.0")
            .author("Austin Haskell")
            .about("SVG Slicer designed to handle large files. Seperates files into quadrants based on printbed size")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .help("File to slice"))
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Filename of output gcode. "))
            .arg(Arg::with_name("width")
                .short("w")
                .long("width")
                .required(true)
                .takes_value(true)
                .help("Width (in mm) of printbed."))
            .arg(Arg::with_name("height")
                .short("h")
                .long("height")
                .required(true)
                .takes_value(true)
                .help("Height (in mm) of printbed."))
            .arg(Arg::with_name("x_scaling")
                .short("x")
                .long("xscale")
                .required(true)
                .takes_value(true)
                .help("Scale svg file to fit x meters in the x direction. "))
            .arg(Arg::with_name("y_scaling")
                .short("y")
                .long("yscale")
                .takes_value(true)
                .help("Scale svg file to fit y meters in the x direction. "))
            .arg(Arg::with_name("preserve_aspect")
                .short("a")
                .long("aspect")
                .takes_value(false)
                .help("If present, will scale to the x scaling but will not honor y scaling to preserve aspect ratio in original file. Defaults to false"))
            .get_matches();

        let input_filename  = arguments.value_of("file").unwrap_or("input.svg");
        let output_filename = arguments.value_of("output").unwrap_or("output.gcode");

        let printbed_width: f32  = arguments.value_of("width").unwrap_or("16").parse().unwrap_or(16.0);
        let printbed_height: f32 = arguments.value_of("height").unwrap_or("16").parse().unwrap_or(16.0);

        let scale_x: f32 = arguments.value_of("x_scaling").unwrap_or("1").parse().unwrap_or(1.0);
        let scale_y: f32 = arguments.value_of("y_scaling").unwrap_or("1").parse().unwrap_or(1.0);

        let preserve_aspect_ratio: bool = arguments.is_present("preserve_aspect");

        ProgramArgs {
            input_file: String::from(input_filename),
            output_file: String::from(output_filename),
            printbed_width: printbed_width,
            printbed_height: printbed_height,
            scaling_x: scale_x,
            scaling_y: scale_y,
            preserve_aspect: preserve_aspect_ratio
        }
    }
}