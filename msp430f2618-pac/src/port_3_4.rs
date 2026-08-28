#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    p3ren: P3ren,
    p4ren: P4ren,
    _reserved2: [u8; 0x06],
    p3in: P3in,
    p3out: P3out,
    p3dir: P3dir,
    p3sel: P3sel,
    p4in: P4in,
    p4out: P4out,
    p4dir: P4dir,
    p4sel: P4sel,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 3 Resistor Enable"]
    #[inline(always)]
    pub const fn p3ren(&self) -> &P3ren {
        &self.p3ren
    }
    #[doc = "0x01 - Port 4 Resistor Enable"]
    #[inline(always)]
    pub const fn p4ren(&self) -> &P4ren {
        &self.p4ren
    }
    #[doc = "0x08 - Port 3 Input"]
    #[inline(always)]
    pub const fn p3in(&self) -> &P3in {
        &self.p3in
    }
    #[doc = "0x09 - Port 3 Output"]
    #[inline(always)]
    pub const fn p3out(&self) -> &P3out {
        &self.p3out
    }
    #[doc = "0x0a - Port 3 Direction"]
    #[inline(always)]
    pub const fn p3dir(&self) -> &P3dir {
        &self.p3dir
    }
    #[doc = "0x0b - Port 3 Selection"]
    #[inline(always)]
    pub const fn p3sel(&self) -> &P3sel {
        &self.p3sel
    }
    #[doc = "0x0c - Port 4 Input"]
    #[inline(always)]
    pub const fn p4in(&self) -> &P4in {
        &self.p4in
    }
    #[doc = "0x0d - Port 4 Output"]
    #[inline(always)]
    pub const fn p4out(&self) -> &P4out {
        &self.p4out
    }
    #[doc = "0x0e - Port 4 Direction"]
    #[inline(always)]
    pub const fn p4dir(&self) -> &P4dir {
        &self.p4dir
    }
    #[doc = "0x0f - Port 4 Selection"]
    #[inline(always)]
    pub const fn p4sel(&self) -> &P4sel {
        &self.p4sel
    }
}
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
#[doc = "P3IN (rw) register accessor: Port 3 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p3in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3in`] module"]
#[doc(alias = "P3IN")]
pub type P3in = crate::Reg<p3in::P3inSpec>;
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "P3OUT (rw) register accessor: Port 3 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p3out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3out`] module"]
#[doc(alias = "P3OUT")]
pub type P3out = crate::Reg<p3out::P3outSpec>;
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "P3DIR (rw) register accessor: Port 3 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p3dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3dir`] module"]
#[doc(alias = "P3DIR")]
pub type P3dir = crate::Reg<p3dir::P3dirSpec>;
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "P3SEL (rw) register accessor: Port 3 Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`p3sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3sel`] module"]
#[doc(alias = "P3SEL")]
pub type P3sel = crate::Reg<p3sel::P3selSpec>;
#[doc = "Port 3 Selection"]
pub mod p3sel;
#[doc = "P4IN (rw) register accessor: Port 4 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p4in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4in`] module"]
#[doc(alias = "P4IN")]
pub type P4in = crate::Reg<p4in::P4inSpec>;
#[doc = "Port 4 Input"]
pub mod p4in;
#[doc = "P4OUT (rw) register accessor: Port 4 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p4out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4out`] module"]
#[doc(alias = "P4OUT")]
pub type P4out = crate::Reg<p4out::P4outSpec>;
#[doc = "Port 4 Output"]
pub mod p4out;
#[doc = "P4DIR (rw) register accessor: Port 4 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p4dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4dir`] module"]
#[doc(alias = "P4DIR")]
pub type P4dir = crate::Reg<p4dir::P4dirSpec>;
#[doc = "Port 4 Direction"]
pub mod p4dir;
#[doc = "P4SEL (rw) register accessor: Port 4 Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`p4sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4sel`] module"]
#[doc(alias = "P4SEL")]
pub type P4sel = crate::Reg<p4sel::P4selSpec>;
#[doc = "Port 4 Selection"]
pub mod p4sel;
