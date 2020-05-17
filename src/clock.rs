#![cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]

//! Functions for configuring the various clocks used on the STM32F4xx.

use drone_cortexm::{fib, fib::ThrFiberFuture, reg::prelude::*, thr::ThrNvic};
use drone_stm32_map::reg;

// const SYSCLK_SW_HSI: u32 = 0b00;
// const SYSCLK_SW_HSE: u32 = 0b01;
const SYSCLK_SW_PLL: u32 = 0b10;
// const APB_DIV1: u32 = 0b000;
const APB_DIV2: u32 = 0b100;
const APB_DIV4: u32 = 0b101;
// const APB_DIV8: u32 = 0b110;
// const APB_DIV16: u32 = 0b111;
const AHB_DIV1: u32 = 0b0000;
// const AHB_DIV2: u32 = 0b1000;
// const AHB_DIV3: u32 = 0b1001;
// const AHB_DIV8: u32 = 0b1010;
// const AHB_DIV16: u32 = 0b1011;
// const AHB_DIV64: u32 = 0b1100;
// const AHB_DIV128: u32 = 0b1101;
// const AHB_DIV256: u32 = 0b1110;
// const AHB_DIV512: u32 = 0b1111;

const AHB_PRESCALER: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 6, 7, 8, 9];
// const APB_PRESCALER: [u8; 8] = [0, 0, 0, 0, 1, 2, 3, 4];


/// A trait that provides the constants needed to setup the main clocks
/// used on the STM32F4xx.
///
/// These clocks are:
/// * System clock. This is used to derive the AHB, APB1, and APB2 clocks.
/// * Bus clock (also referred to as AHB or HCLK) which is the
///   clock that the CPU runs at.
/// * Low speed peripheral clock (APB1).
/// * High speed peripheral clock (APB2).
/// * USB/SDIO/RNG clock.
///
/// The VCO-clock is calculated as the HSE-frequency * PLLN / PLLM.
///
/// The system-clock is calculated as the VCO-clock / PLLP.
///
/// The USB/SDIO/RNG-clock is calclated as the system-clock / PLLQ.
///
/// The AHB clock is equal to the system clock.
///
/// The APB1 clock is equal to the AHB clock divided by 4.
///
/// The APB2 clock is equal to the AHB clock divided by 2.
///
/// There are lots of constraints on the frequencies that the various clocks
/// can have so you should consult the datasheet for the particular processor
/// you're using to determine these constraints.
///
/// # Example
///
/// This example configures the WeAct STM32F411CEU6 board to run at 96 MHz.
/// The WeAct STM32F411CEU6 board has a 25 MHz HSE crystal.
///
/// ```rust
/// impl SystemClock for WeActSystemClock {
///     fn flash_latency(&self) -> u32 {
///         3
///     }
///
///     fn hse_freq(&self) -> u32 {
///         25_000_000
///     }
///
///     fn pllm(&self) -> u32 {
///         25
///     }
///
///     fn plln(&self) -> u32 {
///         192
///     }
///
///     fn pllp(&self) -> u32 {
///         2
///     }
///
///     fn pllq(&self) -> u32 {
///         4
///     }
/// }
/// ```
/// This will result in the following frequencies:
///
/// | Clock | Frequency |
/// | ----- | --------- |
/// | SYSCLK | 96 MHz |
/// | AHB | 96 MHz |
/// | APB1 | 24 MHz |
/// | APB2 | 48 MHz |
/// | USB | 48 MHz |
///
/// While the STM32F411 can go upto 100 MHz, we limit it to 96 MHz so that we
/// can get a "nice" frequency for the USB clock (which should be a multiple
/// of 24 MHz).
pub trait SystemClock {
    /// Specifies the number of wait states to set for FLASH_ACR register. This will
    /// depend on the MCU voltage and system clock speed.
    fn flash_latency(&self) -> u32;

    /// The frequency of the external high speed crystal.
    fn hse_freq(&self) -> u32;

    // VCO-freq = HSE * (PLLN / PLLM)
    // PLL general clock = VCO-freq / PLLP
    // USB/SDIO/RG freq = VCO-freq / PLLQ

    /// PLLM - Division factor for the main PLL (PLL) input clock.
    fn pllm(&self) -> u32;

    /// PLLN - PLL multiplication factor for VCO. The VCO frequency
    /// is calculates as HSE frequency * PLLN / PLLM.
    fn plln(&self) -> u32;

    /// PLLP - PLL division factor for main system clock.
    fn pllp(&self) -> u32;

    /// PLLQ - PLL division factor for USB OTG FS, and SDIO clocks
    fn pllq(&self) -> u32;
}

/// The SystemClockRegs struct stores all of the registers needed to setup
/// the clocks.
pub struct SystemClockRegs<'a, THR: ThrNvic + ThrFiberFuture> {
    flash_acr: reg::flash::Acr<Srt>,
    pwr_cr: reg::pwr::Cr<Srt>,
    rcc_cr: reg::rcc::Cr<Srt>,
    rcc_pllcfgr: reg::rcc::Pllcfgr<Srt>,
    rcc_cfgr: reg::rcc::Cfgr<Srt>,
    rcc_cir: reg::rcc::Cir<Crt>,
    rcc_apb1enr: reg::rcc::Apb1Enr<Srt>,
    thr_rcc: THR,
    sysclk: &'a dyn SystemClock,
}

impl<'a, THR: ThrNvic + ThrFiberFuture> SystemClockRegs<'a, THR> {
    /// Constructs a SystemClockRegs object from individual registers.
    ///
    /// # Example
    /// ```rust
    /// pub fn handler(reg: Regs, thr_init: ThrsInit) {
    ///     let thr = thr::init(thr_init);
    ///     let sysclk = WeActSystemClock::init();
    ///
    ///     let sysclk_regs = SystemClockRegs::init(
    ///         reg.flash_acr,
    ///         reg.pwr_cr,
    ///         reg.rcc_cr,
    ///         reg.rcc_pllcfgr,
    ///         reg.rcc_cfgr,
    ///         reg.rcc_cir.into_copy(),
    ///         reg.rcc_apb1enr,
    ///         thr.rcc,
    ///         &sysclk,
    ///     );
    ///     sysclk_regs.raise_system_frequency().root_wait();
    ///     // ...
    /// }
    /// ```
    pub fn init(
        flash_acr: reg::flash::Acr<Srt>,
        pwr_cr: reg::pwr::Cr<Srt>,
        rcc_cr: reg::rcc::Cr<Srt>,
        rcc_pllcfgr: reg::rcc::Pllcfgr<Srt>,
        rcc_cfgr: reg::rcc::Cfgr<Srt>,
        rcc_cir: reg::rcc::Cir<Crt>,
        rcc_apb1enr: reg::rcc::Apb1Enr<Srt>,
        thr_rcc: THR,
        sysclk: &'a dyn SystemClock,
    ) -> Self {
        Self {
            flash_acr,
            pwr_cr,
            rcc_cr,
            rcc_pllcfgr,
            rcc_cfgr,
            rcc_cir,
            rcc_apb1enr,
            thr_rcc,
            sysclk,
        }
    }

    /// Returns the current frequency of the system clock.
    pub fn clock(&self) -> u32 {
        ((self.sysclk.hse_freq() / self.rcc_pllcfgr.pllm.read_bits())
            * self.rcc_pllcfgr.plln.read_bits())
            / ((self.rcc_pllcfgr.pllp.read_bits() + 1) * 2)
    }

    /// Returns the current AHB frequency.
    pub fn ahb_frequency(&self) -> u32 {
        self.clock() / ((1 << AHB_PRESCALER[self.rcc_cfgr.hpre.read_bits() as usize]) as u32)
    }

    /// Returns the current frequency of the SysTick clock.
    pub fn systick_frequency(&self) -> u32 {
       self.ahb_frequency() / 8
    }

    /// Sets the system clock frequency based on the constants implemented
    /// in the SystemClock trait.
    pub async fn raise_system_frequency(&self) {
        // Enable Power Control Clock.
        self.rcc_apb1enr.pwren.set_bit();

        // Set VOS field in PWR_CR register to Scale 1 (0b11) (HCLK <= 100 MHz)
        self.pwr_cr.modify(|r| r.write_vos(0b11));

        self.thr_rcc.enable_int();
        self.rcc_cir.modify(|r| r.set_hserdyie().set_pllrdyie());

        // We need to move ownership of `hserdyc` and `hserdyf` into the fiber.
        let reg::rcc::Cir {
            hserdyc, hserdyf, ..
        } = self.rcc_cir;
        // Attach a listener that will notify us when RCC_CIR_HSERDYF is asserted.
        let hserdy = self.thr_rcc.add_future(fib::new_fn(move || {
            if hserdyf.read_bit() {
                hserdyc.set_bit();
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }));

        // Turn on the HSE and wait for it to become ready
        self.rcc_cr.hseon.set_bit();
        // Sleep until RCC_CIR_HSERDYF is asserted.
        hserdy.await;

        // We need to move ownership of `pllrdyc` and `pllrdyf` into the fiber.
        let reg::rcc::Cir {
            pllrdyc, pllrdyf, ..
        } = self.rcc_cir;
        // Attach a listener that will notify us when RCC_CIR_PLLRDYF is asserted.
        let pllrdy = self.thr_rcc.add_future(fib::new_fn(move || {
            if pllrdyf.read_bit() {
                pllrdyc.set_bit();
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }));

        self.rcc_pllcfgr.modify(|r| {
            r.set_pllsrc() // HSE oscillator clock selected as PLL input clock
                .write_pllm(self.sysclk.pllm())
                .write_plln(self.sysclk.plln())
                .write_pllp(self.sysclk.pllp() / 2 - 1)
                .write_pllq(self.sysclk.pllq())
        });
        // Enable the PLL.
        self.rcc_cr.modify(|r| r.set_pllon());
        // Sleep until RCC_CIR_PLLRDYF is asserted.
        pllrdy.await;

        // The power-on reset latency is set to zero. Since we're going to be increasing
        // the clock speed, increase the latency before increasing the clock speed.
        let flash_latency = self.sysclk.flash_latency();
        self.flash_acr.modify(|r| r.write_latency(flash_latency));
        if self.flash_acr.load().latency() != flash_latency {
            panic!("LATENCY");
        }

        self.rcc_cfgr.modify(|r| {
            r.write_ppre1(APB_DIV4) //    APB1 = AHB / 4 = 24 MHz (must be < 45 MHz)
                .write_ppre2(APB_DIV2) // APB2 = AHB / 2 = 48 MHz (must be < 90 MHz)
                .write_hpre(AHB_DIV1) //  AHB = SYSCLK / 1 = 96 MHz
                .write_sw(SYSCLK_SW_PLL) // Use PLL for System Clock - 96 MHz
        });

        if self.rcc_cfgr.load().sws() != SYSCLK_SW_PLL {
            panic!("SYSCLK not set to HSE");
        }
    }
}
