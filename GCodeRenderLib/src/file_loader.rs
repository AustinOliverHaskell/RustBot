use indicatif::ProgressBar;
use indicatif::ProgressStyle;

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

#[derive(Copy, Clone, Debug)]
pub struct Quadrant {
    pub x: i32,
    pub y: i32
}

#[derive(Clone, Debug)]
pub struct CommandGrouping {
    pub quadrant: Quadrant,
    pub points: Vec<Point>
}

#[derive(Clone, Debug)]
pub struct FileLoader {
    pub settings: MachineSettings,
    pub commands: Vec<CommandGrouping>
}

impl FileLoader {
    pub fn load(data: String) -> Result<Self, String> {
        // TODO: Make this windows compatable - Austin Haskell
        let lines: Vec<&str> = data.split('\n').collect();

        let progress_bar = ProgressBar::new(lines.len() as u64);
        progress_bar.set_style(ProgressStyle::default_bar().progress_chars("#>-"));

        let mut command_list: Vec<CommandGrouping> = Vec::new();
        let mut settings = MachineSettings {
            printbed_width: 0.0,
            printbed_height: 0.0
        };

        let mut current_quadrant: Option<Quadrant> = None;
        let mut current_points: Vec<Point> = Vec::new();
        for line in lines {
            progress_bar.inc(1);
            let command_char = line.chars().nth(0);
            if command_char.is_some() {
                let starting_char = command_char.unwrap();

                if starting_char == 'Q' {
                    let possible_quadrant = FileLoader::parse_quadrant(line);
                    
                    let quadrant: Quadrant;
                    match possible_quadrant {
                        Ok(quad) => quadrant = quad,
                        Err(err) => return Err(err)
                    }

                    if current_quadrant.is_some() {
                        command_list.push(CommandGrouping {
                            quadrant: current_quadrant.unwrap(),
                            points: current_points.clone()
                        });

                        current_points.clear();
                    }

                    current_quadrant = Some(quadrant);
                }

                if starting_char == 'G' {
                    if current_quadrant.is_none() {
                        return Err(String::from("Malformed file: Points came before quadrant"));
                    }

                    let possible_point = FileLoader::parse_point(line);
                    let point: Point;
                    match possible_point {
                        Ok(p) => point = p,
                        Err(err) => return Err(err)
                    }

                    if point.x > settings.printbed_width || point.y > settings.printbed_height {
                        return Err(String::from("Malformed file: Quadrant contains point outside of range of printbed. "));
                    }

                    current_points.push(point);
                }

                if starting_char == 'w' {
                    let printbed_size = FileLoader::parse_printbed_size(line);
                    settings.printbed_width  = printbed_size.0;
                    settings.printbed_height = printbed_size.1;
                }
            }
        }
        progress_bar.finish();

        if current_quadrant.is_some() {
            command_list.push(CommandGrouping {
                quadrant: current_quadrant.unwrap(),
                points: current_points.clone()
            });
        }

        Ok(FileLoader {
            settings: settings,
            commands: command_list
        })
    }

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

    fn parse_quadrant(line: &str) -> Result<Quadrant, String> {
        let items: Vec<&str> = line.split(' ').collect();

        if items.len() < 3 {
            return Err(String::from("Malformed File: Quadrant does not have enough data"));
        }

        let possible_x: Result<i32, _> = items[1].parse();
        let possible_y: Result<i32, _> = items[2].parse();

        let x: i32;
        let y: i32;
        match possible_x {
            Ok(val) => x = val,
            _ => return Err(String::from("Malformed X value in quadrant"))
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

    fn parse_point(line: &str) -> Result<Point, String> {
        let items: Vec<&str> = line.split(' ').collect();

        if items.len() < 4 {
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
            _ => return Err(String::from("Malformed X value in point"))
        }
        match possible_y {
            Ok(val) => y = val,
            _ => return Err(String::from("Malformed Y value in point"))
        }
        match possible_z {
            Ok(val) => z = val,
            _ => return Err(String::from("Malformed z value in point"))
        }

        Ok(Point {
            x: x,
            y: y,
            z: z
        })
    }
}