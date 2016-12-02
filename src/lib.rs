#![no_std]

extern crate embedded;
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
