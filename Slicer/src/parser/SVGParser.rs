use quick_xml::Reader;
use quick_xml::events::Event;

use crate::parser::svg;
use crate::parser::path;
use crate::parser::polygon;
use crate::parser::rect;
use crate::parser::parser_defs;

pub struct SVGParser {
    pub width: f32,
    pub height: f32,
    pub shapes: Vec<parser_defs::SVGShape>
}

impl SVGParser {
    pub fn load(xml: String) -> Result<Self, String> {
        let mut document = Reader::from_str(&xml);

        let mut viewbox_height: Option<f32> = None;
        let mut viewbox_width:  Option<f32> = None;
        let mut shapes: Vec<parser_defs::SVGShape> = Vec::new();
        let mut processing_buffer:  Vec<u8> = Vec::new();

        loop {
            match document.read_event(&mut processing_buffer) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"svg" => {
                            let view_box = svg::parse_svg_viewbox(e);
                            match view_box {
                                Ok(val) => {
                                    viewbox_width  = Some(val.0);
                                    viewbox_height = Some(val.1);
                                }
                                Err(e) => return Err(e)
                            }
                            println!("SVG Viewbox set to {:?},{:?}", viewbox_width, viewbox_height);

                        },
                        _ => {}
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"path" => {
                            let path = path::parse_svg_path(e);
                            match path {
                                Err(e) => return Err(e),
                                Ok(val) => shapes.push(val)
                            }
                        },
                        b"rect" => {
                            let rectangle = rect::parse_svg_rect(e);
                            match rectangle {
                                Err(e) => return Err(e),
                                Ok(val) => shapes.push(val)
                            }
                        },
                        b"polygon" => {
                            let polygon = polygon::parse_svg_polygon(e);
                            match polygon {
                                Err(e) => return Err(e),
                                Ok(val) => shapes.push(val)
                            }
                        }
                        _ => print!("Got unsupported tag {:?}", e.name())
                    }
                }
                Ok(Event::Text(ref e)) => {}
                Ok(Event::End(ref e)) => {},
                Ok(Event::Eof) => break,
                Err(e) => println!("Error: {:?}", e),
                _ => println!("Got unsupported tag")
            }

            processing_buffer.clear();
        }

        if viewbox_height.is_none() || viewbox_width.is_none() {
            return Result::Err(String::from("Error: No viewbox defined in SVG file. "));
        }

        Ok(SVGParser {
            width: viewbox_width.unwrap(),
            height: viewbox_height.unwrap(),
            shapes: shapes
        })
    }
}