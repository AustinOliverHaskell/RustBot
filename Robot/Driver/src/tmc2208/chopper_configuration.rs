use num;
use num_derive;

#[derive(PartialEq, Debug)]
pub struct ChopperConfiguration {
    toff_time: u8,
    hysteresis_start: u8,
    hysteresis_val: i8,
    time_select: ComparatorBlankTime,
    high_sensitivity: bool,
    microstep_resolution: MicrostepResolution,
    interp_to_256_microsteps: bool,
    double_edge_step_pulses: bool,
    short_to_ground_protection: bool,
    low_side_short_protection: bool
}

impl ChopperConfiguration {
    pub fn from_raw(raw: u32) -> Self {
        return ChopperConfiguration {
            toff_time:                  (raw & 0xF) as u8,
            hysteresis_start:           ((raw >> 4) & 0xF) as u8,
            hysteresis_val:             (((raw >> 8) & 0xF) as i8) - 3,
            time_select:                num::FromPrimitive::from_u32((raw >> 15) & 0x03).unwrap(),
            high_sensitivity:           ((raw >> 17) & 0x01) != 0,
            microstep_resolution:       map_val_to_microstep_resolution(((raw >> 24) & 0x0F) as u8),
            interp_to_256_microsteps:   ((raw >> 28) & 0x01) != 0,
            double_edge_step_pulses:    ((raw >> 29) & 0x01) != 0,
            short_to_ground_protection: ((raw >> 30) & 0x01) != 0,
            low_side_short_protection:  ((raw >> 31) & 0x01) != 0
        }
    }

    pub fn to_raw(self: Self) -> u32 {
        let mut raw: u32 = 0;

        raw = raw | self.toff_time as u32;
        raw = raw | ((self.hysteresis_start as u32) << 4);
        raw = raw | ((((self.hysteresis_val) as i32) + 3) << 8) as u32;
        raw = raw | ((self.time_select as u32) << 15);
        raw = raw | ((self.high_sensitivity as u32) << 17);
        raw = raw | ((self.microstep_resolution as u32) << 24);
        raw = raw | ((self.interp_to_256_microsteps as u32) << 28);
        raw = raw | ((self.double_edge_step_pulses as u32) << 29);
        raw = raw | ((self.short_to_ground_protection as u32) << 30);
        raw = raw | ((self.low_side_short_protection as u32) << 31);

        raw
    }
}

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum ComparatorBlankTime {
    Time16,
    Time24,
    Time32,
    Time40
}

fn map_val_to_microstep_resolution(val: u8) -> MicrostepResolution {
    if val > 8 {
        return MicrostepResolution::Fullstep;
    }
    
    num::FromPrimitive::from_u8(val).unwrap()
}

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum MicrostepResolution {
    Step256,
    Step128,
    Step64,
    Step32,
    Step16,
    Step8,
    Step4,
    Step2,
    Fullstep
}

#[test]
fn from_raw_deserializes() {
    let test_data: u32 = 0b01011000010010011100110101011101;
    let actual = ChopperConfiguration::from_raw(test_data);

    let expected = ChopperConfiguration {
        toff_time: 13,
        hysteresis_start: 5,
        hysteresis_val: 10,
        time_select: ComparatorBlankTime::Time40,
        high_sensitivity: false,
        microstep_resolution: MicrostepResolution::Fullstep,
        interp_to_256_microsteps: true,
        double_edge_step_pulses: false,
        short_to_ground_protection: true,
        low_side_short_protection: false
    };

    assert_eq!(expected, actual);
}

#[test]
fn to_raw_serializes() {
    let expected: u32 = 0b01011000000000011000110101011101;

    let test_data = ChopperConfiguration {
        toff_time: 13,
        hysteresis_start: 5,
        hysteresis_val: 10,
        time_select: ComparatorBlankTime::Time40,
        high_sensitivity: false,
        microstep_resolution: MicrostepResolution::Fullstep,
        interp_to_256_microsteps: true,
        double_edge_step_pulses: false,
        short_to_ground_protection: true,
        low_side_short_protection: false
    };
    let actual = test_data.to_raw();
    let tmp = ChopperConfiguration::from_raw(actual);
    println!("{:?}", tmp);

    assert_eq!(expected, actual);
}

#[test]
fn map_val_to_microstep_resolution_maps() {
    assert_eq!(MicrostepResolution::Fullstep, map_val_to_microstep_resolution(15));
    assert_eq!(MicrostepResolution::Step256,  map_val_to_microstep_resolution(0));
    assert_eq!(MicrostepResolution::Step32,   map_val_to_microstep_resolution(3));
}