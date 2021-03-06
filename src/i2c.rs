// autogenerated

use volatile;
use bit_field::BitField;

# [ doc = "Inter-integrated circuit" ]
# [ repr ( C ) ]
pub struct I2c {
    # [ doc = "0x00 - Control register 1" ]
    pub cr1: volatile::ReadWrite<Cr1>,
    # [ doc = "0x04 - Control register 2" ]
    pub cr2: volatile::ReadWrite<Cr2>,
    # [ doc = "0x08 - Own address register 1" ]
    pub oar1: volatile::ReadWrite<Oar1>,
    # [ doc = "0x0c - Own address register 2" ]
    pub oar2: volatile::ReadWrite<Oar2>,
    # [ doc = "0x10 - Timing register" ]
    pub timingr: volatile::ReadWrite<Timingr>,
    # [ doc = "0x14 - Status register 1" ]
    pub timeoutr: volatile::ReadWrite<Timeoutr>,
    # [ doc = "0x18 - Interrupt and Status register" ]
    pub isr: volatile::ReadWrite<Isr>,
    # [ doc = "0x1c - Interrupt clear register" ]
    pub icr: volatile::WriteOnly<Icr>,
    # [ doc = "0x20 - PEC register" ]
    pub pecr: volatile::ReadOnly<Pecr>,
    # [ doc = "0x24 - Receive data register" ]
    pub rxdr: volatile::ReadOnly<Rxdr>,
    # [ doc = "0x28 - Transmit data register" ]
    pub txdr: volatile::ReadWrite<Txdr>,
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Cr1 {
    bits: u32,
}

impl Cr1 {
    # [ doc = "Bit 0 - Peripheral enable" ]
    pub fn pe(&self) -> bool {
        self.bits.get_bit(0u8)
    }
    # [ doc = "Bit 1 - TX Interrupt enable" ]
    pub fn txie(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 2 - RX Interrupt enable" ]
    pub fn rxie(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 3 - Address match interrupt enable (slave only)" ]
    pub fn addrie(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 4 - Not acknowledge received interrupt enable" ]
    pub fn nackie(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 5 - STOP detection Interrupt enable" ]
    pub fn stopie(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 6 - Transfer Complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 7 - Error interrupts enable" ]
    pub fn errie(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bits 8:11 - Digital noise filter" ]
    pub fn dnf(&self) -> u8 {
        self.bits.get_range(8u8..12u8) as u8
    }
    # [ doc = "Bit 12 - Analog noise filter OFF" ]
    pub fn anfoff(&self) -> bool {
        self.bits.get_bit(12u8)
    }
    # [ doc = "Bit 14 - DMA transmission requests enable" ]
    pub fn txdmaen(&self) -> bool {
        self.bits.get_bit(14u8)
    }
    # [ doc = "Bit 15 - DMA reception requests enable" ]
    pub fn rxdmaen(&self) -> bool {
        self.bits.get_bit(15u8)
    }
    # [ doc = "Bit 16 - Slave byte control" ]
    pub fn sbc(&self) -> bool {
        self.bits.get_bit(16u8)
    }
    # [ doc = "Bit 17 - Clock stretching disable" ]
    pub fn nostretch(&self) -> bool {
        self.bits.get_bit(17u8)
    }
    # [ doc = "Bit 18 - Wakeup from STOP enable" ]
    pub fn wupen(&self) -> bool {
        self.bits.get_bit(18u8)
    }
    # [ doc = "Bit 19 - General call enable" ]
    pub fn gcen(&self) -> bool {
        self.bits.get_bit(19u8)
    }
    # [ doc = "Bit 20 - SMBus Host address enable" ]
    pub fn smbhen(&self) -> bool {
        self.bits.get_bit(20u8)
    }
    # [ doc = "Bit 21 - SMBus Device Default address enable" ]
    pub fn smbden(&self) -> bool {
        self.bits.get_bit(21u8)
    }
    # [ doc = "Bit 22 - SMBUS alert enable" ]
    pub fn alerten(&self) -> bool {
        self.bits.get_bit(22u8)
    }
    # [ doc = "Bit 23 - PEC enable" ]
    pub fn pecen(&self) -> bool {
        self.bits.get_bit(23u8)
    }
}

impl Default for Cr1 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Cr1 { bits: 0u32 }
    }
}

impl Cr1 {
    # [ doc = "Bit 0 - Peripheral enable" ]
    pub fn set_pe(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
    # [ doc = "Bit 1 - TX Interrupt enable" ]
    pub fn set_txie(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 2 - RX Interrupt enable" ]
    pub fn set_rxie(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 3 - Address match interrupt enable (slave only)" ]
    pub fn set_addrie(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 4 - Not acknowledge received interrupt enable" ]
    pub fn set_nackie(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 5 - STOP detection Interrupt enable" ]
    pub fn set_stopie(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 6 - Transfer Complete interrupt enable" ]
    pub fn set_tcie(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 7 - Error interrupts enable" ]
    pub fn set_errie(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bits 8:11 - Digital noise filter" ]
    pub fn set_dnf(&mut self, value: u8) {
        self.bits.set_range(8u8..12u8, value as u32);
    }
    # [ doc = "Bit 12 - Analog noise filter OFF" ]
    pub fn set_anfoff(&mut self, value: bool) {
        self.bits.set_bit(12u8, value);
    }
    # [ doc = "Bit 14 - DMA transmission requests enable" ]
    pub fn set_txdmaen(&mut self, value: bool) {
        self.bits.set_bit(14u8, value);
    }
    # [ doc = "Bit 15 - DMA reception requests enable" ]
    pub fn set_rxdmaen(&mut self, value: bool) {
        self.bits.set_bit(15u8, value);
    }
    # [ doc = "Bit 16 - Slave byte control" ]
    pub fn set_sbc(&mut self, value: bool) {
        self.bits.set_bit(16u8, value);
    }
    # [ doc = "Bit 17 - Clock stretching disable" ]
    pub fn set_nostretch(&mut self, value: bool) {
        self.bits.set_bit(17u8, value);
    }
    # [ doc = "Bit 18 - Wakeup from STOP enable" ]
    pub fn set_wupen(&mut self, value: bool) {
        self.bits.set_bit(18u8, value);
    }
    # [ doc = "Bit 19 - General call enable" ]
    pub fn set_gcen(&mut self, value: bool) {
        self.bits.set_bit(19u8, value);
    }
    # [ doc = "Bit 20 - SMBus Host address enable" ]
    pub fn set_smbhen(&mut self, value: bool) {
        self.bits.set_bit(20u8, value);
    }
    # [ doc = "Bit 21 - SMBus Device Default address enable" ]
    pub fn set_smbden(&mut self, value: bool) {
        self.bits.set_bit(21u8, value);
    }
    # [ doc = "Bit 22 - SMBUS alert enable" ]
    pub fn set_alerten(&mut self, value: bool) {
        self.bits.set_bit(22u8, value);
    }
    # [ doc = "Bit 23 - PEC enable" ]
    pub fn set_pecen(&mut self, value: bool) {
        self.bits.set_bit(23u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Cr2 {
    bits: u32,
}

impl Cr2 {
    # [ doc = "Bit 26 - Packet error checking byte" ]
    pub fn pecbyte(&self) -> bool {
        self.bits.get_bit(26u8)
    }
    # [ doc = "Bit 25 - Automatic end mode (master mode)" ]
    pub fn autoend(&self) -> bool {
        self.bits.get_bit(25u8)
    }
    # [ doc = "Bit 24 - NBYTES reload mode" ]
    pub fn reload(&self) -> bool {
        self.bits.get_bit(24u8)
    }
    # [ doc = "Bits 16:23 - Number of bytes" ]
    pub fn nbytes(&self) -> u8 {
        self.bits.get_range(16u8..24u8) as u8
    }
    # [ doc = "Bit 15 - NACK generation (slave mode)" ]
    pub fn nack(&self) -> bool {
        self.bits.get_bit(15u8)
    }
    # [ doc = "Bit 14 - Stop generation (master mode)" ]
    pub fn stop(&self) -> bool {
        self.bits.get_bit(14u8)
    }
    # [ doc = "Bit 13 - Start generation" ]
    pub fn start(&self) -> bool {
        self.bits.get_bit(13u8)
    }
    # [ doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)" ]
    pub fn head10r(&self) -> bool {
        self.bits.get_bit(12u8)
    }
    # [ doc = "Bit 11 - 10-bit addressing mode (master mode)" ]
    pub fn add10(&self) -> bool {
        self.bits.get_bit(11u8)
    }
    # [ doc = "Bit 10 - Transfer direction (master mode)" ]
    pub fn rd_wrn(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bits 0:9 - Slave address bit (master mode)" ]
    pub fn sadd(&self) -> u16 {
        self.bits.get_range(0u8..10u8) as u16
    }
}

impl Default for Cr2 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Cr2 { bits: 0u32 }
    }
}

impl Cr2 {
    # [ doc = "Bit 26 - Packet error checking byte" ]
    pub fn set_pecbyte(&mut self, value: bool) {
        self.bits.set_bit(26u8, value);
    }
    # [ doc = "Bit 25 - Automatic end mode (master mode)" ]
    pub fn set_autoend(&mut self, value: bool) {
        self.bits.set_bit(25u8, value);
    }
    # [ doc = "Bit 24 - NBYTES reload mode" ]
    pub fn set_reload(&mut self, value: bool) {
        self.bits.set_bit(24u8, value);
    }
    # [ doc = "Bits 16:23 - Number of bytes" ]
    pub fn set_nbytes(&mut self, value: u8) {
        self.bits.set_range(16u8..24u8, value as u32);
    }
    # [ doc = "Bit 15 - NACK generation (slave mode)" ]
    pub fn set_nack(&mut self, value: bool) {
        self.bits.set_bit(15u8, value);
    }
    # [ doc = "Bit 14 - Stop generation (master mode)" ]
    pub fn set_stop(&mut self, value: bool) {
        self.bits.set_bit(14u8, value);
    }
    # [ doc = "Bit 13 - Start generation" ]
    pub fn set_start(&mut self, value: bool) {
        self.bits.set_bit(13u8, value);
    }
    # [ doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)" ]
    pub fn set_head10r(&mut self, value: bool) {
        self.bits.set_bit(12u8, value);
    }
    # [ doc = "Bit 11 - 10-bit addressing mode (master mode)" ]
    pub fn set_add10(&mut self, value: bool) {
        self.bits.set_bit(11u8, value);
    }
    # [ doc = "Bit 10 - Transfer direction (master mode)" ]
    pub fn set_rd_wrn(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bits 0:9 - Slave address bit (master mode)" ]
    pub fn set_sadd(&mut self, value: u16) {
        self.bits.set_range(0u8..10u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Oar1 {
    bits: u32,
}

impl Oar1 {
    # [ doc = "Bits 0:9 - Interface address" ]
    pub fn oa1(&self) -> u16 {
        self.bits.get_range(0u8..10u8) as u16
    }
    # [ doc = "Bit 10 - Own Address 1 10-bit mode" ]
    pub fn oa1mode(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bit 15 - Own Address 1 enable" ]
    pub fn oa1en(&self) -> bool {
        self.bits.get_bit(15u8)
    }
}

impl Default for Oar1 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Oar1 { bits: 0u32 }
    }
}

impl Oar1 {
    # [ doc = "Bits 0:9 - Interface address" ]
    pub fn set_oa1(&mut self, value: u16) {
        self.bits.set_range(0u8..10u8, value as u32);
    }
    # [ doc = "Bit 10 - Own Address 1 10-bit mode" ]
    pub fn set_oa1mode(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bit 15 - Own Address 1 enable" ]
    pub fn set_oa1en(&mut self, value: bool) {
        self.bits.set_bit(15u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Oar2 {
    bits: u32,
}

impl Oar2 {
    # [ doc = "Bits 1:7 - Interface address" ]
    pub fn oa2(&self) -> u8 {
        self.bits.get_range(1u8..8u8) as u8
    }
    # [ doc = "Bits 8:10 - Own Address 2 masks" ]
    pub fn oa2msk(&self) -> u8 {
        self.bits.get_range(8u8..11u8) as u8
    }
    # [ doc = "Bit 15 - Own Address 2 enable" ]
    pub fn oa2en(&self) -> bool {
        self.bits.get_bit(15u8)
    }
}

impl Default for Oar2 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Oar2 { bits: 0u32 }
    }
}

impl Oar2 {
    # [ doc = "Bits 1:7 - Interface address" ]
    pub fn set_oa2(&mut self, value: u8) {
        self.bits.set_range(1u8..8u8, value as u32);
    }
    # [ doc = "Bits 8:10 - Own Address 2 masks" ]
    pub fn set_oa2msk(&mut self, value: u8) {
        self.bits.set_range(8u8..11u8, value as u32);
    }
    # [ doc = "Bit 15 - Own Address 2 enable" ]
    pub fn set_oa2en(&mut self, value: bool) {
        self.bits.set_bit(15u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Timingr {
    bits: u32,
}

impl Timingr {
    # [ doc = "Bits 0:7 - SCL low period (master mode)" ]
    pub fn scll(&self) -> u8 {
        self.bits.get_range(0u8..8u8) as u8
    }
    # [ doc = "Bits 8:15 - SCL high period (master mode)" ]
    pub fn sclh(&self) -> u8 {
        self.bits.get_range(8u8..16u8) as u8
    }
    # [ doc = "Bits 16:19 - Data hold time" ]
    pub fn sdadel(&self) -> u8 {
        self.bits.get_range(16u8..20u8) as u8
    }
    # [ doc = "Bits 20:23 - Data setup time" ]
    pub fn scldel(&self) -> u8 {
        self.bits.get_range(20u8..24u8) as u8
    }
    # [ doc = "Bits 28:31 - Timing prescaler" ]
    pub fn presc(&self) -> u8 {
        self.bits.get_range(28u8..32u8) as u8
    }
}

impl Default for Timingr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Timingr { bits: 0u32 }
    }
}

impl Timingr {
    # [ doc = "Bits 0:7 - SCL low period (master mode)" ]
    pub fn set_scll(&mut self, value: u8) {
        self.bits.set_range(0u8..8u8, value as u32);
    }
    # [ doc = "Bits 8:15 - SCL high period (master mode)" ]
    pub fn set_sclh(&mut self, value: u8) {
        self.bits.set_range(8u8..16u8, value as u32);
    }
    # [ doc = "Bits 16:19 - Data hold time" ]
    pub fn set_sdadel(&mut self, value: u8) {
        self.bits.set_range(16u8..20u8, value as u32);
    }
    # [ doc = "Bits 20:23 - Data setup time" ]
    pub fn set_scldel(&mut self, value: u8) {
        self.bits.set_range(20u8..24u8, value as u32);
    }
    # [ doc = "Bits 28:31 - Timing prescaler" ]
    pub fn set_presc(&mut self, value: u8) {
        self.bits.set_range(28u8..32u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Timeoutr {
    bits: u32,
}

impl Timeoutr {
    # [ doc = "Bits 0:11 - Bus timeout A" ]
    pub fn timeouta(&self) -> u16 {
        self.bits.get_range(0u8..12u8) as u16
    }
    # [ doc = "Bit 12 - Idle clock timeout detection" ]
    pub fn tidle(&self) -> bool {
        self.bits.get_bit(12u8)
    }
    # [ doc = "Bit 15 - Clock timeout enable" ]
    pub fn timouten(&self) -> bool {
        self.bits.get_bit(15u8)
    }
    # [ doc = "Bits 16:27 - Bus timeout B" ]
    pub fn timeoutb(&self) -> u16 {
        self.bits.get_range(16u8..28u8) as u16
    }
    # [ doc = "Bit 31 - Extended clock timeout enable" ]
    pub fn texten(&self) -> bool {
        self.bits.get_bit(31u8)
    }
}

impl Default for Timeoutr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Timeoutr { bits: 0u32 }
    }
}

impl Timeoutr {
    # [ doc = "Bits 0:11 - Bus timeout A" ]
    pub fn set_timeouta(&mut self, value: u16) {
        self.bits.set_range(0u8..12u8, value as u32);
    }
    # [ doc = "Bit 12 - Idle clock timeout detection" ]
    pub fn set_tidle(&mut self, value: bool) {
        self.bits.set_bit(12u8, value);
    }
    # [ doc = "Bit 15 - Clock timeout enable" ]
    pub fn set_timouten(&mut self, value: bool) {
        self.bits.set_bit(15u8, value);
    }
    # [ doc = "Bits 16:27 - Bus timeout B" ]
    pub fn set_timeoutb(&mut self, value: u16) {
        self.bits.set_range(16u8..28u8, value as u32);
    }
    # [ doc = "Bit 31 - Extended clock timeout enable" ]
    pub fn set_texten(&mut self, value: bool) {
        self.bits.set_bit(31u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Isr {
    bits: u32,
}

impl Isr {
    # [ doc = "Bits 17:23 - Address match code (Slave mode)" ]
    pub fn addcode(&self) -> u8 {
        self.bits.get_range(17u8..24u8) as u8
    }
    # [ doc = "Bit 16 - Transfer direction (Slave mode)" ]
    pub fn dir(&self) -> bool {
        self.bits.get_bit(16u8)
    }
    # [ doc = "Bit 15 - Bus busy" ]
    pub fn busy(&self) -> bool {
        self.bits.get_bit(15u8)
    }
    # [ doc = "Bit 13 - SMBus alert" ]
    pub fn alert(&self) -> bool {
        self.bits.get_bit(13u8)
    }
    # [ doc = "Bit 12 - Timeout or t_low detection flag" ]
    pub fn timeout(&self) -> bool {
        self.bits.get_bit(12u8)
    }
    # [ doc = "Bit 11 - PEC Error in reception" ]
    pub fn pecerr(&self) -> bool {
        self.bits.get_bit(11u8)
    }
    # [ doc = "Bit 10 - Overrun/Underrun (slave mode)" ]
    pub fn ovr(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bit 9 - Arbitration lost" ]
    pub fn arlo(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 8 - Bus error" ]
    pub fn berr(&self) -> bool {
        self.bits.get_bit(8u8)
    }
    # [ doc = "Bit 7 - Transfer Complete Reload" ]
    pub fn tcr(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bit 6 - Transfer Complete (master mode)" ]
    pub fn tc(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 5 - Stop detection flag" ]
    pub fn stopf(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - Not acknowledge received flag" ]
    pub fn nackf(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - Address matched (slave mode)" ]
    pub fn addr(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Receive data register not empty (receivers)" ]
    pub fn rxne(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Transmit interrupt status (transmitters)" ]
    pub fn txis(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Transmit data register empty (transmitters)" ]
    pub fn txe(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Isr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Isr { bits: 1u32 }
    }
}

impl Isr {
    # [ doc = "Bit 1 - Transmit interrupt status (transmitters)" ]
    pub fn set_txis(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Transmit data register empty (transmitters)" ]
    pub fn set_txe(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Icr {
    bits: u32,
}

impl Icr {
    # [ doc = "Bit 13 - Alert flag clear" ]
    pub fn alertcf(&self) -> bool {
        self.bits.get_bit(13u8)
    }
    # [ doc = "Bit 12 - Timeout detection flag clear" ]
    pub fn timoutcf(&self) -> bool {
        self.bits.get_bit(12u8)
    }
    # [ doc = "Bit 11 - PEC Error flag clear" ]
    pub fn peccf(&self) -> bool {
        self.bits.get_bit(11u8)
    }
    # [ doc = "Bit 10 - Overrun/Underrun flag clear" ]
    pub fn ovrcf(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bit 9 - Arbitration lost flag clear" ]
    pub fn arlocf(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 8 - Bus error flag clear" ]
    pub fn berrcf(&self) -> bool {
        self.bits.get_bit(8u8)
    }
    # [ doc = "Bit 5 - Stop detection flag clear" ]
    pub fn stopcf(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - Not Acknowledge flag clear" ]
    pub fn nackcf(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - Address Matched flag clear" ]
    pub fn addrcf(&self) -> bool {
        self.bits.get_bit(3u8)
    }
}

impl Default for Icr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Icr { bits: 0u32 }
    }
}

impl Icr {
    # [ doc = "Bit 13 - Alert flag clear" ]
    pub fn set_alertcf(&mut self, value: bool) {
        self.bits.set_bit(13u8, value);
    }
    # [ doc = "Bit 12 - Timeout detection flag clear" ]
    pub fn set_timoutcf(&mut self, value: bool) {
        self.bits.set_bit(12u8, value);
    }
    # [ doc = "Bit 11 - PEC Error flag clear" ]
    pub fn set_peccf(&mut self, value: bool) {
        self.bits.set_bit(11u8, value);
    }
    # [ doc = "Bit 10 - Overrun/Underrun flag clear" ]
    pub fn set_ovrcf(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bit 9 - Arbitration lost flag clear" ]
    pub fn set_arlocf(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 8 - Bus error flag clear" ]
    pub fn set_berrcf(&mut self, value: bool) {
        self.bits.set_bit(8u8, value);
    }
    # [ doc = "Bit 5 - Stop detection flag clear" ]
    pub fn set_stopcf(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 4 - Not Acknowledge flag clear" ]
    pub fn set_nackcf(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - Address Matched flag clear" ]
    pub fn set_addrcf(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Pecr {
    bits: u32,
}

impl Pecr {
    # [ doc = "Bits 0:7 - Packet error checking register" ]
    pub fn pec(&self) -> u8 {
        self.bits.get_range(0u8..8u8) as u8
    }
}

impl Default for Pecr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Pecr { bits: 0u32 }
    }
}

impl Pecr {
    # [ doc = "Bits 0:7 - Packet error checking register" ]
    pub fn set_pec(&mut self, value: u8) {
        self.bits.set_range(0u8..8u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Rxdr {
    bits: u32,
}

impl Rxdr {
    # [ doc = "Bits 0:7 - 8-bit receive data" ]
    pub fn rxdata(&self) -> u8 {
        self.bits.get_range(0u8..8u8) as u8
    }
}

impl Default for Rxdr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Rxdr { bits: 0u32 }
    }
}

impl Rxdr {
    # [ doc = "Bits 0:7 - 8-bit receive data" ]
    pub fn set_rxdata(&mut self, value: u8) {
        self.bits.set_range(0u8..8u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy , PartialEq , Eq ) ]
# [ repr ( C ) ]
pub struct Txdr {
    bits: u32,
}

impl Txdr {
    # [ doc = "Bits 0:7 - 8-bit transmit data" ]
    pub fn txdata(&self) -> u8 {
        self.bits.get_range(0u8..8u8) as u8
    }
}

impl Default for Txdr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Txdr { bits: 0u32 }
    }
}

impl Txdr {
    # [ doc = "Bits 0:7 - 8-bit transmit data" ]
    pub fn set_txdata(&mut self, value: u8) {
        self.bits.set_range(0u8..8u8, value as u32);
    }
}
