#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uca0ctl1_spi: Uca0ctl1Spi,
    uca0ctl0_spi: Uca0ctl0Spi,
    _reserved2: [u8; 0x04],
    uca0br0_spi: Uca0br0Spi,
    uca0br1_spi: Uca0br1Spi,
    _reserved4: [u8; 0x02],
    uca0statw_spi: Uca0statwSpi,
    _reserved5: [u8; 0x01],
    uca0rxbuf_spi: Uca0rxbufSpi,
    uca0txbuf_spi: Uca0txbufSpi,
    _reserved7: [u8; 0x0a],
    uca0ie_spi: Uca0ieSpi,
    _reserved8: [u8; 0x01],
    uca0ifg_spi: Uca0ifgSpi,
    _reserved9: [u8; 0x01],
    uca0iv_spi: Uca0ivSpi,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI A0 Control Register 1"]
    #[inline(always)]
    pub const fn uca0ctl1_spi(&self) -> &Uca0ctl1Spi {
        &self.uca0ctl1_spi
    }
    #[doc = "0x01 - USCI A0 Control Register 0"]
    #[inline(always)]
    pub const fn uca0ctl0_spi(&self) -> &Uca0ctl0Spi {
        &self.uca0ctl0_spi
    }
    #[doc = "0x06 - USCI A0 Baud Rate 0"]
    #[inline(always)]
    pub const fn uca0br0_spi(&self) -> &Uca0br0Spi {
        &self.uca0br0_spi
    }
    #[doc = "0x07 - USCI A0 Baud Rate 1"]
    #[inline(always)]
    pub const fn uca0br1_spi(&self) -> &Uca0br1Spi {
        &self.uca0br1_spi
    }
    #[doc = "0x0a - USCI A0 Status Register"]
    #[inline(always)]
    pub const fn uca0statw_spi(&self) -> &Uca0statwSpi {
        &self.uca0statw_spi
    }
    #[doc = "0x0c - USCI A0 Receive Buffer"]
    #[inline(always)]
    pub const fn uca0rxbuf_spi(&self) -> &Uca0rxbufSpi {
        &self.uca0rxbuf_spi
    }
    #[doc = "0x0e - USCI A0 Transmit Buffer"]
    #[inline(always)]
    pub const fn uca0txbuf_spi(&self) -> &Uca0txbufSpi {
        &self.uca0txbuf_spi
    }
    #[doc = "0x1a - USCI A0 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn uca0ie_spi(&self) -> &Uca0ieSpi {
        &self.uca0ie_spi
    }
    #[doc = "0x1c - USCI A0 Interrupt Flags Register"]
    #[inline(always)]
    pub const fn uca0ifg_spi(&self) -> &Uca0ifgSpi {
        &self.uca0ifg_spi
    }
    #[doc = "0x1e - USCI A0 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn uca0iv_spi(&self) -> &Uca0ivSpi {
        &self.uca0iv_spi
    }
}
#[doc = "UCA0CTL1_SPI (rw) register accessor: USCI A0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctl1_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctl1_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctl1_spi`] module"]
#[doc(alias = "UCA0CTL1_SPI")]
pub type Uca0ctl1Spi = crate::Reg<uca0ctl1_spi::Uca0ctl1SpiSpec>;
#[doc = "USCI A0 Control Register 1"]
pub mod uca0ctl1_spi;
#[doc = "UCA0CTL0_SPI (rw) register accessor: USCI A0 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctl0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctl0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctl0_spi`] module"]
#[doc(alias = "UCA0CTL0_SPI")]
pub type Uca0ctl0Spi = crate::Reg<uca0ctl0_spi::Uca0ctl0SpiSpec>;
#[doc = "USCI A0 Control Register 0"]
pub mod uca0ctl0_spi;
#[doc = "UCA0BR0_SPI (rw) register accessor: USCI A0 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0br0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0br0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0br0_spi`] module"]
#[doc(alias = "UCA0BR0_SPI")]
pub type Uca0br0Spi = crate::Reg<uca0br0_spi::Uca0br0SpiSpec>;
#[doc = "USCI A0 Baud Rate 0"]
pub mod uca0br0_spi;
#[doc = "UCA0BR1_SPI (rw) register accessor: USCI A0 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0br1_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0br1_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0br1_spi`] module"]
#[doc(alias = "UCA0BR1_SPI")]
pub type Uca0br1Spi = crate::Reg<uca0br1_spi::Uca0br1SpiSpec>;
#[doc = "USCI A0 Baud Rate 1"]
pub mod uca0br1_spi;
#[doc = "UCA0STATW_SPI (rw) register accessor: USCI A0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0statw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0statw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0statw_spi`] module"]
#[doc(alias = "UCA0STATW_SPI")]
pub type Uca0statwSpi = crate::Reg<uca0statw_spi::Uca0statwSpiSpec>;
#[doc = "USCI A0 Status Register"]
pub mod uca0statw_spi;
#[doc = "UCA0IE_SPI (rw) register accessor: USCI A0 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ie_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ie_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ie_spi`] module"]
#[doc(alias = "UCA0IE_SPI")]
pub type Uca0ieSpi = crate::Reg<uca0ie_spi::Uca0ieSpiSpec>;
#[doc = "USCI A0 Interrupt Enable Register"]
pub mod uca0ie_spi;
#[doc = "UCA0IFG_SPI (rw) register accessor: USCI A0 Interrupt Flags Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ifg_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ifg_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ifg_spi`] module"]
#[doc(alias = "UCA0IFG_SPI")]
pub type Uca0ifgSpi = crate::Reg<uca0ifg_spi::Uca0ifgSpiSpec>;
#[doc = "USCI A0 Interrupt Flags Register"]
pub mod uca0ifg_spi;
#[doc = "UCA0RXBUF_SPI (rw) register accessor: USCI A0 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0rxbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0rxbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0rxbuf_spi`] module"]
#[doc(alias = "UCA0RXBUF_SPI")]
pub type Uca0rxbufSpi = crate::Reg<uca0rxbuf_spi::Uca0rxbufSpiSpec>;
#[doc = "USCI A0 Receive Buffer"]
pub mod uca0rxbuf_spi;
#[doc = "UCA0TXBUF_SPI (rw) register accessor: USCI A0 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0txbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0txbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0txbuf_spi`] module"]
#[doc(alias = "UCA0TXBUF_SPI")]
pub type Uca0txbufSpi = crate::Reg<uca0txbuf_spi::Uca0txbufSpiSpec>;
#[doc = "USCI A0 Transmit Buffer"]
pub mod uca0txbuf_spi;
#[doc = "UCA0IV_SPI (rw) register accessor: USCI A0 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0iv_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0iv_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0iv_spi`] module"]
#[doc(alias = "UCA0IV_SPI")]
pub type Uca0ivSpi = crate::Reg<uca0iv_spi::Uca0ivSpiSpec>;
#[doc = "USCI A0 Interrupt Vector Register"]
pub mod uca0iv_spi;
