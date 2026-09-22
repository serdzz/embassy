#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uca0ctl1: Uca0ctl1,
    uca0ctl0: Uca0ctl0,
    uca0ctlw1: Uca0ctlw1,
    _reserved3: [u8; 0x02],
    uca0br0: Uca0br0,
    uca0br1: Uca0br1,
    uca0mctlw: Uca0mctlw,
    uca0statw: Uca0statw,
    _reserved7: [u8; 0x01],
    uca0rxbuf: Uca0rxbuf,
    uca0txbuf: Uca0txbuf,
    uca0abctl: Uca0abctl,
    _reserved10: [u8; 0x01],
    uca0irtctl: Uca0irtctl,
    uca0irrctl: Uca0irrctl,
    _reserved12: [u8; 0x0a],
    uca0iv: Uca0iv,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI A0 Control Register 1"]
    #[inline(always)]
    pub const fn uca0ctl1(&self) -> &Uca0ctl1 {
        &self.uca0ctl1
    }
    #[doc = "0x01 - USCI A0 Control Register 0"]
    #[inline(always)]
    pub const fn uca0ctl0(&self) -> &Uca0ctl0 {
        &self.uca0ctl0
    }
    #[doc = "0x02 - USCI A0 Control Word Register 1"]
    #[inline(always)]
    pub const fn uca0ctlw1(&self) -> &Uca0ctlw1 {
        &self.uca0ctlw1
    }
    #[doc = "0x06 - USCI A0 Baud Rate 0"]
    #[inline(always)]
    pub const fn uca0br0(&self) -> &Uca0br0 {
        &self.uca0br0
    }
    #[doc = "0x07 - USCI A0 Baud Rate 1"]
    #[inline(always)]
    pub const fn uca0br1(&self) -> &Uca0br1 {
        &self.uca0br1
    }
    #[doc = "0x08 - USCI A0 Modulation Control"]
    #[inline(always)]
    pub const fn uca0mctlw(&self) -> &Uca0mctlw {
        &self.uca0mctlw
    }
    #[doc = "0x0a - USCI A0 Status Register"]
    #[inline(always)]
    pub const fn uca0statw(&self) -> &Uca0statw {
        &self.uca0statw
    }
    #[doc = "0x0c - USCI A0 Receive Buffer"]
    #[inline(always)]
    pub const fn uca0rxbuf(&self) -> &Uca0rxbuf {
        &self.uca0rxbuf
    }
    #[doc = "0x0e - USCI A0 Transmit Buffer"]
    #[inline(always)]
    pub const fn uca0txbuf(&self) -> &Uca0txbuf {
        &self.uca0txbuf
    }
    #[doc = "0x10 - USCI A0 LIN Control"]
    #[inline(always)]
    pub const fn uca0abctl(&self) -> &Uca0abctl {
        &self.uca0abctl
    }
    #[doc = "0x12 - USCI A0 IrDA Transmit Control"]
    #[inline(always)]
    pub const fn uca0irtctl(&self) -> &Uca0irtctl {
        &self.uca0irtctl
    }
    #[doc = "0x13 - USCI A0 IrDA Receive Control"]
    #[inline(always)]
    pub const fn uca0irrctl(&self) -> &Uca0irrctl {
        &self.uca0irrctl
    }
    #[doc = "0x1e - USCI A0 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn uca0iv(&self) -> &Uca0iv {
        &self.uca0iv
    }
}
#[doc = "UCA0CTL1 (rw) register accessor: USCI A0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctl1`] module"]
#[doc(alias = "UCA0CTL1")]
pub type Uca0ctl1 = crate::Reg<uca0ctl1::Uca0ctl1Spec>;
#[doc = "USCI A0 Control Register 1"]
pub mod uca0ctl1;
#[doc = "UCA0CTL0 (rw) register accessor: USCI A0 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctl0`] module"]
#[doc(alias = "UCA0CTL0")]
pub type Uca0ctl0 = crate::Reg<uca0ctl0::Uca0ctl0Spec>;
#[doc = "USCI A0 Control Register 0"]
pub mod uca0ctl0;
#[doc = "UCA0BR0 (rw) register accessor: USCI A0 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0br0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0br0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0br0`] module"]
#[doc(alias = "UCA0BR0")]
pub type Uca0br0 = crate::Reg<uca0br0::Uca0br0Spec>;
#[doc = "USCI A0 Baud Rate 0"]
pub mod uca0br0;
#[doc = "UCA0BR1 (rw) register accessor: USCI A0 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0br1`] module"]
#[doc(alias = "UCA0BR1")]
pub type Uca0br1 = crate::Reg<uca0br1::Uca0br1Spec>;
#[doc = "USCI A0 Baud Rate 1"]
pub mod uca0br1;
#[doc = "UCA0STATW (rw) register accessor: USCI A0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0statw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0statw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0statw`] module"]
#[doc(alias = "UCA0STATW")]
pub type Uca0statw = crate::Reg<uca0statw::Uca0statwSpec>;
#[doc = "USCI A0 Status Register"]
pub mod uca0statw;
#[doc = "UCA0ABCTL (rw) register accessor: USCI A0 LIN Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0abctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0abctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0abctl`] module"]
#[doc(alias = "UCA0ABCTL")]
pub type Uca0abctl = crate::Reg<uca0abctl::Uca0abctlSpec>;
#[doc = "USCI A0 LIN Control"]
pub mod uca0abctl;
#[doc = "UCA0IRTCTL (rw) register accessor: USCI A0 IrDA Transmit Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0irtctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0irtctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0irtctl`] module"]
#[doc(alias = "UCA0IRTCTL")]
pub type Uca0irtctl = crate::Reg<uca0irtctl::Uca0irtctlSpec>;
#[doc = "USCI A0 IrDA Transmit Control"]
pub mod uca0irtctl;
#[doc = "UCA0IRRCTL (rw) register accessor: USCI A0 IrDA Receive Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0irrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0irrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0irrctl`] module"]
#[doc(alias = "UCA0IRRCTL")]
pub type Uca0irrctl = crate::Reg<uca0irrctl::Uca0irrctlSpec>;
#[doc = "USCI A0 IrDA Receive Control"]
pub mod uca0irrctl;
#[doc = "UCA0CTLW1 (rw) register accessor: USCI A0 Control Word Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctlw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctlw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctlw1`] module"]
#[doc(alias = "UCA0CTLW1")]
pub type Uca0ctlw1 = crate::Reg<uca0ctlw1::Uca0ctlw1Spec>;
#[doc = "USCI A0 Control Word Register 1"]
pub mod uca0ctlw1;
#[doc = "UCA0MCTLW (rw) register accessor: USCI A0 Modulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0mctlw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0mctlw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0mctlw`] module"]
#[doc(alias = "UCA0MCTLW")]
pub type Uca0mctlw = crate::Reg<uca0mctlw::Uca0mctlwSpec>;
#[doc = "USCI A0 Modulation Control"]
pub mod uca0mctlw;
#[doc = "UCA0RXBUF (rw) register accessor: USCI A0 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0rxbuf`] module"]
#[doc(alias = "UCA0RXBUF")]
pub type Uca0rxbuf = crate::Reg<uca0rxbuf::Uca0rxbufSpec>;
#[doc = "USCI A0 Receive Buffer"]
pub mod uca0rxbuf;
#[doc = "UCA0TXBUF (rw) register accessor: USCI A0 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0txbuf`] module"]
#[doc(alias = "UCA0TXBUF")]
pub type Uca0txbuf = crate::Reg<uca0txbuf::Uca0txbufSpec>;
#[doc = "USCI A0 Transmit Buffer"]
pub mod uca0txbuf;
#[doc = "UCA0IV (rw) register accessor: USCI A0 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0iv`] module"]
#[doc(alias = "UCA0IV")]
pub type Uca0iv = crate::Reg<uca0iv::Uca0ivSpec>;
#[doc = "USCI A0 Interrupt Vector Register"]
pub mod uca0iv;
