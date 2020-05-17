//! Register constants used by this crate.

/// Configures the GPIO pin as an input.
pub const MODER_INPUT: u32 = 0b00;

/// Configures the GPIO pin as an output.
pub const MODER_OUTPUT: u32 = 0b01;

/// Configures the GPIO pin as an alternate function.
pub const MODER_AF: u32 = 0b10;

/// Configures the GPIO pin as an analog input.
pub const MODER_ANALOG: u32 = 0b11;

/// Sets the GPIO output speed to low speed.
pub const OSPEEDR_LOW: u32 = 0b00;

/// Sets the GPIO output speed to medium speed.
pub const OSPEEDR_MEDIUM: u32 = 0b01;

/// Sets the GPIO output speed to high speed.
pub const OSPEEDR_HIGH: u32 = 0b10;

/// Sets the GPIO output speed to very high speed.
pub const OSPEEDR_VERY_HIGH: u32 = 0b11;

/// Disables pull-up/pull-down functionality for a particular GPIO input pin.
pub const PUPDR_NONE: u32 = 0b00;

/// Enables pull-up functionality for a particular GPIO input pin.
pub const PUPDR_PULL_UP: u32 = 0b01;

/// Enables pull-down functionality for a particular GPIO input pin.
pub const PUPDR_PULL_DOWN: u32 = 0b10;
