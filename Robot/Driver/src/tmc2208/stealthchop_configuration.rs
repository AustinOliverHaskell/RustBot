use num;
use num_derive;

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum PWMFReq {
    Freq2_1024,
    Freq2_683,
    Freq2_512,
    Freq2_410,
}

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum StandStillMode {
    Normal,
    Freewheel,
    CoilShortLS,
    CoilShortHS
}

#[derive(PartialEq, Debug)]
pub struct StealthchopConfiguration {
    amplitude_offset: u8,
    amplitude_gradient: u8,
    freq_selection: PWMFReq,
    auto_amplitude_scaling: bool,
    auto_gradient_adaptation: bool,
    standstill_mode: StandStillMode,
    regulation_loop_gradient: u8,
    auto_scale_amplitude: u8
}

impl StealthchopConfiguration {
    pub fn from_raw(raw: u32) -> Self {
        return StealthchopConfiguration {
            amplitude_offset: (raw & 0xFF) as u8,
            amplitude_gradient: ((raw >> 8) & 0xFF) as u8,
            freq_selection: num::FromPrimitive::from_u32((raw >> 16) & 0x3).unwrap(),
            auto_amplitude_scaling: (raw >> 18) & 0x01 != 0,
            auto_gradient_adaptation: (raw >> 19) & 0x01 != 0,
            standstill_mode: num::FromPrimitive::from_u32((raw >> 20) & 0x3).unwrap(),
            regulation_loop_gradient: ((raw >> 24) & 0xF) as u8,
            auto_scale_amplitude: ((raw >> 28) & 0xF) as u8
        }
    }
}

#[test]
fn from_raw_parses() {
    let test_data: u32 = 0b01000111010001101010100110101110;
    let actual = StealthchopConfiguration::from_raw(test_data);

    let expected = StealthchopConfiguration {
        amplitude_offset: 174,
        amplitude_gradient: 169,
        freq_selection: PWMFReq::Freq2_512,
        auto_amplitude_scaling: true,
        auto_gradient_adaptation: false,
        standstill_mode: StandStillMode::Normal,
        regulation_loop_gradient: 7,
        auto_scale_amplitude: 4
    };

    assert_eq!(expected, actual);
}