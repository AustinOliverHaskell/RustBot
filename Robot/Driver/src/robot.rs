pub struct Command {
    pub position: (f32, f32),
    pub quadrant: (u16, u16),
    pub pen_lifted: bool
}

pub struct Robot {
    /*pub motors: Vec<Motor>,*/
    pub acceleration: f32,
    pub current_quadrant: (u16, u16),
    pub print_area: (f32, f32)
}



impl Robot {
    pub fn start_robot() -> Self {
        return Robot {
            /*motors: Vec::new(),*/
            acceleration: 0.0,
            current_quadrant: (0, 0),
            print_area: (0.0, 0.0)
        }
    }

    pub fn execute_command(self: &Self, command: &Command) {
    }

    // Note: This function as well as the move_vertical function are unbounded since
    //  the bot can drive wherever.                                 - Austin Haskell
    pub fn move_horizontal(self: &Self, distance_in_mm: f32) {
        // todo: motor code or the code that calls the motor code here once we have that figured out - Austin Haskell
    }

    pub fn move_vertical(self: &Self, distance_in_mm: f32) {
        // todo: motor code or the code that calls the motor code here once we have that figured out - Austin Haskell
    }

    pub fn move_printhead_vertical(self: &Self, distance_in_mm: f32) {
        // todo: stepper motor code here - Austin Haskell
    }

    pub fn move_printhead_horizontal(self: &Self, distance_in_mm: f32) {
        // todo: stepper motor code here - Austin Haskell
    }
}