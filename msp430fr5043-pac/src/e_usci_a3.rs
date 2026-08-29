#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_uca3ctlw: [u8; 0x02],
    uca3ctlw1: Uca3ctlw1,
    _reserved2: [u8; 0x02],
    _reserved_2_uca3: [u8; 0x02],
    uca3mctlw: Uca3mctlw,
    _reserved_4_uca3: [u8; 0x02],
    _reserved_5_uca3: [u8; 0x02],
    _reserved_6_uca3: [u8; 0x02],
    uca3abctl: Uca3abctl,
    uca3irctl: Uca3irctl,
    _reserved9: [u8; 0x06],
    _reserved_9_uca3: [u8; 0x02],
    _reserved_10_uca3: [u8; 0x02],
    _reserved_11_uca3: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub const fn uca3ctlw0_spi(&self) -> &Uca3ctlw0Spi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub const fn uca3ctlw0(&self) -> &Uca3ctlw0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x02 - eUSCI_Ax Control Word Register 1"]
    #[inline(always)]
    pub const fn uca3ctlw1(&self) -> &Uca3ctlw1 {
        &self.uca3ctlw1
    }
    #[doc = "0x06 - eUSCI_Ax Bit Rate Control Register 1"]
    #[inline(always)]
    pub const fn uca3brw_spi(&self) -> &Uca3brwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    #[inline(always)]
    pub const fn uca3brw(&self) -> &Uca3brw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x08 - eUSCI_Ax Modulation Control Word Register"]
    #[inline(always)]
    pub const fn uca3mctlw(&self) -> &Uca3mctlw {
        &self.uca3mctlw
    }
    #[doc = "0x0a - UCA3STATW_SPI"]
    #[inline(always)]
    pub const fn uca3statw_spi(&self) -> &Uca3statwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(10).cast() }
    }
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    #[inline(always)]
    pub const fn uca3statw(&self) -> &Uca3statw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(10).cast() }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub const fn uca3rxbuf_spi(&self) -> &Uca3rxbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub const fn uca3rxbuf(&self) -> &Uca3rxbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uca3txbuf_spi(&self) -> &Uca3txbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uca3txbuf(&self) -> &Uca3txbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x10 - eUSCI_Ax Auto Baud Rate Control Register"]
    #[inline(always)]
    pub const fn uca3abctl(&self) -> &Uca3abctl {
        &self.uca3abctl
    }
    #[doc = "0x12 - eUSCI_Ax IrDA Control Word Register"]
    #[inline(always)]
    pub const fn uca3irctl(&self) -> &Uca3irctl {
        &self.uca3irctl
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub const fn uca3ie_spi(&self) -> &Uca3ieSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub const fn uca3ie(&self) -> &Uca3ie {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub const fn uca3ifg_spi(&self) -> &Uca3ifgSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub const fn uca3ifg(&self) -> &Uca3ifg {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub const fn uca3iv_spi(&self) -> &Uca3ivSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(30).cast() }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub const fn uca3iv(&self) -> &Uca3iv {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(30).cast() }
    }
}
#[doc = "UCA3CTLW0 (rw) register accessor: eUSCI_Ax Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3ctlw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3ctlw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3ctlw0`] module"]
#[doc(alias = "UCA3CTLW0")]
pub type Uca3ctlw0 = crate::Reg<uca3ctlw0::Uca3ctlw0Spec>;
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca3ctlw0;
#[doc = "UCA3CTLW0_SPI (rw) register accessor: eUSCI_Ax Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3ctlw0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3ctlw0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3ctlw0_spi`] module"]
#[doc(alias = "UCA3CTLW0_SPI")]
pub type Uca3ctlw0Spi = crate::Reg<uca3ctlw0_spi::Uca3ctlw0SpiSpec>;
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca3ctlw0_spi;
#[doc = "UCA3CTLW1 (rw) register accessor: eUSCI_Ax Control Word Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3ctlw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3ctlw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3ctlw1`] module"]
#[doc(alias = "UCA3CTLW1")]
pub type Uca3ctlw1 = crate::Reg<uca3ctlw1::Uca3ctlw1Spec>;
#[doc = "eUSCI_Ax Control Word Register 1"]
pub mod uca3ctlw1;
#[doc = "UCA3BRW (rw) register accessor: eUSCI_Ax Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3brw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3brw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3brw`] module"]
#[doc(alias = "UCA3BRW")]
pub type Uca3brw = crate::Reg<uca3brw::Uca3brwSpec>;
#[doc = "eUSCI_Ax Baud Rate Control Word Register"]
pub mod uca3brw;
#[doc = "UCA3BRW_SPI (rw) register accessor: eUSCI_Ax Bit Rate Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3brw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3brw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3brw_spi`] module"]
#[doc(alias = "UCA3BRW_SPI")]
pub type Uca3brwSpi = crate::Reg<uca3brw_spi::Uca3brwSpiSpec>;
#[doc = "eUSCI_Ax Bit Rate Control Register 1"]
pub mod uca3brw_spi;
#[doc = "UCA3MCTLW (rw) register accessor: eUSCI_Ax Modulation Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3mctlw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3mctlw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3mctlw`] module"]
#[doc(alias = "UCA3MCTLW")]
pub type Uca3mctlw = crate::Reg<uca3mctlw::Uca3mctlwSpec>;
#[doc = "eUSCI_Ax Modulation Control Word Register"]
pub mod uca3mctlw;
#[doc = "UCA3STATW (rw) register accessor: eUSCI_Ax Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3statw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3statw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3statw`] module"]
#[doc(alias = "UCA3STATW")]
pub type Uca3statw = crate::Reg<uca3statw::Uca3statwSpec>;
#[doc = "eUSCI_Ax Status Register"]
pub mod uca3statw;
#[doc = "UCA3STATW_SPI (rw) register accessor: UCA3STATW_SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3statw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3statw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3statw_spi`] module"]
#[doc(alias = "UCA3STATW_SPI")]
pub type Uca3statwSpi = crate::Reg<uca3statw_spi::Uca3statwSpiSpec>;
#[doc = "UCA3STATW_SPI"]
pub mod uca3statw_spi;
#[doc = "UCA3RXBUF (rw) register accessor: eUSCI_Ax Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3rxbuf`] module"]
#[doc(alias = "UCA3RXBUF")]
pub type Uca3rxbuf = crate::Reg<uca3rxbuf::Uca3rxbufSpec>;
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca3rxbuf;
#[doc = "UCA3RXBUF_SPI (rw) register accessor: eUSCI_Ax Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3rxbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3rxbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3rxbuf_spi`] module"]
#[doc(alias = "UCA3RXBUF_SPI")]
pub type Uca3rxbufSpi = crate::Reg<uca3rxbuf_spi::Uca3rxbufSpiSpec>;
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca3rxbuf_spi;
#[doc = "UCA3TXBUF (rw) register accessor: eUSCI_Ax Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3txbuf`] module"]
#[doc(alias = "UCA3TXBUF")]
pub type Uca3txbuf = crate::Reg<uca3txbuf::Uca3txbufSpec>;
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca3txbuf;
#[doc = "UCA3TXBUF_SPI (rw) register accessor: eUSCI_Ax Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3txbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3txbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3txbuf_spi`] module"]
#[doc(alias = "UCA3TXBUF_SPI")]
pub type Uca3txbufSpi = crate::Reg<uca3txbuf_spi::Uca3txbufSpiSpec>;
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca3txbuf_spi;
#[doc = "UCA3ABCTL (rw) register accessor: eUSCI_Ax Auto Baud Rate Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3abctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3abctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3abctl`] module"]
#[doc(alias = "UCA3ABCTL")]
pub type Uca3abctl = crate::Reg<uca3abctl::Uca3abctlSpec>;
#[doc = "eUSCI_Ax Auto Baud Rate Control Register"]
pub mod uca3abctl;
#[doc = "UCA3IRCTL (rw) register accessor: eUSCI_Ax IrDA Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3irctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3irctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3irctl`] module"]
#[doc(alias = "UCA3IRCTL")]
pub type Uca3irctl = crate::Reg<uca3irctl::Uca3irctlSpec>;
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod uca3irctl;
#[doc = "UCA3IE (rw) register accessor: eUSCI_Ax Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3ie`] module"]
#[doc(alias = "UCA3IE")]
pub type Uca3ie = crate::Reg<uca3ie::Uca3ieSpec>;
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca3ie;
#[doc = "UCA3IE_SPI (rw) register accessor: eUSCI_Ax Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3ie_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3ie_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3ie_spi`] module"]
#[doc(alias = "UCA3IE_SPI")]
pub type Uca3ieSpi = crate::Reg<uca3ie_spi::Uca3ieSpiSpec>;
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca3ie_spi;
#[doc = "UCA3IFG (rw) register accessor: eUSCI_Ax Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3ifg`] module"]
#[doc(alias = "UCA3IFG")]
pub type Uca3ifg = crate::Reg<uca3ifg::Uca3ifgSpec>;
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca3ifg;
#[doc = "UCA3IFG_SPI (rw) register accessor: eUSCI_Ax Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3ifg_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3ifg_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3ifg_spi`] module"]
#[doc(alias = "UCA3IFG_SPI")]
pub type Uca3ifgSpi = crate::Reg<uca3ifg_spi::Uca3ifgSpiSpec>;
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca3ifg_spi;
#[doc = "UCA3IV (rw) register accessor: eUSCI_Ax Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3iv`] module"]
#[doc(alias = "UCA3IV")]
pub type Uca3iv = crate::Reg<uca3iv::Uca3ivSpec>;
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca3iv;
#[doc = "UCA3IV_SPI (rw) register accessor: eUSCI_Ax Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3iv_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3iv_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca3iv_spi`] module"]
#[doc(alias = "UCA3IV_SPI")]
pub type Uca3ivSpi = crate::Reg<uca3iv_spi::Uca3ivSpiSpec>;
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca3iv_spi;
