use crate::file_loader;

pub struct Rasterizer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct PixelPoint {
    pub x: i32, 
    pub y: i32
}

impl PixelPoint {
    pub fn from_gcode_point(point: file_loader::Point, image_height: usize, scalar: i32) -> Self {
        PixelPoint {
            x: (point.x * scalar as f32).floor() as i32,
            y: (image_height as f32 - (point.y * scalar as f32).floor() - 1.0).abs() as i32
        }
    }
}

impl Rasterizer {
    pub fn create(gcode: &file_loader::CommandGrouping, width: i32, height: i32, scalar: i32) -> Self {

        let image_width:  usize = (width  * scalar) as usize; // +1 because sizing is inclusive
        let image_height: usize = (height * scalar) as usize;
        let image_size:   usize = image_width * image_height * 3; // Three bytes for the color
        let mut image_data: Vec<u8> = vec![255; image_size];

        if gcode.points.len() == 1 {
            let point = PixelPoint::from_gcode_point(gcode.points[0], image_height, scalar);
            Rasterizer::draw_point(&mut image_data, image_width, Color { r: 0, g: 0, b: 0}, scalar, point);
        }
        else {
            let mut last_point = PixelPoint::from_gcode_point(gcode.points[0], image_height, scalar);
            for point in &gcode.points {

                let current_point = PixelPoint::from_gcode_point(*point, image_height, scalar);
                if last_point == current_point {
                    continue;
                }

                Rasterizer::draw_line(&mut image_data[0..image_size], image_width, scalar, last_point, current_point);

                last_point = current_point;
            }
        }

        Rasterizer {
            width: image_width,
            height: image_height,
            data: image_data
        }
    }

    fn draw_line(data: &mut [u8], image_width: usize, scale: i32, start_point: PixelPoint, end_point: PixelPoint) {
        if (end_point.y - start_point.y).abs() < (end_point.x - start_point.x).abs() {
            if start_point.x > end_point.x {
                Rasterizer::draw_line_low(data, image_width, 2, end_point, start_point);
            } else {
                Rasterizer::draw_line_low(data, image_width, 2, start_point, end_point);
            }
        } else {
            if start_point.y > end_point.y {
                Rasterizer::draw_line_high(data, image_width, 2, end_point, start_point);
            } else {
                Rasterizer::draw_line_high(data, image_width, 2, start_point, end_point);
            }
        }

        //Rasterizer::draw_point(data, image_width, Color { r: 0, g: 0, b: 100}, scale, start_point);
        //Rasterizer::draw_point(data, image_width, Color { r: 0, g: 0, b: 100}, scale, end_point);
    }

    fn draw_line_high(data: &mut [u8], image_width: usize, scale: i32, start_point: PixelPoint, end_point: PixelPoint) {
        let mut delta_x = end_point.x - start_point.x;
        let delta_y = end_point.y - start_point.y;

        let mut x_itr = 1; 
        if delta_x < 0 {
            x_itr = -1;
            delta_x = -delta_x;
        }

        let mut distance = 2 * delta_x - delta_y;
        let mut x = start_point.x;

        for y in start_point.y..=end_point.y {
            for thickness in -scale..=scale {
                if y + thickness < 0 || x >= image_width as i32 ||
                   Rasterizer::calc_pixel_position(image_width, x, y + thickness) >= data.len() as i32 { 
                    continue; 
                }

                Rasterizer::set_pixel_color(data, image_width, Color {r: 0, g: 0, b: 0}, x, y + thickness);
            }
            if distance > 0 {
                x = x + x_itr;
                distance = distance + (2 * (delta_x - delta_y));
            } else {
                distance = distance + 2 * delta_x;
            }
        }
    }

    fn draw_line_low(data: &mut [u8], image_width: usize, scale: i32, start_point: PixelPoint, end_point: PixelPoint) {
        let delta_x = end_point.x - start_point.x;
        let mut delta_y = end_point.y - start_point.y;

        let mut y_itr = 1;
        if delta_y < 0 {
            y_itr = -1;
            delta_y = -delta_y;
        }

        let mut distance = (2 * delta_y) - delta_x;
        let mut y = start_point.y;
        for x in start_point.x..=end_point.x {
            for thickness in -scale..=scale {
                if y + thickness < 0 || x >= image_width as i32 || 
                   Rasterizer::calc_pixel_position(image_width, x, y + thickness) >= data.len() as i32 { 
                    continue; 
                }

                Rasterizer::set_pixel_color(data, image_width, Color {r: 0, g: 0, b: 0}, x, y + thickness);
            }

            if distance > 0 {
                y = y + y_itr;
                distance = distance + (2 * (delta_y - delta_x))
            } else {
                distance = distance + 2 * delta_y;
            }
        } 
    }

    fn draw_point(data: &mut [u8], image_width: usize, color: Color, size: i32, point: PixelPoint) {

        let radius = size / 4;
        for y_off in -radius..=radius {
            for x_off in -radius..=radius {
                let x = point.x + x_off;
                let y = point.y + y_off;

                if x < 0 || y < 0 {
                    continue;
                } else if Rasterizer::calc_pixel_position(image_width, x, y) >= data.len() as i32 || x >= image_width as i32 {
                    continue;
                }

                Rasterizer::set_pixel_color(data, image_width, color.clone(), x, y);
            }
        }
    }

    fn set_pixel_color(data: &mut [u8], image_width: usize, color: Color, x: i32, y: i32) {

        let pos = Rasterizer::calc_pixel_position(image_width, x, y);
        data[pos as usize    ] = color.r; 
        data[pos as usize + 1] = color.g; 
        data[pos as usize + 2] = color.b; 
    }

    fn calc_pixel_position(width: usize, x: i32, y: i32) -> i32 {
        y * width as i32 * 3 + x * 3 
    }
}