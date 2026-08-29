#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    p5in: P5in,
    _reserved1: [u8; 0x01],
    p5out: P5out,
    _reserved2: [u8; 0x01],
    p5dir: P5dir,
    _reserved3: [u8; 0x01],
    p5ren: P5ren,
    _reserved4: [u8; 0x03],
    p5sel0: P5sel0,
    _reserved5: [u8; 0x01],
    p5sel1: P5sel1,
    _reserved6: [u8; 0x01],
    p5iv: P5iv,
    _reserved7: [u8; 0x06],
    p5selc: P5selc,
    _reserved8: [u8; 0x01],
    p5ies: P5ies,
    _reserved9: [u8; 0x01],
    p5ie: P5ie,
    _reserved10: [u8; 0x01],
    p5ifg: P5ifg,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 5 Input"]
    #[inline(always)]
    pub const fn p5in(&self) -> &P5in {
        &self.p5in
    }
    #[doc = "0x02 - Port 5 Output"]
    #[inline(always)]
    pub const fn p5out(&self) -> &P5out {
        &self.p5out
    }
    #[doc = "0x04 - Port 5 Direction"]
    #[inline(always)]
    pub const fn p5dir(&self) -> &P5dir {
        &self.p5dir
    }
    #[doc = "0x06 - Port 5 Resistor Enable"]
    #[inline(always)]
    pub const fn p5ren(&self) -> &P5ren {
        &self.p5ren
    }
    #[doc = "0x0a - Port 5 Select 0"]
    #[inline(always)]
    pub const fn p5sel0(&self) -> &P5sel0 {
        &self.p5sel0
    }
    #[doc = "0x0c - Port 5 Select 1"]
    #[inline(always)]
    pub const fn p5sel1(&self) -> &P5sel1 {
        &self.p5sel1
    }
    #[doc = "0x0e - Port 5 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p5iv(&self) -> &P5iv {
        &self.p5iv
    }
    #[doc = "0x16 - Port 5 Complement Select"]
    #[inline(always)]
    pub const fn p5selc(&self) -> &P5selc {
        &self.p5selc
    }
    #[doc = "0x18 - Port 5 Interrupt Edge Select"]
    #[inline(always)]
    pub const fn p5ies(&self) -> &P5ies {
        &self.p5ies
    }
    #[doc = "0x1a - Port 5 Interrupt Enable"]
    #[inline(always)]
    pub const fn p5ie(&self) -> &P5ie {
        &self.p5ie
    }
    #[doc = "0x1c - Port 5 Interrupt Flag"]
    #[inline(always)]
    pub const fn p5ifg(&self) -> &P5ifg {
        &self.p5ifg
    }
}
#[doc = "P5IN (rw) register accessor: Port 5 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p5in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5in`] module"]
#[doc(alias = "P5IN")]
pub type P5in = crate::Reg<p5in::P5inSpec>;
#[doc = "Port 5 Input"]
pub mod p5in;
#[doc = "P5OUT (rw) register accessor: Port 5 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p5out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5out`] module"]
#[doc(alias = "P5OUT")]
pub type P5out = crate::Reg<p5out::P5outSpec>;
#[doc = "Port 5 Output"]
pub mod p5out;
#[doc = "P5DIR (rw) register accessor: Port 5 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p5dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5dir`] module"]
#[doc(alias = "P5DIR")]
pub type P5dir = crate::Reg<p5dir::P5dirSpec>;
#[doc = "Port 5 Direction"]
pub mod p5dir;
#[doc = "P5REN (rw) register accessor: Port 5 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p5ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5ren`] module"]
#[doc(alias = "P5REN")]
pub type P5ren = crate::Reg<p5ren::P5renSpec>;
#[doc = "Port 5 Resistor Enable"]
pub mod p5ren;
#[doc = "P5SEL0 (rw) register accessor: Port 5 Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p5sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5sel0`] module"]
#[doc(alias = "P5SEL0")]
pub type P5sel0 = crate::Reg<p5sel0::P5sel0Spec>;
#[doc = "Port 5 Select 0"]
pub mod p5sel0;
#[doc = "P5SEL1 (rw) register accessor: Port 5 Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`p5sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5sel1`] module"]
#[doc(alias = "P5SEL1")]
pub type P5sel1 = crate::Reg<p5sel1::P5sel1Spec>;
#[doc = "Port 5 Select 1"]
pub mod p5sel1;
#[doc = "P5SELC (rw) register accessor: Port 5 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p5selc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5selc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5selc`] module"]
#[doc(alias = "P5SELC")]
pub type P5selc = crate::Reg<p5selc::P5selcSpec>;
#[doc = "Port 5 Complement Select"]
pub mod p5selc;
#[doc = "P5IES (rw) register accessor: Port 5 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p5ies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5ies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5ies`] module"]
#[doc(alias = "P5IES")]
pub type P5ies = crate::Reg<p5ies::P5iesSpec>;
#[doc = "Port 5 Interrupt Edge Select"]
pub mod p5ies;
#[doc = "P5IE (rw) register accessor: Port 5 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p5ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5ie`] module"]
#[doc(alias = "P5IE")]
pub type P5ie = crate::Reg<p5ie::P5ieSpec>;
#[doc = "Port 5 Interrupt Enable"]
pub mod p5ie;
#[doc = "P5IFG (rw) register accessor: Port 5 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p5ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5ifg`] module"]
#[doc(alias = "P5IFG")]
pub type P5ifg = crate::Reg<p5ifg::P5ifgSpec>;
#[doc = "Port 5 Interrupt Flag"]
pub mod p5ifg;
#[doc = "P5IV (rw) register accessor: Port 5 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p5iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5iv`] module"]
#[doc(alias = "P5IV")]
pub type P5iv = crate::Reg<p5iv::P5ivSpec>;
#[doc = "Port 5 Interrupt Vector Register"]
pub mod p5iv;
