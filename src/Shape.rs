pub struct Shape {
    pub depth: i32;
    pub infill: f32;
}

pub trait Shape {
    fn Draw(self: &Self);
}