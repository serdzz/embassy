#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_uca2ctlw: [u8; 0x02],
    uca2ctlw1: Uca2ctlw1,
    _reserved2: [u8; 0x02],
    _reserved_2_uca2: [u8; 0x02],
    uca2mctlw: Uca2mctlw,
    _reserved_4_uca2: [u8; 0x02],
    _reserved_5_uca2: [u8; 0x02],
    _reserved_6_uca2: [u8; 0x02],
    uca2abctl: Uca2abctl,
    uca2irctl: Uca2irctl,
    _reserved9: [u8; 0x06],
    _reserved_9_uca2: [u8; 0x02],
    _reserved_10_uca2: [u8; 0x02],
    _reserved_11_uca2: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub const fn uca2ctlw0_spi(&self) -> &Uca2ctlw0Spi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub const fn uca2ctlw0(&self) -> &Uca2ctlw0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x02 - eUSCI_Ax Control Word Register 1"]
    #[inline(always)]
    pub const fn uca2ctlw1(&self) -> &Uca2ctlw1 {
        &self.uca2ctlw1
    }
    #[doc = "0x06 - eUSCI_Ax Bit Rate Control Register 1"]
    #[inline(always)]
    pub const fn uca2brw_spi(&self) -> &Uca2brwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    #[inline(always)]
    pub const fn uca2brw(&self) -> &Uca2brw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x08 - eUSCI_Ax Modulation Control Word Register"]
    #[inline(always)]
    pub const fn uca2mctlw(&self) -> &Uca2mctlw {
        &self.uca2mctlw
    }
    #[doc = "0x0a - UCA2STATW_SPI"]
    #[inline(always)]
    pub const fn uca2statw_spi(&self) -> &Uca2statwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(10).cast() }
    }
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    #[inline(always)]
    pub const fn uca2statw(&self) -> &Uca2statw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(10).cast() }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub const fn uca2rxbuf_spi(&self) -> &Uca2rxbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub const fn uca2rxbuf(&self) -> &Uca2rxbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uca2txbuf_spi(&self) -> &Uca2txbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uca2txbuf(&self) -> &Uca2txbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x10 - eUSCI_Ax Auto Baud Rate Control Register"]
    #[inline(always)]
    pub const fn uca2abctl(&self) -> &Uca2abctl {
        &self.uca2abctl
    }
    #[doc = "0x12 - eUSCI_Ax IrDA Control Word Register"]
    #[inline(always)]
    pub const fn uca2irctl(&self) -> &Uca2irctl {
        &self.uca2irctl
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub const fn uca2ie_spi(&self) -> &Uca2ieSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub const fn uca2ie(&self) -> &Uca2ie {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub const fn uca2ifg_spi(&self) -> &Uca2ifgSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub const fn uca2ifg(&self) -> &Uca2ifg {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub const fn uca2iv_spi(&self) -> &Uca2ivSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(30).cast() }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub const fn uca2iv(&self) -> &Uca2iv {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(30).cast() }
    }
}
#[doc = "UCA2CTLW0 (rw) register accessor: eUSCI_Ax Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2ctlw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2ctlw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2ctlw0`] module"]
#[doc(alias = "UCA2CTLW0")]
pub type Uca2ctlw0 = crate::Reg<uca2ctlw0::Uca2ctlw0Spec>;
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca2ctlw0;
#[doc = "UCA2CTLW0_SPI (rw) register accessor: eUSCI_Ax Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2ctlw0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2ctlw0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2ctlw0_spi`] module"]
#[doc(alias = "UCA2CTLW0_SPI")]
pub type Uca2ctlw0Spi = crate::Reg<uca2ctlw0_spi::Uca2ctlw0SpiSpec>;
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca2ctlw0_spi;
#[doc = "UCA2CTLW1 (rw) register accessor: eUSCI_Ax Control Word Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2ctlw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2ctlw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2ctlw1`] module"]
#[doc(alias = "UCA2CTLW1")]
pub type Uca2ctlw1 = crate::Reg<uca2ctlw1::Uca2ctlw1Spec>;
#[doc = "eUSCI_Ax Control Word Register 1"]
pub mod uca2ctlw1;
#[doc = "UCA2BRW (rw) register accessor: eUSCI_Ax Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2brw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2brw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2brw`] module"]
#[doc(alias = "UCA2BRW")]
pub type Uca2brw = crate::Reg<uca2brw::Uca2brwSpec>;
#[doc = "eUSCI_Ax Baud Rate Control Word Register"]
pub mod uca2brw;
#[doc = "UCA2BRW_SPI (rw) register accessor: eUSCI_Ax Bit Rate Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2brw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2brw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2brw_spi`] module"]
#[doc(alias = "UCA2BRW_SPI")]
pub type Uca2brwSpi = crate::Reg<uca2brw_spi::Uca2brwSpiSpec>;
#[doc = "eUSCI_Ax Bit Rate Control Register 1"]
pub mod uca2brw_spi;
#[doc = "UCA2MCTLW (rw) register accessor: eUSCI_Ax Modulation Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2mctlw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2mctlw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2mctlw`] module"]
#[doc(alias = "UCA2MCTLW")]
pub type Uca2mctlw = crate::Reg<uca2mctlw::Uca2mctlwSpec>;
#[doc = "eUSCI_Ax Modulation Control Word Register"]
pub mod uca2mctlw;
#[doc = "UCA2STATW (rw) register accessor: eUSCI_Ax Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2statw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2statw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2statw`] module"]
#[doc(alias = "UCA2STATW")]
pub type Uca2statw = crate::Reg<uca2statw::Uca2statwSpec>;
#[doc = "eUSCI_Ax Status Register"]
pub mod uca2statw;
#[doc = "UCA2STATW_SPI (rw) register accessor: UCA2STATW_SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2statw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2statw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2statw_spi`] module"]
#[doc(alias = "UCA2STATW_SPI")]
pub type Uca2statwSpi = crate::Reg<uca2statw_spi::Uca2statwSpiSpec>;
#[doc = "UCA2STATW_SPI"]
pub mod uca2statw_spi;
#[doc = "UCA2RXBUF (rw) register accessor: eUSCI_Ax Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2rxbuf`] module"]
#[doc(alias = "UCA2RXBUF")]
pub type Uca2rxbuf = crate::Reg<uca2rxbuf::Uca2rxbufSpec>;
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca2rxbuf;
#[doc = "UCA2RXBUF_SPI (rw) register accessor: eUSCI_Ax Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2rxbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2rxbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2rxbuf_spi`] module"]
#[doc(alias = "UCA2RXBUF_SPI")]
pub type Uca2rxbufSpi = crate::Reg<uca2rxbuf_spi::Uca2rxbufSpiSpec>;
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca2rxbuf_spi;
#[doc = "UCA2TXBUF (rw) register accessor: eUSCI_Ax Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2txbuf`] module"]
#[doc(alias = "UCA2TXBUF")]
pub type Uca2txbuf = crate::Reg<uca2txbuf::Uca2txbufSpec>;
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca2txbuf;
#[doc = "UCA2TXBUF_SPI (rw) register accessor: eUSCI_Ax Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2txbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2txbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2txbuf_spi`] module"]
#[doc(alias = "UCA2TXBUF_SPI")]
pub type Uca2txbufSpi = crate::Reg<uca2txbuf_spi::Uca2txbufSpiSpec>;
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca2txbuf_spi;
#[doc = "UCA2ABCTL (rw) register accessor: eUSCI_Ax Auto Baud Rate Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2abctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2abctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2abctl`] module"]
#[doc(alias = "UCA2ABCTL")]
pub type Uca2abctl = crate::Reg<uca2abctl::Uca2abctlSpec>;
#[doc = "eUSCI_Ax Auto Baud Rate Control Register"]
pub mod uca2abctl;
#[doc = "UCA2IRCTL (rw) register accessor: eUSCI_Ax IrDA Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2irctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2irctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2irctl`] module"]
#[doc(alias = "UCA2IRCTL")]
pub type Uca2irctl = crate::Reg<uca2irctl::Uca2irctlSpec>;
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod uca2irctl;
#[doc = "UCA2IE (rw) register accessor: eUSCI_Ax Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2ie`] module"]
#[doc(alias = "UCA2IE")]
pub type Uca2ie = crate::Reg<uca2ie::Uca2ieSpec>;
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca2ie;
#[doc = "UCA2IE_SPI (rw) register accessor: eUSCI_Ax Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2ie_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2ie_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2ie_spi`] module"]
#[doc(alias = "UCA2IE_SPI")]
pub type Uca2ieSpi = crate::Reg<uca2ie_spi::Uca2ieSpiSpec>;
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca2ie_spi;
#[doc = "UCA2IFG (rw) register accessor: eUSCI_Ax Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2ifg`] module"]
#[doc(alias = "UCA2IFG")]
pub type Uca2ifg = crate::Reg<uca2ifg::Uca2ifgSpec>;
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca2ifg;
#[doc = "UCA2IFG_SPI (rw) register accessor: eUSCI_Ax Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2ifg_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2ifg_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2ifg_spi`] module"]
#[doc(alias = "UCA2IFG_SPI")]
pub type Uca2ifgSpi = crate::Reg<uca2ifg_spi::Uca2ifgSpiSpec>;
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca2ifg_spi;
#[doc = "UCA2IV (rw) register accessor: eUSCI_Ax Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2iv`] module"]
#[doc(alias = "UCA2IV")]
pub type Uca2iv = crate::Reg<uca2iv::Uca2ivSpec>;
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca2iv;
#[doc = "UCA2IV_SPI (rw) register accessor: eUSCI_Ax Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2iv_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2iv_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca2iv_spi`] module"]
#[doc(alias = "UCA2IV_SPI")]
pub type Uca2ivSpi = crate::Reg<uca2iv_spi::Uca2ivSpiSpec>;
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca2iv_spi;
