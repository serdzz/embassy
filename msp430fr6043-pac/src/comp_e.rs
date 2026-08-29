#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cectl0: Cectl0,
    cectl1: Cectl1,
    cectl2: Cectl2,
    cectl3: Cectl3,
    _reserved4: [u8; 0x04],
    ceint: Ceint,
    ceiv: Ceiv,
}
impl RegisterBlock {
    #[doc = "0x00 - Comparator Control Register 0"]
    #[inline(always)]
    pub const fn cectl0(&self) -> &Cectl0 {
        &self.cectl0
    }
    #[doc = "0x02 - Comparator Control Register 1"]
    #[inline(always)]
    pub const fn cectl1(&self) -> &Cectl1 {
        &self.cectl1
    }
    #[doc = "0x04 - Comparator Control Register 2"]
    #[inline(always)]
    pub const fn cectl2(&self) -> &Cectl2 {
        &self.cectl2
    }
    #[doc = "0x06 - Comparator Control Register 3"]
    #[inline(always)]
    pub const fn cectl3(&self) -> &Cectl3 {
        &self.cectl3
    }
    #[doc = "0x0c - Comparator Interrupt Control Register"]
    #[inline(always)]
    pub const fn ceint(&self) -> &Ceint {
        &self.ceint
    }
    #[doc = "0x0e - Comparator Interrupt Vector Word Register"]
    #[inline(always)]
    pub const fn ceiv(&self) -> &Ceiv {
        &self.ceiv
    }
}
#[doc = "CECTL0 (rw) register accessor: Comparator Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cectl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cectl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cectl0`] module"]
#[doc(alias = "CECTL0")]
pub type Cectl0 = crate::Reg<cectl0::Cectl0Spec>;
#[doc = "Comparator Control Register 0"]
pub mod cectl0;
#[doc = "CECTL1 (rw) register accessor: Comparator Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cectl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cectl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cectl1`] module"]
#[doc(alias = "CECTL1")]
pub type Cectl1 = crate::Reg<cectl1::Cectl1Spec>;
#[doc = "Comparator Control Register 1"]
pub mod cectl1;
#[doc = "CECTL2 (rw) register accessor: Comparator Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cectl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cectl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cectl2`] module"]
#[doc(alias = "CECTL2")]
pub type Cectl2 = crate::Reg<cectl2::Cectl2Spec>;
#[doc = "Comparator Control Register 2"]
pub mod cectl2;
#[doc = "CECTL3 (rw) register accessor: Comparator Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cectl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cectl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cectl3`] module"]
#[doc(alias = "CECTL3")]
pub type Cectl3 = crate::Reg<cectl3::Cectl3Spec>;
#[doc = "Comparator Control Register 3"]
pub mod cectl3;
#[doc = "CEINT (rw) register accessor: Comparator Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ceint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ceint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ceint`] module"]
#[doc(alias = "CEINT")]
pub type Ceint = crate::Reg<ceint::CeintSpec>;
#[doc = "Comparator Interrupt Control Register"]
pub mod ceint;
#[doc = "CEIV (rw) register accessor: Comparator Interrupt Vector Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ceiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ceiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ceiv`] module"]
#[doc(alias = "CEIV")]
pub type Ceiv = crate::Reg<ceiv::CeivSpec>;
#[doc = "Comparator Interrupt Vector Word Register"]
pub mod ceiv;
