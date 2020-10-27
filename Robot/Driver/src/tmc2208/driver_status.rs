#[derive(PartialEq, Debug)]
pub struct DriverStatus {
    standstill: bool,
    stealth_enabled: bool,
    current_control: u16,
    temp_exceeds_120c: bool,
    temp_exceeds_143c: bool,
    temp_exceeds_150c: bool,
    temp_exceeds_157c: bool,
    open_load_a: bool,
    open_load_b: bool,
    short_a: bool,
    short_b: bool,
    ground_short_a: bool,
    ground_short_b: bool,
    overtemp: bool,
    overtemp_warning: bool
}

impl DriverStatus {
    pub fn from_raw(raw: u32) -> Self {
        return DriverStatus {
            standstill:        (raw >> 31) != 0,
            stealth_enabled:   (raw >> 30) & 0x01 != 0,
            current_control:   ((raw >> 16) & 0x1F) as u16,
            temp_exceeds_120c: (raw  >> 8) & 0x01 != 0,
            temp_exceeds_143c: (raw  >> 9) & 0x01 != 0,
            temp_exceeds_150c: (raw >> 10) & 0x01 != 0,
            temp_exceeds_157c: (raw >> 11) & 0x01 != 0,
            open_load_a:       (raw >> 6) & 0x01 != 0,
            open_load_b:       (raw >> 7) & 0x01 != 0,
            short_a:           (raw >> 4) & 0x01 != 0,
            short_b:           (raw >> 5) & 0x01 != 0,
            ground_short_a:    (raw >> 2) & 0x01 != 0,
            ground_short_b:    (raw >> 3) & 0x01 != 0,
            overtemp:          (raw >> 1) & 0x01 != 0,
            overtemp_warning:   raw & 0x01 != 0
        };
    }
}

#[test]
fn from_raw_creates() {
    let test_data: u32 = 0b00100100111110110110101010100101;

    let actual = DriverStatus::from_raw(test_data);

    let expected = DriverStatus {
        standstill: false,
        stealth_enabled: false,
        current_control: 27,
        temp_exceeds_120c: false,
        temp_exceeds_143c: true,
        temp_exceeds_150c: false,
        temp_exceeds_157c: true,
        open_load_a: false,
        open_load_b: true,
        short_a: false,
        short_b: true,
        ground_short_a: true,
        ground_short_b: false,
        overtemp: false,
        overtemp_warning: true
    };

    assert_eq!(expected, actual);
}