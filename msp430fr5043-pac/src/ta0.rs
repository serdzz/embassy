#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ta0ctl: Ta0ctl,
    ta0cctl0: Ta0cctl0,
    ta0cctl1: Ta0cctl1,
    ta0cctl2: Ta0cctl2,
    _reserved4: [u8; 0x08],
    ta0r: Ta0r,
    ta0ccr0: Ta0ccr0,
    ta0ccr1: Ta0ccr1,
    ta0ccr2: Ta0ccr2,
    _reserved8: [u8; 0x08],
    ta0ex0: Ta0ex0,
    _reserved9: [u8; 0x0c],
    ta0iv: Ta0iv,
}
impl RegisterBlock {
    #[doc = "0x00 - TimerAx Control Register"]
    #[inline(always)]
    pub const fn ta0ctl(&self) -> &Ta0ctl {
        &self.ta0ctl
    }
    #[doc = "0x02 - Timer_A Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn ta0cctl0(&self) -> &Ta0cctl0 {
        &self.ta0cctl0
    }
    #[doc = "0x04 - Timer_A Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn ta0cctl1(&self) -> &Ta0cctl1 {
        &self.ta0cctl1
    }
    #[doc = "0x06 - Timer_A Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn ta0cctl2(&self) -> &Ta0cctl2 {
        &self.ta0cctl2
    }
    #[doc = "0x10 - TimerA register"]
    #[inline(always)]
    pub const fn ta0r(&self) -> &Ta0r {
        &self.ta0r
    }
    #[doc = "0x12 - Timer_A Capture/Compare Register"]
    #[inline(always)]
    pub const fn ta0ccr0(&self) -> &Ta0ccr0 {
        &self.ta0ccr0
    }
    #[doc = "0x14 - Timer_A Capture/Compare Register"]
    #[inline(always)]
    pub const fn ta0ccr1(&self) -> &Ta0ccr1 {
        &self.ta0ccr1
    }
    #[doc = "0x16 - Timer_A Capture/Compare Register"]
    #[inline(always)]
    pub const fn ta0ccr2(&self) -> &Ta0ccr2 {
        &self.ta0ccr2
    }
    #[doc = "0x20 - TimerAx Expansion 0 Register"]
    #[inline(always)]
    pub const fn ta0ex0(&self) -> &Ta0ex0 {
        &self.ta0ex0
    }
    #[doc = "0x2e - TimerAx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ta0iv(&self) -> &Ta0iv {
        &self.ta0iv
    }
}
#[doc = "TA0CTL (rw) register accessor: TimerAx Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0ctl`] module"]
#[doc(alias = "TA0CTL")]
pub type Ta0ctl = crate::Reg<ta0ctl::Ta0ctlSpec>;
#[doc = "TimerAx Control Register"]
pub mod ta0ctl;
#[doc = "TA0CCTL0 (rw) register accessor: Timer_A Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta0cctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta0cctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0cctl0`] module"]
#[doc(alias = "TA0CCTL0")]
pub type Ta0cctl0 = crate::Reg<ta0cctl0::Ta0cctl0Spec>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta0cctl0;
#[doc = "TA0CCTL1 (rw) register accessor: Timer_A Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta0cctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta0cctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0cctl1`] module"]
#[doc(alias = "TA0CCTL1")]
pub type Ta0cctl1 = crate::Reg<ta0cctl1::Ta0cctl1Spec>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta0cctl1;
#[doc = "TA0CCTL2 (rw) register accessor: Timer_A Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta0cctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta0cctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0cctl2`] module"]
#[doc(alias = "TA0CCTL2")]
pub type Ta0cctl2 = crate::Reg<ta0cctl2::Ta0cctl2Spec>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta0cctl2;
#[doc = "TA0R (rw) register accessor: TimerA register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0r`] module"]
#[doc(alias = "TA0R")]
pub type Ta0r = crate::Reg<ta0r::Ta0rSpec>;
#[doc = "TimerA register"]
pub mod ta0r;
#[doc = "TA0CCR0 (rw) register accessor: Timer_A Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta0ccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta0ccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0ccr0`] module"]
#[doc(alias = "TA0CCR0")]
pub type Ta0ccr0 = crate::Reg<ta0ccr0::Ta0ccr0Spec>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta0ccr0;
#[doc = "TA0CCR1 (rw) register accessor: Timer_A Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta0ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta0ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0ccr1`] module"]
#[doc(alias = "TA0CCR1")]
pub type Ta0ccr1 = crate::Reg<ta0ccr1::Ta0ccr1Spec>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta0ccr1;
#[doc = "TA0CCR2 (rw) register accessor: Timer_A Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta0ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta0ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0ccr2`] module"]
#[doc(alias = "TA0CCR2")]
pub type Ta0ccr2 = crate::Reg<ta0ccr2::Ta0ccr2Spec>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta0ccr2;
#[doc = "TA0EX0 (rw) register accessor: TimerAx Expansion 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta0ex0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta0ex0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0ex0`] module"]
#[doc(alias = "TA0EX0")]
pub type Ta0ex0 = crate::Reg<ta0ex0::Ta0ex0Spec>;
#[doc = "TimerAx Expansion 0 Register"]
pub mod ta0ex0;
#[doc = "TA0IV (rw) register accessor: TimerAx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta0iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta0iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0iv`] module"]
#[doc(alias = "TA0IV")]
pub type Ta0iv = crate::Reg<ta0iv::Ta0ivSpec>;
#[doc = "TimerAx Interrupt Vector Register"]
pub mod ta0iv;
