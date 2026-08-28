#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ucb0ctl0_spi: Ucb0ctl0Spi,
    ucb0ctl1_spi: Ucb0ctl1Spi,
    ucb0br0_spi: Ucb0br0Spi,
    ucb0br1_spi: Ucb0br1Spi,
    _reserved4: [u8; 0x01],
    ucb0stat_spi: Ucb0statSpi,
    ucb0rxbuf_spi: Ucb0rxbufSpi,
    ucb0txbuf_spi: Ucb0txbufSpi,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 0"]
    #[inline(always)]
    pub const fn ucb0ctl0_spi(&self) -> &Ucb0ctl0Spi {
        &self.ucb0ctl0_spi
    }
    #[doc = "0x01 - USCI B0 Control Register 1"]
    #[inline(always)]
    pub const fn ucb0ctl1_spi(&self) -> &Ucb0ctl1Spi {
        &self.ucb0ctl1_spi
    }
    #[doc = "0x02 - USCI B0 Baud Rate 0"]
    #[inline(always)]
    pub const fn ucb0br0_spi(&self) -> &Ucb0br0Spi {
        &self.ucb0br0_spi
    }
    #[doc = "0x03 - USCI B0 Baud Rate 1"]
    #[inline(always)]
    pub const fn ucb0br1_spi(&self) -> &Ucb0br1Spi {
        &self.ucb0br1_spi
    }
    #[doc = "0x05 - USCI B0 Status Register"]
    #[inline(always)]
    pub const fn ucb0stat_spi(&self) -> &Ucb0statSpi {
        &self.ucb0stat_spi
    }
    #[doc = "0x06 - USCI B0 Receive Buffer"]
    #[inline(always)]
    pub const fn ucb0rxbuf_spi(&self) -> &Ucb0rxbufSpi {
        &self.ucb0rxbuf_spi
    }
    #[doc = "0x07 - USCI B0 Transmit Buffer"]
    #[inline(always)]
    pub const fn ucb0txbuf_spi(&self) -> &Ucb0txbufSpi {
        &self.ucb0txbuf_spi
    }
}
#[doc = "UCB0CTL0_SPI (rw) register accessor: USCI B0 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctl0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctl0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl0_spi`] module"]
#[doc(alias = "UCB0CTL0_SPI")]
pub type Ucb0ctl0Spi = crate::Reg<ucb0ctl0_spi::Ucb0ctl0SpiSpec>;
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0_spi;
#[doc = "UCB0CTL1_SPI (rw) register accessor: USCI B0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctl1_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctl1_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl1_spi`] module"]
#[doc(alias = "UCB0CTL1_SPI")]
pub type Ucb0ctl1Spi = crate::Reg<ucb0ctl1_spi::Ucb0ctl1SpiSpec>;
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1_spi;
#[doc = "UCB0BR0_SPI (rw) register accessor: USCI B0 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0br0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0br0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0br0_spi`] module"]
#[doc(alias = "UCB0BR0_SPI")]
pub type Ucb0br0Spi = crate::Reg<ucb0br0_spi::Ucb0br0SpiSpec>;
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0_spi;
#[doc = "UCB0BR1_SPI (rw) register accessor: USCI B0 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0br1_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0br1_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0br1_spi`] module"]
#[doc(alias = "UCB0BR1_SPI")]
pub type Ucb0br1Spi = crate::Reg<ucb0br1_spi::Ucb0br1SpiSpec>;
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1_spi;
#[doc = "UCB0STAT_SPI (rw) register accessor: USCI B0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0stat_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0stat_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0stat_spi`] module"]
#[doc(alias = "UCB0STAT_SPI")]
pub type Ucb0statSpi = crate::Reg<ucb0stat_spi::Ucb0statSpiSpec>;
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat_spi;
#[doc = "UCB0RXBUF_SPI (rw) register accessor: USCI B0 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0rxbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0rxbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0rxbuf_spi`] module"]
#[doc(alias = "UCB0RXBUF_SPI")]
pub type Ucb0rxbufSpi = crate::Reg<ucb0rxbuf_spi::Ucb0rxbufSpiSpec>;
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf_spi;
#[doc = "UCB0TXBUF_SPI (rw) register accessor: USCI B0 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0txbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0txbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0txbuf_spi`] module"]
#[doc(alias = "UCB0TXBUF_SPI")]
pub type Ucb0txbufSpi = crate::Reg<ucb0txbuf_spi::Ucb0txbufSpiSpec>;
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf_spi;
