use crate::gcode_parser;
use crate::defs::Color;
use crate::quadrant::Quadrant;

pub struct ImageData {
    pub width: usize,
    pub height: usize,
    pub bytes_per_pixel: u8,
    pub data: Vec<u8>,
    pub id: Quadrant
}

pub struct RenderSettings {
    pub render_orphan_points: bool,
    pub outline_quadrants: bool,
    pub line_color: Color,
    pub outline_color: Color,
    pub point_color: Color,
    pub pixels_per_mm: u32,
    // @todo: Add other settings, maybe there could be a sub-struct 
    //  that has the master image settings? - Austin Haskell
}

impl RenderSettings {
    pub fn Default() -> Self {
        RenderSettings {
            render_orphan_points: true,
            line_color: Color::Black(),
            point_color: Color::Blue(),

            pixels_per_mm: 5,

            outline_color: Color::Red(),
            outline_quadrants: true
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct PixelPoint {
    pub x: u32, 
    pub y: u32
}

impl PixelPoint {
    pub fn from_gcode_point(point: gcode_parser::Point, image_height: usize, scalar: u32) -> Self {
        PixelPoint {
            x: (point.x * scalar as f32).floor() as u32,
            y: (image_height as f32 - (point.y * scalar as f32).floor() - 1.0).abs() as u32
        }
    }
}

impl ImageData {
    pub fn create(gcode: &gcode_parser::CommandGrouping, machine_settings: &gcode_parser::MachineSettings, render_settings: &RenderSettings) -> Self {

        // @todo: Fix this. Switching from floating point to integers like this is going to cause some distortion for the actual shapes. It'll be small but the errors
        //  could add up. Maybe it's better to have some kind of mapping function and a fixed image size that has the same aspect ratio as the printbed? - Austin Haskell 
        let width: u32 = machine_settings.printbed_width.abs().floor() as u32;
        let height: u32 = machine_settings.printbed_height.abs().floor() as u32;

        let image_width:  usize = (width  * render_settings.pixels_per_mm) as usize; // +1 because sizing is inclusive
        let image_height: usize = (height * render_settings.pixels_per_mm) as usize;
        let image_size:   usize = image_width * image_height * 3; // Three bytes for the color
        let mut image_data: Vec<u8> = vec![255; image_size];

        let mut pixel_scaling: u32 = (render_settings.pixels_per_mm as f32).sqrt().floor() as u32;
        if pixel_scaling < 1 {
            pixel_scaling = 2;
        }

        if gcode.points.len() == 1 {
            let point = PixelPoint::from_gcode_point(gcode.points[0], image_height, render_settings.pixels_per_mm);
            ImageData::draw_point(&mut image_data, image_width, Color { r: 0, g: 0, b: 0}, render_settings.pixels_per_mm, point);
        }
        else {
            let mut last_point: Option<PixelPoint> = Some(PixelPoint::from_gcode_point(gcode.points[0], image_height, render_settings.pixels_per_mm));
            for point in &gcode.points {

                let current_point = PixelPoint::from_gcode_point(*point, image_height, render_settings.pixels_per_mm);
                if render_settings.render_orphan_points && point.z == 0.0 {
                    ImageData::draw_point(&mut image_data, image_width, Color { r: 0, g: 0, b: 100}, pixel_scaling, current_point);
                }

                // Pen Lift
                if point.z < 0.0 {
                    last_point = None;
                    continue;
                }

                if last_point.is_some() {
                    if last_point.unwrap() == current_point {
                        continue;
                    }

                    
                    ImageData::draw_line(&mut image_data[0..image_size], image_width, pixel_scaling, last_point.unwrap(), current_point);
                }

                last_point = Some(current_point);
            }
        }

        ImageData {
            width: image_width,
            height: image_height,
            bytes_per_pixel: 3,
            data: image_data,
            id: gcode.quadrant
        }
    }

    pub fn create_master(rendered_quadrants: &Vec<ImageData>, render_settings: &RenderSettings, largest_quadrant: &Quadrant, smallest_quadrant: &Quadrant) -> Self {

        // @todo: After playing with this, might be a good idea to possibly split the width, height, and bytes_per_pixel off the ImageData struct. - Austin Haskell
        assert!(rendered_quadrants.len() > 0);

        let bytes_per_pixel: usize = rendered_quadrants[0].bytes_per_pixel as usize;

        let image_width: usize  = (largest_quadrant.x + 1) as usize * rendered_quadrants[0].width;
        let image_height: usize = (largest_quadrant.y + 1) as usize * rendered_quadrants[0].height;
        let image_data: Vec<u8> = vec![100; image_height * image_width * bytes_per_pixel];

        let quadrant_width  = rendered_quadrants[0].width  * bytes_per_pixel;
        let quadrant_height = rendered_quadrants[0].height * bytes_per_pixel;

        for quad in rendered_quadrants {
            let flipped_y = quad.id.y - largest_quadrant.y;
            let mut line_pre = flipped_y as usize * self.image_width * 3 * self.quadrant_height as usize 
                + quad.id.x as usize * quadrant_width as usize; 

            for y in 0..quadrant_height {
                for x in 0..quadrant_width {
                    let pixel_coord: usize = (y * quadrant_width + x) as usize; 

                    self.data[line_pre + x as usize] = quad.image_data[pixel_coord];
                }
                line_pre += self.image_width * 3;
            }
        }

        ImageData {

        }
    }

    fn draw_line(data: &mut [u8], image_width: usize, scale: u32, start_point: PixelPoint, end_point: PixelPoint) {
        if (end_point.y - start_point.y) < (end_point.x - start_point.x) {
            if start_point.x > end_point.x {
                ImageData::draw_line_low(data, image_width, scale, end_point, start_point);
            } else {
                ImageData::draw_line_low(data, image_width, scale, start_point, end_point);
            }
        } else {
            if start_point.y > end_point.y {
                ImageData::draw_line_high(data, image_width, scale, end_point, start_point);
            } else {
                ImageData::draw_line_high(data, image_width, scale, start_point, end_point);
            }
        }
    }

    fn draw_line_high(data: &mut [u8], image_width: usize, scale: u32, start_point: PixelPoint, end_point: PixelPoint) {
        let mut delta_x: i32 = end_point.x as i32 - start_point.x as i32;
        let delta_y: i32 = end_point.y as i32 - start_point.y as i32;

        let mut x_itr: i32 = 1; 
        if delta_x < 0 {
            x_itr = -1;
            delta_x = -delta_x;
        }

        let mut distance = 2 * delta_x - delta_y;
        let mut x: i32 = start_point.x as i32;

        for y in start_point.y..=end_point.y {

            assert!(x > 0);
            assert!(y > 0);

            ImageData::draw_point(data, image_width, Color {r: 0, g: 0, b: 0}, scale, PixelPoint { x: x as u32, y: y });

            if distance > 0 {
                x = x + x_itr;
                distance = distance + (2 * (delta_x - delta_y));
            } else {
                distance = distance + 2 * delta_x;
            }
        }
    }

    fn draw_line_low(data: &mut [u8], image_width: usize, scale: u32, start_point: PixelPoint, end_point: PixelPoint) {
        let delta_x: i32 = end_point.x as i32 - start_point.x as i32;
        let mut delta_y: i32 = end_point.y as i32 - start_point.y as i32;

        let mut y_itr: i32 = 1;
        if delta_y < 0 {
            y_itr = -1;
            delta_y = -delta_y;
        }

        let mut distance = (2 * delta_y) - delta_x;
        let mut y: i32 = start_point.y as i32;
        for x in start_point.x..=end_point.x {
            
            assert!(y > 0);
            assert!(x > 0);

            ImageData::draw_point(data, image_width, Color {r: 0, g: 0, b: 0}, scale, PixelPoint { x: x, y: y as u32});

            if distance > 0 {
                y = y + y_itr;
                distance = distance + (2 * (delta_y - delta_x))
            } else {
                distance = distance + 2 * delta_y;
            }
        } 
    }

    fn draw_point(data: &mut [u8], image_width: usize, color: Color, size: u32, point: PixelPoint) {

        let radius: i32 = size as i32 / 4;
        for y_off in -radius..=radius {
            for x_off in -radius..=radius {
                let x: i32 = point.x as i32 + x_off as i32;
                let y: i32 = point.y as i32 + y_off as i32;

                if x < 0 || y < 0 {
                    continue;
                } else if ImageData::calc_pixel_position(image_width, x, y) >= data.len() as i32 || x >= image_width as i32 {
                    continue;
                }

                ImageData::set_pixel_color(data, image_width, color.clone(), x, y);
            }
        }
    }

    fn set_pixel_color(data: &mut [u8], image_width: usize, color: Color, x: i32, y: i32) {

        let pos = ImageData::calc_pixel_position(image_width, x, y);
        data[pos as usize    ] = color.r; 
        data[pos as usize + 1] = color.g; 
        data[pos as usize + 2] = color.b; 
    }

    fn calc_pixel_position(width: usize, x: i32, y: i32) -> i32 {
        y * width as i32 * 3 + x * 3 
    }
}