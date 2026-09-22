#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ucb0ctl1_spi: Ucb0ctl1Spi,
    ucb0ctl0_spi: Ucb0ctl0Spi,
    _reserved2: [u8; 0x04],
    ucb0br0_spi: Ucb0br0Spi,
    ucb0br1_spi: Ucb0br1Spi,
    _reserved4: [u8; 0x04],
    ucb0rxbuf_spi: Ucb0rxbufSpi,
    ucb0txbuf_spi: Ucb0txbufSpi,
    _reserved6: [u8; 0x1a],
    ucb0ie_spi: Ucb0ieSpi,
    ucb0ifg_spi: Ucb0ifgSpi,
    ucb0iv_spi: Ucb0ivSpi,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 1"]
    #[inline(always)]
    pub const fn ucb0ctl1_spi(&self) -> &Ucb0ctl1Spi {
        &self.ucb0ctl1_spi
    }
    #[doc = "0x01 - USCI B0 Control Register 0"]
    #[inline(always)]
    pub const fn ucb0ctl0_spi(&self) -> &Ucb0ctl0Spi {
        &self.ucb0ctl0_spi
    }
    #[doc = "0x06 - USCI B0 Baud Rate 0"]
    #[inline(always)]
    pub const fn ucb0br0_spi(&self) -> &Ucb0br0Spi {
        &self.ucb0br0_spi
    }
    #[doc = "0x07 - USCI B0 Baud Rate 1"]
    #[inline(always)]
    pub const fn ucb0br1_spi(&self) -> &Ucb0br1Spi {
        &self.ucb0br1_spi
    }
    #[doc = "0x0c - USCI B0 Receive Buffer"]
    #[inline(always)]
    pub const fn ucb0rxbuf_spi(&self) -> &Ucb0rxbufSpi {
        &self.ucb0rxbuf_spi
    }
    #[doc = "0x0e - USCI B0 Transmit Buffer"]
    #[inline(always)]
    pub const fn ucb0txbuf_spi(&self) -> &Ucb0txbufSpi {
        &self.ucb0txbuf_spi
    }
    #[doc = "0x2a - USCI B0 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb0ie_spi(&self) -> &Ucb0ieSpi {
        &self.ucb0ie_spi
    }
    #[doc = "0x2c - USCI B0 Interrupt Flags Register"]
    #[inline(always)]
    pub const fn ucb0ifg_spi(&self) -> &Ucb0ifgSpi {
        &self.ucb0ifg_spi
    }
    #[doc = "0x2e - USCI B0 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucb0iv_spi(&self) -> &Ucb0ivSpi {
        &self.ucb0iv_spi
    }
}
#[doc = "UCB0CTL1_SPI (rw) register accessor: USCI B0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctl1_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctl1_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl1_spi`] module"]
#[doc(alias = "UCB0CTL1_SPI")]
pub type Ucb0ctl1Spi = crate::Reg<ucb0ctl1_spi::Ucb0ctl1SpiSpec>;
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1_spi;
#[doc = "UCB0CTL0_SPI (rw) register accessor: USCI B0 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctl0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctl0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl0_spi`] module"]
#[doc(alias = "UCB0CTL0_SPI")]
pub type Ucb0ctl0Spi = crate::Reg<ucb0ctl0_spi::Ucb0ctl0SpiSpec>;
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0_spi;
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
#[doc = "UCB0IE_SPI (rw) register accessor: USCI B0 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ie_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ie_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ie_spi`] module"]
#[doc(alias = "UCB0IE_SPI")]
pub type Ucb0ieSpi = crate::Reg<ucb0ie_spi::Ucb0ieSpiSpec>;
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie_spi;
#[doc = "UCB0IFG_SPI (rw) register accessor: USCI B0 Interrupt Flags Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ifg_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ifg_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ifg_spi`] module"]
#[doc(alias = "UCB0IFG_SPI")]
pub type Ucb0ifgSpi = crate::Reg<ucb0ifg_spi::Ucb0ifgSpiSpec>;
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg_spi;
#[doc = "UCB0IV_SPI (rw) register accessor: USCI B0 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0iv_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0iv_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0iv_spi`] module"]
#[doc(alias = "UCB0IV_SPI")]
pub type Ucb0ivSpi = crate::Reg<ucb0iv_spi::Ucb0ivSpiSpec>;
#[doc = "USCI B0 Interrupt Vector Register"]
pub mod ucb0iv_spi;
