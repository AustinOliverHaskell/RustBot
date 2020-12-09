#![allow(non_snake_case)] 

mod GCode;
use GCode::*;

mod Util;
use Util::*;

mod Translator;
use Translator::*;

mod TranslatorUtil;
use TranslatorUtil::*;

use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::Event;

fn main() {
    /*for event in svg::open("..\\ArtGenerator\\image.svg").unwrap() {
        match event {
            Event::Tag(Path, _, attributes) => {
                let raw_data = attributes.get("d").unwrap();
                //svg::node::Node()
                //let data = Data::parse(raw_data).unwrap();

                //println!("{:?}", data);
            }
            _ => {}
        }
    }*/
    println!("Starting slicer");


    let machine = Translator::Translator {
        printbed_width: 16.0,
        printbed_height: 16.0
    };

    let mut points: Vec<(f32, f32)> = Vec::new();
    points.push((0.0, 0.0));
    points.push((32.2, 14.5));

    let mut single_boudary_points: Vec<(f32, f32)> = Vec::new();
    single_boudary_points.push((0.0, 0.0));
    single_boudary_points.push((45.03, 19.24));

    //let mut output = machine.Line(points, 1.0);
    //for item in output {
    //    println!("{:?}", item);
    //}
    println!("--------------------------------------------");
    let mut output = machine.Line(single_boudary_points, 1.0);
    for item in output {
        println!("{:?}", item);
    }
}
