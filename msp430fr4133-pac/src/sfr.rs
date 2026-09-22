#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfrie1: Sfrie1,
    sfrifg1: Sfrifg1,
    sfrrpcr: Sfrrpcr,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Enable 1"]
    #[inline(always)]
    pub const fn sfrie1(&self) -> &Sfrie1 {
        &self.sfrie1
    }
    #[doc = "0x02 - Interrupt Flag 1"]
    #[inline(always)]
    pub const fn sfrifg1(&self) -> &Sfrifg1 {
        &self.sfrifg1
    }
    #[doc = "0x04 - RESET Pin Control Register"]
    #[inline(always)]
    pub const fn sfrrpcr(&self) -> &Sfrrpcr {
        &self.sfrrpcr
    }
}
#[doc = "SFRIE1 (rw) register accessor: Interrupt Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrie1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrie1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfrie1`] module"]
#[doc(alias = "SFRIE1")]
pub type Sfrie1 = crate::Reg<sfrie1::Sfrie1Spec>;
#[doc = "Interrupt Enable 1"]
pub mod sfrie1;
#[doc = "SFRIFG1 (rw) register accessor: Interrupt Flag 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrifg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrifg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfrifg1`] module"]
#[doc(alias = "SFRIFG1")]
pub type Sfrifg1 = crate::Reg<sfrifg1::Sfrifg1Spec>;
#[doc = "Interrupt Flag 1"]
pub mod sfrifg1;
#[doc = "SFRRPCR (rw) register accessor: RESET Pin Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrrpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrrpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfrrpcr`] module"]
#[doc(alias = "SFRRPCR")]
pub type Sfrrpcr = crate::Reg<sfrrpcr::SfrrpcrSpec>;
#[doc = "RESET Pin Control Register"]
pub mod sfrrpcr;
