// TODO figure out a way to add GPIO constants

// PUT CODE IN RUN OR TEST MODE
pub const TUNE_MODE: bool = false;
    
// CONSTANTS FOR SERVO ANGLES
pub const LOOK_LEFT_RIGHT_MIN: u32 = 0;
pub const LOOK_LEFT_RIGHT_MAX: u32 = 180;

pub const LOOK_UP_DOWN_MIN: u32 = 0;
pub const LOOK_UP_DOWN_MAX: u32 = 180;

pub const LOOK_LEFT_TOP_MIN: u32 = 0;
pub const LOOK_LEFT_TOP_MAX: u32 = 180;

pub const LOOK_LEFT_BOTTOM_MIN: u32 = 0;
pub const LOOK_LEFT_BOTTOM_MAX: u32 = 180;

pub const LOOK_RIGHT_TOP_MIN: u32 = 0;
pub const LOOK_RIGHT_TOP_MAX: u32 = 180;

pub const LOOK_RIGHT_BOTTOM_MIN: u32 = 0;
pub const LOOK_RIGHT_BOTTOM_MAX: u32 = 180;

// ONLY CHANGE IF YOU HAVE ANOTHER I2C DEVICE WITH SAME ADDRESS
pub const I2C_ADDRESS: u8 = 0x43;  

