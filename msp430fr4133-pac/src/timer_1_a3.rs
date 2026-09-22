#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ta1ctl: Ta1ctl,
    ta1cctl0: Ta1cctl0,
    ta1cctl1: Ta1cctl1,
    ta1cctl2: Ta1cctl2,
    _reserved4: [u8; 0x08],
    ta1r: Ta1r,
    ta1ccr0: Ta1ccr0,
    ta1ccr1: Ta1ccr1,
    ta1ccr2: Ta1ccr2,
    _reserved8: [u8; 0x08],
    ta1ex0: Ta1ex0,
    _reserved9: [u8; 0x0c],
    ta1iv: Ta1iv,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer1_A3 Control"]
    #[inline(always)]
    pub const fn ta1ctl(&self) -> &Ta1ctl {
        &self.ta1ctl
    }
    #[doc = "0x02 - Timer1_A3 Capture/Compare Control 0"]
    #[inline(always)]
    pub const fn ta1cctl0(&self) -> &Ta1cctl0 {
        &self.ta1cctl0
    }
    #[doc = "0x04 - Timer1_A3 Capture/Compare Control 1"]
    #[inline(always)]
    pub const fn ta1cctl1(&self) -> &Ta1cctl1 {
        &self.ta1cctl1
    }
    #[doc = "0x06 - Timer1_A3 Capture/Compare Control 2"]
    #[inline(always)]
    pub const fn ta1cctl2(&self) -> &Ta1cctl2 {
        &self.ta1cctl2
    }
    #[doc = "0x10 - Timer1_A3"]
    #[inline(always)]
    pub const fn ta1r(&self) -> &Ta1r {
        &self.ta1r
    }
    #[doc = "0x12 - Timer1_A3 Capture/Compare 0"]
    #[inline(always)]
    pub const fn ta1ccr0(&self) -> &Ta1ccr0 {
        &self.ta1ccr0
    }
    #[doc = "0x14 - Timer1_A3 Capture/Compare 1"]
    #[inline(always)]
    pub const fn ta1ccr1(&self) -> &Ta1ccr1 {
        &self.ta1ccr1
    }
    #[doc = "0x16 - Timer1_A3 Capture/Compare 2"]
    #[inline(always)]
    pub const fn ta1ccr2(&self) -> &Ta1ccr2 {
        &self.ta1ccr2
    }
    #[doc = "0x20 - Timer1_A3 Expansion Register 0"]
    #[inline(always)]
    pub const fn ta1ex0(&self) -> &Ta1ex0 {
        &self.ta1ex0
    }
    #[doc = "0x2e - Timer1_A3 Interrupt Vector Word"]
    #[inline(always)]
    pub const fn ta1iv(&self) -> &Ta1iv {
        &self.ta1iv
    }
}
#[doc = "TA1CTL (rw) register accessor: Timer1_A3 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ta1ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta1ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1ctl`] module"]
#[doc(alias = "TA1CTL")]
pub type Ta1ctl = crate::Reg<ta1ctl::Ta1ctlSpec>;
#[doc = "Timer1_A3 Control"]
pub mod ta1ctl;
#[doc = "TA1CCTL0 (rw) register accessor: Timer1_A3 Capture/Compare Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ta1cctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta1cctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1cctl0`] module"]
#[doc(alias = "TA1CCTL0")]
pub type Ta1cctl0 = crate::Reg<ta1cctl0::Ta1cctl0Spec>;
#[doc = "Timer1_A3 Capture/Compare Control 0"]
pub mod ta1cctl0;
#[doc = "TA1CCTL1 (rw) register accessor: Timer1_A3 Capture/Compare Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ta1cctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta1cctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1cctl1`] module"]
#[doc(alias = "TA1CCTL1")]
pub type Ta1cctl1 = crate::Reg<ta1cctl1::Ta1cctl1Spec>;
#[doc = "Timer1_A3 Capture/Compare Control 1"]
pub mod ta1cctl1;
#[doc = "TA1CCTL2 (rw) register accessor: Timer1_A3 Capture/Compare Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ta1cctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta1cctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1cctl2`] module"]
#[doc(alias = "TA1CCTL2")]
pub type Ta1cctl2 = crate::Reg<ta1cctl2::Ta1cctl2Spec>;
#[doc = "Timer1_A3 Capture/Compare Control 2"]
pub mod ta1cctl2;
#[doc = "TA1R (rw) register accessor: Timer1_A3\n\nYou can [`read`](crate::Reg::read) this register and get [`ta1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1r`] module"]
#[doc(alias = "TA1R")]
pub type Ta1r = crate::Reg<ta1r::Ta1rSpec>;
#[doc = "Timer1_A3"]
pub mod ta1r;
#[doc = "TA1CCR0 (rw) register accessor: Timer1_A3 Capture/Compare 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ta1ccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta1ccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1ccr0`] module"]
#[doc(alias = "TA1CCR0")]
pub type Ta1ccr0 = crate::Reg<ta1ccr0::Ta1ccr0Spec>;
#[doc = "Timer1_A3 Capture/Compare 0"]
pub mod ta1ccr0;
#[doc = "TA1CCR1 (rw) register accessor: Timer1_A3 Capture/Compare 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ta1ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta1ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1ccr1`] module"]
#[doc(alias = "TA1CCR1")]
pub type Ta1ccr1 = crate::Reg<ta1ccr1::Ta1ccr1Spec>;
#[doc = "Timer1_A3 Capture/Compare 1"]
pub mod ta1ccr1;
#[doc = "TA1CCR2 (rw) register accessor: Timer1_A3 Capture/Compare 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ta1ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta1ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1ccr2`] module"]
#[doc(alias = "TA1CCR2")]
pub type Ta1ccr2 = crate::Reg<ta1ccr2::Ta1ccr2Spec>;
#[doc = "Timer1_A3 Capture/Compare 2"]
pub mod ta1ccr2;
#[doc = "TA1EX0 (rw) register accessor: Timer1_A3 Expansion Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ta1ex0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta1ex0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1ex0`] module"]
#[doc(alias = "TA1EX0")]
pub type Ta1ex0 = crate::Reg<ta1ex0::Ta1ex0Spec>;
#[doc = "Timer1_A3 Expansion Register 0"]
pub mod ta1ex0;
#[doc = "TA1IV (rw) register accessor: Timer1_A3 Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ta1iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta1iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1iv`] module"]
#[doc(alias = "TA1IV")]
pub type Ta1iv = crate::Reg<ta1iv::Ta1ivSpec>;
#[doc = "Timer1_A3 Interrupt Vector Word"]
pub mod ta1iv;
