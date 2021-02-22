#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::borrow::Cow;
use png::*;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use threadpool::ThreadPool;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

mod file_loader;
use file_loader::*;

mod rasterizer;
use rasterizer::*;

mod arguments;
use arguments::*;

mod defs;
use defs::RenderedQuadrant;

mod master_img;
use master_img::*;

fn main() {
    
    let possible_program_args = ProgramArgs::new();
    let program_args;
    match possible_program_args {
        Some(val) => program_args = val,
        None => { println!("No command line arguments given and no config file found. Exiting. "); return; }
    }

    program_args.dump("run_config");

    println!("Running gcode rendering on input file: {:?} and base output file name of {:?}", program_args.input_file, program_args.output_file);
    println!("Using a threadpool with {:?} workers. ", program_args.thread_count);

    println!("Loading gcode file... ");
    let loader_result: Result<FileLoader, String>;
    let raw_data = fs::read_to_string(program_args.input_file);
    match raw_data {
        Ok(data) => loader_result = FileLoader::load(data),
        _ => { println!("Error: Input file must point to a valid file. "); return }
    }

    let loader: FileLoader;
    match loader_result {
        Ok(fl) => loader = fl,
        Err(e) => { println!("Error: Input file was invalid ->{:?}", e); return;}
    }

    println!("Rendering with a width of {:?}mm and a height of {:?}mm. Using {:?} pixels per mm. ", 
        loader.settings.printbed_width,
        loader.settings.printbed_height,
        program_args.pixel_size);

    let progress_bar = ProgressBar::new(loader.commands.len() as u64);
    progress_bar.set_style(ProgressStyle::default_bar().progress_chars("#>-"));

    let printbed_width  = loader.settings.printbed_width as i32;
    let printbed_height = loader.settings.printbed_height as i32;
    let pixel_scaling = program_args.pixel_size as i32;
    let draw_points = program_args.render_all_points;

    let workers = threadpool::ThreadPool::new(program_args.thread_count as usize);

    let command_len = loader.commands.len();
    let (tx, rx): (Sender<RenderedQuadrant>, Receiver<RenderedQuadrant>) = mpsc::channel();
    for command in loader.commands {
        let output_base = program_args.output_file.clone();

        let transmitter = tx.clone();
        workers.execute( move || {
            let img = Rasterizer::create(
                &command, 
                printbed_width, 
                printbed_height, 
                pixel_scaling,
                draw_points);
    
            let quad_data = RenderedQuadrant {
                id: (command.quadrant.x, command.quadrant.y),
                image_data: img.data.clone()
            };
            
            let mut filename_builder = string_builder::Builder::default();
            filename_builder.append(output_base);
            filename_builder.append(command.quadrant.x.to_string());
            filename_builder.append("_");
            filename_builder.append(command.quadrant.y.to_string());
            filename_builder.append(".png");

            write_png(&filename_builder.string().unwrap(), img.width, img.height, img.data).unwrap();

            if transmitter.send(quad_data).is_err() {}
        });
    }

    let mut master_image_data: Vec<RenderedQuadrant> = Vec::new();

    let mut largest_quadrant: (i32, i32) = (0, 0);
    // Note: I dont know if this could break. As in, I dont know if theres a scenario 
    //  where the active workers is 0 but there is still queued jobs. - Austin Haskell
    while master_image_data.len() != command_len {

        let master_img_quadrant = rx.try_recv();
        if master_img_quadrant.is_ok() {

            let image_data = master_img_quadrant.unwrap();
            let id = image_data.id.clone();

            if id.0 > largest_quadrant.0 {
                largest_quadrant.0 = id.0;
            }
            if id.1 > largest_quadrant.1 {
                largest_quadrant.1 = id.1;
            }

            master_image_data.push(image_data);
        }

        progress_bar.set_position(master_image_data.len() as u64);
    }

    workers.join();
    progress_bar.finish();

    if program_args.create_joined {
        println!("Creating master image from quadrants... ");
        //create_joined_image(master_image_data, printbed_width, printbed_height, program_args.pixel_size as i32, program_args.outline_quadrants);
        let mut master_img = MasterImage::new(
            (printbed_width as u32 * program_args.pixel_size) as usize, 
            (printbed_height as u32 * program_args.pixel_size) as usize,
            largest_quadrant);

        for quad in master_image_data {
            master_img.add(quad);
        }

        if program_args.outline_quadrants {
            master_img.outline(255, 0, 0);
        }

        let mut filename_builder = string_builder::Builder::default();
            filename_builder.append(program_args.output_file);
            filename_builder.append("_");
            filename_builder.append("master.png");

        write_png(&filename_builder.string().unwrap(), 
            master_img.image_width,
            master_img.image_height,
            master_img.data
        ).unwrap();

        println!("Done.");
    }

    println!("Completed rendering. ");

}

// TODO: Figure out if the library is causing the slowdown. Seems to me like the write takes wayyyy too long for how much I'm writing.
//  maybe use the native library calls and make the header myself? - Austin Haskell
fn write_png(filename: &str, width: usize, height: usize, img_data: Vec<u8>) -> Result<(), &str> {
    let file = File::create(Path::new(filename));
    if file.is_err() {
        return Err("Failed to create file. Is the path valid?");
    }

    let ref mut buffered_writer = BufWriter::new(file.unwrap());

    let mut encoder = png::Encoder::new(buffered_writer, width as u32, height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);

    let writer = encoder.write_header();
    if writer.is_err() {
        return Err("Failed to encode PNG header. ");
    }
    
    match writer.unwrap().write_image_data(&img_data) {
        Err(e) => println!("{:?}", e),
        _ => {},
    }

    Ok(())
}