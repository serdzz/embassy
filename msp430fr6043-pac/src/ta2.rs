#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ta2ctl: Ta2ctl,
    ta2cctl0: Ta2cctl0,
    ta2cctl1: Ta2cctl1,
    _reserved3: [u8; 0x0a],
    ta2r: Ta2r,
    ta2ccr0: Ta2ccr0,
    ta2ccr1: Ta2ccr1,
    _reserved6: [u8; 0x0a],
    ta2ex0: Ta2ex0,
    _reserved7: [u8; 0x0c],
    ta2iv: Ta2iv,
}
impl RegisterBlock {
    #[doc = "0x00 - TimerAx Control Register"]
    #[inline(always)]
    pub const fn ta2ctl(&self) -> &Ta2ctl {
        &self.ta2ctl
    }
    #[doc = "0x02 - Timer_A Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn ta2cctl0(&self) -> &Ta2cctl0 {
        &self.ta2cctl0
    }
    #[doc = "0x04 - Timer_A Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn ta2cctl1(&self) -> &Ta2cctl1 {
        &self.ta2cctl1
    }
    #[doc = "0x10 - TimerA register"]
    #[inline(always)]
    pub const fn ta2r(&self) -> &Ta2r {
        &self.ta2r
    }
    #[doc = "0x12 - Timer_A Capture/Compare Register"]
    #[inline(always)]
    pub const fn ta2ccr0(&self) -> &Ta2ccr0 {
        &self.ta2ccr0
    }
    #[doc = "0x14 - Timer_A Capture/Compare Register"]
    #[inline(always)]
    pub const fn ta2ccr1(&self) -> &Ta2ccr1 {
        &self.ta2ccr1
    }
    #[doc = "0x20 - TimerAx Expansion 0 Register"]
    #[inline(always)]
    pub const fn ta2ex0(&self) -> &Ta2ex0 {
        &self.ta2ex0
    }
    #[doc = "0x2e - TimerAx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ta2iv(&self) -> &Ta2iv {
        &self.ta2iv
    }
}
#[doc = "TA2CTL (rw) register accessor: TimerAx Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta2ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta2ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2ctl`] module"]
#[doc(alias = "TA2CTL")]
pub type Ta2ctl = crate::Reg<ta2ctl::Ta2ctlSpec>;
#[doc = "TimerAx Control Register"]
pub mod ta2ctl;
#[doc = "TA2CCTL0 (rw) register accessor: Timer_A Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta2cctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta2cctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2cctl0`] module"]
#[doc(alias = "TA2CCTL0")]
pub type Ta2cctl0 = crate::Reg<ta2cctl0::Ta2cctl0Spec>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta2cctl0;
#[doc = "TA2CCTL1 (rw) register accessor: Timer_A Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta2cctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta2cctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2cctl1`] module"]
#[doc(alias = "TA2CCTL1")]
pub type Ta2cctl1 = crate::Reg<ta2cctl1::Ta2cctl1Spec>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta2cctl1;
#[doc = "TA2R (rw) register accessor: TimerA register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2r`] module"]
#[doc(alias = "TA2R")]
pub type Ta2r = crate::Reg<ta2r::Ta2rSpec>;
#[doc = "TimerA register"]
pub mod ta2r;
#[doc = "TA2CCR0 (rw) register accessor: Timer_A Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta2ccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta2ccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2ccr0`] module"]
#[doc(alias = "TA2CCR0")]
pub type Ta2ccr0 = crate::Reg<ta2ccr0::Ta2ccr0Spec>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta2ccr0;
#[doc = "TA2CCR1 (rw) register accessor: Timer_A Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta2ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta2ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2ccr1`] module"]
#[doc(alias = "TA2CCR1")]
pub type Ta2ccr1 = crate::Reg<ta2ccr1::Ta2ccr1Spec>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta2ccr1;
#[doc = "TA2EX0 (rw) register accessor: TimerAx Expansion 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta2ex0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta2ex0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2ex0`] module"]
#[doc(alias = "TA2EX0")]
pub type Ta2ex0 = crate::Reg<ta2ex0::Ta2ex0Spec>;
#[doc = "TimerAx Expansion 0 Register"]
pub mod ta2ex0;
#[doc = "TA2IV (rw) register accessor: TimerAx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta2iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta2iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2iv`] module"]
#[doc(alias = "TA2IV")]
pub type Ta2iv = crate::Reg<ta2iv::Ta2ivSpec>;
#[doc = "TimerAx Interrupt Vector Register"]
pub mod ta2iv;
