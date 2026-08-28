#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    u0ctl: U0ctl,
    u0tctl: U0tctl,
    u0rctl: U0rctl,
    u0mctl: U0mctl,
    u0br0: U0br0,
    u0br1: U0br1,
    u0rxbuf: U0rxbuf,
    u0txbuf: U0txbuf,
}
impl RegisterBlock {
    #[doc = "0x00 - USART 0 Control"]
    #[inline(always)]
    pub const fn u0ctl(&self) -> &U0ctl {
        &self.u0ctl
    }
    #[doc = "0x01 - USART 0 Transmit Control"]
    #[inline(always)]
    pub const fn u0tctl(&self) -> &U0tctl {
        &self.u0tctl
    }
    #[doc = "0x02 - USART 0 Receive Control"]
    #[inline(always)]
    pub const fn u0rctl(&self) -> &U0rctl {
        &self.u0rctl
    }
    #[doc = "0x03 - USART 0 Modulation Control"]
    #[inline(always)]
    pub const fn u0mctl(&self) -> &U0mctl {
        &self.u0mctl
    }
    #[doc = "0x04 - USART 0 Baud Rate 0"]
    #[inline(always)]
    pub const fn u0br0(&self) -> &U0br0 {
        &self.u0br0
    }
    #[doc = "0x05 - USART 0 Baud Rate 1"]
    #[inline(always)]
    pub const fn u0br1(&self) -> &U0br1 {
        &self.u0br1
    }
    #[doc = "0x06 - USART 0 Receive Buffer"]
    #[inline(always)]
    pub const fn u0rxbuf(&self) -> &U0rxbuf {
        &self.u0rxbuf
    }
    #[doc = "0x07 - USART 0 Transmit Buffer"]
    #[inline(always)]
    pub const fn u0txbuf(&self) -> &U0txbuf {
        &self.u0txbuf
    }
}
#[doc = "U0CTL (rw) register accessor: USART 0 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`u0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u0ctl`] module"]
#[doc(alias = "U0CTL")]
pub type U0ctl = crate::Reg<u0ctl::U0ctlSpec>;
#[doc = "USART 0 Control"]
pub mod u0ctl;
#[doc = "U0TCTL (rw) register accessor: USART 0 Transmit Control\n\nYou can [`read`](crate::Reg::read) this register and get [`u0tctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u0tctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u0tctl`] module"]
#[doc(alias = "U0TCTL")]
pub type U0tctl = crate::Reg<u0tctl::U0tctlSpec>;
#[doc = "USART 0 Transmit Control"]
pub mod u0tctl;
#[doc = "U0RCTL (rw) register accessor: USART 0 Receive Control\n\nYou can [`read`](crate::Reg::read) this register and get [`u0rctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u0rctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u0rctl`] module"]
#[doc(alias = "U0RCTL")]
pub type U0rctl = crate::Reg<u0rctl::U0rctlSpec>;
#[doc = "USART 0 Receive Control"]
pub mod u0rctl;
#[doc = "U0MCTL (rw) register accessor: USART 0 Modulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`u0mctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u0mctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u0mctl`] module"]
#[doc(alias = "U0MCTL")]
pub type U0mctl = crate::Reg<u0mctl::U0mctlSpec>;
#[doc = "USART 0 Modulation Control"]
pub mod u0mctl;
#[doc = "U0BR0 (rw) register accessor: USART 0 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`u0br0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u0br0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u0br0`] module"]
#[doc(alias = "U0BR0")]
pub type U0br0 = crate::Reg<u0br0::U0br0Spec>;
#[doc = "USART 0 Baud Rate 0"]
pub mod u0br0;
#[doc = "U0BR1 (rw) register accessor: USART 0 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`u0br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u0br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u0br1`] module"]
#[doc(alias = "U0BR1")]
pub type U0br1 = crate::Reg<u0br1::U0br1Spec>;
#[doc = "USART 0 Baud Rate 1"]
pub mod u0br1;
#[doc = "U0RXBUF (rw) register accessor: USART 0 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`u0rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u0rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u0rxbuf`] module"]
#[doc(alias = "U0RXBUF")]
pub type U0rxbuf = crate::Reg<u0rxbuf::U0rxbufSpec>;
#[doc = "USART 0 Receive Buffer"]
pub mod u0rxbuf;
#[doc = "U0TXBUF (rw) register accessor: USART 0 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`u0txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u0txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u0txbuf`] module"]
#[doc(alias = "U0TXBUF")]
pub type U0txbuf = crate::Reg<u0txbuf::U0txbufSpec>;
#[doc = "USART 0 Transmit Buffer"]
pub mod u0txbuf;
