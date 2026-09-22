#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    p3in: P3in,
    p4in: P4in,
    p3out: P3out,
    p4out: P4out,
    p3dir: P3dir,
    p4dir: P4dir,
    p3ren: P3ren,
    p4ren: P4ren,
    _reserved8: [u8; 0x02],
    p3sel0: P3sel0,
    p4sel0: P4sel0,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 3 Input"]
    #[inline(always)]
    pub const fn p3in(&self) -> &P3in {
        &self.p3in
    }
    #[doc = "0x01 - Port 4 Input"]
    #[inline(always)]
    pub const fn p4in(&self) -> &P4in {
        &self.p4in
    }
    #[doc = "0x02 - Port 3 Output"]
    #[inline(always)]
    pub const fn p3out(&self) -> &P3out {
        &self.p3out
    }
    #[doc = "0x03 - Port 4 Output"]
    #[inline(always)]
    pub const fn p4out(&self) -> &P4out {
        &self.p4out
    }
    #[doc = "0x04 - Port 3 Direction"]
    #[inline(always)]
    pub const fn p3dir(&self) -> &P3dir {
        &self.p3dir
    }
    #[doc = "0x05 - Port 4 Direction"]
    #[inline(always)]
    pub const fn p4dir(&self) -> &P4dir {
        &self.p4dir
    }
    #[doc = "0x06 - Port 3 Resistor Enable"]
    #[inline(always)]
    pub const fn p3ren(&self) -> &P3ren {
        &self.p3ren
    }
    #[doc = "0x07 - Port 4 Resistor Enable"]
    #[inline(always)]
    pub const fn p4ren(&self) -> &P4ren {
        &self.p4ren
    }
    #[doc = "0x0a - Port 3 Selection 0"]
    #[inline(always)]
    pub const fn p3sel0(&self) -> &P3sel0 {
        &self.p3sel0
    }
    #[doc = "0x0b - Port 4 Selection 0"]
    #[inline(always)]
    pub const fn p4sel0(&self) -> &P4sel0 {
        &self.p4sel0
    }
}
#[doc = "P3IN (rw) register accessor: Port 3 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p3in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3in`] module"]
#[doc(alias = "P3IN")]
pub type P3in = crate::Reg<p3in::P3inSpec>;
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "P4IN (rw) register accessor: Port 4 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p4in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4in`] module"]
#[doc(alias = "P4IN")]
pub type P4in = crate::Reg<p4in::P4inSpec>;
#[doc = "Port 4 Input"]
pub mod p4in;
#[doc = "P3OUT (rw) register accessor: Port 3 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p3out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3out`] module"]
#[doc(alias = "P3OUT")]
pub type P3out = crate::Reg<p3out::P3outSpec>;
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "P4OUT (rw) register accessor: Port 4 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p4out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4out`] module"]
#[doc(alias = "P4OUT")]
pub type P4out = crate::Reg<p4out::P4outSpec>;
#[doc = "Port 4 Output"]
pub mod p4out;
#[doc = "P3DIR (rw) register accessor: Port 3 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p3dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3dir`] module"]
#[doc(alias = "P3DIR")]
pub type P3dir = crate::Reg<p3dir::P3dirSpec>;
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "P4DIR (rw) register accessor: Port 4 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p4dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4dir`] module"]
#[doc(alias = "P4DIR")]
pub type P4dir = crate::Reg<p4dir::P4dirSpec>;
#[doc = "Port 4 Direction"]
pub mod p4dir;
#[doc = "P3REN (rw) register accessor: Port 3 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p3ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3ren`] module"]
#[doc(alias = "P3REN")]
pub type P3ren = crate::Reg<p3ren::P3renSpec>;
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "P4REN (rw) register accessor: Port 4 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p4ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4ren`] module"]
#[doc(alias = "P4REN")]
pub type P4ren = crate::Reg<p4ren::P4renSpec>;
#[doc = "Port 4 Resistor Enable"]
pub mod p4ren;
#[doc = "P3SEL0 (rw) register accessor: Port 3 Selection 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p3sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3sel0`] module"]
#[doc(alias = "P3SEL0")]
pub type P3sel0 = crate::Reg<p3sel0::P3sel0Spec>;
#[doc = "Port 3 Selection 0"]
pub mod p3sel0;
#[doc = "P4SEL0 (rw) register accessor: Port 4 Selection 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p4sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4sel0`] module"]
#[doc(alias = "P4SEL0")]
pub type P4sel0 = crate::Reg<p4sel0::P4sel0Spec>;
#[doc = "Port 4 Selection 0"]
pub mod p4sel0;
