#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    p8in: P8in,
    _reserved1: [u8; 0x01],
    p8out: P8out,
    _reserved2: [u8; 0x01],
    p8dir: P8dir,
    _reserved3: [u8; 0x01],
    p8ren: P8ren,
    _reserved4: [u8; 0x03],
    p8sel0: P8sel0,
    _reserved5: [u8; 0x01],
    p8sel1: P8sel1,
    _reserved6: [u8; 0x09],
    p8selc: P8selc,
    _reserved7: [u8; 0x01],
    p8ies: P8ies,
    _reserved8: [u8; 0x01],
    p8ie: P8ie,
    _reserved9: [u8; 0x01],
    p8ifg: P8ifg,
    p8iv: P8iv,
}
impl RegisterBlock {
    #[doc = "0x01 - Port 8 Input"]
    #[inline(always)]
    pub const fn p8in(&self) -> &P8in {
        &self.p8in
    }
    #[doc = "0x03 - Port 8 Output"]
    #[inline(always)]
    pub const fn p8out(&self) -> &P8out {
        &self.p8out
    }
    #[doc = "0x05 - Port 8 Direction"]
    #[inline(always)]
    pub const fn p8dir(&self) -> &P8dir {
        &self.p8dir
    }
    #[doc = "0x07 - Port 8 Resistor Enable"]
    #[inline(always)]
    pub const fn p8ren(&self) -> &P8ren {
        &self.p8ren
    }
    #[doc = "0x0b - Port 8 Select 0"]
    #[inline(always)]
    pub const fn p8sel0(&self) -> &P8sel0 {
        &self.p8sel0
    }
    #[doc = "0x0d - Port 8 Select 1"]
    #[inline(always)]
    pub const fn p8sel1(&self) -> &P8sel1 {
        &self.p8sel1
    }
    #[doc = "0x17 - Port 8 Complement Select"]
    #[inline(always)]
    pub const fn p8selc(&self) -> &P8selc {
        &self.p8selc
    }
    #[doc = "0x19 - Port 8 Interrupt Edge Select"]
    #[inline(always)]
    pub const fn p8ies(&self) -> &P8ies {
        &self.p8ies
    }
    #[doc = "0x1b - Port 8 Interrupt Enable"]
    #[inline(always)]
    pub const fn p8ie(&self) -> &P8ie {
        &self.p8ie
    }
    #[doc = "0x1d - Port 8 Interrupt Flag"]
    #[inline(always)]
    pub const fn p8ifg(&self) -> &P8ifg {
        &self.p8ifg
    }
    #[doc = "0x1e - Port 8 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p8iv(&self) -> &P8iv {
        &self.p8iv
    }
}
#[doc = "P8IN (rw) register accessor: Port 8 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p8in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8in`] module"]
#[doc(alias = "P8IN")]
pub type P8in = crate::Reg<p8in::P8inSpec>;
#[doc = "Port 8 Input"]
pub mod p8in;
#[doc = "P8OUT (rw) register accessor: Port 8 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p8out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8out`] module"]
#[doc(alias = "P8OUT")]
pub type P8out = crate::Reg<p8out::P8outSpec>;
#[doc = "Port 8 Output"]
pub mod p8out;
#[doc = "P8DIR (rw) register accessor: Port 8 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p8dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8dir`] module"]
#[doc(alias = "P8DIR")]
pub type P8dir = crate::Reg<p8dir::P8dirSpec>;
#[doc = "Port 8 Direction"]
pub mod p8dir;
#[doc = "P8REN (rw) register accessor: Port 8 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p8ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8ren`] module"]
#[doc(alias = "P8REN")]
pub type P8ren = crate::Reg<p8ren::P8renSpec>;
#[doc = "Port 8 Resistor Enable"]
pub mod p8ren;
#[doc = "P8SEL0 (rw) register accessor: Port 8 Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p8sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8sel0`] module"]
#[doc(alias = "P8SEL0")]
pub type P8sel0 = crate::Reg<p8sel0::P8sel0Spec>;
#[doc = "Port 8 Select 0"]
pub mod p8sel0;
#[doc = "P8SEL1 (rw) register accessor: Port 8 Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`p8sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8sel1`] module"]
#[doc(alias = "P8SEL1")]
pub type P8sel1 = crate::Reg<p8sel1::P8sel1Spec>;
#[doc = "Port 8 Select 1"]
pub mod p8sel1;
#[doc = "P8SELC (rw) register accessor: Port 8 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p8selc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8selc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8selc`] module"]
#[doc(alias = "P8SELC")]
pub type P8selc = crate::Reg<p8selc::P8selcSpec>;
#[doc = "Port 8 Complement Select"]
pub mod p8selc;
#[doc = "P8IES (rw) register accessor: Port 8 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p8ies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8ies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8ies`] module"]
#[doc(alias = "P8IES")]
pub type P8ies = crate::Reg<p8ies::P8iesSpec>;
#[doc = "Port 8 Interrupt Edge Select"]
pub mod p8ies;
#[doc = "P8IE (rw) register accessor: Port 8 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p8ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8ie`] module"]
#[doc(alias = "P8IE")]
pub type P8ie = crate::Reg<p8ie::P8ieSpec>;
#[doc = "Port 8 Interrupt Enable"]
pub mod p8ie;
#[doc = "P8IFG (rw) register accessor: Port 8 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p8ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8ifg`] module"]
#[doc(alias = "P8IFG")]
pub type P8ifg = crate::Reg<p8ifg::P8ifgSpec>;
#[doc = "Port 8 Interrupt Flag"]
pub mod p8ifg;
#[doc = "P8IV (rw) register accessor: Port 8 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p8iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8iv`] module"]
#[doc(alias = "P8IV")]
pub type P8iv = crate::Reg<p8iv::P8ivSpec>;
#[doc = "Port 8 Interrupt Vector Register"]
pub mod p8iv;
