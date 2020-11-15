// TODO: Remove this eventually and fix these warnings, for now it's just annoying since this is just getting started. - Austin Haskell
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs;
use libc::*;
use num;
#[macro_use]
extern crate num_derive;

pub mod parser { pub mod gcode; }
pub mod translator { pub mod gcode_translator; }
pub mod tmc2208 { 
    pub mod tmc2208; 
    pub mod tmc_packet;
    pub mod stealthchop_configuration;
    pub mod register_defs;
    pub mod otp_configuration;
    pub mod driver_status;
    pub mod chopper_configuration;
}
pub mod robot;
pub mod motor;

use parser::gcode::*;
use translator::gcode_translator::*;
use robot::*;
use motor::*;
use tmc2208::*;

use std::ffi::CString;
use std::os::raw::c_char;

// GPIO pins 2, 3, 4, 17, 27, and 10 are for motors
// UART 4 and 5 are for stepper control
// Matching physical pins are as follows
//        |  Rx  |  Tx  |
// UART 4 |  21  |  24  |
// UART 5 |  33  |  32  |

// GPIO   |  #  | Real |
//        |  2  |  3  | 
//        |  3  |  5  |
//        |  4  |  7  |
//        | 17  | 11  |
//        | 27  | 13  | 
//        | 10  | 19  |

fn main() {
    let print_area: (f32, f32) = (16.0, 16.0);

    /*
    unsafe {
        let mut fd = libc::open(
            CString::new("/dev/ttyAMA3").unwrap().as_ptr(), 
            O_RDWR | O_NOCTTY | O_NDELAY);

        

    }*/

    /*
    let robot = Robot {
        acceleration: 1.0,
        current_quadrant: (0, 0),
        print_area: print_area
    };

    let commands = translator::gcode_translator::translate_to_internal_command_list(&raw_commands, print_area);
    */
}
