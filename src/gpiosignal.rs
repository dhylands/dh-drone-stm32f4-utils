//! Some concrete implementations of the Signal trait for GPIO pins.

use crate::consts::*;
use crate::signal::Signal;

use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::pin::{GpioPinMap, GpioPinPeriph};

/// Implements a Signal which is on when a GPIO pin is at a logic low.
///
/// # Example
///
/// On the WeAct SMT32F411CEU6 board, the onboard LED is wired between
/// VCC and CPU pin PC13. This means that to turn the LED on, you need
/// to drive GPIO pin PC13 low.
///
/// ```rust
///     let led = GpioSignalActiveLow::init(periph_gpio_c13!(reg));
///     led.on()
/// ```
pub struct GpioSignalActiveLow<PIN>
where
    PIN: GpioPinMap,
{
    pin: GpioPinPeriph<PIN>,
}

impl<PIN: GpioPinMap> GpioSignalActiveLow<PIN> {
    /// Associate a pin with this Signal. Turning this type of Signal on
    /// will set the GPIO Pin low.
    pub fn init(pin: GpioPinPeriph<PIN>) -> Self {
        pin.gpio_moder_moder.write_bits(MODER_OUTPUT);
        pin.gpio_otyper_ot.clear_bit(); // 0 = Push-Pull
        pin.gpio_ospeedr_ospeedr.write_bits(OSPEEDR_VERY_HIGH);

        Self { pin }
    }
}

impl<PIN: GpioPinMap> Signal for GpioSignalActiveLow<PIN> {
    /// Turns a Signal on by setting a GPIO pin low.
    fn on(&self) {
        self.pin.gpio_bsrr_br.set_bit();
    }

    /// Turns a Signal off by setting a GPIO pin high.
    fn off(&self) {
        self.pin.gpio_bsrr_bs.set_bit()
    }
}

/// Implements a Signal which is on when a GPIO pin is at a logic high.
///
/// # Example
///
/// If an LED is wired between CPU pin PB5 and GND, then you would turn
/// the LED on by setting pin PB5 to a logic high.
///
/// ```rust
///     let led = GpioSignalActiveHigh::init(periph_gpio_b5!(reg));
///     led.on()
/// ```
pub struct GpioSignalActiveHigh<PIN>
where
    PIN: GpioPinMap,
{
    pin: GpioPinPeriph<PIN>,
}

impl<PIN: GpioPinMap> GpioSignalActiveHigh<PIN> {
    /// Associate a pin with this Signal. Turning this type of Signal on
    /// will set the GPIO Pin high.
    pub fn init(pin: GpioPinPeriph<PIN>) -> Self {
        pin.gpio_moder_moder.write_bits(MODER_OUTPUT);
        pin.gpio_otyper_ot.clear_bit(); // 0 = Push-Pull
        pin.gpio_ospeedr_ospeedr.write_bits(OSPEEDR_VERY_HIGH);

        Self { pin }
    }
}

impl<PIN: GpioPinMap> Signal for GpioSignalActiveHigh<PIN> {
    /// Turns a Signal on by setting a GPIO pin high.
    fn on(&self) {
        self.pin.gpio_bsrr_bs.set_bit();
    }

    /// Turns a Signal off by setting a GPIO pin low.
    fn off(&self) {
        self.pin.gpio_bsrr_br.set_bit();
    }
}
