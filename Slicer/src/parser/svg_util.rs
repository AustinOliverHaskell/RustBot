// .parse() will only take the type provided. So if parsing 30 and the type is float then 
// .parse() will throw an error as it's an int. That's the reason behind this terrible-ness
// - Austin Haskell
pub fn parse_possible_float(data: &str) -> f32 {
    let val: f32;
    if data.contains(".") {
        val = data.parse().unwrap();
    } else {
        let parsed: i32 = data.parse().unwrap();
        val = parsed as f32;
    }

    val
}

// Two-Dimentionalizes a flat list. 12, 1, 4, 12, 5, 7 -> (12, 1), (4, 12), (5, 7)
pub fn create_xy_point_list(list: &Vec<f32>) -> Vec<(f32, f32)> {
    let mut point_list: Vec<(f32, f32)> = Vec::new();

    let mut point: Option<(f32, f32)> = None;
    for item in list {
        if point.is_none() {
            point = Some((item.clone(), 0.0));
        } else {
            point = Some((point.unwrap().0, item.clone()));
            point_list.push(point.unwrap());
            point = None;
        }
    }

    point_list
}