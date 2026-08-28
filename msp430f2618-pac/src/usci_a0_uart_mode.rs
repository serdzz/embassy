#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    uca0abctl: Uca0abctl,
    uca0irtctl: Uca0irtctl,
    uca0irrctl: Uca0irrctl,
    uca0ctl0: Uca0ctl0,
    uca0ctl1: Uca0ctl1,
    uca0br0: Uca0br0,
    uca0br1: Uca0br1,
    uca0mctl: Uca0mctl,
    uca0stat: Uca0stat,
    uca0rxbuf: Uca0rxbuf,
    uca0txbuf: Uca0txbuf,
}
impl RegisterBlock {
    #[doc = "0x01 - USCI A0 LIN Control"]
    #[inline(always)]
    pub const fn uca0abctl(&self) -> &Uca0abctl {
        &self.uca0abctl
    }
    #[doc = "0x02 - USCI A0 IrDA Transmit Control"]
    #[inline(always)]
    pub const fn uca0irtctl(&self) -> &Uca0irtctl {
        &self.uca0irtctl
    }
    #[doc = "0x03 - USCI A0 IrDA Receive Control"]
    #[inline(always)]
    pub const fn uca0irrctl(&self) -> &Uca0irrctl {
        &self.uca0irrctl
    }
    #[doc = "0x04 - USCI A0 Control Register 0"]
    #[inline(always)]
    pub const fn uca0ctl0(&self) -> &Uca0ctl0 {
        &self.uca0ctl0
    }
    #[doc = "0x05 - USCI A0 Control Register 1"]
    #[inline(always)]
    pub const fn uca0ctl1(&self) -> &Uca0ctl1 {
        &self.uca0ctl1
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
    pub const fn uca0mctl(&self) -> &Uca0mctl {
        &self.uca0mctl
    }
    #[doc = "0x09 - USCI A0 Status Register"]
    #[inline(always)]
    pub const fn uca0stat(&self) -> &Uca0stat {
        &self.uca0stat
    }
    #[doc = "0x0a - USCI A0 Receive Buffer"]
    #[inline(always)]
    pub const fn uca0rxbuf(&self) -> &Uca0rxbuf {
        &self.uca0rxbuf
    }
    #[doc = "0x0b - USCI A0 Transmit Buffer"]
    #[inline(always)]
    pub const fn uca0txbuf(&self) -> &Uca0txbuf {
        &self.uca0txbuf
    }
}
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
#[doc = "UCA0CTL0 (rw) register accessor: USCI A0 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctl0`] module"]
#[doc(alias = "UCA0CTL0")]
pub type Uca0ctl0 = crate::Reg<uca0ctl0::Uca0ctl0Spec>;
#[doc = "USCI A0 Control Register 0"]
pub mod uca0ctl0;
#[doc = "UCA0CTL1 (rw) register accessor: USCI A0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctl1`] module"]
#[doc(alias = "UCA0CTL1")]
pub type Uca0ctl1 = crate::Reg<uca0ctl1::Uca0ctl1Spec>;
#[doc = "USCI A0 Control Register 1"]
pub mod uca0ctl1;
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
#[doc = "UCA0MCTL (rw) register accessor: USCI A0 Modulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0mctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0mctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0mctl`] module"]
#[doc(alias = "UCA0MCTL")]
pub type Uca0mctl = crate::Reg<uca0mctl::Uca0mctlSpec>;
#[doc = "USCI A0 Modulation Control"]
pub mod uca0mctl;
#[doc = "UCA0STAT (rw) register accessor: USCI A0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0stat`] module"]
#[doc(alias = "UCA0STAT")]
pub type Uca0stat = crate::Reg<uca0stat::Uca0statSpec>;
#[doc = "USCI A0 Status Register"]
pub mod uca0stat;
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
