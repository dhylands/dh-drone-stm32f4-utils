use crate::led::Led;

use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::pin::{GpioPinMap, GpioPinPeriph};

pub enum Active {
    Low,
    High,
}

pub struct GpioLed<PIN>
where
    PIN: GpioPinMap,
{
    pin: GpioPinPeriph<PIN>,
    active: Active,
}

impl<PIN: GpioPinMap> GpioLed<PIN> {
    /// Associate a pin with this LED, and indicate if it turns on by driving
    // gpio line low or high.
    pub fn init(pin: GpioPinPeriph<PIN>, active: Active) -> Self {
        pin.gpio_moder_moder.write_bits(0b01); // 0b01 - General purpose output mode.
        pin.gpio_otyper_ot.clear_bit(); // 0 = Push-Pull
        pin.gpio_ospeedr_ospeedr.write_bits(0b11); // 0b11 - High Speed

        Self { pin, active }
    }
}

impl<PIN: GpioPinMap> Led for GpioLed<PIN> {
    /// Turns a LED on by manipulating a GPIO pin.
    fn on(&self) {
        match self.active {
            Active::Low => self.pin.gpio_bsrr_br.set_bit(),
            Active::High => self.pin.gpio_bsrr_bs.set_bit(),
        }
    }

    /// Turns a LED off by manipulating a GPIO pin.
    fn off(&self) {
        match self.active {
            Active::Low => self.pin.gpio_bsrr_bs.set_bit(),
            Active::High => self.pin.gpio_bsrr_br.set_bit(),
        }
    }
}
