#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use clap::{Arg, App};
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png::*;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

mod file_loader;
use file_loader::*;

mod rasterizer;
use rasterizer::*;

fn main() {
    let arguments = App::new("GCode Rendering")
        .version("0.1.0")
        .author("Austin Haskell")
        .about("Debugging tool to view what a given gcode snippet will look like")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("File to generate images from"))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .help("Base output filename, this filename will have the quadrant rendered appended"))
        .arg(Arg::with_name("pixels_per_mm")
            .short("p")
            .long("pixel")
            .takes_value(true)
            .help("Number of pixels to use per mm, default is 1"))
        .get_matches();

    let input_file = arguments.value_of("file").unwrap_or("input.gcode");
    let output_base = arguments.value_of("output").unwrap_or("quadrant");
    let pixel_scalar: i32 = arguments.value_of("pixels_per_mm").unwrap_or("1").parse().unwrap_or(1);

    println!("Running gcode rendering on input file: {:?} and base output file name of {:?}", input_file, output_base);

    println!("Loading gcode file... ");
    let loader_result: Result<FileLoader, String>;
    let raw_data = fs::read_to_string(input_file);
    match raw_data {
        Ok(data) => loader_result = FileLoader::load(data),
        _ => { println!("Error: Input file must point to a valid file. "); return }
    }

    let loader: FileLoader;
    match loader_result {
        Ok(fl) => loader = fl,
        Err(e) => { println!("Error: Input file was invalid ->{:?}", e); return;}
    }
    println!("Complete!");

    println!("Rendering with a width of {:?}mm and a height of {:?}mm. Using {:?} pixels per mm. ", 
        loader.settings.printbed_width,
        loader.settings.printbed_height,
        pixel_scalar);

    let progress_bar = ProgressBar::new(loader.commands.len() as u64);
    progress_bar.set_style(ProgressStyle::default_bar().progress_chars("#>-"));

    //TODO: Make this multithreaded. - Austin Haskell
    for command in loader.commands {
        let img = Rasterizer::create(
            &command, 
            loader.settings.printbed_width as i32, 
            loader.settings.printbed_height as i32, 
            pixel_scalar);

        let filename = String::from(output_base) + &command.quadrant.x.to_string() + &String::from("_") + &command.quadrant.y.to_string() + &String::from(".png");
        let path = Path::new(&filename);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, img.width as u32, img.height as u32); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        
        writer.write_image_data(&img.data).unwrap(); // Save

        progress_bar.inc(1);
    }
    progress_bar.finish();

    println!("Completed rendering. ");

}
