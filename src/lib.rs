#![no_std]

use embedded::components::gpio::stm32f7::Gpio;

pub extern crate embedded;
extern crate volatile;
extern crate bit_field;
#[macro_use]
extern crate once;

pub mod adc;
pub mod c_adc;
pub mod can;
pub mod cec;
pub mod crc;
pub mod cryp;
pub mod dac;
pub mod dbg;
pub mod dcmi;
pub mod dma;
pub mod dma2d;
pub mod ethernet_mac;
pub mod ethernet_mmc;
pub mod ethernet_ptp;
pub mod ethernet_dma;
pub mod exti;
pub mod flash;
pub mod fmc;
pub mod hash;
pub mod i2c;
pub mod iwdg;
pub mod lptim;
pub mod ltdc;
pub mod nvic;
pub mod otg_fs_global;
pub mod otg_fs_host;
pub mod otg_fs_device;
pub mod otg_fs_pwrclk;
pub mod otg_hs_global;
pub mod otg_hs_host;
pub mod otg_hs_device;
pub mod otg_hs_pwrclk;
pub mod pwr;
pub mod quadspi;
pub mod rcc;
pub mod rng;
pub mod rtc;
pub mod sai;
pub mod sdmmc;
pub mod spi;
pub mod spdif_rx;
pub mod syscfg;
pub mod tim1;
pub mod tim8;
pub mod tim2;
pub mod tim3;
pub mod tim4;
pub mod tim5;
pub mod tim9;
pub mod tim12;
pub mod tim10;
pub mod tim11;
pub mod tim13;
pub mod tim14;
pub mod tim6;
pub mod tim7;
pub mod usart;
pub mod wwdg;

pub struct Hardware {
    pub adc_1: &'static mut adc::Adc,
    pub adc_2: &'static mut adc::Adc,
    pub adc_3: &'static mut adc::Adc,
    pub c_adc: &'static mut c_adc::CAdc,
    pub can_1: &'static mut can::Can,
    pub can_2: &'static mut can::Can,
    pub cec: &'static mut cec::Cec,
    pub crc: &'static mut crc::Crc,
    pub cryp: &'static mut cryp::Cryp,
    pub dac: &'static mut dac::Dac,
    pub dbg: &'static mut dbg::Dbg,
    pub dcmi: &'static mut dcmi::Dcmi,
    pub dma_1: &'static mut dma::Dma,
    pub dma_2: &'static mut dma::Dma,
    pub dma2d: &'static mut dma2d::Dma2d,
    pub ethernet_dma: &'static mut ethernet_dma::EthernetDma,
    pub ethernet_mac: &'static mut ethernet_mac::EthernetMac,
    pub ethernet_mmc: &'static mut ethernet_mmc::EthernetMmc,
    pub ethernet_ptp: &'static mut ethernet_ptp::EthernetPtp,
    pub exti: &'static mut exti::Exti,
    pub flash: &'static mut flash::Flash,
    pub fmc: &'static mut fmc::Fmc,
    pub gpio_a: &'static mut Gpio,
    pub gpio_b: &'static mut Gpio,
    pub gpio_c: &'static mut Gpio,
    pub gpio_d: &'static mut Gpio,
    pub gpio_e: &'static mut Gpio,
    pub gpio_f: &'static mut Gpio,
    pub gpio_g: &'static mut Gpio,
    pub gpio_h: &'static mut Gpio,
    pub gpio_i: &'static mut Gpio,
    pub gpio_j: &'static mut Gpio,
    pub gpio_k: &'static mut Gpio,
    pub hash: &'static mut hash::Hash,
    pub i2c_1: &'static mut i2c::I2c,
    pub i2c_2: &'static mut i2c::I2c,
    pub i2c_3: &'static mut i2c::I2c,
    pub i2c_4: &'static mut i2c::I2c,
    pub iwdg: &'static mut iwdg::Iwdg,
    pub lptim: &'static mut lptim::Lptim,
    pub ltdc: &'static mut ltdc::Ltdc,
    pub nvic: &'static mut nvic::Nvic,
    pub otg_fs_device: &'static mut otg_fs_device::OtgFsDevice,
    pub otg_fs_global: &'static mut otg_fs_global::OtgFsGlobal,
    pub otg_fs_host: &'static mut otg_fs_host::OtgFsHost,
    pub otg_fs_pwrclk: &'static mut otg_fs_pwrclk::OtgFsPwrclk,
    pub otg_hs_device: &'static mut otg_hs_device::OtgHsDevice,
    pub otg_hs_global: &'static mut otg_hs_global::OtgHsGlobal,
    pub otg_hs_host: &'static mut otg_hs_host::OtgHsHost,
    pub otg_hs_pwrclk: &'static mut otg_hs_pwrclk::OtgHsPwrclk,
    pub pwr: &'static mut pwr::Pwr,
    pub quadspi: &'static mut quadspi::Quadspi,
    pub rcc: &'static mut rcc::Rcc,
    pub rng: &'static mut rng::Rng,
    pub rtc: &'static mut rtc::Rtc,
    pub sai_1: &'static mut sai::Sai,
    pub sai_2: &'static mut sai::Sai,
    pub sdmmc: &'static mut sdmmc::Sdmmc,
    pub spdif_rx: &'static mut spdif_rx::SpdifRx,
    pub spi_1: &'static mut spi::Spi,
    pub spi_2: &'static mut spi::Spi,
    pub spi_3: &'static mut spi::Spi,
    pub spi_4: &'static mut spi::Spi,
    pub spi_5: &'static mut spi::Spi,
    pub spi_6: &'static mut spi::Spi,
    pub syscfg: &'static mut syscfg::Syscfg,
    pub tim1: &'static mut tim1::Tim1,
    pub tim8: &'static mut tim8::Tim8,
    pub tim2: &'static mut tim2::Tim2,
    pub tim3: &'static mut tim3::Tim3,
    pub tim4: &'static mut tim4::Tim4,
    pub tim5: &'static mut tim5::Tim5,
    pub tim9: &'static mut tim9::Tim9,
    pub tim12: &'static mut tim12::Tim12,
    pub tim10: &'static mut tim10::Tim10,
    pub tim11: &'static mut tim11::Tim11,
    pub tim13: &'static mut tim13::Tim13,
    pub tim14: &'static mut tim14::Tim14,
    pub tim6: &'static mut tim6::Tim6,
    pub tim7: &'static mut tim7::Tim7,
    pub uart_4: &'static mut usart::Usart,
    pub uart_5: &'static mut usart::Usart,
    pub uart_7: &'static mut usart::Usart,
    pub uart_8: &'static mut usart::Usart,
    pub usart_1: &'static mut usart::Usart,
    pub usart_2: &'static mut usart::Usart,
    pub usart_3: &'static mut usart::Usart,
    pub usart_6: &'static mut usart::Usart,
    pub wwdg: &'static mut wwdg::Wwdg,
}

pub fn hw() -> Hardware {
    assert_has_not_been_called!();
    Hardware {
        rng: unsafe { from_addr(0x50060800) },
        hash: unsafe { from_addr(0x50060400) },
        cryp: unsafe { from_addr(0x50060000) },
        dcmi: unsafe { from_addr(0x50050000) },
        fmc: unsafe { from_addr(0xa0000000) },
        dbg: unsafe { from_addr(0xe0042000) },
        dma_2: unsafe { from_addr(0x40026400) },
        dma_1: unsafe { from_addr(0x40026000) },
        rcc: unsafe { from_addr(0x40023800) },
        gpio_d: unsafe { from_addr(0x40020c00) },
        gpio_c: unsafe { from_addr(0x40020800) },
        gpio_k: unsafe { from_addr(0x40022800) },
        gpio_j: unsafe { from_addr(0x40022400) },
        gpio_i: unsafe { from_addr(0x40022000) },
        gpio_h: unsafe { from_addr(0x40021c00) },
        gpio_g: unsafe { from_addr(0x40021800) },
        gpio_f: unsafe { from_addr(0x40021400) },
        gpio_e: unsafe { from_addr(0x40021000) },
        gpio_b: unsafe { from_addr(0x40020400) },
        gpio_a: unsafe { from_addr(0x40020000) },
        syscfg: unsafe { from_addr(0x40013800) },
        spi_1: unsafe { from_addr(0x40013000) },
        spi_2: unsafe { from_addr(0x40003800) },
        spi_3: unsafe { from_addr(0x40003c00) },
        spi_4: unsafe { from_addr(0x40013400) },
        spi_5: unsafe { from_addr(0x40015000) },
        spi_6: unsafe { from_addr(0x40015400) },
        adc_1: unsafe { from_addr(0x40012000) },
        adc_2: unsafe { from_addr(0x40012100) },
        adc_3: unsafe { from_addr(0x40012200) },
        dac: unsafe { from_addr(0x40007400) },
        pwr: unsafe { from_addr(0x40007000) },
        iwdg: unsafe { from_addr(0x40003000) },
        wwdg: unsafe { from_addr(0x40002c00) },
        c_adc: unsafe { from_addr(0x40012300) },
        tim1: unsafe { from_addr(0x40010000) },
        tim8: unsafe { from_addr(0x40010400) },
        tim2: unsafe { from_addr(0x40000000) },
        tim3: unsafe { from_addr(0x40000400) },
        tim4: unsafe { from_addr(0x40000800) },
        tim5: unsafe { from_addr(0x40000c00) },
        tim9: unsafe { from_addr(0x40014000) },
        tim12: unsafe { from_addr(0x40001800) },
        tim10: unsafe { from_addr(0x40014400) },
        tim11: unsafe { from_addr(0x40014800) },
        tim13: unsafe { from_addr(0x40001c00) },
        tim14: unsafe { from_addr(0x40002000) },
        tim6: unsafe { from_addr(0x40001000) },
        tim7: unsafe { from_addr(0x40001400) },
        ethernet_mac: unsafe { from_addr(0x40028000) },
        ethernet_mmc: unsafe { from_addr(0x40028100) },
        ethernet_ptp: unsafe { from_addr(0x40028700) },
        ethernet_dma: unsafe { from_addr(0x40029000) },
        crc: unsafe { from_addr(0x40023000) },
        can_1: unsafe { from_addr(0x40006400) },
        can_2: unsafe { from_addr(0x40006800) },
        nvic: unsafe { from_addr(0xe000e000) },
        flash: unsafe { from_addr(0x40023c00) },
        exti: unsafe { from_addr(0x40013c00) },
        ltdc: unsafe { from_addr(0x40016800) },
        sai_1: unsafe { from_addr(0x40015800) },
        sai_2: unsafe { from_addr(0x40015c00) },
        dma2d: unsafe { from_addr(0x4002b000) },
        quadspi: unsafe { from_addr(0xa0001000) },
        cec: unsafe { from_addr(0x40006c00) },
        spdif_rx: unsafe { from_addr(0x40004000) },
        sdmmc: unsafe { from_addr(0x40012c00) },
        lptim: unsafe { from_addr(0x40002400) },
        i2c_1: unsafe { from_addr(0x40005400) },
        i2c_2: unsafe { from_addr(0x40005800) },
        i2c_3: unsafe { from_addr(0x40005c00) },
        i2c_4: unsafe { from_addr(0x40006000) },
        rtc: unsafe { from_addr(0x40002800) },
        usart_6: unsafe { from_addr(0x40011400) },
        usart_1: unsafe { from_addr(0x40011000) },
        usart_3: unsafe { from_addr(0x40004800) },
        usart_2: unsafe { from_addr(0x40004400) },
        uart_5: unsafe { from_addr(0x40005000) },
        uart_4: unsafe { from_addr(0x40004c00) },
        uart_8: unsafe { from_addr(0x40007c00) },
        uart_7: unsafe { from_addr(0x40007800) },
        otg_fs_global: unsafe { from_addr(0x50000000) },
        otg_fs_host: unsafe { from_addr(0x50000400) },
        otg_fs_device: unsafe { from_addr(0x50000800) },
        otg_fs_pwrclk: unsafe { from_addr(0x50000e00) },
        otg_hs_global: unsafe { from_addr(0x40040000) },
        otg_hs_host: unsafe { from_addr(0x40040400) },
        otg_hs_device: unsafe { from_addr(0x40040800) },
        otg_hs_pwrclk: unsafe { from_addr(0x40040e00) },
    }
}

unsafe fn from_addr<T>(addr: usize) -> &'static mut T {
    &mut *(addr as *const T as *mut T)
}
