//! A crate of common code for STM32F4xx microcontrollers using
//! [Drone OS](https://drone-os.com/).
//!
//! # Usage
//!
//! Add the crate to your `Cargo.toml` dependencies:
//!
//! ```toml
//! [dependencies]
//! drone-stm32f4-utils = { git = "https://github.com/dhylands/drone-stm32f4-utils" }
//! ```

#![no_std]
#![warn(missing_docs)]

pub mod clock;
pub mod consts;
pub mod gpiosignal;
pub mod signal;
