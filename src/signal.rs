//! A trait for controlling a signal (i.e. a gpio output like a relay or LED).
//!
/// A Signal has the notion of being on or off which is independent of how that
/// signal is manifested in the hardware.
///
/// Some signals will be on when the output is a logic high and some signals
/// will be on when the output is a logic low.
pub trait Signal {
    /// Turns the signal on.
    fn on(&self);

    /// Turns the signal off.
    fn off(&self);
}
