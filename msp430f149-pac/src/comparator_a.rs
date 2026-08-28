#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    cactl1: Cactl1,
    cactl2: Cactl2,
    capd: Capd,
}
impl RegisterBlock {
    #[doc = "0x01 - Comparator A Control 1"]
    #[inline(always)]
    pub const fn cactl1(&self) -> &Cactl1 {
        &self.cactl1
    }
    #[doc = "0x02 - Comparator A Control 2"]
    #[inline(always)]
    pub const fn cactl2(&self) -> &Cactl2 {
        &self.cactl2
    }
    #[doc = "0x03 - Comparator A Port Disable"]
    #[inline(always)]
    pub const fn capd(&self) -> &Capd {
        &self.capd
    }
}
#[doc = "CACTL1 (rw) register accessor: Comparator A Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cactl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cactl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cactl1`] module"]
#[doc(alias = "CACTL1")]
pub type Cactl1 = crate::Reg<cactl1::Cactl1Spec>;
#[doc = "Comparator A Control 1"]
pub mod cactl1;
#[doc = "CACTL2 (rw) register accessor: Comparator A Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cactl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cactl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cactl2`] module"]
#[doc(alias = "CACTL2")]
pub type Cactl2 = crate::Reg<cactl2::Cactl2Spec>;
#[doc = "Comparator A Control 2"]
pub mod cactl2;
#[doc = "CAPD (rw) register accessor: Comparator A Port Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`capd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capd`] module"]
#[doc(alias = "CAPD")]
pub type Capd = crate::Reg<capd::CapdSpec>;
#[doc = "Comparator A Port Disable"]
pub mod capd;
