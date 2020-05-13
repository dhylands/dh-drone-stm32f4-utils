/// A trait for control an LED.
pub trait Led {
    /// Turns the LED on.
    fn on(&self);

    /// Turns the LED off.
    fn off(&self);
}
