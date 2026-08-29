#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sdhsiidx: Sdhsiidx,
    sdhsmis: Sdhsmis,
    sdhsris: Sdhsris,
    sdhsimsc: Sdhsimsc,
    sdhsicr: Sdhsicr,
    sdhsisr: Sdhsisr,
    sdhsdesclo: Sdhsdesclo,
    sdhsdeschi: Sdhsdeschi,
    sdhsctl0: Sdhsctl0,
    sdhsctl1: Sdhsctl1,
    sdhsctl2: Sdhsctl2,
    sdhsctl3: Sdhsctl3,
    sdhsctl4: Sdhsctl4,
    sdhsctl5: Sdhsctl5,
    sdhsctl6: Sdhsctl6,
    sdhsctl7: Sdhsctl7,
    _reserved16: [u8; 0x02],
    sdhsdt: Sdhsdt,
    sdhswinhith: Sdhswinhith,
    sdhswinloth: Sdhswinloth,
    sdhsdtcda: Sdhsdtcda,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn sdhsiidx(&self) -> &Sdhsiidx {
        &self.sdhsiidx
    }
    #[doc = "0x02 - Masked Interrupt Status and Clear Register"]
    #[inline(always)]
    pub const fn sdhsmis(&self) -> &Sdhsmis {
        &self.sdhsmis
    }
    #[doc = "0x04 - Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn sdhsris(&self) -> &Sdhsris {
        &self.sdhsris
    }
    #[doc = "0x06 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn sdhsimsc(&self) -> &Sdhsimsc {
        &self.sdhsimsc
    }
    #[doc = "0x08 - Interrupt Clear Register."]
    #[inline(always)]
    pub const fn sdhsicr(&self) -> &Sdhsicr {
        &self.sdhsicr
    }
    #[doc = "0x0a - Interrupt Set Register."]
    #[inline(always)]
    pub const fn sdhsisr(&self) -> &Sdhsisr {
        &self.sdhsisr
    }
    #[doc = "0x0c - SDHS Descriptor Register L."]
    #[inline(always)]
    pub const fn sdhsdesclo(&self) -> &Sdhsdesclo {
        &self.sdhsdesclo
    }
    #[doc = "0x0e - SDHS Descriptor Register H."]
    #[inline(always)]
    pub const fn sdhsdeschi(&self) -> &Sdhsdeschi {
        &self.sdhsdeschi
    }
    #[doc = "0x10 - SDHS Control Register 0"]
    #[inline(always)]
    pub const fn sdhsctl0(&self) -> &Sdhsctl0 {
        &self.sdhsctl0
    }
    #[doc = "0x12 - SDHS Control Register 1"]
    #[inline(always)]
    pub const fn sdhsctl1(&self) -> &Sdhsctl1 {
        &self.sdhsctl1
    }
    #[doc = "0x14 - SDHS Control Register 2"]
    #[inline(always)]
    pub const fn sdhsctl2(&self) -> &Sdhsctl2 {
        &self.sdhsctl2
    }
    #[doc = "0x16 - SDHS Control Register 3"]
    #[inline(always)]
    pub const fn sdhsctl3(&self) -> &Sdhsctl3 {
        &self.sdhsctl3
    }
    #[doc = "0x18 - SDHS Control Register 4"]
    #[inline(always)]
    pub const fn sdhsctl4(&self) -> &Sdhsctl4 {
        &self.sdhsctl4
    }
    #[doc = "0x1a - SDHS Control Register 5"]
    #[inline(always)]
    pub const fn sdhsctl5(&self) -> &Sdhsctl5 {
        &self.sdhsctl5
    }
    #[doc = "0x1c - SDHS Control Register 6"]
    #[inline(always)]
    pub const fn sdhsctl6(&self) -> &Sdhsctl6 {
        &self.sdhsctl6
    }
    #[doc = "0x1e - SDHS Control Register 7"]
    #[inline(always)]
    pub const fn sdhsctl7(&self) -> &Sdhsctl7 {
        &self.sdhsctl7
    }
    #[doc = "0x22 - SDHS Data Converstion Register"]
    #[inline(always)]
    pub const fn sdhsdt(&self) -> &Sdhsdt {
        &self.sdhsdt
    }
    #[doc = "0x24 - SDHS Window Comparator High Threshold Register."]
    #[inline(always)]
    pub const fn sdhswinhith(&self) -> &Sdhswinhith {
        &self.sdhswinhith
    }
    #[doc = "0x26 - SDHS Window Comparator Low Threshold Register."]
    #[inline(always)]
    pub const fn sdhswinloth(&self) -> &Sdhswinloth {
        &self.sdhswinloth
    }
    #[doc = "0x28 - DTC destination address register"]
    #[inline(always)]
    pub const fn sdhsdtcda(&self) -> &Sdhsdtcda {
        &self.sdhsdtcda
    }
}
#[doc = "SDHSIIDX (rw) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsiidx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsiidx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsiidx`] module"]
#[doc(alias = "SDHSIIDX")]
pub type Sdhsiidx = crate::Reg<sdhsiidx::SdhsiidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod sdhsiidx;
#[doc = "SDHSMIS (rw) register accessor: Masked Interrupt Status and Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsmis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsmis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsmis`] module"]
#[doc(alias = "SDHSMIS")]
pub type Sdhsmis = crate::Reg<sdhsmis::SdhsmisSpec>;
#[doc = "Masked Interrupt Status and Clear Register"]
pub mod sdhsmis;
#[doc = "SDHSRIS (rw) register accessor: Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsris::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsris::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsris`] module"]
#[doc(alias = "SDHSRIS")]
pub type Sdhsris = crate::Reg<sdhsris::SdhsrisSpec>;
#[doc = "Raw Interrupt Status Register"]
pub mod sdhsris;
#[doc = "SDHSIMSC (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsimsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsimsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsimsc`] module"]
#[doc(alias = "SDHSIMSC")]
pub type Sdhsimsc = crate::Reg<sdhsimsc::SdhsimscSpec>;
#[doc = "Interrupt Mask Register"]
pub mod sdhsimsc;
#[doc = "SDHSICR (rw) register accessor: Interrupt Clear Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsicr`] module"]
#[doc(alias = "SDHSICR")]
pub type Sdhsicr = crate::Reg<sdhsicr::SdhsicrSpec>;
#[doc = "Interrupt Clear Register."]
pub mod sdhsicr;
#[doc = "SDHSISR (rw) register accessor: Interrupt Set Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsisr`] module"]
#[doc(alias = "SDHSISR")]
pub type Sdhsisr = crate::Reg<sdhsisr::SdhsisrSpec>;
#[doc = "Interrupt Set Register."]
pub mod sdhsisr;
#[doc = "SDHSDESCLO (rw) register accessor: SDHS Descriptor Register L.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsdesclo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsdesclo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsdesclo`] module"]
#[doc(alias = "SDHSDESCLO")]
pub type Sdhsdesclo = crate::Reg<sdhsdesclo::SdhsdescloSpec>;
#[doc = "SDHS Descriptor Register L."]
pub mod sdhsdesclo;
#[doc = "SDHSDESCHI (rw) register accessor: SDHS Descriptor Register H.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsdeschi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsdeschi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsdeschi`] module"]
#[doc(alias = "SDHSDESCHI")]
pub type Sdhsdeschi = crate::Reg<sdhsdeschi::SdhsdeschiSpec>;
#[doc = "SDHS Descriptor Register H."]
pub mod sdhsdeschi;
#[doc = "SDHSCTL0 (rw) register accessor: SDHS Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsctl0`] module"]
#[doc(alias = "SDHSCTL0")]
pub type Sdhsctl0 = crate::Reg<sdhsctl0::Sdhsctl0Spec>;
#[doc = "SDHS Control Register 0"]
pub mod sdhsctl0;
#[doc = "SDHSCTL1 (rw) register accessor: SDHS Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsctl1`] module"]
#[doc(alias = "SDHSCTL1")]
pub type Sdhsctl1 = crate::Reg<sdhsctl1::Sdhsctl1Spec>;
#[doc = "SDHS Control Register 1"]
pub mod sdhsctl1;
#[doc = "SDHSCTL2 (rw) register accessor: SDHS Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsctl2`] module"]
#[doc(alias = "SDHSCTL2")]
pub type Sdhsctl2 = crate::Reg<sdhsctl2::Sdhsctl2Spec>;
#[doc = "SDHS Control Register 2"]
pub mod sdhsctl2;
#[doc = "SDHSCTL3 (rw) register accessor: SDHS Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsctl3`] module"]
#[doc(alias = "SDHSCTL3")]
pub type Sdhsctl3 = crate::Reg<sdhsctl3::Sdhsctl3Spec>;
#[doc = "SDHS Control Register 3"]
pub mod sdhsctl3;
#[doc = "SDHSCTL4 (rw) register accessor: SDHS Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsctl4`] module"]
#[doc(alias = "SDHSCTL4")]
pub type Sdhsctl4 = crate::Reg<sdhsctl4::Sdhsctl4Spec>;
#[doc = "SDHS Control Register 4"]
pub mod sdhsctl4;
#[doc = "SDHSCTL5 (rw) register accessor: SDHS Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsctl5`] module"]
#[doc(alias = "SDHSCTL5")]
pub type Sdhsctl5 = crate::Reg<sdhsctl5::Sdhsctl5Spec>;
#[doc = "SDHS Control Register 5"]
pub mod sdhsctl5;
#[doc = "SDHSCTL6 (rw) register accessor: SDHS Control Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsctl6`] module"]
#[doc(alias = "SDHSCTL6")]
pub type Sdhsctl6 = crate::Reg<sdhsctl6::Sdhsctl6Spec>;
#[doc = "SDHS Control Register 6"]
pub mod sdhsctl6;
#[doc = "SDHSCTL7 (rw) register accessor: SDHS Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsctl7`] module"]
#[doc(alias = "SDHSCTL7")]
pub type Sdhsctl7 = crate::Reg<sdhsctl7::Sdhsctl7Spec>;
#[doc = "SDHS Control Register 7"]
pub mod sdhsctl7;
#[doc = "SDHSDT (rw) register accessor: SDHS Data Converstion Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsdt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsdt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsdt`] module"]
#[doc(alias = "SDHSDT")]
pub type Sdhsdt = crate::Reg<sdhsdt::SdhsdtSpec>;
#[doc = "SDHS Data Converstion Register"]
pub mod sdhsdt;
#[doc = "SDHSWINHITH (rw) register accessor: SDHS Window Comparator High Threshold Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhswinhith::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhswinhith::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhswinhith`] module"]
#[doc(alias = "SDHSWINHITH")]
pub type Sdhswinhith = crate::Reg<sdhswinhith::SdhswinhithSpec>;
#[doc = "SDHS Window Comparator High Threshold Register."]
pub mod sdhswinhith;
#[doc = "SDHSWINLOTH (rw) register accessor: SDHS Window Comparator Low Threshold Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhswinloth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhswinloth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhswinloth`] module"]
#[doc(alias = "SDHSWINLOTH")]
pub type Sdhswinloth = crate::Reg<sdhswinloth::SdhswinlothSpec>;
#[doc = "SDHS Window Comparator Low Threshold Register."]
pub mod sdhswinloth;
#[doc = "SDHSDTCDA (rw) register accessor: DTC destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsdtcda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsdtcda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhsdtcda`] module"]
#[doc(alias = "SDHSDTCDA")]
pub type Sdhsdtcda = crate::Reg<sdhsdtcda::SdhsdtcdaSpec>;
#[doc = "DTC destination address register"]
pub mod sdhsdtcda;
