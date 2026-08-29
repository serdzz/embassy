#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rcctl0: Rcctl0,
    rcctl1: Rcctl1,
}
impl RegisterBlock {
    #[doc = "0x00 - RAM Controller Control 0"]
    #[inline(always)]
    pub const fn rcctl0(&self) -> &Rcctl0 {
        &self.rcctl0
    }
    #[doc = "0x02 - RAM Controller Control 1"]
    #[inline(always)]
    pub const fn rcctl1(&self) -> &Rcctl1 {
        &self.rcctl1
    }
}
#[doc = "RCCTL0 (rw) register accessor: RAM Controller Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rcctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcctl0`] module"]
#[doc(alias = "RCCTL0")]
pub type Rcctl0 = crate::Reg<rcctl0::Rcctl0Spec>;
#[doc = "RAM Controller Control 0"]
pub mod rcctl0;
#[doc = "RCCTL1 (rw) register accessor: RAM Controller Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rcctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcctl1`] module"]
#[doc(alias = "RCCTL1")]
pub type Rcctl1 = crate::Reg<rcctl1::Rcctl1Spec>;
#[doc = "RAM Controller Control 1"]
pub mod rcctl1;
