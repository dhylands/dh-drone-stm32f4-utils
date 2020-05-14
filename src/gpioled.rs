use crate::led::Led;

use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::pin::{GpioPinMap, GpioPinPeriph};

pub struct GpioLedActiveLow<PIN>
where
    PIN: GpioPinMap,
{
    pin: GpioPinPeriph<PIN>,
}

impl<PIN: GpioPinMap> GpioLedActiveLow<PIN> {
    /// Associate a pin with this LED.
    pub fn init(pin: GpioPinPeriph<PIN>) -> Self {
        pin.gpio_moder_moder.write_bits(0b01); // 0b01 - General purpose output mode.
        pin.gpio_otyper_ot.clear_bit(); // 0 = Push-Pull
        pin.gpio_ospeedr_ospeedr.write_bits(0b11); // 0b11 - High Speed

        Self { pin }
    }
}

impl<PIN: GpioPinMap> Led for GpioLedActiveLow<PIN> {
    /// Turns a LED on by setting a GPIO pin low.
    fn on(&self) {
        self.pin.gpio_bsrr_br.set_bit();
    }

    /// Turns a LED off by setting a GPIO pin high.
    fn off(&self) {
        self.pin.gpio_bsrr_bs.set_bit()
    }
}

pub struct GpioLedActiveHigh<PIN>
where
    PIN: GpioPinMap,
{
    pin: GpioPinPeriph<PIN>,
}

impl<PIN: GpioPinMap> GpioLedActiveHigh<PIN> {
    /// Associate a pin with this LED, and indicate if it turns on by driving
    // gpio line low or high.
    pub fn init(pin: GpioPinPeriph<PIN>) -> Self {
        pin.gpio_moder_moder.write_bits(0b01); // 0b01 - General purpose output mode.
        pin.gpio_otyper_ot.clear_bit(); // 0 = Push-Pull
        pin.gpio_ospeedr_ospeedr.write_bits(0b11); // 0b11 - High Speed

        Self { pin }
    }
}

impl<PIN: GpioPinMap> Led for GpioLedActiveHigh<PIN> {
    /// Turns a LED on by setting a GPIO pin high.
    fn on(&self) {
        self.pin.gpio_bsrr_bs.set_bit();
    }

    /// Turns a LED off by setting a GPIO pin low.
    fn off(&self) {
        self.pin.gpio_bsrr_br.set_bit();
    }
}
