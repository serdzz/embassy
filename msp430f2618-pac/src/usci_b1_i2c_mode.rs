#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ucb1ctl0: Ucb1ctl0,
    ucb1ctl1: Ucb1ctl1,
    ucb1br0: Ucb1br0,
    ucb1br1: Ucb1br1,
    ucb1i2cie: Ucb1i2cie,
    ucb1stat: Ucb1stat,
    ucb1rxbuf: Ucb1rxbuf,
    ucb1txbuf: Ucb1txbuf,
    _reserved8: [u8; 0x9c],
    ucb1i2coa: Ucb1i2coa,
    ucb1i2csa: Ucb1i2csa,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI B1 Control Register 0"]
    #[inline(always)]
    pub const fn ucb1ctl0(&self) -> &Ucb1ctl0 {
        &self.ucb1ctl0
    }
    #[doc = "0x01 - USCI B1 Control Register 1"]
    #[inline(always)]
    pub const fn ucb1ctl1(&self) -> &Ucb1ctl1 {
        &self.ucb1ctl1
    }
    #[doc = "0x02 - USCI B1 Baud Rate 0"]
    #[inline(always)]
    pub const fn ucb1br0(&self) -> &Ucb1br0 {
        &self.ucb1br0
    }
    #[doc = "0x03 - USCI B1 Baud Rate 1"]
    #[inline(always)]
    pub const fn ucb1br1(&self) -> &Ucb1br1 {
        &self.ucb1br1
    }
    #[doc = "0x04 - USCI B1 I2C Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb1i2cie(&self) -> &Ucb1i2cie {
        &self.ucb1i2cie
    }
    #[doc = "0x05 - USCI B1 Status Register"]
    #[inline(always)]
    pub const fn ucb1stat(&self) -> &Ucb1stat {
        &self.ucb1stat
    }
    #[doc = "0x06 - USCI B1 Receive Buffer"]
    #[inline(always)]
    pub const fn ucb1rxbuf(&self) -> &Ucb1rxbuf {
        &self.ucb1rxbuf
    }
    #[doc = "0x07 - USCI B1 Transmit Buffer"]
    #[inline(always)]
    pub const fn ucb1txbuf(&self) -> &Ucb1txbuf {
        &self.ucb1txbuf
    }
    #[doc = "0xa4 - USCI B1 I2C Own Address"]
    #[inline(always)]
    pub const fn ucb1i2coa(&self) -> &Ucb1i2coa {
        &self.ucb1i2coa
    }
    #[doc = "0xa6 - USCI B1 I2C Slave Address"]
    #[inline(always)]
    pub const fn ucb1i2csa(&self) -> &Ucb1i2csa {
        &self.ucb1i2csa
    }
}
#[doc = "UCB1CTL0 (rw) register accessor: USCI B1 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ctl0`] module"]
#[doc(alias = "UCB1CTL0")]
pub type Ucb1ctl0 = crate::Reg<ucb1ctl0::Ucb1ctl0Spec>;
#[doc = "USCI B1 Control Register 0"]
pub mod ucb1ctl0;
#[doc = "UCB1CTL1 (rw) register accessor: USCI B1 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ctl1`] module"]
#[doc(alias = "UCB1CTL1")]
pub type Ucb1ctl1 = crate::Reg<ucb1ctl1::Ucb1ctl1Spec>;
#[doc = "USCI B1 Control Register 1"]
pub mod ucb1ctl1;
#[doc = "UCB1BR0 (rw) register accessor: USCI B1 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1br0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1br0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1br0`] module"]
#[doc(alias = "UCB1BR0")]
pub type Ucb1br0 = crate::Reg<ucb1br0::Ucb1br0Spec>;
#[doc = "USCI B1 Baud Rate 0"]
pub mod ucb1br0;
#[doc = "UCB1BR1 (rw) register accessor: USCI B1 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1br1`] module"]
#[doc(alias = "UCB1BR1")]
pub type Ucb1br1 = crate::Reg<ucb1br1::Ucb1br1Spec>;
#[doc = "USCI B1 Baud Rate 1"]
pub mod ucb1br1;
#[doc = "UCB1I2CIE (rw) register accessor: USCI B1 I2C Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1i2cie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1i2cie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1i2cie`] module"]
#[doc(alias = "UCB1I2CIE")]
pub type Ucb1i2cie = crate::Reg<ucb1i2cie::Ucb1i2cieSpec>;
#[doc = "USCI B1 I2C Interrupt Enable Register"]
pub mod ucb1i2cie;
#[doc = "UCB1STAT (rw) register accessor: USCI B1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1stat`] module"]
#[doc(alias = "UCB1STAT")]
pub type Ucb1stat = crate::Reg<ucb1stat::Ucb1statSpec>;
#[doc = "USCI B1 Status Register"]
pub mod ucb1stat;
#[doc = "UCB1RXBUF (rw) register accessor: USCI B1 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1rxbuf`] module"]
#[doc(alias = "UCB1RXBUF")]
pub type Ucb1rxbuf = crate::Reg<ucb1rxbuf::Ucb1rxbufSpec>;
#[doc = "USCI B1 Receive Buffer"]
pub mod ucb1rxbuf;
#[doc = "UCB1TXBUF (rw) register accessor: USCI B1 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1txbuf`] module"]
#[doc(alias = "UCB1TXBUF")]
pub type Ucb1txbuf = crate::Reg<ucb1txbuf::Ucb1txbufSpec>;
#[doc = "USCI B1 Transmit Buffer"]
pub mod ucb1txbuf;
#[doc = "UCB1I2COA (rw) register accessor: USCI B1 I2C Own Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1i2coa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1i2coa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1i2coa`] module"]
#[doc(alias = "UCB1I2COA")]
pub type Ucb1i2coa = crate::Reg<ucb1i2coa::Ucb1i2coaSpec>;
#[doc = "USCI B1 I2C Own Address"]
pub mod ucb1i2coa;
#[doc = "UCB1I2CSA (rw) register accessor: USCI B1 I2C Slave Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1i2csa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1i2csa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1i2csa`] module"]
#[doc(alias = "UCB1I2CSA")]
pub type Ucb1i2csa = crate::Reg<ucb1i2csa::Ucb1i2csaSpec>;
#[doc = "USCI B1 I2C Slave Address"]
pub mod ucb1i2csa;
