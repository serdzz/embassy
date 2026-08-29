#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    refctl0: Refctl0,
}
impl RegisterBlock {
    #[doc = "0x00 - REF Control Register 0"]
    #[inline(always)]
    pub const fn refctl0(&self) -> &Refctl0 {
        &self.refctl0
    }
}
#[doc = "REFCTL0 (rw) register accessor: REF Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`refctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refctl0`] module"]
#[doc(alias = "REFCTL0")]
pub type Refctl0 = crate::Reg<refctl0::Refctl0Spec>;
#[doc = "REF Control Register 0"]
pub mod refctl0;
