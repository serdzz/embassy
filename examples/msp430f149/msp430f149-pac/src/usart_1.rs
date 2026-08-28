#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    u1ctl: U1ctl,
    u1tctl: U1tctl,
    u1rctl: U1rctl,
    u1mctl: U1mctl,
    u1br0: U1br0,
    u1br1: U1br1,
    u1rxbuf: U1rxbuf,
    u1txbuf: U1txbuf,
}
impl RegisterBlock {
    #[doc = "0x00 - USART 1 Control"]
    #[inline(always)]
    pub const fn u1ctl(&self) -> &U1ctl {
        &self.u1ctl
    }
    #[doc = "0x01 - USART 1 Transmit Control"]
    #[inline(always)]
    pub const fn u1tctl(&self) -> &U1tctl {
        &self.u1tctl
    }
    #[doc = "0x02 - USART 1 Receive Control"]
    #[inline(always)]
    pub const fn u1rctl(&self) -> &U1rctl {
        &self.u1rctl
    }
    #[doc = "0x03 - USART 1 Modulation Control"]
    #[inline(always)]
    pub const fn u1mctl(&self) -> &U1mctl {
        &self.u1mctl
    }
    #[doc = "0x04 - USART 1 Baud Rate 0"]
    #[inline(always)]
    pub const fn u1br0(&self) -> &U1br0 {
        &self.u1br0
    }
    #[doc = "0x05 - USART 1 Baud Rate 1"]
    #[inline(always)]
    pub const fn u1br1(&self) -> &U1br1 {
        &self.u1br1
    }
    #[doc = "0x06 - USART 1 Receive Buffer"]
    #[inline(always)]
    pub const fn u1rxbuf(&self) -> &U1rxbuf {
        &self.u1rxbuf
    }
    #[doc = "0x07 - USART 1 Transmit Buffer"]
    #[inline(always)]
    pub const fn u1txbuf(&self) -> &U1txbuf {
        &self.u1txbuf
    }
}
#[doc = "U1CTL (rw) register accessor: USART 1 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`u1ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u1ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u1ctl`] module"]
#[doc(alias = "U1CTL")]
pub type U1ctl = crate::Reg<u1ctl::U1ctlSpec>;
#[doc = "USART 1 Control"]
pub mod u1ctl;
#[doc = "U1TCTL (rw) register accessor: USART 1 Transmit Control\n\nYou can [`read`](crate::Reg::read) this register and get [`u1tctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u1tctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u1tctl`] module"]
#[doc(alias = "U1TCTL")]
pub type U1tctl = crate::Reg<u1tctl::U1tctlSpec>;
#[doc = "USART 1 Transmit Control"]
pub mod u1tctl;
#[doc = "U1RCTL (rw) register accessor: USART 1 Receive Control\n\nYou can [`read`](crate::Reg::read) this register and get [`u1rctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u1rctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u1rctl`] module"]
#[doc(alias = "U1RCTL")]
pub type U1rctl = crate::Reg<u1rctl::U1rctlSpec>;
#[doc = "USART 1 Receive Control"]
pub mod u1rctl;
#[doc = "U1MCTL (rw) register accessor: USART 1 Modulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`u1mctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u1mctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u1mctl`] module"]
#[doc(alias = "U1MCTL")]
pub type U1mctl = crate::Reg<u1mctl::U1mctlSpec>;
#[doc = "USART 1 Modulation Control"]
pub mod u1mctl;
#[doc = "U1BR0 (rw) register accessor: USART 1 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`u1br0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u1br0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u1br0`] module"]
#[doc(alias = "U1BR0")]
pub type U1br0 = crate::Reg<u1br0::U1br0Spec>;
#[doc = "USART 1 Baud Rate 0"]
pub mod u1br0;
#[doc = "U1BR1 (rw) register accessor: USART 1 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`u1br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u1br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u1br1`] module"]
#[doc(alias = "U1BR1")]
pub type U1br1 = crate::Reg<u1br1::U1br1Spec>;
#[doc = "USART 1 Baud Rate 1"]
pub mod u1br1;
#[doc = "U1RXBUF (rw) register accessor: USART 1 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`u1rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u1rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u1rxbuf`] module"]
#[doc(alias = "U1RXBUF")]
pub type U1rxbuf = crate::Reg<u1rxbuf::U1rxbufSpec>;
#[doc = "USART 1 Receive Buffer"]
pub mod u1rxbuf;
#[doc = "U1TXBUF (rw) register accessor: USART 1 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`u1txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u1txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u1txbuf`] module"]
#[doc(alias = "U1TXBUF")]
pub type U1txbuf = crate::Reg<u1txbuf::U1txbufSpec>;
#[doc = "USART 1 Transmit Buffer"]
pub mod u1txbuf;
