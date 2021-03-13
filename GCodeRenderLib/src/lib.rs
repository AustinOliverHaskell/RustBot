mod gcode_parser;
mod image_data;
mod defs;
mod quadrant;

use gcode_parser::*;
use image_data::*;
use defs::Color;
use quadrant::*;


pub fn render_quadrant(parser: &GCodeParser, quad: Quadrant, settings: &RenderSettings) -> ImageData {

    let command_group = parser.commands[&quad];

    ImageData::create(&command_group[0], &parser.settings, &settings)
}

pub fn render_all_quadrants(parser: &GCodeParser, settings: &RenderSettings) -> Vec<ImageData> {

    let mut rendered_quadrants: Vec<ImageData> = Vec::new();
    for quad in parser.commands {
        rendered_quadrants.push(render_quadrant(parser, quad.0, &settings));
    }

    rendered_quadrants
}

// @todo: Figure out if it's an issue that create_master_image always uses the parsers largest and smallest quads. If someone was using this lib and they
//  just rendered two quads and passed a vector that they made then the image size would possibly be bigger than what they passed. - Austin Haskell
// @todo: Add background color as a param. This will need some extra refactoring. - Austin Haskell
pub fn create_master_image(parser: &GCodeParser, rendered_quadrants: &Vec<ImageData>, settings: &RenderSettings) -> ImageData {
    ImageData::create_master(rendered_quadrants, settings, &parser.largest_quadrant, &parser.smallest_quadrant)
}
