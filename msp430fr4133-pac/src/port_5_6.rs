#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    p5in: P5in,
    p6in: P6in,
    p5out: P5out,
    p6out: P6out,
    p5dir: P5dir,
    p6dir: P6dir,
    p5ren: P5ren,
    p6ren: P6ren,
    _reserved8: [u8; 0x02],
    p5sel0: P5sel0,
    p6sel0: P6sel0,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 5 Input"]
    #[inline(always)]
    pub const fn p5in(&self) -> &P5in {
        &self.p5in
    }
    #[doc = "0x01 - Port 6 Input"]
    #[inline(always)]
    pub const fn p6in(&self) -> &P6in {
        &self.p6in
    }
    #[doc = "0x02 - Port 5 Output"]
    #[inline(always)]
    pub const fn p5out(&self) -> &P5out {
        &self.p5out
    }
    #[doc = "0x03 - Port 6 Output"]
    #[inline(always)]
    pub const fn p6out(&self) -> &P6out {
        &self.p6out
    }
    #[doc = "0x04 - Port 5 Direction"]
    #[inline(always)]
    pub const fn p5dir(&self) -> &P5dir {
        &self.p5dir
    }
    #[doc = "0x05 - Port 6 Direction"]
    #[inline(always)]
    pub const fn p6dir(&self) -> &P6dir {
        &self.p6dir
    }
    #[doc = "0x06 - Port 5 Resistor Enable"]
    #[inline(always)]
    pub const fn p5ren(&self) -> &P5ren {
        &self.p5ren
    }
    #[doc = "0x07 - Port 6 Resistor Enable"]
    #[inline(always)]
    pub const fn p6ren(&self) -> &P6ren {
        &self.p6ren
    }
    #[doc = "0x0a - Port 5 Selection 0"]
    #[inline(always)]
    pub const fn p5sel0(&self) -> &P5sel0 {
        &self.p5sel0
    }
    #[doc = "0x0b - Port 6 Selection 0"]
    #[inline(always)]
    pub const fn p6sel0(&self) -> &P6sel0 {
        &self.p6sel0
    }
}
#[doc = "P5IN (rw) register accessor: Port 5 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p5in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5in`] module"]
#[doc(alias = "P5IN")]
pub type P5in = crate::Reg<p5in::P5inSpec>;
#[doc = "Port 5 Input"]
pub mod p5in;
#[doc = "P6IN (rw) register accessor: Port 6 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p6in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6in`] module"]
#[doc(alias = "P6IN")]
pub type P6in = crate::Reg<p6in::P6inSpec>;
#[doc = "Port 6 Input"]
pub mod p6in;
#[doc = "P5OUT (rw) register accessor: Port 5 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p5out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5out`] module"]
#[doc(alias = "P5OUT")]
pub type P5out = crate::Reg<p5out::P5outSpec>;
#[doc = "Port 5 Output"]
pub mod p5out;
#[doc = "P6OUT (rw) register accessor: Port 6 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p6out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6out`] module"]
#[doc(alias = "P6OUT")]
pub type P6out = crate::Reg<p6out::P6outSpec>;
#[doc = "Port 6 Output"]
pub mod p6out;
#[doc = "P5DIR (rw) register accessor: Port 5 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p5dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5dir`] module"]
#[doc(alias = "P5DIR")]
pub type P5dir = crate::Reg<p5dir::P5dirSpec>;
#[doc = "Port 5 Direction"]
pub mod p5dir;
#[doc = "P6DIR (rw) register accessor: Port 6 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p6dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6dir`] module"]
#[doc(alias = "P6DIR")]
pub type P6dir = crate::Reg<p6dir::P6dirSpec>;
#[doc = "Port 6 Direction"]
pub mod p6dir;
#[doc = "P5REN (rw) register accessor: Port 5 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p5ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5ren`] module"]
#[doc(alias = "P5REN")]
pub type P5ren = crate::Reg<p5ren::P5renSpec>;
#[doc = "Port 5 Resistor Enable"]
pub mod p5ren;
#[doc = "P6REN (rw) register accessor: Port 6 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p6ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6ren`] module"]
#[doc(alias = "P6REN")]
pub type P6ren = crate::Reg<p6ren::P6renSpec>;
#[doc = "Port 6 Resistor Enable"]
pub mod p6ren;
#[doc = "P5SEL0 (rw) register accessor: Port 5 Selection 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p5sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5sel0`] module"]
#[doc(alias = "P5SEL0")]
pub type P5sel0 = crate::Reg<p5sel0::P5sel0Spec>;
#[doc = "Port 5 Selection 0"]
pub mod p5sel0;
#[doc = "P6SEL0 (rw) register accessor: Port 6 Selection 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p6sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6sel0`] module"]
#[doc(alias = "P6SEL0")]
pub type P6sel0 = crate::Reg<p6sel0::P6sel0Spec>;
#[doc = "Port 6 Selection 0"]
pub mod p6sel0;
