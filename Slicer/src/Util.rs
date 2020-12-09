pub fn float_mod(a: f32, b: f32) -> f32 {

    // TODO: Refactor this, it could be made cleaner / more efficiant - Austin Haskell

    if b == 0.0 || a == 0.0 {
        return 0.0;
    }

    if a < b {
        return a / b;
    }

    let mut isNegative = false;
    let mut divisor = b;
    if divisor < 0.0 {
        isNegative = true;
        divisor *= -1.0;
    }

    let mut value: f32 = a;
    if value < 0.0 {
        value *= -1.0;
        isNegative = !isNegative;
    }

    let mut divisions: i32 = 0;
    while value > divisor {
        value -= divisor;
        divisions += 1;
    }

    /*let remainder;
    if a > 0.0 {
        remainder = ((a * value) as i32) as f32 / 10.0;
    } else {
        remainder = (((a * -1.0) * value) as i32) as f32 / 10.0;
    }*/
    let mut result = divisions as f32;// + remainder;

    if isNegative {
        result *= -1.0;
    }

    result
}

pub fn compare_gcode_line_vectors(list_a: Vec<String>, list_b: Vec<String>) -> bool {
    if list_a.len() != list_b.len() {
        return false;
    }

    for i in 0..list_a.len() {
        if list_a[i] != list_b[i] {
            return false;
        }
    }

    true
}


#[test]
fn FloatMod_Calculates() {

    let expected: f32 = 1.4;
    let actual: f32 = float_mod(59.8, 46.5);

    assert_eq!(expected, actual)
}

#[test]
fn FloatMod_Negatives_Calculates() {
    let expected: f32 = -1.4;

    let actual_b_negative: f32 = float_mod(59.8, -46.5);
    let actual_a_negative: f32 = float_mod(-59.8, 46.5);

    assert_eq!(expected, actual_a_negative);
    assert_eq!(expected, actual_b_negative);

    let expected_2: f32 = 1.4;

    let actual_c_negative: f32 = float_mod(-59.8, -46.5);

    assert_eq!(expected_2, actual_c_negative);
}

#[test]
fn compare_gcode_line_vectors_Are_not_equal() {
    let list_a: Vec<String> = 
        vec![
            String::from("a"), 
            String::from("b"),
            String::from("Some other string")];

    let list_b: Vec<String> = 
        vec![
            String::from("a"), 
            String::from("c"),
            String::from("Some other string")];

    assert_eq!(false, compare_gcode_line_vectors(list_a, list_b));
}

#[test]
fn compare_gcode_line_vectors_Are_equal() {
    let list_a: Vec<String> = 
        vec![
            String::from("a"), 
            String::from("b"),
            String::from("Some other string")];

    let list_b: Vec<String> = 
        vec![
            String::from("a"), 
            String::from("b"),
            String::from("Some other string")];

    assert_eq!(true, compare_gcode_line_vectors(list_a, list_b));
}