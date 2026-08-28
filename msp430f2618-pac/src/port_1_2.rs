#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    p1in: P1in,
    p1out: P1out,
    p1dir: P1dir,
    p1ifg: P1ifg,
    p1ies: P1ies,
    p1ie: P1ie,
    p1sel: P1sel,
    p1ren: P1ren,
    p2in: P2in,
    p2out: P2out,
    p2dir: P2dir,
    p2ifg: P2ifg,
    p2ies: P2ies,
    p2ie: P2ie,
    p2sel: P2sel,
    p2ren: P2ren,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 1 Input"]
    #[inline(always)]
    pub const fn p1in(&self) -> &P1in {
        &self.p1in
    }
    #[doc = "0x01 - Port 1 Output"]
    #[inline(always)]
    pub const fn p1out(&self) -> &P1out {
        &self.p1out
    }
    #[doc = "0x02 - Port 1 Direction"]
    #[inline(always)]
    pub const fn p1dir(&self) -> &P1dir {
        &self.p1dir
    }
    #[doc = "0x03 - Port 1 Interrupt Flag"]
    #[inline(always)]
    pub const fn p1ifg(&self) -> &P1ifg {
        &self.p1ifg
    }
    #[doc = "0x04 - Port 1 Interrupt Edge Select"]
    #[inline(always)]
    pub const fn p1ies(&self) -> &P1ies {
        &self.p1ies
    }
    #[doc = "0x05 - Port 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn p1ie(&self) -> &P1ie {
        &self.p1ie
    }
    #[doc = "0x06 - Port 1 Selection"]
    #[inline(always)]
    pub const fn p1sel(&self) -> &P1sel {
        &self.p1sel
    }
    #[doc = "0x07 - Port 1 Resistor Enable"]
    #[inline(always)]
    pub const fn p1ren(&self) -> &P1ren {
        &self.p1ren
    }
    #[doc = "0x08 - Port 2 Input"]
    #[inline(always)]
    pub const fn p2in(&self) -> &P2in {
        &self.p2in
    }
    #[doc = "0x09 - Port 2 Output"]
    #[inline(always)]
    pub const fn p2out(&self) -> &P2out {
        &self.p2out
    }
    #[doc = "0x0a - Port 2 Direction"]
    #[inline(always)]
    pub const fn p2dir(&self) -> &P2dir {
        &self.p2dir
    }
    #[doc = "0x0b - Port 2 Interrupt Flag"]
    #[inline(always)]
    pub const fn p2ifg(&self) -> &P2ifg {
        &self.p2ifg
    }
    #[doc = "0x0c - Port 2 Interrupt Edge Select"]
    #[inline(always)]
    pub const fn p2ies(&self) -> &P2ies {
        &self.p2ies
    }
    #[doc = "0x0d - Port 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn p2ie(&self) -> &P2ie {
        &self.p2ie
    }
    #[doc = "0x0e - Port 2 Selection"]
    #[inline(always)]
    pub const fn p2sel(&self) -> &P2sel {
        &self.p2sel
    }
    #[doc = "0x0f - Port 2 Resistor Enable"]
    #[inline(always)]
    pub const fn p2ren(&self) -> &P2ren {
        &self.p2ren
    }
}
#[doc = "P1IN (rw) register accessor: Port 1 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p1in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1in`] module"]
#[doc(alias = "P1IN")]
pub type P1in = crate::Reg<p1in::P1inSpec>;
#[doc = "Port 1 Input"]
pub mod p1in;
#[doc = "P1OUT (rw) register accessor: Port 1 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p1out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1out`] module"]
#[doc(alias = "P1OUT")]
pub type P1out = crate::Reg<p1out::P1outSpec>;
#[doc = "Port 1 Output"]
pub mod p1out;
#[doc = "P1DIR (rw) register accessor: Port 1 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p1dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1dir`] module"]
#[doc(alias = "P1DIR")]
pub type P1dir = crate::Reg<p1dir::P1dirSpec>;
#[doc = "Port 1 Direction"]
pub mod p1dir;
#[doc = "P1IFG (rw) register accessor: Port 1 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1ifg`] module"]
#[doc(alias = "P1IFG")]
pub type P1ifg = crate::Reg<p1ifg::P1ifgSpec>;
#[doc = "Port 1 Interrupt Flag"]
pub mod p1ifg;
#[doc = "P1IES (rw) register accessor: Port 1 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1ies`] module"]
#[doc(alias = "P1IES")]
pub type P1ies = crate::Reg<p1ies::P1iesSpec>;
#[doc = "Port 1 Interrupt Edge Select"]
pub mod p1ies;
#[doc = "P1IE (rw) register accessor: Port 1 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1ie`] module"]
#[doc(alias = "P1IE")]
pub type P1ie = crate::Reg<p1ie::P1ieSpec>;
#[doc = "Port 1 Interrupt Enable"]
pub mod p1ie;
#[doc = "P1SEL (rw) register accessor: Port 1 Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`p1sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1sel`] module"]
#[doc(alias = "P1SEL")]
pub type P1sel = crate::Reg<p1sel::P1selSpec>;
#[doc = "Port 1 Selection"]
pub mod p1sel;
#[doc = "P1REN (rw) register accessor: Port 1 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1ren`] module"]
#[doc(alias = "P1REN")]
pub type P1ren = crate::Reg<p1ren::P1renSpec>;
#[doc = "Port 1 Resistor Enable"]
pub mod p1ren;
#[doc = "P2IN (rw) register accessor: Port 2 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p2in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2in`] module"]
#[doc(alias = "P2IN")]
pub type P2in = crate::Reg<p2in::P2inSpec>;
#[doc = "Port 2 Input"]
pub mod p2in;
#[doc = "P2OUT (rw) register accessor: Port 2 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p2out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2out`] module"]
#[doc(alias = "P2OUT")]
pub type P2out = crate::Reg<p2out::P2outSpec>;
#[doc = "Port 2 Output"]
pub mod p2out;
#[doc = "P2DIR (rw) register accessor: Port 2 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p2dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2dir`] module"]
#[doc(alias = "P2DIR")]
pub type P2dir = crate::Reg<p2dir::P2dirSpec>;
#[doc = "Port 2 Direction"]
pub mod p2dir;
#[doc = "P2IFG (rw) register accessor: Port 2 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2ifg`] module"]
#[doc(alias = "P2IFG")]
pub type P2ifg = crate::Reg<p2ifg::P2ifgSpec>;
#[doc = "Port 2 Interrupt Flag"]
pub mod p2ifg;
#[doc = "P2IES (rw) register accessor: Port 2 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2ies`] module"]
#[doc(alias = "P2IES")]
pub type P2ies = crate::Reg<p2ies::P2iesSpec>;
#[doc = "Port 2 Interrupt Edge Select"]
pub mod p2ies;
#[doc = "P2IE (rw) register accessor: Port 2 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2ie`] module"]
#[doc(alias = "P2IE")]
pub type P2ie = crate::Reg<p2ie::P2ieSpec>;
#[doc = "Port 2 Interrupt Enable"]
pub mod p2ie;
#[doc = "P2SEL (rw) register accessor: Port 2 Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`p2sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2sel`] module"]
#[doc(alias = "P2SEL")]
pub type P2sel = crate::Reg<p2sel::P2selSpec>;
#[doc = "Port 2 Selection"]
pub mod p2sel;
#[doc = "P2REN (rw) register accessor: Port 2 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2ren`] module"]
#[doc(alias = "P2REN")]
pub type P2ren = crate::Reg<p2ren::P2renSpec>;
#[doc = "Port 2 Resistor Enable"]
pub mod p2ren;
