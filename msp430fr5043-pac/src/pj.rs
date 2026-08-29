#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pjin: Pjin,
    pjout: Pjout,
    pjdir: Pjdir,
    pjren: Pjren,
    _reserved4: [u8; 0x02],
    pjsel0: Pjsel0,
    pjsel1: Pjsel1,
    _reserved6: [u8; 0x08],
    pjselc: Pjselc,
}
impl RegisterBlock {
    #[doc = "0x00 - Port J Input"]
    #[inline(always)]
    pub const fn pjin(&self) -> &Pjin {
        &self.pjin
    }
    #[doc = "0x02 - Port J Output"]
    #[inline(always)]
    pub const fn pjout(&self) -> &Pjout {
        &self.pjout
    }
    #[doc = "0x04 - Port J Direction"]
    #[inline(always)]
    pub const fn pjdir(&self) -> &Pjdir {
        &self.pjdir
    }
    #[doc = "0x06 - Port J Resistor Enable"]
    #[inline(always)]
    pub const fn pjren(&self) -> &Pjren {
        &self.pjren
    }
    #[doc = "0x0a - Port J Select 0"]
    #[inline(always)]
    pub const fn pjsel0(&self) -> &Pjsel0 {
        &self.pjsel0
    }
    #[doc = "0x0c - Port J Select 1"]
    #[inline(always)]
    pub const fn pjsel1(&self) -> &Pjsel1 {
        &self.pjsel1
    }
    #[doc = "0x16 - Port J Complement Select"]
    #[inline(always)]
    pub const fn pjselc(&self) -> &Pjselc {
        &self.pjselc
    }
}
#[doc = "PJIN (rw) register accessor: Port J Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pjin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjin`] module"]
#[doc(alias = "PJIN")]
pub type Pjin = crate::Reg<pjin::PjinSpec>;
#[doc = "Port J Input"]
pub mod pjin;
#[doc = "PJOUT (rw) register accessor: Port J Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pjout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjout`] module"]
#[doc(alias = "PJOUT")]
pub type Pjout = crate::Reg<pjout::PjoutSpec>;
#[doc = "Port J Output"]
pub mod pjout;
#[doc = "PJDIR (rw) register accessor: Port J Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pjdir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjdir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjdir`] module"]
#[doc(alias = "PJDIR")]
pub type Pjdir = crate::Reg<pjdir::PjdirSpec>;
#[doc = "Port J Direction"]
pub mod pjdir;
#[doc = "PJREN (rw) register accessor: Port J Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pjren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjren`] module"]
#[doc(alias = "PJREN")]
pub type Pjren = crate::Reg<pjren::PjrenSpec>;
#[doc = "Port J Resistor Enable"]
pub mod pjren;
#[doc = "PJSEL0 (rw) register accessor: Port J Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pjsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjsel0`] module"]
#[doc(alias = "PJSEL0")]
pub type Pjsel0 = crate::Reg<pjsel0::Pjsel0Spec>;
#[doc = "Port J Select 0"]
pub mod pjsel0;
#[doc = "PJSEL1 (rw) register accessor: Port J Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pjsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjsel1`] module"]
#[doc(alias = "PJSEL1")]
pub type Pjsel1 = crate::Reg<pjsel1::Pjsel1Spec>;
#[doc = "Port J Select 1"]
pub mod pjsel1;
#[doc = "PJSELC (rw) register accessor: Port J Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pjselc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjselc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjselc`] module"]
#[doc(alias = "PJSELC")]
pub type Pjselc = crate::Reg<pjselc::PjselcSpec>;
#[doc = "Port J Complement Select"]
pub mod pjselc;
