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

mod file_loader;
use file_loader::*;

mod rasterizer;
use rasterizer::*;

mod arguments;
use arguments::*;

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

    //TODO: Make this multithreaded. - Austin Haskell
    let printbed_width  = loader.settings.printbed_width as i32;
    let printbed_height = loader.settings.printbed_height as i32;
    let pixel_scaling = program_args.pixel_size as i32;

    let workers = threadpool::ThreadPool::new(program_args.thread_count as usize);

    let command_len = loader.commands.len();
    for command in loader.commands {
        let output_base = program_args.output_file.clone();

        workers.execute( move || {
            let img = Rasterizer::create(
                &command, 
                printbed_width, 
                printbed_height, 
                pixel_scaling);
    
            let mut filename_builder = string_builder::Builder::default();
            filename_builder.append(output_base);
            filename_builder.append(command.quadrant.x.to_string());
            filename_builder.append("_");
            filename_builder.append(command.quadrant.y.to_string());
            filename_builder.append(".png");

            let filename = filename_builder.string().unwrap();
            let file = File::create(Path::new(&filename)).unwrap();
            let ref mut buffered_writer = BufWriter::new(file);
    
            let mut encoder = png::Encoder::new(buffered_writer, img.width as u32, img.height as u32);
            encoder.set_color(png::ColorType::RGB);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            
            writer.write_image_data(&img.data).unwrap(); // Save
        });
    }

    // Note: I dont know if this could break. As in, I dont know if theres a scenario 
    //  where the active workers is 0 but there is still queued jobs. - Austin Haskell
    while workers.active_count() != 0 {
        progress_bar.set_position((command_len - workers.queued_count()) as u64);
    }

    workers.join();

    progress_bar.finish();

    println!("Completed rendering. ");

}
