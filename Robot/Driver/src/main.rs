// Todo: Remove this eventually and fix these warnings, for now it's just annoying since this is just getting started. - Austin Haskell
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fs;
use libc::*;

pub mod parser { pub mod gcode; }
pub mod translator { pub mod gcode_translator; }
pub mod robot;
pub mod motor;

use parser::gcode::*;
use translator::gcode_translator::*;
use robot::*;
use motor::*;

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

fn CalculateCRC(bytes: &Vec<u8>) -> u8 {
    let mut crc: u8 = 0;
    for byte in bytes {
        let mut val = byte.clone();

        for _ in 0..7 {
            if (crc >> 7) ^ (val & 0x01) != 0 {
                crc = crc << 1 ^ 0x07;
            } else {
                crc = crc << 1;
            }
        
            val = val >> 1;
        }
    }

    crc
}

fn make_packet(register: u8, payload: u32) -> Vec<u8> {
    let header: u8 = 175; // 10101111
    let slave_address: u8 = 0;
    let register_address = (register << 1) & 0x1;

    let mut packet: Vec<u8> = Vec::new();
    packet.push(header);
    packet.push(slave_address);
    packet.push(register_address);

    let a: u8 = (payload >> 24) as u8;
    let b: u8 = (payload >> 16) as u8;
    let c: u8 = (payload >> 8 ) as u8;
    let d: u8 = payload as u8;

    packet.push(a);
    packet.push(b);
    packet.push(c);
    packet.push(d);

    let crc = CalculateCRC(&packet);

    packet.push(crc);

    packet
}

fn main() {
    let print_area: (f32, f32) = (16.0, 16.0);

    unsafe {
        let mut fd = libc::open(
            CString::new("/dev/ttyAMA3").unwrap().as_ptr(), 
            O_RDWR | O_NOCTTY | O_NDELAY);

        

    }

    /*
    let robot = Robot {
        acceleration: 1.0,
        current_quadrant: (0, 0),
        print_area: print_area
    };

    let commands = translator::gcode_translator::translate_to_internal_command_list(&raw_commands, print_area);
    */
}
