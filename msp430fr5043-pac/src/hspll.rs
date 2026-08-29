#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hsplliidx: Hsplliidx,
    hspllmis: Hspllmis,
    hspllris: Hspllris,
    hspllimsc: Hspllimsc,
    hspllicr: Hspllicr,
    hspllisr: Hspllisr,
    hsplldesclo: Hsplldesclo,
    hsplldeschi: Hsplldeschi,
    hspllctl: Hspllctl,
    hspllussxtlctl: Hspllussxtlctl,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn hsplliidx(&self) -> &Hsplliidx {
        &self.hsplliidx
    }
    #[doc = "0x02 - Masked Interrupt Status Register."]
    #[inline(always)]
    pub const fn hspllmis(&self) -> &Hspllmis {
        &self.hspllmis
    }
    #[doc = "0x04 - Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn hspllris(&self) -> &Hspllris {
        &self.hspllris
    }
    #[doc = "0x06 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hspllimsc(&self) -> &Hspllimsc {
        &self.hspllimsc
    }
    #[doc = "0x08 - Interrupt Flag Clear Register."]
    #[inline(always)]
    pub const fn hspllicr(&self) -> &Hspllicr {
        &self.hspllicr
    }
    #[doc = "0x0a - Interrupt Flag Set Register."]
    #[inline(always)]
    pub const fn hspllisr(&self) -> &Hspllisr {
        &self.hspllisr
    }
    #[doc = "0x0c - HSPLL Descriptor Register L."]
    #[inline(always)]
    pub const fn hsplldesclo(&self) -> &Hsplldesclo {
        &self.hsplldesclo
    }
    #[doc = "0x0e - HSPLL Descriptor Register H."]
    #[inline(always)]
    pub const fn hsplldeschi(&self) -> &Hsplldeschi {
        &self.hsplldeschi
    }
    #[doc = "0x10 - HSPLL Control Register"]
    #[inline(always)]
    pub const fn hspllctl(&self) -> &Hspllctl {
        &self.hspllctl
    }
    #[doc = "0x12 - USSXT Control Register"]
    #[inline(always)]
    pub const fn hspllussxtlctl(&self) -> &Hspllussxtlctl {
        &self.hspllussxtlctl
    }
}
#[doc = "HSPLLIIDX (rw) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsplliidx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsplliidx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsplliidx`] module"]
#[doc(alias = "HSPLLIIDX")]
pub type Hsplliidx = crate::Reg<hsplliidx::HsplliidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod hsplliidx;
#[doc = "HSPLLMIS (rw) register accessor: Masked Interrupt Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hspllmis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspllmis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspllmis`] module"]
#[doc(alias = "HSPLLMIS")]
pub type Hspllmis = crate::Reg<hspllmis::HspllmisSpec>;
#[doc = "Masked Interrupt Status Register."]
pub mod hspllmis;
#[doc = "HSPLLRIS (rw) register accessor: Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hspllris::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspllris::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspllris`] module"]
#[doc(alias = "HSPLLRIS")]
pub type Hspllris = crate::Reg<hspllris::HspllrisSpec>;
#[doc = "Raw Interrupt Status Register"]
pub mod hspllris;
#[doc = "HSPLLIMSC (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hspllimsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspllimsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspllimsc`] module"]
#[doc(alias = "HSPLLIMSC")]
pub type Hspllimsc = crate::Reg<hspllimsc::HspllimscSpec>;
#[doc = "Interrupt Mask Register"]
pub mod hspllimsc;
#[doc = "HSPLLICR (rw) register accessor: Interrupt Flag Clear Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hspllicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspllicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspllicr`] module"]
#[doc(alias = "HSPLLICR")]
pub type Hspllicr = crate::Reg<hspllicr::HspllicrSpec>;
#[doc = "Interrupt Flag Clear Register."]
pub mod hspllicr;
#[doc = "HSPLLISR (rw) register accessor: Interrupt Flag Set Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hspllisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspllisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspllisr`] module"]
#[doc(alias = "HSPLLISR")]
pub type Hspllisr = crate::Reg<hspllisr::HspllisrSpec>;
#[doc = "Interrupt Flag Set Register."]
pub mod hspllisr;
#[doc = "HSPLLDESCLO (rw) register accessor: HSPLL Descriptor Register L.\n\nYou can [`read`](crate::Reg::read) this register and get [`hsplldesclo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsplldesclo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsplldesclo`] module"]
#[doc(alias = "HSPLLDESCLO")]
pub type Hsplldesclo = crate::Reg<hsplldesclo::HsplldescloSpec>;
#[doc = "HSPLL Descriptor Register L."]
pub mod hsplldesclo;
#[doc = "HSPLLDESCHI (rw) register accessor: HSPLL Descriptor Register H.\n\nYou can [`read`](crate::Reg::read) this register and get [`hsplldeschi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsplldeschi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsplldeschi`] module"]
#[doc(alias = "HSPLLDESCHI")]
pub type Hsplldeschi = crate::Reg<hsplldeschi::HsplldeschiSpec>;
#[doc = "HSPLL Descriptor Register H."]
pub mod hsplldeschi;
#[doc = "HSPLLCTL (rw) register accessor: HSPLL Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hspllctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspllctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspllctl`] module"]
#[doc(alias = "HSPLLCTL")]
pub type Hspllctl = crate::Reg<hspllctl::HspllctlSpec>;
#[doc = "HSPLL Control Register"]
pub mod hspllctl;
#[doc = "HSPLLUSSXTLCTL (rw) register accessor: USSXT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hspllussxtlctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspllussxtlctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspllussxtlctl`] module"]
#[doc(alias = "HSPLLUSSXTLCTL")]
pub type Hspllussxtlctl = crate::Reg<hspllussxtlctl::HspllussxtlctlSpec>;
#[doc = "USSXT Control Register"]
pub mod hspllussxtlctl;
