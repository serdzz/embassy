#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    p9in: P9in,
    _reserved1: [u8; 0x01],
    p9out: P9out,
    _reserved2: [u8; 0x01],
    p9dir: P9dir,
    _reserved3: [u8; 0x01],
    p9ren: P9ren,
    _reserved4: [u8; 0x03],
    p9sel0: P9sel0,
    _reserved5: [u8; 0x01],
    p9sel1: P9sel1,
    _reserved6: [u8; 0x01],
    p9iv: P9iv,
    _reserved7: [u8; 0x06],
    p9selc: P9selc,
    _reserved8: [u8; 0x01],
    p9ies: P9ies,
    _reserved9: [u8; 0x01],
    p9ie: P9ie,
    _reserved10: [u8; 0x01],
    p9ifg: P9ifg,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 9 Input"]
    #[inline(always)]
    pub const fn p9in(&self) -> &P9in {
        &self.p9in
    }
    #[doc = "0x02 - Port 9 Output"]
    #[inline(always)]
    pub const fn p9out(&self) -> &P9out {
        &self.p9out
    }
    #[doc = "0x04 - Port 9 Direction"]
    #[inline(always)]
    pub const fn p9dir(&self) -> &P9dir {
        &self.p9dir
    }
    #[doc = "0x06 - Port 9 Resistor Enable"]
    #[inline(always)]
    pub const fn p9ren(&self) -> &P9ren {
        &self.p9ren
    }
    #[doc = "0x0a - Port 9 Select 0"]
    #[inline(always)]
    pub const fn p9sel0(&self) -> &P9sel0 {
        &self.p9sel0
    }
    #[doc = "0x0c - Port 9 Select 1"]
    #[inline(always)]
    pub const fn p9sel1(&self) -> &P9sel1 {
        &self.p9sel1
    }
    #[doc = "0x0e - Port 9 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p9iv(&self) -> &P9iv {
        &self.p9iv
    }
    #[doc = "0x16 - Port 9 Complement Select"]
    #[inline(always)]
    pub const fn p9selc(&self) -> &P9selc {
        &self.p9selc
    }
    #[doc = "0x18 - Port 9 Interrupt Edge Select"]
    #[inline(always)]
    pub const fn p9ies(&self) -> &P9ies {
        &self.p9ies
    }
    #[doc = "0x1a - Port 9 Interrupt Enable"]
    #[inline(always)]
    pub const fn p9ie(&self) -> &P9ie {
        &self.p9ie
    }
    #[doc = "0x1c - Port 9 Interrupt Flag"]
    #[inline(always)]
    pub const fn p9ifg(&self) -> &P9ifg {
        &self.p9ifg
    }
}
#[doc = "P9IN (rw) register accessor: Port 9 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p9in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p9in`] module"]
#[doc(alias = "P9IN")]
pub type P9in = crate::Reg<p9in::P9inSpec>;
#[doc = "Port 9 Input"]
pub mod p9in;
#[doc = "P9OUT (rw) register accessor: Port 9 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p9out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p9out`] module"]
#[doc(alias = "P9OUT")]
pub type P9out = crate::Reg<p9out::P9outSpec>;
#[doc = "Port 9 Output"]
pub mod p9out;
#[doc = "P9DIR (rw) register accessor: Port 9 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p9dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p9dir`] module"]
#[doc(alias = "P9DIR")]
pub type P9dir = crate::Reg<p9dir::P9dirSpec>;
#[doc = "Port 9 Direction"]
pub mod p9dir;
#[doc = "P9REN (rw) register accessor: Port 9 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p9ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p9ren`] module"]
#[doc(alias = "P9REN")]
pub type P9ren = crate::Reg<p9ren::P9renSpec>;
#[doc = "Port 9 Resistor Enable"]
pub mod p9ren;
#[doc = "P9SEL0 (rw) register accessor: Port 9 Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p9sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p9sel0`] module"]
#[doc(alias = "P9SEL0")]
pub type P9sel0 = crate::Reg<p9sel0::P9sel0Spec>;
#[doc = "Port 9 Select 0"]
pub mod p9sel0;
#[doc = "P9SEL1 (rw) register accessor: Port 9 Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`p9sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p9sel1`] module"]
#[doc(alias = "P9SEL1")]
pub type P9sel1 = crate::Reg<p9sel1::P9sel1Spec>;
#[doc = "Port 9 Select 1"]
pub mod p9sel1;
#[doc = "P9SELC (rw) register accessor: Port 9 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p9selc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9selc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p9selc`] module"]
#[doc(alias = "P9SELC")]
pub type P9selc = crate::Reg<p9selc::P9selcSpec>;
#[doc = "Port 9 Complement Select"]
pub mod p9selc;
#[doc = "P9IES (rw) register accessor: Port 9 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p9ies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9ies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p9ies`] module"]
#[doc(alias = "P9IES")]
pub type P9ies = crate::Reg<p9ies::P9iesSpec>;
#[doc = "Port 9 Interrupt Edge Select"]
pub mod p9ies;
#[doc = "P9IE (rw) register accessor: Port 9 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p9ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p9ie`] module"]
#[doc(alias = "P9IE")]
pub type P9ie = crate::Reg<p9ie::P9ieSpec>;
#[doc = "Port 9 Interrupt Enable"]
pub mod p9ie;
#[doc = "P9IFG (rw) register accessor: Port 9 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p9ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p9ifg`] module"]
#[doc(alias = "P9IFG")]
pub type P9ifg = crate::Reg<p9ifg::P9ifgSpec>;
#[doc = "Port 9 Interrupt Flag"]
pub mod p9ifg;
#[doc = "P9IV (rw) register accessor: Port 9 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p9iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p9iv`] module"]
#[doc(alias = "P9IV")]
pub type P9iv = crate::Reg<p9iv::P9ivSpec>;
#[doc = "Port 9 Interrupt Vector Register"]
pub mod p9iv;
