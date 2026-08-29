#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    p7in: P7in,
    _reserved1: [u8; 0x01],
    p7out: P7out,
    _reserved2: [u8; 0x01],
    p7dir: P7dir,
    _reserved3: [u8; 0x01],
    p7ren: P7ren,
    _reserved4: [u8; 0x03],
    p7sel0: P7sel0,
    _reserved5: [u8; 0x01],
    p7sel1: P7sel1,
    _reserved6: [u8; 0x01],
    p7iv: P7iv,
    _reserved7: [u8; 0x06],
    p7selc: P7selc,
    _reserved8: [u8; 0x01],
    p7ies: P7ies,
    _reserved9: [u8; 0x01],
    p7ie: P7ie,
    _reserved10: [u8; 0x01],
    p7ifg: P7ifg,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 7 Input"]
    #[inline(always)]
    pub const fn p7in(&self) -> &P7in {
        &self.p7in
    }
    #[doc = "0x02 - Port 7 Output"]
    #[inline(always)]
    pub const fn p7out(&self) -> &P7out {
        &self.p7out
    }
    #[doc = "0x04 - Port 7 Direction"]
    #[inline(always)]
    pub const fn p7dir(&self) -> &P7dir {
        &self.p7dir
    }
    #[doc = "0x06 - Port 7 Resistor Enable"]
    #[inline(always)]
    pub const fn p7ren(&self) -> &P7ren {
        &self.p7ren
    }
    #[doc = "0x0a - Port 7 Select 0"]
    #[inline(always)]
    pub const fn p7sel0(&self) -> &P7sel0 {
        &self.p7sel0
    }
    #[doc = "0x0c - Port 7 Select 1"]
    #[inline(always)]
    pub const fn p7sel1(&self) -> &P7sel1 {
        &self.p7sel1
    }
    #[doc = "0x0e - Port 7 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p7iv(&self) -> &P7iv {
        &self.p7iv
    }
    #[doc = "0x16 - Port 7 Complement Select"]
    #[inline(always)]
    pub const fn p7selc(&self) -> &P7selc {
        &self.p7selc
    }
    #[doc = "0x18 - Port 7 Interrupt Edge Select"]
    #[inline(always)]
    pub const fn p7ies(&self) -> &P7ies {
        &self.p7ies
    }
    #[doc = "0x1a - Port 7 Interrupt Enable"]
    #[inline(always)]
    pub const fn p7ie(&self) -> &P7ie {
        &self.p7ie
    }
    #[doc = "0x1c - Port 7 Interrupt Flag"]
    #[inline(always)]
    pub const fn p7ifg(&self) -> &P7ifg {
        &self.p7ifg
    }
}
#[doc = "P7IN (rw) register accessor: Port 7 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p7in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7in`] module"]
#[doc(alias = "P7IN")]
pub type P7in = crate::Reg<p7in::P7inSpec>;
#[doc = "Port 7 Input"]
pub mod p7in;
#[doc = "P7OUT (rw) register accessor: Port 7 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p7out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7out`] module"]
#[doc(alias = "P7OUT")]
pub type P7out = crate::Reg<p7out::P7outSpec>;
#[doc = "Port 7 Output"]
pub mod p7out;
#[doc = "P7DIR (rw) register accessor: Port 7 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p7dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7dir`] module"]
#[doc(alias = "P7DIR")]
pub type P7dir = crate::Reg<p7dir::P7dirSpec>;
#[doc = "Port 7 Direction"]
pub mod p7dir;
#[doc = "P7REN (rw) register accessor: Port 7 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p7ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7ren`] module"]
#[doc(alias = "P7REN")]
pub type P7ren = crate::Reg<p7ren::P7renSpec>;
#[doc = "Port 7 Resistor Enable"]
pub mod p7ren;
#[doc = "P7SEL0 (rw) register accessor: Port 7 Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p7sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7sel0`] module"]
#[doc(alias = "P7SEL0")]
pub type P7sel0 = crate::Reg<p7sel0::P7sel0Spec>;
#[doc = "Port 7 Select 0"]
pub mod p7sel0;
#[doc = "P7SEL1 (rw) register accessor: Port 7 Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`p7sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7sel1`] module"]
#[doc(alias = "P7SEL1")]
pub type P7sel1 = crate::Reg<p7sel1::P7sel1Spec>;
#[doc = "Port 7 Select 1"]
pub mod p7sel1;
#[doc = "P7SELC (rw) register accessor: Port 7 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p7selc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7selc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7selc`] module"]
#[doc(alias = "P7SELC")]
pub type P7selc = crate::Reg<p7selc::P7selcSpec>;
#[doc = "Port 7 Complement Select"]
pub mod p7selc;
#[doc = "P7IES (rw) register accessor: Port 7 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p7ies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7ies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7ies`] module"]
#[doc(alias = "P7IES")]
pub type P7ies = crate::Reg<p7ies::P7iesSpec>;
#[doc = "Port 7 Interrupt Edge Select"]
pub mod p7ies;
#[doc = "P7IE (rw) register accessor: Port 7 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p7ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7ie`] module"]
#[doc(alias = "P7IE")]
pub type P7ie = crate::Reg<p7ie::P7ieSpec>;
#[doc = "Port 7 Interrupt Enable"]
pub mod p7ie;
#[doc = "P7IFG (rw) register accessor: Port 7 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p7ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7ifg`] module"]
#[doc(alias = "P7IFG")]
pub type P7ifg = crate::Reg<p7ifg::P7ifgSpec>;
#[doc = "Port 7 Interrupt Flag"]
pub mod p7ifg;
#[doc = "P7IV (rw) register accessor: Port 7 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p7iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7iv`] module"]
#[doc(alias = "P7IV")]
pub type P7iv = crate::Reg<p7iv::P7ivSpec>;
#[doc = "Port 7 Interrupt Vector Register"]
pub mod p7iv;
