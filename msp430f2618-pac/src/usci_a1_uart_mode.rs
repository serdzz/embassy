#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    uca1abctl: Uca1abctl,
    uca1irtctl: Uca1irtctl,
    uca1irrctl: Uca1irrctl,
    uca1ctl0: Uca1ctl0,
    uca1ctl1: Uca1ctl1,
    uca1br0: Uca1br0,
    uca1br1: Uca1br1,
    uca1mctl: Uca1mctl,
    uca1stat: Uca1stat,
    uca1rxbuf: Uca1rxbuf,
    uca1txbuf: Uca1txbuf,
}
impl RegisterBlock {
    #[doc = "0x01 - USCI A1 LIN Control"]
    #[inline(always)]
    pub const fn uca1abctl(&self) -> &Uca1abctl {
        &self.uca1abctl
    }
    #[doc = "0x02 - USCI A1 IrDA Transmit Control"]
    #[inline(always)]
    pub const fn uca1irtctl(&self) -> &Uca1irtctl {
        &self.uca1irtctl
    }
    #[doc = "0x03 - USCI A1 IrDA Receive Control"]
    #[inline(always)]
    pub const fn uca1irrctl(&self) -> &Uca1irrctl {
        &self.uca1irrctl
    }
    #[doc = "0x04 - USCI A1 Control Register 0"]
    #[inline(always)]
    pub const fn uca1ctl0(&self) -> &Uca1ctl0 {
        &self.uca1ctl0
    }
    #[doc = "0x05 - USCI A1 Control Register 1"]
    #[inline(always)]
    pub const fn uca1ctl1(&self) -> &Uca1ctl1 {
        &self.uca1ctl1
    }
    #[doc = "0x06 - USCI A1 Baud Rate 0"]
    #[inline(always)]
    pub const fn uca1br0(&self) -> &Uca1br0 {
        &self.uca1br0
    }
    #[doc = "0x07 - USCI A1 Baud Rate 1"]
    #[inline(always)]
    pub const fn uca1br1(&self) -> &Uca1br1 {
        &self.uca1br1
    }
    #[doc = "0x08 - USCI A1 Modulation Control"]
    #[inline(always)]
    pub const fn uca1mctl(&self) -> &Uca1mctl {
        &self.uca1mctl
    }
    #[doc = "0x09 - USCI A1 Status Register"]
    #[inline(always)]
    pub const fn uca1stat(&self) -> &Uca1stat {
        &self.uca1stat
    }
    #[doc = "0x0a - USCI A1 Receive Buffer"]
    #[inline(always)]
    pub const fn uca1rxbuf(&self) -> &Uca1rxbuf {
        &self.uca1rxbuf
    }
    #[doc = "0x0b - USCI A1 Transmit Buffer"]
    #[inline(always)]
    pub const fn uca1txbuf(&self) -> &Uca1txbuf {
        &self.uca1txbuf
    }
}
#[doc = "UCA1ABCTL (rw) register accessor: USCI A1 LIN Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1abctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1abctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1abctl`] module"]
#[doc(alias = "UCA1ABCTL")]
pub type Uca1abctl = crate::Reg<uca1abctl::Uca1abctlSpec>;
#[doc = "USCI A1 LIN Control"]
pub mod uca1abctl;
#[doc = "UCA1IRTCTL (rw) register accessor: USCI A1 IrDA Transmit Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1irtctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1irtctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1irtctl`] module"]
#[doc(alias = "UCA1IRTCTL")]
pub type Uca1irtctl = crate::Reg<uca1irtctl::Uca1irtctlSpec>;
#[doc = "USCI A1 IrDA Transmit Control"]
pub mod uca1irtctl;
#[doc = "UCA1IRRCTL (rw) register accessor: USCI A1 IrDA Receive Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1irrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1irrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1irrctl`] module"]
#[doc(alias = "UCA1IRRCTL")]
pub type Uca1irrctl = crate::Reg<uca1irrctl::Uca1irrctlSpec>;
#[doc = "USCI A1 IrDA Receive Control"]
pub mod uca1irrctl;
#[doc = "UCA1CTL0 (rw) register accessor: USCI A1 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ctl0`] module"]
#[doc(alias = "UCA1CTL0")]
pub type Uca1ctl0 = crate::Reg<uca1ctl0::Uca1ctl0Spec>;
#[doc = "USCI A1 Control Register 0"]
pub mod uca1ctl0;
#[doc = "UCA1CTL1 (rw) register accessor: USCI A1 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ctl1`] module"]
#[doc(alias = "UCA1CTL1")]
pub type Uca1ctl1 = crate::Reg<uca1ctl1::Uca1ctl1Spec>;
#[doc = "USCI A1 Control Register 1"]
pub mod uca1ctl1;
#[doc = "UCA1BR0 (rw) register accessor: USCI A1 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1br0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1br0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1br0`] module"]
#[doc(alias = "UCA1BR0")]
pub type Uca1br0 = crate::Reg<uca1br0::Uca1br0Spec>;
#[doc = "USCI A1 Baud Rate 0"]
pub mod uca1br0;
#[doc = "UCA1BR1 (rw) register accessor: USCI A1 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1br1`] module"]
#[doc(alias = "UCA1BR1")]
pub type Uca1br1 = crate::Reg<uca1br1::Uca1br1Spec>;
#[doc = "USCI A1 Baud Rate 1"]
pub mod uca1br1;
#[doc = "UCA1MCTL (rw) register accessor: USCI A1 Modulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1mctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1mctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1mctl`] module"]
#[doc(alias = "UCA1MCTL")]
pub type Uca1mctl = crate::Reg<uca1mctl::Uca1mctlSpec>;
#[doc = "USCI A1 Modulation Control"]
pub mod uca1mctl;
#[doc = "UCA1STAT (rw) register accessor: USCI A1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1stat`] module"]
#[doc(alias = "UCA1STAT")]
pub type Uca1stat = crate::Reg<uca1stat::Uca1statSpec>;
#[doc = "USCI A1 Status Register"]
pub mod uca1stat;
#[doc = "UCA1RXBUF (rw) register accessor: USCI A1 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1rxbuf`] module"]
#[doc(alias = "UCA1RXBUF")]
pub type Uca1rxbuf = crate::Reg<uca1rxbuf::Uca1rxbufSpec>;
#[doc = "USCI A1 Receive Buffer"]
pub mod uca1rxbuf;
#[doc = "UCA1TXBUF (rw) register accessor: USCI A1 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1txbuf`] module"]
#[doc(alias = "UCA1TXBUF")]
pub type Uca1txbuf = crate::Reg<uca1txbuf::Uca1txbufSpec>;
#[doc = "USCI A1 Transmit Buffer"]
pub mod uca1txbuf;
