pub struct Command {
    pub x: f32,
    pub y: f32,
    pub pen_lifted: bool
}

pub struct Robot { 
    pub acceleration: f32,
    pub print_area_x: f32, // Note: This is in mm - Austin Haskell
    pub print_area_y: f32  // See above
}

impl Robot {
    pub fn new(acceleration: f32, x: f32, y: f32) -> Self {
        return Robot {
            acceleration: acceleration,
            print_area_x: x,
            print_area_y: y
        }
    }

    pub fn execute_command(command: &Command) {

    }

    // Note: This function as well as the move_vertical function are unbounded since
    //  the bot can drive wherever.                                 - Austin Haskell
    pub fn move_horizontal(distance_in_mm: f32) {
        // todo: motor code or the code that calls the motor code here once we have that figured out - Austin Haskell
    }

    pub fn move_vertical(distance_in_mm: f32) {
        // todo: motor code or the code that calls the motor code here once we have that figured out - Austin Haskell
    }

    pub fn move_printhead_vertical(self: &Self, distance_in_mm: f32) {
        // todo: stepper motor code here - Austin Haskell
    }

    pub fn move_printhead_horizontal(self: &Self, distance_in_mm: f32) {
        // todo: stepper motor code here - Austin Haskell
    }
}