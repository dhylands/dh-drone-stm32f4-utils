/// A trait for control a signal (i.e. a gpio output like a relay or LED).
pub trait Signal {
    /// Turns the signal on.
    fn on(&self);

    /// Turns the signal off.
    fn off(&self);
}
