// autogenerated

use volatile;
use bit_field::BitField;

# [ doc = "Secure digital input/output interface" ]
# [ repr ( C ) ]
pub struct Sdmmc {
    # [ doc = "0x00 - power control register" ]
    pub power: volatile::ReadWrite<Power>,
    # [ doc = "0x04 - SDI clock control register" ]
    pub clkcr: volatile::ReadWrite<Clkcr>,
    # [ doc = "0x08 - argument register" ]
    pub arg: volatile::ReadWrite<Arg>,
    # [ doc = "0x0c - command register" ]
    pub cmd: volatile::ReadWrite<Cmd>,
    # [ doc = "0x10 - command response register" ]
    pub respcmd: volatile::ReadOnly<Respcmd>,
    # [ doc = "0x14 - response 1..4 register" ]
    pub resp1: volatile::ReadOnly<Resp1>,
    # [ doc = "0x18 - response 1..4 register" ]
    pub resp2: volatile::ReadOnly<Resp2>,
    # [ doc = "0x1c - response 1..4 register" ]
    pub resp3: volatile::ReadOnly<Resp3>,
    # [ doc = "0x20 - response 1..4 register" ]
    pub resp4: volatile::ReadOnly<Resp4>,
    # [ doc = "0x24 - data timer register" ]
    pub dtimer: volatile::ReadWrite<Dtimer>,
    # [ doc = "0x28 - data length register" ]
    pub dlen: volatile::ReadWrite<Dlen>,
    # [ doc = "0x2c - data control register" ]
    pub dctrl: volatile::ReadWrite<Dctrl>,
    # [ doc = "0x30 - data counter register" ]
    pub dcount: volatile::ReadOnly<Dcount>,
    # [ doc = "0x34 - status register" ]
    pub sta: volatile::ReadOnly<Sta>,
    # [ doc = "0x38 - interrupt clear register" ]
    pub icr: volatile::ReadWrite<Icr>,
    # [ doc = "0x3c - mask register" ]
    pub mask: volatile::ReadWrite<Mask>,
    _reserved0: [u8; 8usize],
    # [ doc = "0x48 - FIFO counter register" ]
    pub fifocnt: volatile::ReadOnly<Fifocnt>,
    _reserved1: [u8; 52usize],
    # [ doc = "0x80 - data FIFO register" ]
    pub fifo: volatile::ReadWrite<Fifo>,
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Power {
    bits: u32,
}

impl Power {
    # [ doc = "Bits 0:1 - PWRCTRL" ]
    pub fn pwrctrl(&self) -> u8 {
        self.bits.get_range(0u8..2u8) as u8
    }
}

impl Default for Power {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Power { bits: 0u32 }
    }
}

impl Power {
    # [ doc = "Bits 0:1 - PWRCTRL" ]
    pub fn set_pwrctrl(&mut self, value: u8) {
        self.bits.set_range(0u8..2u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Clkcr {
    bits: u32,
}

impl Clkcr {
    # [ doc = "Bit 14 - HW Flow Control enable" ]
    pub fn hwfc_en(&self) -> bool {
        self.bits.get_bit(14u8)
    }
    # [ doc = "Bit 13 - SDIO_CK dephasing selection bit" ]
    pub fn negedge(&self) -> bool {
        self.bits.get_bit(13u8)
    }
    # [ doc = "Bits 11:12 - Wide bus mode enable bit" ]
    pub fn widbus(&self) -> u8 {
        self.bits.get_range(11u8..13u8) as u8
    }
    # [ doc = "Bit 10 - Clock divider bypass enable bit" ]
    pub fn bypass(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bit 9 - Power saving configuration bit" ]
    pub fn pwrsav(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 8 - Clock enable bit" ]
    pub fn clken(&self) -> bool {
        self.bits.get_bit(8u8)
    }
    # [ doc = "Bits 0:7 - Clock divide factor" ]
    pub fn clkdiv(&self) -> u8 {
        self.bits.get_range(0u8..8u8) as u8
    }
}

impl Default for Clkcr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Clkcr { bits: 0u32 }
    }
}

impl Clkcr {
    # [ doc = "Bit 14 - HW Flow Control enable" ]
    pub fn set_hwfc_en(&mut self, value: bool) {
        self.bits.set_bit(14u8, value);
    }
    # [ doc = "Bit 13 - SDIO_CK dephasing selection bit" ]
    pub fn set_negedge(&mut self, value: bool) {
        self.bits.set_bit(13u8, value);
    }
    # [ doc = "Bits 11:12 - Wide bus mode enable bit" ]
    pub fn set_widbus(&mut self, value: u8) {
        self.bits.set_range(11u8..13u8, value as u32);
    }
    # [ doc = "Bit 10 - Clock divider bypass enable bit" ]
    pub fn set_bypass(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bit 9 - Power saving configuration bit" ]
    pub fn set_pwrsav(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 8 - Clock enable bit" ]
    pub fn set_clken(&mut self, value: bool) {
        self.bits.set_bit(8u8, value);
    }
    # [ doc = "Bits 0:7 - Clock divide factor" ]
    pub fn set_clkdiv(&mut self, value: u8) {
        self.bits.set_range(0u8..8u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Arg {
    bits: u32,
}

impl Arg {
    # [ doc = "Bits 0:31 - Command argument" ]
    pub fn cmdarg(&self) -> u32 {
        self.bits.get_range(0u8..32u8) as u32
    }
}

impl Default for Arg {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Arg { bits: 0u32 }
    }
}

impl Arg {
    # [ doc = "Bits 0:31 - Command argument" ]
    pub fn set_cmdarg(&mut self, value: u32) {
        self.bits.set_range(0u8..32u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cmd {
    bits: u32,
}

impl Cmd {
    # [ doc = "Bit 11 - SD I/O suspend command" ]
    pub fn sdiosuspend(&self) -> bool {
        self.bits.get_bit(11u8)
    }
    # [ doc = "Bit 10 - Command path state machine (CPSM) Enable bit" ]
    pub fn cpsmen(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)" ]
    pub fn waitpend(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 8 - CPSM waits for interrupt request" ]
    pub fn waitint(&self) -> bool {
        self.bits.get_bit(8u8)
    }
    # [ doc = "Bits 6:7 - Wait for response bits" ]
    pub fn waitresp(&self) -> u8 {
        self.bits.get_range(6u8..8u8) as u8
    }
    # [ doc = "Bits 0:5 - Command index" ]
    pub fn cmdindex(&self) -> u8 {
        self.bits.get_range(0u8..6u8) as u8
    }
}

impl Default for Cmd {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Cmd { bits: 0u32 }
    }
}

impl Cmd {
    # [ doc = "Bit 11 - SD I/O suspend command" ]
    pub fn set_sdiosuspend(&mut self, value: bool) {
        self.bits.set_bit(11u8, value);
    }
    # [ doc = "Bit 10 - Command path state machine (CPSM) Enable bit" ]
    pub fn set_cpsmen(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)" ]
    pub fn set_waitpend(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 8 - CPSM waits for interrupt request" ]
    pub fn set_waitint(&mut self, value: bool) {
        self.bits.set_bit(8u8, value);
    }
    # [ doc = "Bits 6:7 - Wait for response bits" ]
    pub fn set_waitresp(&mut self, value: u8) {
        self.bits.set_range(6u8..8u8, value as u32);
    }
    # [ doc = "Bits 0:5 - Command index" ]
    pub fn set_cmdindex(&mut self, value: u8) {
        self.bits.set_range(0u8..6u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Respcmd {
    bits: u32,
}

impl Respcmd {
    # [ doc = "Bits 0:5 - Response command index" ]
    pub fn respcmd(&self) -> u8 {
        self.bits.get_range(0u8..6u8) as u8
    }
}

impl Default for Respcmd {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Respcmd { bits: 0u32 }
    }
}

impl Respcmd {
    # [ doc = "Bits 0:5 - Response command index" ]
    pub fn set_respcmd(&mut self, value: u8) {
        self.bits.set_range(0u8..6u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Resp1 {
    bits: u32,
}

impl Resp1 {
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn cardstatus1(&self) -> u32 {
        self.bits.get_range(0u8..32u8) as u32
    }
}

impl Default for Resp1 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Resp1 { bits: 0u32 }
    }
}

impl Resp1 {
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn set_cardstatus1(&mut self, value: u32) {
        self.bits.set_range(0u8..32u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Resp2 {
    bits: u32,
}

impl Resp2 {
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn cardstatus2(&self) -> u32 {
        self.bits.get_range(0u8..32u8) as u32
    }
}

impl Default for Resp2 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Resp2 { bits: 0u32 }
    }
}

impl Resp2 {
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn set_cardstatus2(&mut self, value: u32) {
        self.bits.set_range(0u8..32u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Resp3 {
    bits: u32,
}

impl Resp3 {
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn cardstatus3(&self) -> u32 {
        self.bits.get_range(0u8..32u8) as u32
    }
}

impl Default for Resp3 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Resp3 { bits: 0u32 }
    }
}

impl Resp3 {
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn set_cardstatus3(&mut self, value: u32) {
        self.bits.set_range(0u8..32u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Resp4 {
    bits: u32,
}

impl Resp4 {
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn cardstatus4(&self) -> u32 {
        self.bits.get_range(0u8..32u8) as u32
    }
}

impl Default for Resp4 {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Resp4 { bits: 0u32 }
    }
}

impl Resp4 {
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn set_cardstatus4(&mut self, value: u32) {
        self.bits.set_range(0u8..32u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dtimer {
    bits: u32,
}

impl Dtimer {
    # [ doc = "Bits 0:31 - Data timeout period" ]
    pub fn datatime(&self) -> u32 {
        self.bits.get_range(0u8..32u8) as u32
    }
}

impl Default for Dtimer {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Dtimer { bits: 0u32 }
    }
}

impl Dtimer {
    # [ doc = "Bits 0:31 - Data timeout period" ]
    pub fn set_datatime(&mut self, value: u32) {
        self.bits.set_range(0u8..32u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dlen {
    bits: u32,
}

impl Dlen {
    # [ doc = "Bits 0:24 - Data length value" ]
    pub fn datalength(&self) -> u32 {
        self.bits.get_range(0u8..25u8) as u32
    }
}

impl Default for Dlen {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Dlen { bits: 0u32 }
    }
}

impl Dlen {
    # [ doc = "Bits 0:24 - Data length value" ]
    pub fn set_datalength(&mut self, value: u32) {
        self.bits.set_range(0u8..25u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dctrl {
    bits: u32,
}

impl Dctrl {
    # [ doc = "Bit 11 - SD I/O enable functions" ]
    pub fn sdioen(&self) -> bool {
        self.bits.get_bit(11u8)
    }
    # [ doc = "Bit 10 - Read wait mode" ]
    pub fn rwmod(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bit 9 - Read wait stop" ]
    pub fn rwstop(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 8 - Read wait start" ]
    pub fn rwstart(&self) -> bool {
        self.bits.get_bit(8u8)
    }
    # [ doc = "Bits 4:7 - Data block size" ]
    pub fn dblocksize(&self) -> u8 {
        self.bits.get_range(4u8..8u8) as u8
    }
    # [ doc = "Bit 3 - DMA enable bit" ]
    pub fn dmaen(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer" ]
    pub fn dtmode(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Data transfer direction selection" ]
    pub fn dtdir(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - DTEN" ]
    pub fn dten(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Dctrl {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Dctrl { bits: 0u32 }
    }
}

impl Dctrl {
    # [ doc = "Bit 11 - SD I/O enable functions" ]
    pub fn set_sdioen(&mut self, value: bool) {
        self.bits.set_bit(11u8, value);
    }
    # [ doc = "Bit 10 - Read wait mode" ]
    pub fn set_rwmod(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bit 9 - Read wait stop" ]
    pub fn set_rwstop(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 8 - Read wait start" ]
    pub fn set_rwstart(&mut self, value: bool) {
        self.bits.set_bit(8u8, value);
    }
    # [ doc = "Bits 4:7 - Data block size" ]
    pub fn set_dblocksize(&mut self, value: u8) {
        self.bits.set_range(4u8..8u8, value as u32);
    }
    # [ doc = "Bit 3 - DMA enable bit" ]
    pub fn set_dmaen(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer" ]
    pub fn set_dtmode(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Data transfer direction selection" ]
    pub fn set_dtdir(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - DTEN" ]
    pub fn set_dten(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dcount {
    bits: u32,
}

impl Dcount {
    # [ doc = "Bits 0:24 - Data count value" ]
    pub fn datacount(&self) -> u32 {
        self.bits.get_range(0u8..25u8) as u32
    }
}

impl Default for Dcount {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Dcount { bits: 0u32 }
    }
}

impl Dcount {
    # [ doc = "Bits 0:24 - Data count value" ]
    pub fn set_datacount(&mut self, value: u32) {
        self.bits.set_range(0u8..25u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sta {
    bits: u32,
}

impl Sta {
    # [ doc = "Bit 23 - CE-ATA command completion signal received for CMD61" ]
    pub fn ceataend(&self) -> bool {
        self.bits.get_bit(23u8)
    }
    # [ doc = "Bit 22 - SDIO interrupt received" ]
    pub fn sdioit(&self) -> bool {
        self.bits.get_bit(22u8)
    }
    # [ doc = "Bit 21 - Data available in receive FIFO" ]
    pub fn rxdavl(&self) -> bool {
        self.bits.get_bit(21u8)
    }
    # [ doc = "Bit 20 - Data available in transmit FIFO" ]
    pub fn txdavl(&self) -> bool {
        self.bits.get_bit(20u8)
    }
    # [ doc = "Bit 19 - Receive FIFO empty" ]
    pub fn rxfifoe(&self) -> bool {
        self.bits.get_bit(19u8)
    }
    # [ doc = "Bit 18 - Transmit FIFO empty" ]
    pub fn txfifoe(&self) -> bool {
        self.bits.get_bit(18u8)
    }
    # [ doc = "Bit 17 - Receive FIFO full" ]
    pub fn rxfifof(&self) -> bool {
        self.bits.get_bit(17u8)
    }
    # [ doc = "Bit 16 - Transmit FIFO full" ]
    pub fn txfifof(&self) -> bool {
        self.bits.get_bit(16u8)
    }
    # [ doc = "Bit 15 - Receive FIFO half full: there are at least 8 words in the FIFO" ]
    pub fn rxfifohf(&self) -> bool {
        self.bits.get_bit(15u8)
    }
    # [ doc = "Bit 14 - Transmit FIFO half empty: at least 8 words can be written into the FIFO" ]
    pub fn txfifohe(&self) -> bool {
        self.bits.get_bit(14u8)
    }
    # [ doc = "Bit 13 - Data receive in progress" ]
    pub fn rxact(&self) -> bool {
        self.bits.get_bit(13u8)
    }
    # [ doc = "Bit 12 - Data transmit in progress" ]
    pub fn txact(&self) -> bool {
        self.bits.get_bit(12u8)
    }
    # [ doc = "Bit 11 - Command transfer in progress" ]
    pub fn cmdact(&self) -> bool {
        self.bits.get_bit(11u8)
    }
    # [ doc = "Bit 10 - Data block sent/received (CRC check passed)" ]
    pub fn dbckend(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bit 9 - Start bit not detected on all data signals in wide bus mode" ]
    pub fn stbiterr(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 8 - Data end (data counter, SDIDCOUNT, is zero)" ]
    pub fn dataend(&self) -> bool {
        self.bits.get_bit(8u8)
    }
    # [ doc = "Bit 7 - Command sent (no response required)" ]
    pub fn cmdsent(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bit 6 - Command response received (CRC check passed)" ]
    pub fn cmdrend(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 5 - Received FIFO overrun error" ]
    pub fn rxoverr(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - Transmit FIFO underrun error" ]
    pub fn txunderr(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - Data timeout" ]
    pub fn dtimeout(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Command response timeout" ]
    pub fn ctimeout(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Data block sent/received (CRC check failed)" ]
    pub fn dcrcfail(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Command response received (CRC check failed)" ]
    pub fn ccrcfail(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Sta {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Sta { bits: 0u32 }
    }
}

impl Sta {
    # [ doc = "Bit 23 - CE-ATA command completion signal received for CMD61" ]
    pub fn set_ceataend(&mut self, value: bool) {
        self.bits.set_bit(23u8, value);
    }
    # [ doc = "Bit 22 - SDIO interrupt received" ]
    pub fn set_sdioit(&mut self, value: bool) {
        self.bits.set_bit(22u8, value);
    }
    # [ doc = "Bit 21 - Data available in receive FIFO" ]
    pub fn set_rxdavl(&mut self, value: bool) {
        self.bits.set_bit(21u8, value);
    }
    # [ doc = "Bit 20 - Data available in transmit FIFO" ]
    pub fn set_txdavl(&mut self, value: bool) {
        self.bits.set_bit(20u8, value);
    }
    # [ doc = "Bit 19 - Receive FIFO empty" ]
    pub fn set_rxfifoe(&mut self, value: bool) {
        self.bits.set_bit(19u8, value);
    }
    # [ doc = "Bit 18 - Transmit FIFO empty" ]
    pub fn set_txfifoe(&mut self, value: bool) {
        self.bits.set_bit(18u8, value);
    }
    # [ doc = "Bit 17 - Receive FIFO full" ]
    pub fn set_rxfifof(&mut self, value: bool) {
        self.bits.set_bit(17u8, value);
    }
    # [ doc = "Bit 16 - Transmit FIFO full" ]
    pub fn set_txfifof(&mut self, value: bool) {
        self.bits.set_bit(16u8, value);
    }
    # [ doc = "Bit 15 - Receive FIFO half full: there are at least 8 words in the FIFO" ]
    pub fn set_rxfifohf(&mut self, value: bool) {
        self.bits.set_bit(15u8, value);
    }
    # [ doc = "Bit 14 - Transmit FIFO half empty: at least 8 words can be written into the FIFO" ]
    pub fn set_txfifohe(&mut self, value: bool) {
        self.bits.set_bit(14u8, value);
    }
    # [ doc = "Bit 13 - Data receive in progress" ]
    pub fn set_rxact(&mut self, value: bool) {
        self.bits.set_bit(13u8, value);
    }
    # [ doc = "Bit 12 - Data transmit in progress" ]
    pub fn set_txact(&mut self, value: bool) {
        self.bits.set_bit(12u8, value);
    }
    # [ doc = "Bit 11 - Command transfer in progress" ]
    pub fn set_cmdact(&mut self, value: bool) {
        self.bits.set_bit(11u8, value);
    }
    # [ doc = "Bit 10 - Data block sent/received (CRC check passed)" ]
    pub fn set_dbckend(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bit 9 - Start bit not detected on all data signals in wide bus mode" ]
    pub fn set_stbiterr(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 8 - Data end (data counter, SDIDCOUNT, is zero)" ]
    pub fn set_dataend(&mut self, value: bool) {
        self.bits.set_bit(8u8, value);
    }
    # [ doc = "Bit 7 - Command sent (no response required)" ]
    pub fn set_cmdsent(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bit 6 - Command response received (CRC check passed)" ]
    pub fn set_cmdrend(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 5 - Received FIFO overrun error" ]
    pub fn set_rxoverr(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 4 - Transmit FIFO underrun error" ]
    pub fn set_txunderr(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - Data timeout" ]
    pub fn set_dtimeout(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Command response timeout" ]
    pub fn set_ctimeout(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Data block sent/received (CRC check failed)" ]
    pub fn set_dcrcfail(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Command response received (CRC check failed)" ]
    pub fn set_ccrcfail(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Icr {
    bits: u32,
}

impl Icr {
    # [ doc = "Bit 23 - CEATAEND flag clear bit" ]
    pub fn ceataendc(&self) -> bool {
        self.bits.get_bit(23u8)
    }
    # [ doc = "Bit 22 - SDIOIT flag clear bit" ]
    pub fn sdioitc(&self) -> bool {
        self.bits.get_bit(22u8)
    }
    # [ doc = "Bit 10 - DBCKEND flag clear bit" ]
    pub fn dbckendc(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bit 9 - STBITERR flag clear bit" ]
    pub fn stbiterrc(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 8 - DATAEND flag clear bit" ]
    pub fn dataendc(&self) -> bool {
        self.bits.get_bit(8u8)
    }
    # [ doc = "Bit 7 - CMDSENT flag clear bit" ]
    pub fn cmdsentc(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bit 6 - CMDREND flag clear bit" ]
    pub fn cmdrendc(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 5 - RXOVERR flag clear bit" ]
    pub fn rxoverrc(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - TXUNDERR flag clear bit" ]
    pub fn txunderrc(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - DTIMEOUT flag clear bit" ]
    pub fn dtimeoutc(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - CTIMEOUT flag clear bit" ]
    pub fn ctimeoutc(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - DCRCFAIL flag clear bit" ]
    pub fn dcrcfailc(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - CCRCFAIL flag clear bit" ]
    pub fn ccrcfailc(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Icr {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Icr { bits: 0u32 }
    }
}

impl Icr {
    # [ doc = "Bit 23 - CEATAEND flag clear bit" ]
    pub fn set_ceataendc(&mut self, value: bool) {
        self.bits.set_bit(23u8, value);
    }
    # [ doc = "Bit 22 - SDIOIT flag clear bit" ]
    pub fn set_sdioitc(&mut self, value: bool) {
        self.bits.set_bit(22u8, value);
    }
    # [ doc = "Bit 10 - DBCKEND flag clear bit" ]
    pub fn set_dbckendc(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bit 9 - STBITERR flag clear bit" ]
    pub fn set_stbiterrc(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 8 - DATAEND flag clear bit" ]
    pub fn set_dataendc(&mut self, value: bool) {
        self.bits.set_bit(8u8, value);
    }
    # [ doc = "Bit 7 - CMDSENT flag clear bit" ]
    pub fn set_cmdsentc(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bit 6 - CMDREND flag clear bit" ]
    pub fn set_cmdrendc(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 5 - RXOVERR flag clear bit" ]
    pub fn set_rxoverrc(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 4 - TXUNDERR flag clear bit" ]
    pub fn set_txunderrc(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - DTIMEOUT flag clear bit" ]
    pub fn set_dtimeoutc(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - CTIMEOUT flag clear bit" ]
    pub fn set_ctimeoutc(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - DCRCFAIL flag clear bit" ]
    pub fn set_dcrcfailc(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - CCRCFAIL flag clear bit" ]
    pub fn set_ccrcfailc(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Mask {
    bits: u32,
}

impl Mask {
    # [ doc = "Bit 23 - CE-ATA command completion signal received interrupt enable" ]
    pub fn ceataendie(&self) -> bool {
        self.bits.get_bit(23u8)
    }
    # [ doc = "Bit 22 - SDIO mode interrupt received interrupt enable" ]
    pub fn sdioitie(&self) -> bool {
        self.bits.get_bit(22u8)
    }
    # [ doc = "Bit 21 - Data available in Rx FIFO interrupt enable" ]
    pub fn rxdavlie(&self) -> bool {
        self.bits.get_bit(21u8)
    }
    # [ doc = "Bit 20 - Data available in Tx FIFO interrupt enable" ]
    pub fn txdavlie(&self) -> bool {
        self.bits.get_bit(20u8)
    }
    # [ doc = "Bit 19 - Rx FIFO empty interrupt enable" ]
    pub fn rxfifoeie(&self) -> bool {
        self.bits.get_bit(19u8)
    }
    # [ doc = "Bit 18 - Tx FIFO empty interrupt enable" ]
    pub fn txfifoeie(&self) -> bool {
        self.bits.get_bit(18u8)
    }
    # [ doc = "Bit 17 - Rx FIFO full interrupt enable" ]
    pub fn rxfifofie(&self) -> bool {
        self.bits.get_bit(17u8)
    }
    # [ doc = "Bit 16 - Tx FIFO full interrupt enable" ]
    pub fn txfifofie(&self) -> bool {
        self.bits.get_bit(16u8)
    }
    # [ doc = "Bit 15 - Rx FIFO half full interrupt enable" ]
    pub fn rxfifohfie(&self) -> bool {
        self.bits.get_bit(15u8)
    }
    # [ doc = "Bit 14 - Tx FIFO half empty interrupt enable" ]
    pub fn txfifoheie(&self) -> bool {
        self.bits.get_bit(14u8)
    }
    # [ doc = "Bit 13 - Data receive acting interrupt enable" ]
    pub fn rxactie(&self) -> bool {
        self.bits.get_bit(13u8)
    }
    # [ doc = "Bit 12 - Data transmit acting interrupt enable" ]
    pub fn txactie(&self) -> bool {
        self.bits.get_bit(12u8)
    }
    # [ doc = "Bit 11 - Command acting interrupt enable" ]
    pub fn cmdactie(&self) -> bool {
        self.bits.get_bit(11u8)
    }
    # [ doc = "Bit 10 - Data block end interrupt enable" ]
    pub fn dbckendie(&self) -> bool {
        self.bits.get_bit(10u8)
    }
    # [ doc = "Bit 9 - Start bit error interrupt enable" ]
    pub fn stbiterrie(&self) -> bool {
        self.bits.get_bit(9u8)
    }
    # [ doc = "Bit 8 - Data end interrupt enable" ]
    pub fn dataendie(&self) -> bool {
        self.bits.get_bit(8u8)
    }
    # [ doc = "Bit 7 - Command sent interrupt enable" ]
    pub fn cmdsentie(&self) -> bool {
        self.bits.get_bit(7u8)
    }
    # [ doc = "Bit 6 - Command response received interrupt enable" ]
    pub fn cmdrendie(&self) -> bool {
        self.bits.get_bit(6u8)
    }
    # [ doc = "Bit 5 - Rx FIFO overrun error interrupt enable" ]
    pub fn rxoverrie(&self) -> bool {
        self.bits.get_bit(5u8)
    }
    # [ doc = "Bit 4 - Tx FIFO underrun error interrupt enable" ]
    pub fn txunderrie(&self) -> bool {
        self.bits.get_bit(4u8)
    }
    # [ doc = "Bit 3 - Data timeout interrupt enable" ]
    pub fn dtimeoutie(&self) -> bool {
        self.bits.get_bit(3u8)
    }
    # [ doc = "Bit 2 - Command timeout interrupt enable" ]
    pub fn ctimeoutie(&self) -> bool {
        self.bits.get_bit(2u8)
    }
    # [ doc = "Bit 1 - Data CRC fail interrupt enable" ]
    pub fn dcrcfailie(&self) -> bool {
        self.bits.get_bit(1u8)
    }
    # [ doc = "Bit 0 - Command CRC fail interrupt enable" ]
    pub fn ccrcfailie(&self) -> bool {
        self.bits.get_bit(0u8)
    }
}

impl Default for Mask {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Mask { bits: 0u32 }
    }
}

impl Mask {
    # [ doc = "Bit 23 - CE-ATA command completion signal received interrupt enable" ]
    pub fn set_ceataendie(&mut self, value: bool) {
        self.bits.set_bit(23u8, value);
    }
    # [ doc = "Bit 22 - SDIO mode interrupt received interrupt enable" ]
    pub fn set_sdioitie(&mut self, value: bool) {
        self.bits.set_bit(22u8, value);
    }
    # [ doc = "Bit 21 - Data available in Rx FIFO interrupt enable" ]
    pub fn set_rxdavlie(&mut self, value: bool) {
        self.bits.set_bit(21u8, value);
    }
    # [ doc = "Bit 20 - Data available in Tx FIFO interrupt enable" ]
    pub fn set_txdavlie(&mut self, value: bool) {
        self.bits.set_bit(20u8, value);
    }
    # [ doc = "Bit 19 - Rx FIFO empty interrupt enable" ]
    pub fn set_rxfifoeie(&mut self, value: bool) {
        self.bits.set_bit(19u8, value);
    }
    # [ doc = "Bit 18 - Tx FIFO empty interrupt enable" ]
    pub fn set_txfifoeie(&mut self, value: bool) {
        self.bits.set_bit(18u8, value);
    }
    # [ doc = "Bit 17 - Rx FIFO full interrupt enable" ]
    pub fn set_rxfifofie(&mut self, value: bool) {
        self.bits.set_bit(17u8, value);
    }
    # [ doc = "Bit 16 - Tx FIFO full interrupt enable" ]
    pub fn set_txfifofie(&mut self, value: bool) {
        self.bits.set_bit(16u8, value);
    }
    # [ doc = "Bit 15 - Rx FIFO half full interrupt enable" ]
    pub fn set_rxfifohfie(&mut self, value: bool) {
        self.bits.set_bit(15u8, value);
    }
    # [ doc = "Bit 14 - Tx FIFO half empty interrupt enable" ]
    pub fn set_txfifoheie(&mut self, value: bool) {
        self.bits.set_bit(14u8, value);
    }
    # [ doc = "Bit 13 - Data receive acting interrupt enable" ]
    pub fn set_rxactie(&mut self, value: bool) {
        self.bits.set_bit(13u8, value);
    }
    # [ doc = "Bit 12 - Data transmit acting interrupt enable" ]
    pub fn set_txactie(&mut self, value: bool) {
        self.bits.set_bit(12u8, value);
    }
    # [ doc = "Bit 11 - Command acting interrupt enable" ]
    pub fn set_cmdactie(&mut self, value: bool) {
        self.bits.set_bit(11u8, value);
    }
    # [ doc = "Bit 10 - Data block end interrupt enable" ]
    pub fn set_dbckendie(&mut self, value: bool) {
        self.bits.set_bit(10u8, value);
    }
    # [ doc = "Bit 9 - Start bit error interrupt enable" ]
    pub fn set_stbiterrie(&mut self, value: bool) {
        self.bits.set_bit(9u8, value);
    }
    # [ doc = "Bit 8 - Data end interrupt enable" ]
    pub fn set_dataendie(&mut self, value: bool) {
        self.bits.set_bit(8u8, value);
    }
    # [ doc = "Bit 7 - Command sent interrupt enable" ]
    pub fn set_cmdsentie(&mut self, value: bool) {
        self.bits.set_bit(7u8, value);
    }
    # [ doc = "Bit 6 - Command response received interrupt enable" ]
    pub fn set_cmdrendie(&mut self, value: bool) {
        self.bits.set_bit(6u8, value);
    }
    # [ doc = "Bit 5 - Rx FIFO overrun error interrupt enable" ]
    pub fn set_rxoverrie(&mut self, value: bool) {
        self.bits.set_bit(5u8, value);
    }
    # [ doc = "Bit 4 - Tx FIFO underrun error interrupt enable" ]
    pub fn set_txunderrie(&mut self, value: bool) {
        self.bits.set_bit(4u8, value);
    }
    # [ doc = "Bit 3 - Data timeout interrupt enable" ]
    pub fn set_dtimeoutie(&mut self, value: bool) {
        self.bits.set_bit(3u8, value);
    }
    # [ doc = "Bit 2 - Command timeout interrupt enable" ]
    pub fn set_ctimeoutie(&mut self, value: bool) {
        self.bits.set_bit(2u8, value);
    }
    # [ doc = "Bit 1 - Data CRC fail interrupt enable" ]
    pub fn set_dcrcfailie(&mut self, value: bool) {
        self.bits.set_bit(1u8, value);
    }
    # [ doc = "Bit 0 - Command CRC fail interrupt enable" ]
    pub fn set_ccrcfailie(&mut self, value: bool) {
        self.bits.set_bit(0u8, value);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Fifocnt {
    bits: u32,
}

impl Fifocnt {
    # [ doc = "Bits 0:23 - Remaining number of words to be written to or read from the FIFO" ]
    pub fn fifocount(&self) -> u32 {
        self.bits.get_range(0u8..24u8) as u32
    }
}

impl Default for Fifocnt {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Fifocnt { bits: 0u32 }
    }
}

impl Fifocnt {
    # [ doc = "Bits 0:23 - Remaining number of words to be written to or read from the FIFO" ]
    pub fn set_fifocount(&mut self, value: u32) {
        self.bits.set_range(0u8..24u8, value as u32);
    }
}

# [ derive ( Debug , Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Fifo {
    bits: u32,
}

impl Fifo {
    # [ doc = "Bits 0:31 - Receive and transmit FIFO data" ]
    pub fn fifodata(&self) -> u32 {
        self.bits.get_range(0u8..32u8) as u32
    }
}

impl Default for Fifo {
    # [ doc = r" Reset value" ]
    fn default() -> Self {
        Fifo { bits: 0u32 }
    }
}

impl Fifo {
    # [ doc = "Bits 0:31 - Receive and transmit FIFO data" ]
    pub fn set_fifodata(&mut self, value: u32) {
        self.bits.set_range(0u8..32u8, value as u32);
    }
}
