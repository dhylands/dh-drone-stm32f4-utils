#![cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]

use drone_cortexm::{fib, fib::ThrFiberFuture, reg::prelude::*, thr::ThrNvic};
use drone_stm32_map::reg;

pub const SYSCLK_SW_HSI: u32 = 0b00;
pub const SYSCLK_SW_HSE: u32 = 0b01;
pub const SYSCLK_SW_PLL: u32 = 0b10;

pub const APB_DIV1: u32 = 0b000;
pub const APB_DIV2: u32 = 0b100;
pub const APB_DIV4: u32 = 0b101;
pub const APB_DIV8: u32 = 0b110;
pub const APB_DIV16: u32 = 0b111;

pub const AHB_DIV1: u32 = 0b0000;
pub const AHB_DIV2: u32 = 0b1000;
pub const AHB_DIV3: u32 = 0b1001;
pub const AHB_DIV8: u32 = 0b1010;
pub const AHB_DIV16: u32 = 0b1011;
pub const AHB_DIV64: u32 = 0b1100;
pub const AHB_DIV128: u32 = 0b1101;
pub const AHB_DIV256: u32 = 0b1110;
pub const AHB_DIV512: u32 = 0b1111;

pub const PLLP_DIV2: u32 = 0b00;

pub trait SystemClock {
    /// Specifies the number of wait states to set for FLASH_ACR register.
    fn flash_latency(&self) -> u32;

    /// HSE crystal frequency.
    fn hse_freq(&self) -> u32;

    /// HSI crystal frequency.
    fn hsi_freq(&self) -> u32;

    // VCO-freq = HSE * (PLLN / PLLM)
    // PLL general clock = VCO-freq / PLLP
    // USB/SDIO/RG freq = VCO-freq / PLLQ

    /// PLLM - Division factor for the main PLL (PLL) input clock
    fn pllm(&self) -> u32;

    /// PLLN - Main PLL multiplication factor for VCO
    fn plln(&self) -> u32;

    /// PLLP - Main PLL division factor for main system clock
    fn pllp(&self) -> u32;

    /// PLLQ - Main PLL division factor for USB OTG FS, and SDIO clocks
    fn pllq(&self) -> u32;
}

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

impl <'a, THR: ThrNvic + ThrFiberFuture> SystemClockRegs<'a, THR> {
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
    pub fn clock(&self) -> u32
    {
        ((self.sysclk.hse_freq() / self.rcc_pllcfgr.pllm.read_bits()) * self.rcc_pllcfgr.plln.read_bits())
            / ((self.rcc_pllcfgr.pllp.read_bits() + 1) * 2)
    }

    /// Returns the current frequency of the SysTick clock.
    pub fn systick_frequency(&self) -> u32
    {
        self.clock() / 8
    }

    /// Sets the system clock frequency based on the constants.
    pub async fn raise_system_frequency(&self)
    {
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
                .write_pllp(self.sysclk.pllp())
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
