pub const MODER_INPUT: u32 = 0b00;
pub const MODER_OUTPUT: u32 = 0b01;
pub const MODER_AF: u32 = 0b10;
pub const MODER_ANALOG: u32 = 0b11;

pub const OTYPER_PUSH_PULL: u32 = 0;
pub const OTYPER_OPEN_DRAIN: u32 = 1;

pub const OSPEEDR_LOW: u32 = 0b00;
pub const OSPEEDR_MEDIUM: u32 = 0b01;
pub const OSPEEDR_HIGH: u32 = 0b10;
pub const OSPEEDR_VERY_HIGH: u32 = 0b11;

pub const PUPDR_NONE: u32 = 0b00;
pub const PUPDR_PULL_UP: u32 = 0b01;
pub const PUPDR_PULL_DOWN: u32 = 0b10;
