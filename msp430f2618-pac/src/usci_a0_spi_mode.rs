#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uca0ctl0_spi: Uca0ctl0Spi,
    uca0ctl1_spi: Uca0ctl1Spi,
    uca0br0_spi: Uca0br0Spi,
    uca0br1_spi: Uca0br1Spi,
    uca0mctl_spi: Uca0mctlSpi,
    uca0stat_spi: Uca0statSpi,
    uca0rxbuf_spi: Uca0rxbufSpi,
    uca0txbuf_spi: Uca0txbufSpi,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI A0 Control Register 0"]
    #[inline(always)]
    pub const fn uca0ctl0_spi(&self) -> &Uca0ctl0Spi {
        &self.uca0ctl0_spi
    }
    #[doc = "0x01 - USCI A0 Control Register 1"]
    #[inline(always)]
    pub const fn uca0ctl1_spi(&self) -> &Uca0ctl1Spi {
        &self.uca0ctl1_spi
    }
    #[doc = "0x02 - USCI A0 Baud Rate 0"]
    #[inline(always)]
    pub const fn uca0br0_spi(&self) -> &Uca0br0Spi {
        &self.uca0br0_spi
    }
    #[doc = "0x03 - USCI A0 Baud Rate 1"]
    #[inline(always)]
    pub const fn uca0br1_spi(&self) -> &Uca0br1Spi {
        &self.uca0br1_spi
    }
    #[doc = "0x04 - USCI A0 Modulation Control"]
    #[inline(always)]
    pub const fn uca0mctl_spi(&self) -> &Uca0mctlSpi {
        &self.uca0mctl_spi
    }
    #[doc = "0x05 - USCI A0 Status Register"]
    #[inline(always)]
    pub const fn uca0stat_spi(&self) -> &Uca0statSpi {
        &self.uca0stat_spi
    }
    #[doc = "0x06 - USCI A0 Receive Buffer"]
    #[inline(always)]
    pub const fn uca0rxbuf_spi(&self) -> &Uca0rxbufSpi {
        &self.uca0rxbuf_spi
    }
    #[doc = "0x07 - USCI A0 Transmit Buffer"]
    #[inline(always)]
    pub const fn uca0txbuf_spi(&self) -> &Uca0txbufSpi {
        &self.uca0txbuf_spi
    }
}
#[doc = "UCA0CTL0_SPI (rw) register accessor: USCI A0 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctl0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctl0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctl0_spi`] module"]
#[doc(alias = "UCA0CTL0_SPI")]
pub type Uca0ctl0Spi = crate::Reg<uca0ctl0_spi::Uca0ctl0SpiSpec>;
#[doc = "USCI A0 Control Register 0"]
pub mod uca0ctl0_spi;
#[doc = "UCA0CTL1_SPI (rw) register accessor: USCI A0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctl1_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctl1_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctl1_spi`] module"]
#[doc(alias = "UCA0CTL1_SPI")]
pub type Uca0ctl1Spi = crate::Reg<uca0ctl1_spi::Uca0ctl1SpiSpec>;
#[doc = "USCI A0 Control Register 1"]
pub mod uca0ctl1_spi;
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
#[doc = "UCA0MCTL_SPI (rw) register accessor: USCI A0 Modulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0mctl_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0mctl_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0mctl_spi`] module"]
#[doc(alias = "UCA0MCTL_SPI")]
pub type Uca0mctlSpi = crate::Reg<uca0mctl_spi::Uca0mctlSpiSpec>;
#[doc = "USCI A0 Modulation Control"]
pub mod uca0mctl_spi;
#[doc = "UCA0STAT_SPI (rw) register accessor: USCI A0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0stat_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0stat_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0stat_spi`] module"]
#[doc(alias = "UCA0STAT_SPI")]
pub type Uca0statSpi = crate::Reg<uca0stat_spi::Uca0statSpiSpec>;
#[doc = "USCI A0 Status Register"]
pub mod uca0stat_spi;
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
