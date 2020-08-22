pub struct Command {
    pub x: f32,
    pub y: f32,
    pub pen_lifted: bool
}

pub struct Robot { 
    pub acceleration: f32
}

impl Robot {
    pub fn new(acceleration: f32) -> Self {
        return Robot {
            acceleration: acceleration
        }
    }
}