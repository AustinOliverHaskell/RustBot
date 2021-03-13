use crate::quadrant::Quadrant;

#[derive(Clone, Debug, PartialEq)]
pub struct RenderedQuadrant {
    pub id: Quadrant,
    pub image_data: Vec<u8>
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn Black() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 255
        }
    }

    pub fn Blue() -> Self {
        Color {
            r: 0,
            g: 0,
            b: 255
        }
    }

    pub fn Red() -> Self {
        Color {
            r: 255,
            g: 0,
            b: 0
        }
    }
}