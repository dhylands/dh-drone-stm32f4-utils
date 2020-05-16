use crate::consts::*;
use crate::signal::Signal;

use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::pin::{GpioPinMap, GpioPinPeriph};

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
