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

struct RenderedQuadrant {
    id: (i32, i32),
    image_data: Vec<u8>
}

fn main() {
    
    let program_args = ProgramArgs::new();

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
                pixel_scaling);
    
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
    // Note: I dont know if this could break. As in, I dont know if theres a scenario 
    //  where the active workers is 0 but there is still queued jobs. - Austin Haskell
    while master_image_data.len() != command_len {

        let master_img_quadrant = rx.try_recv();
        if master_img_quadrant.is_ok() {
            master_image_data.push(master_img_quadrant.unwrap());
        }

        progress_bar.set_position(master_image_data.len() as u64);
    }

    workers.join();
    progress_bar.finish();

    if program_args.create_joined {
        println!("Creating master image from quadrants... ");
        create_joined_image(master_image_data, printbed_width, printbed_height, program_args.pixel_size as i32);
        println!("Done.");
    }

    println!("Completed rendering. ");

}

fn create_joined_image(rendered_quadrants: Vec<RenderedQuadrant>, quadrant_img_width: i32, quadrant_img_height: i32, scale: i32) {
    
    let mut largest_quadrants: (i32, i32) = (0, 0);
    for quadrant in &rendered_quadrants {
        if quadrant.id.0 > largest_quadrants.0 {
            largest_quadrants.0 = quadrant.id.0;
        }
        if quadrant.id.1 > largest_quadrants.1 {
            largest_quadrants.1 = quadrant.id.1;
        }
    }

    let image_width: usize  = ((largest_quadrants.0 + 1) * quadrant_img_width  * scale) as usize;
    let image_height: usize = ((largest_quadrants.1 + 1) * quadrant_img_height * scale) as usize;
    let mut master_image: Vec<u8> = vec![100; image_height * image_width * 3];

    // TODO: Clean this up, it's pretty unreadable. 
    // The problem I was trying to solve was mapping each of the image data chunks to the master image's 
    //  data. They're both single dimentional array's representing two dimentional data so there's a lot 
    //  of offsets and shifting going on. Also the quadrant 0,0 goes in the bottom left and not the top 
    //  left, so there's also a flip of the quadrants y value. 
    for quadrant in rendered_quadrants {
        let quadrant_width = quadrant_img_width * scale * 3;
        let quadrant_height = quadrant_img_width * scale;

        let flipped_y = (quadrant.id.1 - largest_quadrants.1).abs();
        let mut line_pre = flipped_y as usize * image_width * 3 * quadrant_height as usize 
            + quadrant.id.0 as usize * quadrant_width as usize; 

        for y in 0..quadrant_height {
            for x in 0..quadrant_width {
                let pixel_coord: usize = (y * quadrant_width + x) as usize; 

                master_image[line_pre + x as usize] = quadrant.image_data[pixel_coord];
            }
            line_pre += image_width * 3;
        }
    }

    write_png("master_img.png", 
        image_width,
        image_height,
        master_image
    ).unwrap();
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