#[derive(PartialEq, Eq, PartialOrd, Debug, Clone, Copy, Hash)]
pub struct Quadrant {
    pub x: u32,
    pub y: u32
}

impl Quadrant {
    pub fn Max() -> Self {
        Quadrant {
            x: u32::MAX,
            y: u32::MAX
        }
    }

    pub fn Min() -> Self {
        Quadrant {
            x: 0,
            y: 0
        }
    }
}
