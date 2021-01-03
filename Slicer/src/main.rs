#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs::*;
use std::io::Write;
use clap::*;    
use indicatif::{ProgressBar, ProgressStyle};
use string_builder::Builder;

mod GCode;
use GCode::*;

mod Util;
use Util::*;

mod Translator;
use Translator::*;

mod TranslatorUtil;
use TranslatorUtil::*;

mod parser { 
    pub mod svg; 
    pub mod path;
    pub mod SVGParser; 
    pub mod parser_defs; 
}
use parser::*;

mod Scalar;
use Scalar::*;

mod Postprocessor;
use Postprocessor::*;

fn main() {
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
            .takes_value(true)
            .help("If true, will scale to the x scaling but will not honor y scaling to preserve aspect ratio in original file. Defaults to false"))
        .get_matches();

    let input_filename  = arguments.value_of("file").unwrap_or("input.svg");
    let output_filename = arguments.value_of("output").unwrap_or("output.gcode");

    let printbed_width: f32  = arguments.value_of("width").unwrap_or("16").parse().unwrap_or(16.0);
    let printbed_height: f32 = arguments.value_of("height").unwrap_or("16").parse().unwrap_or(16.0);

    let scale_x: f32 = arguments.value_of("x_scaling").unwrap_or("1").parse().unwrap_or(1.0);
    let scale_y: f32 = arguments.value_of("y_scaling").unwrap_or("1").parse().unwrap_or(1.0);

    let preserve_aspect_ratio: bool = arguments.value_of("preserve_aspect").unwrap_or("false").parse().unwrap_or(false);

    println!("Running slicer with input file: {:?}", input_filename);
    println!("Using printer dimentions of {:?}mm in the x direction and {:?}mm in the y direction", printbed_width, printbed_height);

    let svg_data = read_to_string(input_filename);

    if svg_data.is_err() {
        println!("Unable to open input file, please check that input file is a valid SVG file. ");
        return;
    }
    print!("Parsing svg file... ");
    let possible_parser = SVGParser::SVGParser::load(svg_data.unwrap());
    let parser: SVGParser::SVGParser;
    match possible_parser {
        Ok(p) => parser = p,
        Err(msg) => {println!("{:?}", msg); return;}
    }
    println!("Done. ");

    let translator = Translator::Translator {
        printbed_width: printbed_width,
        printbed_height: printbed_height
    };

    let point_list = Scalar::scale_points(parser.shapes, 1.0, 1.0);

    let mut quadrant_blocks: Vec<QuadrantBlock> = Vec::new();
    for list in point_list {
        quadrant_blocks.append(&mut translator.Line(list));
    }

    let final_gcode = Postprocessor::postprocess(quadrant_blocks); 

    let output_file = File::create(output_filename);
    if output_file.is_err() {
        println!("Error: Unable to create/open output file. ");
    }

    let mut builder = string_builder::Builder::default();
    builder.append("w");
    builder.append(printbed_width.to_string());
    builder.append(" h");
    builder.append(printbed_height.to_string());
    builder.append('\n');
    for gcode in final_gcode {
        builder.append(gcode.Write());
        builder.append('\n');
    }

    let output = File::create(output_filename);
    if output.is_err() {
        println!("Error: Unable to create output file. Ensure that the output file name is valid and you have proper write permissions. ");
    }

    let file_write_status = output.unwrap().write_all(builder.string().unwrap().as_bytes());
    if file_write_status.is_err() {
        println!("Error: Was able to create file, but failed to write to it. Possible permissions issue? ");
    }

    println!("Slicing complete!");

}
