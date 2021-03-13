use std::collections::HashMap;

use crate::quadrant::Quadrant;

#[derive(Copy, Clone, Debug)]
pub struct MachineSettings {
    pub printbed_width: f32,
    pub printbed_height: f32
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Clone, Debug)]
pub struct CommandGrouping {
    pub quadrant: Quadrant,
    pub points: Vec<Point>
}

#[derive(Clone, Debug)]
pub struct GCodeParser {
    pub settings: MachineSettings,
    pub commands: HashMap<Quadrant, Vec<CommandGrouping>>,
    pub smallest_quadrant: Quadrant, // @todo: Maybe make these refrences? - Austin Haskell
    pub largest_quadrant: Quadrant
}

const MOVE_ITEMS_PER_LINE:     usize = 4;
const QUADRANT_ITEMS_PER_LINE: usize = 3;

impl GCodeParser {
    pub fn load(data: String) -> Result<Self, String> {
        let lines: Vec<&str> = data.split('\n').collect();

        let mut command_map: HashMap<Quadrant, Vec<CommandGrouping>> = HashMap::new();
        let smallest_quadrant = Quadrant::Max();
        let largest_quadrant  = Quadrant::Min();

        let mut settings = MachineSettings {
            printbed_width: 0.0,
            printbed_height: 0.0
        };

        let mut current_quadrant: Option<Quadrant> = None;
        let mut current_points:   Vec<Point> = Vec::new();

        for line in lines {
            let command_char = line.chars().nth(0);
            if command_char.is_none() { continue; }

            let starting_char = command_char.unwrap();
            match starting_char {
                'Q' => {
                    let possible_quadrant = GCodeParser::parse_quadrant(line);
                
                    let quadrant: Quadrant;
                    match possible_quadrant {
                        Ok(quad) => quadrant = quad,
                        Err(err) => return Err(err)
                    }

                    if quadrant > largest_quadrant {
                        largest_quadrant = quadrant.clone();
                    }
                    if quadrant < smallest_quadrant {
                        smallest_quadrant = quadrant.clone();
                    }

                    if !command_map.contains_key(&current_quadrant.unwrap()) {
                        command_map.insert(current_quadrant.unwrap(), Vec::new());
                    } 

                    command_map[&current_quadrant.unwrap()].push(
                        CommandGrouping {
                            quadrant: current_quadrant.unwrap(),
                            points:   current_points.clone()
                    });

                    current_points.clear();
                    current_quadrant = Some(quadrant);
                },
                'G' => {
                    if current_quadrant.is_none() {
                        return Err(String::from("Malformed file: Points came before quadrant"));
                    }
    
                    let possible_point = GCodeParser::parse_point(line);
                    let point: Point;
                    match possible_point {
                        Ok(p)    => point = p,
                        Err(err) => return Err(err)
                    }
    
                    if point.x > settings.printbed_width || point.y > settings.printbed_height {
                        return Err(String::from("Malformed file: Quadrant contains point outside of range of printbed. "));
                    }
    
                    current_points.push(point);
                },
                'w' => {
                    let printbed_size = GCodeParser::parse_printbed_size(line);
                    settings.printbed_width  = printbed_size.0;
                    settings.printbed_height = printbed_size.1;
                },
                _ => {
                    println!("Got unknown gcode command");
                }
            }
        }

        if current_quadrant.is_some() {
            if !command_map.contains_key(&current_quadrant.unwrap()) {
                command_map.insert(current_quadrant.unwrap(), Vec::new());
            } 

            command_map[&current_quadrant.unwrap()].push(
                CommandGrouping {
                    quadrant: current_quadrant.unwrap(),
                    points: current_points.clone()
            });
        }

        Ok(GCodeParser {
            settings:          settings,
            commands:          command_map,
            smallest_quadrant: smallest_quadrant,
            largest_quadrant:  largest_quadrant
        })
    }

    // Parses a line matching: w32.5 h55.6
    fn parse_printbed_size(line: &str) -> (f32, f32) {
        let mut retval: (f32, f32) = (0.0, 0.0);

        let sizes: Vec<&str> = line.split(' ').collect();
        for size in sizes {
            let first_char = size.chars().nth(0);

            if first_char.is_none() {
                return (0.0, 0.0);
            }

            if first_char.unwrap() == 'w' {
                retval.0 = size[1..].parse::<f32>().unwrap_or(0.0);
            }
            else if first_char.unwrap() == 'h' {
                retval.1 = size[1..].parse::<f32>().unwrap_or(0.0);
            }
        }

        retval
    }

    // Parses a line matching: Q1 12 40 
    fn parse_quadrant(line: &str) -> Result<Quadrant, String> {
        let items: Vec<&str> = line.split(' ').collect();

        if items.len() < QUADRANT_ITEMS_PER_LINE {
            return Err(String::from("Malformed File: Quadrant does not have enough data"));
        }

        let possible_x: Result<u32, _> = items[1].parse();
        let possible_y: Result<u32, _> = items[2].parse();

        let x: u32;
        let y: u32;
        match possible_x {
            Ok(val) => x = val,
            _       => return Err(String::from("Malformed X value in quadrant"))
        }
        match possible_y {
            Ok(val) => y = val,
            _ => return Err(String::from("Malformed Y value in quadrant"))
        }

        Ok(Quadrant {
            x: x,
            y: y
        })
    }

    // Parses a line matching: G1 10 10 -1
    fn parse_point(line: &str) -> Result<Point, String> {
        let items: Vec<&str> = line.split(' ').collect();

        if items.len() < MOVE_ITEMS_PER_LINE {
            return Err(String::from("Malformed File: Quadrant does not have enough data"));
        }

        let possible_x: Result<f32, _> = items[1].parse();
        let possible_y: Result<f32, _> = items[2].parse();
        let possible_z: Result<f32, _> = items[3].parse();

        let x: f32;
        let y: f32;
        let z: f32;
        match possible_x {
            Ok(val) => x = val,
            _       => return Err(String::from("Malformed X value in point"))
        }
        match possible_y {
            Ok(val) => y = val,
            _       => return Err(String::from("Malformed Y value in point"))
        }
        match possible_z {
            Ok(val) => z = val,
            _       => return Err(String::from("Malformed z value in point"))
        }

        Ok(Point {
            x: x,
            y: y,
            z: z
        })
    }
}