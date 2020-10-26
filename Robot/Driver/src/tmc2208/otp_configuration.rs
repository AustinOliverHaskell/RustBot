use num;
use num_derive;

pub struct OtpConfigurationRaw {
    Byte0: u8,
    Byte1: u8,
    Byte2: u8
}

pub struct OtpConfiguration {
    /* Byte 0 */
    ottrim: OTTRIM,
    internal_rsense: RSense,
    tbl: bool,
    /* Byte 1 */
    pwm_grad: PWMGrad, 
    pwm_autograd: bool,
    tpwm_threshold: TPWMThreshold,
    /* Byte 2 */
    pwm_ofs: PWMOfs,
    pwm_reg: PWMReg,
    pwm_freq: PWMFreq,
    hold_delay: IHoldDelay,
    hold: IHold,
    stealth_chop_enabled: bool 
}

impl OtpConfiguration {
    pub fn from_raw(byte0: u8, byte1: u8, byte2: u8) -> Self {
        /*Byte 0*/
        let ottrim: OTTRIM          = num::FromPrimitive::from_u8((byte0 >> 5) & 0x01).unwrap();
        let internal_rsense: RSense = num::FromPrimitive::from_u8((byte0 >> 6) & 0x01).unwrap();
        let tbl: bool               = ((byte0 >> 7) & 0x01) != 0;

        /*Byte 1*/
        let pwm_grad: PWMGrad             = num::FromPrimitive::from_u8(byte1 & 0x0F).unwrap();
        let pwm_autograd: bool            = ((byte1 >> 4) & 0x01) != 0;
        let tpwm_threshold: TPWMThreshold = num::FromPrimitive::from_u8((byte1 >> 5) & 0x07).unwrap();

        /*Byte 2*/
        let pwm_ofs: PWMOfs        = num::FromPrimitive::from_u8(byte2 & 0x01).unwrap();
        let pwm_reg: PWMReg        = num::FromPrimitive::from_u8((byte2 >> 1) & 0x01).unwrap();
        let pwm_freq: PWMFreq      = num::FromPrimitive::from_u8((byte2 >> 2) & 0x01).unwrap();
        let hold_delay: IHoldDelay = num::FromPrimitive::from_u8((byte2 >> 3) & 0x03).unwrap();
        let hold: IHold            = num::FromPrimitive::from_u8((byte2 >> 5) & 0x03).unwrap();
        let stealth_chop_enabled: bool = (byte2 >> 7) == 0;

        return OtpConfiguration {
            ottrim: ottrim,
            internal_rsense: internal_rsense,
            tbl: tbl,

            pwm_grad: pwm_grad,
            pwm_autograd: pwm_autograd,
            tpwm_threshold: tpwm_threshold,

            pwm_ofs: pwm_ofs,
            pwm_reg: pwm_reg,
            pwm_freq: pwm_freq,
            hold_delay: hold_delay,
            hold: hold,
            stealth_chop_enabled: stealth_chop_enabled
        }
    }

    pub fn to_raw(self: Self) -> OtpConfigurationRaw {
        
        let mut byte0: u8 = 0;
        // Datasheet recommends not using bits 0-4 of byte 0
        byte0 = ((self.ottrim as u8)          << 5) | byte0;
        byte0 = ((self.internal_rsense as u8) << 6) | byte0;
        byte0 = ((self.tbl as u8)             << 7) | byte0;

        let mut byte1: u8 = 0;
        if self.stealth_chop_enabled {
            byte1 = (self.pwm_grad as u8)              | byte1;
            byte1 = ((self.pwm_autograd as u8)   << 4) | byte1;
            byte1 = ((self.tpwm_threshold as u8) << 5) | byte1;
        }

        let mut byte2: u8 = 0;
        if self.stealth_chop_enabled {
            byte2 = (self.pwm_ofs as u8) | byte2;
        } else {
            byte2 = (1 << 7) | byte2;
        }

        byte2 = ((self.pwm_reg as u8)    << 1) | byte2;
        byte2 = ((self.pwm_freq as u8)   << 2) | byte2;
        byte2 = ((self.hold_delay as u8) << 3) | byte2;
        byte2 = ((self.hold as u8)       << 5) | byte2;
        
        return OtpConfigurationRaw {
            Byte0: byte0,
            Byte1: byte1,
            Byte2: byte2
        }
    }
}

impl OtpConfigurationRaw {
    pub fn make_configuration(
        /* Byte 0 */
        ottrim: OTTRIM,
        internal_rsense: RSense,
        tbl: bool,
        /* Byte 1 */
        pwm_grad: PWMGrad, 
        pwm_autograd: bool,
        tpwm_threshold: TPWMThreshold,
        /* Byte 2 */
        pwm_ofs: PWMOfs,
        pwm_reg: PWMReg,
        pwm_freq: PWMFreq,
        hold_delay: IHoldDelay,
        hold: IHold,
        stealth_chop_enabled: bool 
    ) -> OtpConfigurationRaw {

        let config = OtpConfiguration {
            ottrim: ottrim,
            internal_rsense: internal_rsense,
            tbl: tbl,
            pwm_grad: pwm_grad, 
            pwm_autograd: pwm_autograd,
            tpwm_threshold: tpwm_threshold,
            pwm_ofs: pwm_ofs,
            pwm_reg: pwm_reg,
            pwm_freq: pwm_freq,
            hold_delay: hold_delay,
            hold: hold,
            stealth_chop_enabled: stealth_chop_enabled
        };

        return config.to_raw();
    }
}

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum PWMGrad {
    Grad14,
    Grad16,
    Grad18,
    Grad21,
    Grad24,
    Grad27,
    Grad31,
    Grad35,
    Grad40,
    Grad46,
    Grad52,
    Grad59,
    Grad67,
    Grad77,
    Grad88,
    Grad100
}

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum TPWMThreshold {
    Threshold0,
    Threshold200,
    Threshold300,
    Threshold400,
    Threshold500,
    Threshold800,
    Threshold1200,
    Threshold4000
}

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum IHoldDelay {
    Delay1,
    Delay2,
    Delay4,
    Delay8
}

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum IHold {
    Hold16,
    Hold2,
    Hold8,
    Hold24
}

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum PWMReg {
    FourPerCycle,
    OnePerCycle
}

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum PWMFreq {
    Freq2_683,
    Freq2_512
}

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum PWMOfs {
    Ofs36,
    Ofs00
}

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum OTTRIM {
    C143,
    C150
}

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum RSense {
    External,
    Internal
}

#[test]
fn configuration_from_raw_converts() {
    let test_byte0: u8 = 0b00100000;
    let test_byte1: u8 = 0b01100110;
    let test_byte2: u8 = 0b11110001;

    let actual = OtpConfiguration::from_raw(test_byte0, test_byte1, test_byte2);

    let expected = OtpConfiguration {
        ottrim: OTTRIM::C150,
        internal_rsense: RSense::External,
        tbl: false,

        pwm_grad: PWMGrad::Grad31,
        pwm_autograd: false,
        tpwm_threshold: TPWMThreshold::Threshold400,

        pwm_ofs: PWMOfs::Ofs00,
        pwm_reg: PWMReg::FourPerCycle,
        pwm_freq: PWMFreq::Freq2_683,
        hold_delay: IHoldDelay::Delay4,
        hold: IHold::Hold24,
        stealth_chop_enabled: false
    };

    assert_eq!(expected.ottrim, actual.ottrim);
    assert_eq!(expected.internal_rsense, actual.internal_rsense);
    assert_eq!(expected.tbl, actual.tbl);

    assert_eq!(expected.pwm_grad, actual.pwm_grad);
    assert_eq!(expected.pwm_autograd, actual.pwm_autograd);
    assert_eq!(expected.tpwm_threshold, actual.tpwm_threshold);

    assert_eq!(expected.pwm_ofs, actual.pwm_ofs);
    assert_eq!(expected.pwm_reg, actual.pwm_reg);
    assert_eq!(expected.pwm_freq, actual.pwm_freq);
    assert_eq!(expected.hold_delay, actual.hold_delay);
    assert_eq!(expected.hold, actual.hold);
    assert_eq!(expected.stealth_chop_enabled, actual.stealth_chop_enabled);
}

#[test]
fn configuration_to_raw_without_stealthchop_converts() {

    let test_conf = OtpConfiguration {
        ottrim: OTTRIM::C150,
        internal_rsense: RSense::External,
        tbl: false,

        pwm_grad: num::FromPrimitive::from_u8(0).unwrap(),
        pwm_autograd: false,
        tpwm_threshold: num::FromPrimitive::from_u8(0).unwrap(),

        pwm_ofs: num::FromPrimitive::from_u8(0).unwrap(),
        pwm_reg: PWMReg::FourPerCycle,
        pwm_freq: PWMFreq::Freq2_683,
        hold_delay: IHoldDelay::Delay4,
        hold: IHold::Hold24,
        stealth_chop_enabled: false
    };

    let actual = test_conf.to_raw();

    let expected_byte0: u8 = 0b00100000;
    let expected_byte1: u8 = 0;          // When stealthchop is enabled this byte does nothing
    let expected_byte2: u8 = 0b11110000;

    assert_eq!(expected_byte0, actual.Byte0);
    assert_eq!(expected_byte1, actual.Byte1);
    assert_eq!(expected_byte2, actual.Byte2);
}

#[test]
fn configuration_to_raw_with_stealthchop_converts() {

    let test_conf = OtpConfiguration {
        ottrim: OTTRIM::C150,
        internal_rsense: RSense::External,
        tbl: false,

        pwm_grad: PWMGrad::Grad31,
        pwm_autograd: false,
        tpwm_threshold: TPWMThreshold::Threshold400,

        pwm_ofs: PWMOfs::Ofs00,
        pwm_reg: PWMReg::FourPerCycle,
        pwm_freq: PWMFreq::Freq2_683,
        hold_delay: IHoldDelay::Delay4,
        hold: IHold::Hold24,
        stealth_chop_enabled: true
    };

    let actual = test_conf.to_raw();

    let expected_byte0: u8 = 0b00100000;
    let expected_byte1: u8 = 0b01100110;
    let expected_byte2: u8 = 0b01110001;

    assert_eq!(expected_byte0, actual.Byte0);
    assert_eq!(expected_byte1, actual.Byte1);
    assert_eq!(expected_byte2, actual.Byte2);
}