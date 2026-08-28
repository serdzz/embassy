#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ucb1ctl0_spi: Ucb1ctl0Spi,
    ucb1ctl1_spi: Ucb1ctl1Spi,
    ucb1br0_spi: Ucb1br0Spi,
    ucb1br1_spi: Ucb1br1Spi,
    _reserved4: [u8; 0x01],
    ucb1stat_spi: Ucb1statSpi,
    ucb1rxbuf_spi: Ucb1rxbufSpi,
    ucb1txbuf_spi: Ucb1txbufSpi,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI B1 Control Register 0"]
    #[inline(always)]
    pub const fn ucb1ctl0_spi(&self) -> &Ucb1ctl0Spi {
        &self.ucb1ctl0_spi
    }
    #[doc = "0x01 - USCI B1 Control Register 1"]
    #[inline(always)]
    pub const fn ucb1ctl1_spi(&self) -> &Ucb1ctl1Spi {
        &self.ucb1ctl1_spi
    }
    #[doc = "0x02 - USCI B1 Baud Rate 0"]
    #[inline(always)]
    pub const fn ucb1br0_spi(&self) -> &Ucb1br0Spi {
        &self.ucb1br0_spi
    }
    #[doc = "0x03 - USCI B1 Baud Rate 1"]
    #[inline(always)]
    pub const fn ucb1br1_spi(&self) -> &Ucb1br1Spi {
        &self.ucb1br1_spi
    }
    #[doc = "0x05 - USCI B1 Status Register"]
    #[inline(always)]
    pub const fn ucb1stat_spi(&self) -> &Ucb1statSpi {
        &self.ucb1stat_spi
    }
    #[doc = "0x06 - USCI B1 Receive Buffer"]
    #[inline(always)]
    pub const fn ucb1rxbuf_spi(&self) -> &Ucb1rxbufSpi {
        &self.ucb1rxbuf_spi
    }
    #[doc = "0x07 - USCI B1 Transmit Buffer"]
    #[inline(always)]
    pub const fn ucb1txbuf_spi(&self) -> &Ucb1txbufSpi {
        &self.ucb1txbuf_spi
    }
}
#[doc = "UCB1CTL0_SPI (rw) register accessor: USCI B1 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1ctl0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1ctl0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ctl0_spi`] module"]
#[doc(alias = "UCB1CTL0_SPI")]
pub type Ucb1ctl0Spi = crate::Reg<ucb1ctl0_spi::Ucb1ctl0SpiSpec>;
#[doc = "USCI B1 Control Register 0"]
pub mod ucb1ctl0_spi;
#[doc = "UCB1CTL1_SPI (rw) register accessor: USCI B1 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1ctl1_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1ctl1_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ctl1_spi`] module"]
#[doc(alias = "UCB1CTL1_SPI")]
pub type Ucb1ctl1Spi = crate::Reg<ucb1ctl1_spi::Ucb1ctl1SpiSpec>;
#[doc = "USCI B1 Control Register 1"]
pub mod ucb1ctl1_spi;
#[doc = "UCB1BR0_SPI (rw) register accessor: USCI B1 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1br0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1br0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1br0_spi`] module"]
#[doc(alias = "UCB1BR0_SPI")]
pub type Ucb1br0Spi = crate::Reg<ucb1br0_spi::Ucb1br0SpiSpec>;
#[doc = "USCI B1 Baud Rate 0"]
pub mod ucb1br0_spi;
#[doc = "UCB1BR1_SPI (rw) register accessor: USCI B1 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1br1_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1br1_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1br1_spi`] module"]
#[doc(alias = "UCB1BR1_SPI")]
pub type Ucb1br1Spi = crate::Reg<ucb1br1_spi::Ucb1br1SpiSpec>;
#[doc = "USCI B1 Baud Rate 1"]
pub mod ucb1br1_spi;
#[doc = "UCB1STAT_SPI (rw) register accessor: USCI B1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1stat_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1stat_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1stat_spi`] module"]
#[doc(alias = "UCB1STAT_SPI")]
pub type Ucb1statSpi = crate::Reg<ucb1stat_spi::Ucb1statSpiSpec>;
#[doc = "USCI B1 Status Register"]
pub mod ucb1stat_spi;
#[doc = "UCB1RXBUF_SPI (rw) register accessor: USCI B1 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1rxbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1rxbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1rxbuf_spi`] module"]
#[doc(alias = "UCB1RXBUF_SPI")]
pub type Ucb1rxbufSpi = crate::Reg<ucb1rxbuf_spi::Ucb1rxbufSpiSpec>;
#[doc = "USCI B1 Receive Buffer"]
pub mod ucb1rxbuf_spi;
#[doc = "UCB1TXBUF_SPI (rw) register accessor: USCI B1 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1txbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1txbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1txbuf_spi`] module"]
#[doc(alias = "UCB1TXBUF_SPI")]
pub type Ucb1txbufSpi = crate::Reg<ucb1txbuf_spi::Ucb1txbufSpiSpec>;
#[doc = "USCI B1 Transmit Buffer"]
pub mod ucb1txbuf_spi;
