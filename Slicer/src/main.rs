#![allow(non_snake_case)] 

mod GCode;
use GCode::*;

mod Machine;
use Machine::*;

use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::Event;

fn main() {
    for event in svg::open("..\\ArtGenerator\\image.svg").unwrap() {
        match event {
            Event::Tag(Path, _, attributes) => {
                let raw_data = attributes.get("d").unwrap();
                //svg::node::Node()
                //let data = Data::parse(raw_data).unwrap();

                //println!("{:?}", data);
            }
            _ => {}
        }
    }
    
    println!("Hello, world!");
}
