#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    p7in: P7in,
    p8in: P8in,
    p7out: P7out,
    p8out: P8out,
    p7dir: P7dir,
    p8dir: P8dir,
    p7ren: P7ren,
    p8ren: P8ren,
    _reserved8: [u8; 0x02],
    p7sel0: P7sel0,
    p8sel0: P8sel0,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 7 Input"]
    #[inline(always)]
    pub const fn p7in(&self) -> &P7in {
        &self.p7in
    }
    #[doc = "0x01 - Port 8 Input"]
    #[inline(always)]
    pub const fn p8in(&self) -> &P8in {
        &self.p8in
    }
    #[doc = "0x02 - Port 7 Output"]
    #[inline(always)]
    pub const fn p7out(&self) -> &P7out {
        &self.p7out
    }
    #[doc = "0x03 - Port 8 Output"]
    #[inline(always)]
    pub const fn p8out(&self) -> &P8out {
        &self.p8out
    }
    #[doc = "0x04 - Port 7 Direction"]
    #[inline(always)]
    pub const fn p7dir(&self) -> &P7dir {
        &self.p7dir
    }
    #[doc = "0x05 - Port 8 Direction"]
    #[inline(always)]
    pub const fn p8dir(&self) -> &P8dir {
        &self.p8dir
    }
    #[doc = "0x06 - Port 7 Resistor Enable"]
    #[inline(always)]
    pub const fn p7ren(&self) -> &P7ren {
        &self.p7ren
    }
    #[doc = "0x07 - Port 8 Resistor Enable"]
    #[inline(always)]
    pub const fn p8ren(&self) -> &P8ren {
        &self.p8ren
    }
    #[doc = "0x0a - Port 7 Selection 0"]
    #[inline(always)]
    pub const fn p7sel0(&self) -> &P7sel0 {
        &self.p7sel0
    }
    #[doc = "0x0b - Port 8 Selection 0"]
    #[inline(always)]
    pub const fn p8sel0(&self) -> &P8sel0 {
        &self.p8sel0
    }
}
#[doc = "P7IN (rw) register accessor: Port 7 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p7in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7in`] module"]
#[doc(alias = "P7IN")]
pub type P7in = crate::Reg<p7in::P7inSpec>;
#[doc = "Port 7 Input"]
pub mod p7in;
#[doc = "P8IN (rw) register accessor: Port 8 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p8in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8in`] module"]
#[doc(alias = "P8IN")]
pub type P8in = crate::Reg<p8in::P8inSpec>;
#[doc = "Port 8 Input"]
pub mod p8in;
#[doc = "P7OUT (rw) register accessor: Port 7 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p7out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7out`] module"]
#[doc(alias = "P7OUT")]
pub type P7out = crate::Reg<p7out::P7outSpec>;
#[doc = "Port 7 Output"]
pub mod p7out;
#[doc = "P8OUT (rw) register accessor: Port 8 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p8out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8out`] module"]
#[doc(alias = "P8OUT")]
pub type P8out = crate::Reg<p8out::P8outSpec>;
#[doc = "Port 8 Output"]
pub mod p8out;
#[doc = "P7DIR (rw) register accessor: Port 7 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p7dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7dir`] module"]
#[doc(alias = "P7DIR")]
pub type P7dir = crate::Reg<p7dir::P7dirSpec>;
#[doc = "Port 7 Direction"]
pub mod p7dir;
#[doc = "P8DIR (rw) register accessor: Port 8 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p8dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8dir`] module"]
#[doc(alias = "P8DIR")]
pub type P8dir = crate::Reg<p8dir::P8dirSpec>;
#[doc = "Port 8 Direction"]
pub mod p8dir;
#[doc = "P7REN (rw) register accessor: Port 7 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p7ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7ren`] module"]
#[doc(alias = "P7REN")]
pub type P7ren = crate::Reg<p7ren::P7renSpec>;
#[doc = "Port 7 Resistor Enable"]
pub mod p7ren;
#[doc = "P8REN (rw) register accessor: Port 8 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p8ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8ren`] module"]
#[doc(alias = "P8REN")]
pub type P8ren = crate::Reg<p8ren::P8renSpec>;
#[doc = "Port 8 Resistor Enable"]
pub mod p8ren;
#[doc = "P7SEL0 (rw) register accessor: Port 7 Selection 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p7sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7sel0`] module"]
#[doc(alias = "P7SEL0")]
pub type P7sel0 = crate::Reg<p7sel0::P7sel0Spec>;
#[doc = "Port 7 Selection 0"]
pub mod p7sel0;
#[doc = "P8SEL0 (rw) register accessor: Port 8 Selection 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p8sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8sel0`] module"]
#[doc(alias = "P8SEL0")]
pub type P8sel0 = crate::Reg<p8sel0::P8sel0Spec>;
#[doc = "Port 8 Selection 0"]
pub mod p8sel0;
