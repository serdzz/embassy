#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tbiv: Tbiv,
    _reserved1: [u8; 0x60],
    tbctl: Tbctl,
    tbcctl0: Tbcctl0,
    tbcctl1: Tbcctl1,
    tbcctl2: Tbcctl2,
    tbcctl3: Tbcctl3,
    tbcctl4: Tbcctl4,
    tbcctl5: Tbcctl5,
    tbcctl6: Tbcctl6,
    tbr: Tbr,
    tbccr0: Tbccr0,
    tbccr1: Tbccr1,
    tbccr2: Tbccr2,
    tbccr3: Tbccr3,
    tbccr4: Tbccr4,
    tbccr5: Tbccr5,
    tbccr6: Tbccr6,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer B Interrupt Vector Word"]
    #[inline(always)]
    pub const fn tbiv(&self) -> &Tbiv {
        &self.tbiv
    }
    #[doc = "0x62 - Timer B Control"]
    #[inline(always)]
    pub const fn tbctl(&self) -> &Tbctl {
        &self.tbctl
    }
    #[doc = "0x64 - Timer B Capture/Compare Control 0"]
    #[inline(always)]
    pub const fn tbcctl0(&self) -> &Tbcctl0 {
        &self.tbcctl0
    }
    #[doc = "0x66 - Timer B Capture/Compare Control 1"]
    #[inline(always)]
    pub const fn tbcctl1(&self) -> &Tbcctl1 {
        &self.tbcctl1
    }
    #[doc = "0x68 - Timer B Capture/Compare Control 2"]
    #[inline(always)]
    pub const fn tbcctl2(&self) -> &Tbcctl2 {
        &self.tbcctl2
    }
    #[doc = "0x6a - Timer B Capture/Compare Control 3"]
    #[inline(always)]
    pub const fn tbcctl3(&self) -> &Tbcctl3 {
        &self.tbcctl3
    }
    #[doc = "0x6c - Timer B Capture/Compare Control 4"]
    #[inline(always)]
    pub const fn tbcctl4(&self) -> &Tbcctl4 {
        &self.tbcctl4
    }
    #[doc = "0x6e - Timer B Capture/Compare Control 5"]
    #[inline(always)]
    pub const fn tbcctl5(&self) -> &Tbcctl5 {
        &self.tbcctl5
    }
    #[doc = "0x70 - Timer B Capture/Compare Control 6"]
    #[inline(always)]
    pub const fn tbcctl6(&self) -> &Tbcctl6 {
        &self.tbcctl6
    }
    #[doc = "0x72 - Timer B Counter Register"]
    #[inline(always)]
    pub const fn tbr(&self) -> &Tbr {
        &self.tbr
    }
    #[doc = "0x74 - Timer B Capture/Compare 0"]
    #[inline(always)]
    pub const fn tbccr0(&self) -> &Tbccr0 {
        &self.tbccr0
    }
    #[doc = "0x76 - Timer B Capture/Compare 1"]
    #[inline(always)]
    pub const fn tbccr1(&self) -> &Tbccr1 {
        &self.tbccr1
    }
    #[doc = "0x78 - Timer B Capture/Compare 2"]
    #[inline(always)]
    pub const fn tbccr2(&self) -> &Tbccr2 {
        &self.tbccr2
    }
    #[doc = "0x7a - Timer B Capture/Compare 3"]
    #[inline(always)]
    pub const fn tbccr3(&self) -> &Tbccr3 {
        &self.tbccr3
    }
    #[doc = "0x7c - Timer B Capture/Compare 4"]
    #[inline(always)]
    pub const fn tbccr4(&self) -> &Tbccr4 {
        &self.tbccr4
    }
    #[doc = "0x7e - Timer B Capture/Compare 5"]
    #[inline(always)]
    pub const fn tbccr5(&self) -> &Tbccr5 {
        &self.tbccr5
    }
    #[doc = "0x80 - Timer B Capture/Compare 6"]
    #[inline(always)]
    pub const fn tbccr6(&self) -> &Tbccr6 {
        &self.tbccr6
    }
}
#[doc = "TBIV (rw) register accessor: Timer B Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`tbiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbiv`] module"]
#[doc(alias = "TBIV")]
pub type Tbiv = crate::Reg<tbiv::TbivSpec>;
#[doc = "Timer B Interrupt Vector Word"]
pub mod tbiv;
#[doc = "TBCTL (rw) register accessor: Timer B Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbctl`] module"]
#[doc(alias = "TBCTL")]
pub type Tbctl = crate::Reg<tbctl::TbctlSpec>;
#[doc = "Timer B Control"]
pub mod tbctl;
#[doc = "TBCCTL0 (rw) register accessor: Timer B Capture/Compare Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tbcctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbcctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbcctl0`] module"]
#[doc(alias = "TBCCTL0")]
pub type Tbcctl0 = crate::Reg<tbcctl0::Tbcctl0Spec>;
#[doc = "Timer B Capture/Compare Control 0"]
pub mod tbcctl0;
#[doc = "TBCCTL1 (rw) register accessor: Timer B Capture/Compare Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tbcctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbcctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbcctl1`] module"]
#[doc(alias = "TBCCTL1")]
pub type Tbcctl1 = crate::Reg<tbcctl1::Tbcctl1Spec>;
#[doc = "Timer B Capture/Compare Control 1"]
pub mod tbcctl1;
#[doc = "TBCCTL2 (rw) register accessor: Timer B Capture/Compare Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tbcctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbcctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbcctl2`] module"]
#[doc(alias = "TBCCTL2")]
pub type Tbcctl2 = crate::Reg<tbcctl2::Tbcctl2Spec>;
#[doc = "Timer B Capture/Compare Control 2"]
pub mod tbcctl2;
#[doc = "TBCCTL3 (rw) register accessor: Timer B Capture/Compare Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`tbcctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbcctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbcctl3`] module"]
#[doc(alias = "TBCCTL3")]
pub type Tbcctl3 = crate::Reg<tbcctl3::Tbcctl3Spec>;
#[doc = "Timer B Capture/Compare Control 3"]
pub mod tbcctl3;
#[doc = "TBCCTL4 (rw) register accessor: Timer B Capture/Compare Control 4\n\nYou can [`read`](crate::Reg::read) this register and get [`tbcctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbcctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbcctl4`] module"]
#[doc(alias = "TBCCTL4")]
pub type Tbcctl4 = crate::Reg<tbcctl4::Tbcctl4Spec>;
#[doc = "Timer B Capture/Compare Control 4"]
pub mod tbcctl4;
#[doc = "TBCCTL5 (rw) register accessor: Timer B Capture/Compare Control 5\n\nYou can [`read`](crate::Reg::read) this register and get [`tbcctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbcctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbcctl5`] module"]
#[doc(alias = "TBCCTL5")]
pub type Tbcctl5 = crate::Reg<tbcctl5::Tbcctl5Spec>;
#[doc = "Timer B Capture/Compare Control 5"]
pub mod tbcctl5;
#[doc = "TBCCTL6 (rw) register accessor: Timer B Capture/Compare Control 6\n\nYou can [`read`](crate::Reg::read) this register and get [`tbcctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbcctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbcctl6`] module"]
#[doc(alias = "TBCCTL6")]
pub type Tbcctl6 = crate::Reg<tbcctl6::Tbcctl6Spec>;
#[doc = "Timer B Capture/Compare Control 6"]
pub mod tbcctl6;
#[doc = "TBR (rw) register accessor: Timer B Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbr`] module"]
#[doc(alias = "TBR")]
pub type Tbr = crate::Reg<tbr::TbrSpec>;
#[doc = "Timer B Counter Register"]
pub mod tbr;
#[doc = "TBCCR0 (rw) register accessor: Timer B Capture/Compare 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbccr0`] module"]
#[doc(alias = "TBCCR0")]
pub type Tbccr0 = crate::Reg<tbccr0::Tbccr0Spec>;
#[doc = "Timer B Capture/Compare 0"]
pub mod tbccr0;
#[doc = "TBCCR1 (rw) register accessor: Timer B Capture/Compare 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbccr1`] module"]
#[doc(alias = "TBCCR1")]
pub type Tbccr1 = crate::Reg<tbccr1::Tbccr1Spec>;
#[doc = "Timer B Capture/Compare 1"]
pub mod tbccr1;
#[doc = "TBCCR2 (rw) register accessor: Timer B Capture/Compare 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbccr2`] module"]
#[doc(alias = "TBCCR2")]
pub type Tbccr2 = crate::Reg<tbccr2::Tbccr2Spec>;
#[doc = "Timer B Capture/Compare 2"]
pub mod tbccr2;
#[doc = "TBCCR3 (rw) register accessor: Timer B Capture/Compare 3\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbccr3`] module"]
#[doc(alias = "TBCCR3")]
pub type Tbccr3 = crate::Reg<tbccr3::Tbccr3Spec>;
#[doc = "Timer B Capture/Compare 3"]
pub mod tbccr3;
#[doc = "TBCCR4 (rw) register accessor: Timer B Capture/Compare 4\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbccr4`] module"]
#[doc(alias = "TBCCR4")]
pub type Tbccr4 = crate::Reg<tbccr4::Tbccr4Spec>;
#[doc = "Timer B Capture/Compare 4"]
pub mod tbccr4;
#[doc = "TBCCR5 (rw) register accessor: Timer B Capture/Compare 5\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbccr5`] module"]
#[doc(alias = "TBCCR5")]
pub type Tbccr5 = crate::Reg<tbccr5::Tbccr5Spec>;
#[doc = "Timer B Capture/Compare 5"]
pub mod tbccr5;
#[doc = "TBCCR6 (rw) register accessor: Timer B Capture/Compare 6\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbccr6`] module"]
#[doc(alias = "TBCCR6")]
pub type Tbccr6 = crate::Reg<tbccr6::Tbccr6Spec>;
#[doc = "Timer B Capture/Compare 6"]
pub mod tbccr6;
