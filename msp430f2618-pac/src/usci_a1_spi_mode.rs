#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uca1ctl0_spi: Uca1ctl0Spi,
    uca1ctl1_spi: Uca1ctl1Spi,
    uca1br0_spi: Uca1br0Spi,
    uca1br1_spi: Uca1br1Spi,
    uca1mctl_spi: Uca1mctlSpi,
    uca1stat_spi: Uca1statSpi,
    uca1rxbuf_spi: Uca1rxbufSpi,
    uca1txbuf_spi: Uca1txbufSpi,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI A1 Control Register 0"]
    #[inline(always)]
    pub const fn uca1ctl0_spi(&self) -> &Uca1ctl0Spi {
        &self.uca1ctl0_spi
    }
    #[doc = "0x01 - USCI A1 Control Register 1"]
    #[inline(always)]
    pub const fn uca1ctl1_spi(&self) -> &Uca1ctl1Spi {
        &self.uca1ctl1_spi
    }
    #[doc = "0x02 - USCI A1 Baud Rate 0"]
    #[inline(always)]
    pub const fn uca1br0_spi(&self) -> &Uca1br0Spi {
        &self.uca1br0_spi
    }
    #[doc = "0x03 - USCI A1 Baud Rate 1"]
    #[inline(always)]
    pub const fn uca1br1_spi(&self) -> &Uca1br1Spi {
        &self.uca1br1_spi
    }
    #[doc = "0x04 - USCI A1 Modulation Control"]
    #[inline(always)]
    pub const fn uca1mctl_spi(&self) -> &Uca1mctlSpi {
        &self.uca1mctl_spi
    }
    #[doc = "0x05 - USCI A1 Status Register"]
    #[inline(always)]
    pub const fn uca1stat_spi(&self) -> &Uca1statSpi {
        &self.uca1stat_spi
    }
    #[doc = "0x06 - USCI A1 Receive Buffer"]
    #[inline(always)]
    pub const fn uca1rxbuf_spi(&self) -> &Uca1rxbufSpi {
        &self.uca1rxbuf_spi
    }
    #[doc = "0x07 - USCI A1 Transmit Buffer"]
    #[inline(always)]
    pub const fn uca1txbuf_spi(&self) -> &Uca1txbufSpi {
        &self.uca1txbuf_spi
    }
}
#[doc = "UCA1CTL0_SPI (rw) register accessor: USCI A1 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ctl0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ctl0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ctl0_spi`] module"]
#[doc(alias = "UCA1CTL0_SPI")]
pub type Uca1ctl0Spi = crate::Reg<uca1ctl0_spi::Uca1ctl0SpiSpec>;
#[doc = "USCI A1 Control Register 0"]
pub mod uca1ctl0_spi;
#[doc = "UCA1CTL1_SPI (rw) register accessor: USCI A1 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ctl1_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ctl1_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ctl1_spi`] module"]
#[doc(alias = "UCA1CTL1_SPI")]
pub type Uca1ctl1Spi = crate::Reg<uca1ctl1_spi::Uca1ctl1SpiSpec>;
#[doc = "USCI A1 Control Register 1"]
pub mod uca1ctl1_spi;
#[doc = "UCA1BR0_SPI (rw) register accessor: USCI A1 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1br0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1br0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1br0_spi`] module"]
#[doc(alias = "UCA1BR0_SPI")]
pub type Uca1br0Spi = crate::Reg<uca1br0_spi::Uca1br0SpiSpec>;
#[doc = "USCI A1 Baud Rate 0"]
pub mod uca1br0_spi;
#[doc = "UCA1BR1_SPI (rw) register accessor: USCI A1 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1br1_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1br1_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1br1_spi`] module"]
#[doc(alias = "UCA1BR1_SPI")]
pub type Uca1br1Spi = crate::Reg<uca1br1_spi::Uca1br1SpiSpec>;
#[doc = "USCI A1 Baud Rate 1"]
pub mod uca1br1_spi;
#[doc = "UCA1MCTL_SPI (rw) register accessor: USCI A1 Modulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1mctl_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1mctl_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1mctl_spi`] module"]
#[doc(alias = "UCA1MCTL_SPI")]
pub type Uca1mctlSpi = crate::Reg<uca1mctl_spi::Uca1mctlSpiSpec>;
#[doc = "USCI A1 Modulation Control"]
pub mod uca1mctl_spi;
#[doc = "UCA1STAT_SPI (rw) register accessor: USCI A1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1stat_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1stat_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1stat_spi`] module"]
#[doc(alias = "UCA1STAT_SPI")]
pub type Uca1statSpi = crate::Reg<uca1stat_spi::Uca1statSpiSpec>;
#[doc = "USCI A1 Status Register"]
pub mod uca1stat_spi;
#[doc = "UCA1RXBUF_SPI (rw) register accessor: USCI A1 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1rxbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1rxbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1rxbuf_spi`] module"]
#[doc(alias = "UCA1RXBUF_SPI")]
pub type Uca1rxbufSpi = crate::Reg<uca1rxbuf_spi::Uca1rxbufSpiSpec>;
#[doc = "USCI A1 Receive Buffer"]
pub mod uca1rxbuf_spi;
#[doc = "UCA1TXBUF_SPI (rw) register accessor: USCI A1 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1txbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1txbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1txbuf_spi`] module"]
#[doc(alias = "UCA1TXBUF_SPI")]
pub type Uca1txbufSpi = crate::Reg<uca1txbuf_spi::Uca1txbufSpiSpec>;
#[doc = "USCI A1 Transmit Buffer"]
pub mod uca1txbuf_spi;
