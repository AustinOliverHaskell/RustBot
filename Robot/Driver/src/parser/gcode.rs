#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n"; 

#[derive(Debug)]
pub struct GCode {
    pub command: String,
    pub x: f32,
    pub y: f32,
    pub z: f32 // Including a Z because some slicers will map a lift as a raising of the Z axis rather than using the lift command. - Austin Haskell
}

impl GCode {
    pub fn deserialize(raw: &String) -> Vec<GCode> {
        let mut parsed_values: Vec<GCode> = Vec::new();
    
        for line in raw.split(LINE_ENDING) {

            if line.starts_with(';') || line.len() < 1 {
                continue; 
            }

            let parsed_line: Vec<&str> = filter_out_invalids(line.split(' ').collect());
            match construct_gcode_from_line(parsed_line) {
                Ok(code) => parsed_values.push(code),
                Err(_) => continue
            }
        }
        parsed_values
    }
}

pub fn construct_gcode_from_line(line: Vec<&str>) -> Result<GCode, &str> {
    let mut code = GCode {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        command: String::from("")
    };
    let supported_g_codes: Vec<&str> = vec!["G0", "G1", "G10"];

    for parsed_command in line {

        let first_letter = parsed_command.chars().next().unwrap_or_default();
        match first_letter {
            'X' => code.x = ignore_letter_and_parse(parsed_command),
            'Y' => code.y = ignore_letter_and_parse(parsed_command),
            'Z' => code.z = ignore_letter_and_parse(parsed_command),
            'G' => {
                if !supported_g_codes.contains(&parsed_command) {
                    return Err("GCode command not supported");
                }
                code.command = parsed_command.to_string();
            },
            'M' => return Err("Command has no implementation"),
            _ => continue
        }
    }

    Ok(code)
}


fn filter_out_invalids(line: Vec<&str>) -> Vec<&str> {

    let mut cleaned_line: Vec<&str> = Vec::new();

    for parsed_command in line {
        if parsed_command.is_empty() { continue; }

        let first_letter = parsed_command.chars().next().unwrap_or_default();
        match first_letter {
            ' '  => continue,
            ';' => break, // Once we hit a comment, just stop - Austin Haskell
            _ => cleaned_line.push(parsed_command)
        }
    }
    cleaned_line
}

fn ignore_letter_and_parse(val: &str) -> f32 {
    if val.len() < 1 { return 0.0; }
    (&val[1..]).parse::<f32>().unwrap_or_default()
}









#[test]
fn ignore_letter_and_parse_zero_length_defaults() {
    assert_eq!(ignore_letter_and_parse(""), 0.0);
}

#[test]
fn ignore_letter_and_parse_parses() {
    assert_eq!(ignore_letter_and_parse("X-189.1"), -189.1);
    assert_eq!(ignore_letter_and_parse("X1"), 1.0);
}

#[test]
fn deserialize_unknown_type_ignored_deserializes() {
    let result = GCode::deserialize(&String::from("J1 X1.1422 Y-1.0178 F1016"));

    assert_eq!(result.len(), 0);
}

#[test]
fn deserialize_comments_ignored_deserializes() {
    let result = GCode::deserialize(&String::from(";G1 X1.1422 Y-1.0178 F1016"));

    assert_eq!(result.len(), 0);
}

#[test]
fn deserialize_commands_without_xyz_deserializes() {
    let result = GCode::deserialize(&String::from(
        "G21         ; Set units to mm
         G90         ; Absolute positioning
         G1          ; Move to clearance level"));

    assert_eq!(result.len(), 1);
}