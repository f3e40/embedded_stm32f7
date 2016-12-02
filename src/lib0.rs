#![allow(unused_imports)]

#![no_std]

extern crate volatile;
#[macro_use]
extern crate once;
extern crate bit_field;

pub mod rng;
pub mod hash;
pub mod cryp;
pub mod dcmi;
pub mod fmc;
pub mod dbg;
pub mod dma2;
pub mod dma1;
pub mod rcc;
pub mod gpiod;
pub mod gpioc;
pub mod gpiok;
pub mod gpioj;
pub mod gpioi;
pub mod gpioh;
pub mod gpiog;
pub mod gpiof;
pub mod gpioe;
pub mod gpiob;
pub mod gpioa;
pub mod syscfg;
pub mod spi1;
pub mod spi2;
pub mod spi3;
pub mod spi4;
pub mod spi5;
pub mod spi6;
pub mod adc1;
pub mod adc2;
pub mod adc3;
pub mod dac;
pub mod pwr;
pub mod iwdg;
pub mod wwdg;
pub mod c_adc;
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
pub mod ethernet_mac;
pub mod ethernet_mmc;
pub mod ethernet_ptp;
pub mod ethernet_dma;
pub mod crc;
pub mod can1;
pub mod can2;
pub mod nvic;
pub mod flash;
pub mod exti;
pub mod ltdc;
pub mod sai1;
pub mod sai2;
pub mod dma2d;
pub mod quadspi;
pub mod cec;
pub mod spdif_rx;
pub mod sdmmc;
pub mod lptim1;
pub mod i2c1;
pub mod i2c2;
pub mod i2c3;
pub mod i2c4;
pub mod rtc;
pub mod usart6;
pub mod usart1;
pub mod usart3;
pub mod usart2;
pub mod uart5;
pub mod uart4;
pub mod uart8;
pub mod uart7;
pub mod otg_fs_global;
pub mod otg_fs_host;
pub mod otg_fs_device;
pub mod otg_fs_pwrclk;
pub mod otg_hs_global;
pub mod otg_hs_host;
pub mod otg_hs_device;
pub mod otg_hs_pwrclk;

pub struct Hardware {
    pub rng: &'static mut rng::Rng,
    pub hash: &'static mut hash::Hash,
    pub cryp: &'static mut cryp::Cryp,
    pub dcmi: &'static mut dcmi::Dcmi,
    pub fmc: &'static mut fmc::Fmc,
    pub dbg: &'static mut dbg::Dbg,
    pub dma2: &'static mut dma2::Dma2,
    pub dma1: &'static mut dma1::Dma1,
    pub rcc: &'static mut rcc::Rcc,
    pub gpiod: &'static mut gpiod::Gpiod,
    pub gpioc: &'static mut gpioc::Gpioc,
    pub gpiok: &'static mut gpiok::Gpiok,
    pub gpioj: &'static mut gpioj::Gpioj,
    pub gpioi: &'static mut gpioi::Gpioi,
    pub gpioh: &'static mut gpioh::Gpioh,
    pub gpiog: &'static mut gpiog::Gpiog,
    pub gpiof: &'static mut gpiof::Gpiof,
    pub gpioe: &'static mut gpioe::Gpioe,
    pub gpiob: &'static mut gpiob::Gpiob,
    pub gpioa: &'static mut gpioa::Gpioa,
    pub syscfg: &'static mut syscfg::Syscfg,
    pub spi1: &'static mut spi1::Spi1,
    pub spi2: &'static mut spi2::Spi2,
    pub spi3: &'static mut spi3::Spi3,
    pub spi4: &'static mut spi4::Spi4,
    pub spi5: &'static mut spi5::Spi5,
    pub spi6: &'static mut spi6::Spi6,
    pub adc1: &'static mut adc1::Adc1,
    pub adc2: &'static mut adc2::Adc2,
    pub adc3: &'static mut adc3::Adc3,
    pub dac: &'static mut dac::Dac,
    pub pwr: &'static mut pwr::Pwr,
    pub iwdg: &'static mut iwdg::Iwdg,
    pub wwdg: &'static mut wwdg::Wwdg,
    pub c_adc: &'static mut c_adc::CAdc,
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
    pub ethernet_mac: &'static mut ethernet_mac::EthernetMac,
    pub ethernet_mmc: &'static mut ethernet_mmc::EthernetMmc,
    pub ethernet_ptp: &'static mut ethernet_ptp::EthernetPtp,
    pub ethernet_dma: &'static mut ethernet_dma::EthernetDma,
    pub crc: &'static mut crc::Crc,
    pub can1: &'static mut can1::Can1,
    pub can2: &'static mut can2::Can2,
    pub nvic: &'static mut nvic::Nvic,
    pub flash: &'static mut flash::Flash,
    pub exti: &'static mut exti::Exti,
    pub ltdc: &'static mut ltdc::Ltdc,
    pub sai1: &'static mut sai1::Sai1,
    pub sai2: &'static mut sai2::Sai2,
    pub dma2d: &'static mut dma2d::Dma2d,
    pub quadspi: &'static mut quadspi::Quadspi,
    pub cec: &'static mut cec::Cec,
    pub spdif_rx: &'static mut spdif_rx::SpdifRx,
    pub sdmmc: &'static mut sdmmc::Sdmmc,
    pub lptim1: &'static mut lptim1::Lptim1,
    pub i2c1: &'static mut i2c1::I2c1,
    pub i2c2: &'static mut i2c2::I2c2,
    pub i2c3: &'static mut i2c3::I2c3,
    pub i2c4: &'static mut i2c4::I2c4,
    pub rtc: &'static mut rtc::Rtc,
    pub usart6: &'static mut usart6::Usart6,
    pub usart1: &'static mut usart1::Usart1,
    pub usart3: &'static mut usart3::Usart3,
    pub usart2: &'static mut usart2::Usart2,
    pub uart5: &'static mut uart5::Uart5,
    pub uart4: &'static mut uart4::Uart4,
    pub uart8: &'static mut uart8::Uart8,
    pub uart7: &'static mut uart7::Uart7,
    pub otg_fs_global: &'static mut otg_fs_global::OtgFsGlobal,
    pub otg_fs_host: &'static mut otg_fs_host::OtgFsHost,
    pub otg_fs_device: &'static mut otg_fs_device::OtgFsDevice,
    pub otg_fs_pwrclk: &'static mut otg_fs_pwrclk::OtgFsPwrclk,
    pub otg_hs_global: &'static mut otg_hs_global::OtgHsGlobal,
    pub otg_hs_host: &'static mut otg_hs_host::OtgHsHost,
    pub otg_hs_device: &'static mut otg_hs_device::OtgHsDevice,
    pub otg_hs_pwrclk: &'static mut otg_hs_pwrclk::OtgHsPwrclk,
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
        dma2: unsafe { from_addr(0x40026400) },
        dma1: unsafe { from_addr(0x40026000) },
        rcc: unsafe { from_addr(0x40023800) },
        gpiod: unsafe { from_addr(0x40020c00) },
        gpioc: unsafe { from_addr(0x40020800) },
        gpiok: unsafe { from_addr(0x40022800) },
        gpioj: unsafe { from_addr(0x40022400) },
        gpioi: unsafe { from_addr(0x40022000) },
        gpioh: unsafe { from_addr(0x40021c00) },
        gpiog: unsafe { from_addr(0x40021800) },
        gpiof: unsafe { from_addr(0x40021400) },
        gpioe: unsafe { from_addr(0x40021000) },
        gpiob: unsafe { from_addr(0x40020400) },
        gpioa: unsafe { from_addr(0x40020000) },
        syscfg: unsafe { from_addr(0x40013800) },
        spi1: unsafe { from_addr(0x40013000) },
        spi2: unsafe { from_addr(0x40003800) },
        spi3: unsafe { from_addr(0x40003c00) },
        spi4: unsafe { from_addr(0x40013400) },
        spi5: unsafe { from_addr(0x40015000) },
        spi6: unsafe { from_addr(0x40015400) },
        adc1: unsafe { from_addr(0x40012000) },
        adc2: unsafe { from_addr(0x40012100) },
        adc3: unsafe { from_addr(0x40012200) },
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
        can1: unsafe { from_addr(0x40006400) },
        can2: unsafe { from_addr(0x40006800) },
        nvic: unsafe { from_addr(0xe000e000) },
        flash: unsafe { from_addr(0x40023c00) },
        exti: unsafe { from_addr(0x40013c00) },
        ltdc: unsafe { from_addr(0x40016800) },
        sai1: unsafe { from_addr(0x40015800) },
        sai2: unsafe { from_addr(0x40015c00) },
        dma2d: unsafe { from_addr(0x4002b000) },
        quadspi: unsafe { from_addr(0xa0001000) },
        cec: unsafe { from_addr(0x40006c00) },
        spdif_rx: unsafe { from_addr(0x40004000) },
        sdmmc: unsafe { from_addr(0x40012c00) },
        lptim1: unsafe { from_addr(0x40002400) },
        i2c1: unsafe { from_addr(0x40005400) },
        i2c2: unsafe { from_addr(0x40005800) },
        i2c3: unsafe { from_addr(0x40005c00) },
        i2c4: unsafe { from_addr(0x40006000) },
        rtc: unsafe { from_addr(0x40002800) },
        usart6: unsafe { from_addr(0x40011400) },
        usart1: unsafe { from_addr(0x40011000) },
        usart3: unsafe { from_addr(0x40004800) },
        usart2: unsafe { from_addr(0x40004400) },
        uart5: unsafe { from_addr(0x40005000) },
        uart4: unsafe { from_addr(0x40004c00) },
        uart8: unsafe { from_addr(0x40007c00) },
        uart7: unsafe { from_addr(0x40007800) },
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
