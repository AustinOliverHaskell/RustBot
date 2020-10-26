// General Registers
const GCONF       : u8 = 0x00;
const GSTAT       : u8 = 0x01;
const IFCNT       : u8 = 0x02;
const SLAVECONF   : u8 = 0x03;
const OTP_PROG    : u8 = 0x04;
const OTP_READ    : u8 = 0x05;
const IOIN        : u8 = 0x06;
const FACTORY_CONF: u8 = 0x07;

// Velocity Control Registers
const IHOLD_RUN   : u8 = 0x10;
const TPOWER_DOWN : u8 = 0x11;
const TSTEP       : u8 = 0x12;
const TPWMTHRS    : u8 = 0x13;
const VACTUAL     : u8 = 0x22;

// Sequencer Registers
const MSCNT       : u8 = 0x6A;
const MSCURACT    : u8 = 0x6B;

// Chopper Control Registers 
const CHOPCONF    : u8 = 0x6C;
const DRV_STATUS  : u8 = 0x6F;
const PWMCONF     : u8 = 0x70;
const PWM_SCALE   : u8 = 0x71;
const PWM_AUTO    : u8 = 0x72;

// Others
const OTP_MAGIC: u8 = 0xBD;