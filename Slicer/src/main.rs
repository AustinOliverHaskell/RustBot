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
mod Scalar;
use Scalar::*;
mod Postprocessor;
use Postprocessor::*;
mod parser { 
    pub mod svg; 
    pub mod svg_util;
    pub mod polygon;
    pub mod path;
    pub mod path_defs;
    pub mod SVGParser; 
    pub mod parser_defs; 
    pub mod regex_defs;
    pub mod rect;
    pub mod svg_commands {
        pub mod bezier;
        pub mod line;
        pub mod eliptical_arcs;
    }
}
use parser::*;
mod arguments;
use arguments::*;



fn main() {
    let possible_program_args = ProgramArgs::new();
    let program_args;
    match possible_program_args {
        None => {
            println!("No command line arguments specified and no config file found. Exiting. ");
            return;
        },
        Some(val) => program_args = val 
    }

    // 0 ---------- Dump Arguments ----------
    program_args.dump("run_config");

    println!("Running slicer with input file: {:?}", program_args.input_file);
    println!("Using printer dimentions of {:?}mm in the x direction and {:?}mm in the y direction", program_args.printbed_width, program_args.printbed_height);

    let svg_data = read_to_string(program_args.input_file);

    if svg_data.is_err() {
        println!("Unable to open input file, please check that input file is a valid SVG file. ");
        return;
    }
    print!("Parsing svg file... ");
    // 1 ---------- Parsing File ----------
    let possible_parser = SVGParser::SVGParser::load(svg_data.unwrap());
    let parser: SVGParser::SVGParser;
    match possible_parser {
        Ok(p) => parser = p,
        Err(msg) => {println!("{:?}", msg); return;}
    }
    println!("Done. ");

    let translator = Translator::Translator {
        printbed_width: program_args.printbed_width,
        printbed_height: program_args.printbed_height
    };

    // 2 ---------- Scale Points ----------
    let point_list = Scalar::scale_points(
        parser.shapes, 
        program_args.scaling_x, 
        program_args.scaling_y, 
        program_args.preserve_aspect);

    // 3 ---------- Slice ----------
    let mut quadrant_blocks: Vec<QuadrantBlock> = Vec::new();
    for list in point_list {
        quadrant_blocks.append(&mut translator.Line(list));
    }

    // 4 ---------- Optimize ----------
    let final_gcode = Postprocessor::postprocess(quadrant_blocks); 

    let output_file = File::create(program_args.output_file.clone());
    if output_file.is_err() {
        println!("Error: Unable to create/open output file. ");
    }

    let mut builder = string_builder::Builder::default();
    builder.append("w");
    builder.append(program_args.printbed_width.to_string());
    builder.append(" h");
    builder.append(program_args.printbed_height.to_string());
    builder.append('\n');
    for gcode in final_gcode {
        builder.append(gcode.Write());
        builder.append('\n');
    }

    let output = File::create(program_args.output_file.clone());
    if output.is_err() {
        println!("Error: Unable to create output file. Ensure that the output file name is valid and you have proper write permissions. ");
    }

    let file_write_status = output.unwrap().write_all(builder.string().unwrap().as_bytes());
    if file_write_status.is_err() {
        println!("Error: Was able to create file, but failed to write to it. Possible permissions issue? ");
    }

    println!("Slicing complete!");

}
