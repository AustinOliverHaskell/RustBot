/*
    The following matches commands formatted as 
    Letter (possible space or comma) number (possible space or comma) number
*/
pub static SVG_COMMAND_GROUPING: &str = r"[a-zA-Z]([ ,]*[\-0-9]*[\.0-9]*[ ,]*)*";

/*
    Used for splitting on either a comma or a space
*/
pub static COMMA_OR_SPACE: &str = r"[, ]+";

pub static SEPERATE_TWO_NUMBERS: &str = r"\-*[0-9]*\.*[0-9]*";