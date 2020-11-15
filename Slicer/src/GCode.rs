// I'm going to make this non-standard GCode by adding in custom commands not in the standard. 
// Non-standard commands
//   Q1 -> Change quadrants

#[derive(Debug, PartialEq, Clone)]
pub struct Word {
    pub letter: char,
    pub value: u16
}

#[derive(Debug, PartialEq, Clone)]
pub struct GCode {
    pub command: Word,
    pub x: f32,
    pub y: f32,
    pub z: f32 // Including a Z because some slicers will map a lift as a raising of the Z axis rather than using the lift command. - Austin Haskell
}

impl Word {
    pub fn ToString(self: &Self) -> String {
        String::from(self.letter.to_string() + &self.value.to_string())
    }
}

impl GCode {
    pub fn Write(self: &Self) -> String {
        let mut line: String = self.command.ToString();
        line += &self.x.to_string();
        line += " ";
        line += &self.y.to_string();
        line += " ";
        line += &self.z.to_string();

        line
    }
}