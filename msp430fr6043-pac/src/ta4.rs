#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ta4ctl: Ta4ctl,
    ta4cctl0: Ta4cctl0,
    ta4cctl1: Ta4cctl1,
    _reserved3: [u8; 0x0a],
    ta4r: Ta4r,
    ta4ccr0: Ta4ccr0,
    ta4ccr1: Ta4ccr1,
    _reserved6: [u8; 0x0a],
    ta4ex0: Ta4ex0,
    _reserved7: [u8; 0x0c],
    ta4iv: Ta4iv,
}
impl RegisterBlock {
    #[doc = "0x00 - TimerAx Control Register"]
    #[inline(always)]
    pub const fn ta4ctl(&self) -> &Ta4ctl {
        &self.ta4ctl
    }
    #[doc = "0x02 - Timer_A Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn ta4cctl0(&self) -> &Ta4cctl0 {
        &self.ta4cctl0
    }
    #[doc = "0x04 - Timer_A Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn ta4cctl1(&self) -> &Ta4cctl1 {
        &self.ta4cctl1
    }
    #[doc = "0x10 - TimerA register"]
    #[inline(always)]
    pub const fn ta4r(&self) -> &Ta4r {
        &self.ta4r
    }
    #[doc = "0x12 - Timer_A Capture/Compare Register"]
    #[inline(always)]
    pub const fn ta4ccr0(&self) -> &Ta4ccr0 {
        &self.ta4ccr0
    }
    #[doc = "0x14 - Timer_A Capture/Compare Register"]
    #[inline(always)]
    pub const fn ta4ccr1(&self) -> &Ta4ccr1 {
        &self.ta4ccr1
    }
    #[doc = "0x20 - TimerAx Expansion 0 Register"]
    #[inline(always)]
    pub const fn ta4ex0(&self) -> &Ta4ex0 {
        &self.ta4ex0
    }
    #[doc = "0x2e - TimerAx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ta4iv(&self) -> &Ta4iv {
        &self.ta4iv
    }
}
#[doc = "TA4CTL (rw) register accessor: TimerAx Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta4ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta4ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta4ctl`] module"]
#[doc(alias = "TA4CTL")]
pub type Ta4ctl = crate::Reg<ta4ctl::Ta4ctlSpec>;
#[doc = "TimerAx Control Register"]
pub mod ta4ctl;
#[doc = "TA4CCTL0 (rw) register accessor: Timer_A Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta4cctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta4cctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta4cctl0`] module"]
#[doc(alias = "TA4CCTL0")]
pub type Ta4cctl0 = crate::Reg<ta4cctl0::Ta4cctl0Spec>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta4cctl0;
#[doc = "TA4CCTL1 (rw) register accessor: Timer_A Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta4cctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta4cctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta4cctl1`] module"]
#[doc(alias = "TA4CCTL1")]
pub type Ta4cctl1 = crate::Reg<ta4cctl1::Ta4cctl1Spec>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta4cctl1;
#[doc = "TA4R (rw) register accessor: TimerA register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta4r`] module"]
#[doc(alias = "TA4R")]
pub type Ta4r = crate::Reg<ta4r::Ta4rSpec>;
#[doc = "TimerA register"]
pub mod ta4r;
#[doc = "TA4CCR0 (rw) register accessor: Timer_A Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta4ccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta4ccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta4ccr0`] module"]
#[doc(alias = "TA4CCR0")]
pub type Ta4ccr0 = crate::Reg<ta4ccr0::Ta4ccr0Spec>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta4ccr0;
#[doc = "TA4CCR1 (rw) register accessor: Timer_A Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta4ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta4ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta4ccr1`] module"]
#[doc(alias = "TA4CCR1")]
pub type Ta4ccr1 = crate::Reg<ta4ccr1::Ta4ccr1Spec>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta4ccr1;
#[doc = "TA4EX0 (rw) register accessor: TimerAx Expansion 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta4ex0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta4ex0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta4ex0`] module"]
#[doc(alias = "TA4EX0")]
pub type Ta4ex0 = crate::Reg<ta4ex0::Ta4ex0Spec>;
#[doc = "TimerAx Expansion 0 Register"]
pub mod ta4ex0;
#[doc = "TA4IV (rw) register accessor: TimerAx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta4iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta4iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta4iv`] module"]
#[doc(alias = "TA4IV")]
pub type Ta4iv = crate::Reg<ta4iv::Ta4ivSpec>;
#[doc = "TimerAx Interrupt Vector Register"]
pub mod ta4iv;
