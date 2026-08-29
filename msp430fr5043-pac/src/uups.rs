#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uupsiidx: Uupsiidx,
    uupsmis: Uupsmis,
    uupsris: Uupsris,
    uupsimsc: Uupsimsc,
    uupsicr: Uupsicr,
    uupsisr: Uupsisr,
    uupsdesclo: Uupsdesclo,
    uupsdeschi: Uupsdeschi,
    uupsctl: Uupsctl,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn uupsiidx(&self) -> &Uupsiidx {
        &self.uupsiidx
    }
    #[doc = "0x02 - Masked Interrupt Status Register"]
    #[inline(always)]
    pub const fn uupsmis(&self) -> &Uupsmis {
        &self.uupsmis
    }
    #[doc = "0x04 - Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn uupsris(&self) -> &Uupsris {
        &self.uupsris
    }
    #[doc = "0x06 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn uupsimsc(&self) -> &Uupsimsc {
        &self.uupsimsc
    }
    #[doc = "0x08 - Interrupt Clear Register."]
    #[inline(always)]
    pub const fn uupsicr(&self) -> &Uupsicr {
        &self.uupsicr
    }
    #[doc = "0x0a - Interrupt Flag Set Register."]
    #[inline(always)]
    pub const fn uupsisr(&self) -> &Uupsisr {
        &self.uupsisr
    }
    #[doc = "0x0c - UUPS Descriptor Register L."]
    #[inline(always)]
    pub const fn uupsdesclo(&self) -> &Uupsdesclo {
        &self.uupsdesclo
    }
    #[doc = "0x0e - UUPS Descriptor Register H."]
    #[inline(always)]
    pub const fn uupsdeschi(&self) -> &Uupsdeschi {
        &self.uupsdeschi
    }
    #[doc = "0x10 - UUPS Control"]
    #[inline(always)]
    pub const fn uupsctl(&self) -> &Uupsctl {
        &self.uupsctl
    }
}
#[doc = "UUPSIIDX (rw) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsiidx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsiidx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uupsiidx`] module"]
#[doc(alias = "UUPSIIDX")]
pub type Uupsiidx = crate::Reg<uupsiidx::UupsiidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod uupsiidx;
#[doc = "UUPSMIS (rw) register accessor: Masked Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsmis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsmis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uupsmis`] module"]
#[doc(alias = "UUPSMIS")]
pub type Uupsmis = crate::Reg<uupsmis::UupsmisSpec>;
#[doc = "Masked Interrupt Status Register"]
pub mod uupsmis;
#[doc = "UUPSRIS (rw) register accessor: Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsris::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsris::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uupsris`] module"]
#[doc(alias = "UUPSRIS")]
pub type Uupsris = crate::Reg<uupsris::UupsrisSpec>;
#[doc = "Raw Interrupt Status Register"]
pub mod uupsris;
#[doc = "UUPSIMSC (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsimsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsimsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uupsimsc`] module"]
#[doc(alias = "UUPSIMSC")]
pub type Uupsimsc = crate::Reg<uupsimsc::UupsimscSpec>;
#[doc = "Interrupt Mask Register"]
pub mod uupsimsc;
#[doc = "UUPSICR (rw) register accessor: Interrupt Clear Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uupsicr`] module"]
#[doc(alias = "UUPSICR")]
pub type Uupsicr = crate::Reg<uupsicr::UupsicrSpec>;
#[doc = "Interrupt Clear Register."]
pub mod uupsicr;
#[doc = "UUPSISR (rw) register accessor: Interrupt Flag Set Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uupsisr`] module"]
#[doc(alias = "UUPSISR")]
pub type Uupsisr = crate::Reg<uupsisr::UupsisrSpec>;
#[doc = "Interrupt Flag Set Register."]
pub mod uupsisr;
#[doc = "UUPSDESCLO (rw) register accessor: UUPS Descriptor Register L.\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsdesclo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsdesclo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uupsdesclo`] module"]
#[doc(alias = "UUPSDESCLO")]
pub type Uupsdesclo = crate::Reg<uupsdesclo::UupsdescloSpec>;
#[doc = "UUPS Descriptor Register L."]
pub mod uupsdesclo;
#[doc = "UUPSDESCHI (rw) register accessor: UUPS Descriptor Register H.\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsdeschi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsdeschi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uupsdeschi`] module"]
#[doc(alias = "UUPSDESCHI")]
pub type Uupsdeschi = crate::Reg<uupsdeschi::UupsdeschiSpec>;
#[doc = "UUPS Descriptor Register H."]
pub mod uupsdeschi;
#[doc = "UUPSCTL (rw) register accessor: UUPS Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uupsctl`] module"]
#[doc(alias = "UUPSCTL")]
pub type Uupsctl = crate::Reg<uupsctl::UupsctlSpec>;
#[doc = "UUPS Control"]
pub mod uupsctl;
