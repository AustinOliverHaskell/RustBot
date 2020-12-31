use crate::defs::RenderedQuadrant;

pub struct MasterImage {
    pub image_width: usize,
    pub image_height: usize,
    pub data: Vec<u8>,
    pub quadrant_width: usize,
    pub quadrant_height: usize,
    pub largest_quadrant: (i32, i32)
}


impl MasterImage {
    pub fn new(quadrant_height: usize, quadrant_width: usize, largest_quadrants: (i32, i32)) -> Self {
    
        let image_width: usize  = (largest_quadrants.0 + 1) as usize * quadrant_width;
        let image_height: usize = (largest_quadrants.1 + 1) as usize * quadrant_height;
        let image_data: Vec<u8> = vec![100; image_height * image_width * 3];

        MasterImage {
            image_width: image_width,
            image_height: image_height,
            data: image_data,
            quadrant_width: quadrant_width,
            quadrant_height: quadrant_height,
            largest_quadrant: largest_quadrants
        }
    }

    pub fn add(self: &mut Self, quad: RenderedQuadrant) {

        let quadrant_width = self.quadrant_width * 3;

        let flipped_y = (quad.id.1 - self.largest_quadrant.1).abs();
        let mut line_pre = flipped_y as usize * self.image_width * 3 * self.quadrant_height as usize 
            + quad.id.0 as usize * quadrant_width as usize; 

        for y in 0..self.quadrant_height {
            for x in 0..quadrant_width {
                let pixel_coord: usize = (y * quadrant_width + x) as usize; 

                self.data[line_pre + x as usize] = quad.image_data[pixel_coord];
            }
            line_pre += self.image_width * 3;
        }
    }

    pub fn outline(self: &mut Self, r: u8, g: u8, b: u8) {

        for y in 1..=self.largest_quadrant.1 {
            let row: usize = y as usize * self.quadrant_height * self.image_width * 3;
            let mut x = row;
            while x < row + self.image_width * 3 {
    
                self.data[x as usize]     = 255;
                self.data[x as usize + 1] = 0;
                self.data[x as usize + 2] = 0;
    
                x += 3;
            }
        }
    
        for x in 1..=self.largest_quadrant.0 {
            let column: usize = x as usize * self.quadrant_width * 3;
            let mut y = 0; 
            while y < self.quadrant_height * self.image_width * 3 * (self.largest_quadrant.1 + 1) as usize{
    
                let index = y as usize + column as usize;
                if index > self.data.len() {
                    break;
                }
    
                self.data[index] = 255;
                self.data[index + 1] = 0;
                self.data[index + 2] = 0;
    
                y += self.image_width * 3;
            }
        }
    }
}
