#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ie1: Ie1,
    ie2: Ie2,
    ifg1: Ifg1,
    ifg2: Ifg2,
    _reserved4: [u8; 0x02],
    uc1ie: Uc1ie,
    uc1ifg: Uc1ifg,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Enable 1"]
    #[inline(always)]
    pub const fn ie1(&self) -> &Ie1 {
        &self.ie1
    }
    #[doc = "0x01 - Interrupt Enable 2"]
    #[inline(always)]
    pub const fn ie2(&self) -> &Ie2 {
        &self.ie2
    }
    #[doc = "0x02 - Interrupt Flag 1"]
    #[inline(always)]
    pub const fn ifg1(&self) -> &Ifg1 {
        &self.ifg1
    }
    #[doc = "0x03 - Interrupt Flag 2"]
    #[inline(always)]
    pub const fn ifg2(&self) -> &Ifg2 {
        &self.ifg2
    }
    #[doc = "0x06 - USCI 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn uc1ie(&self) -> &Uc1ie {
        &self.uc1ie
    }
    #[doc = "0x07 - ISCI 1 Interrupt Flags"]
    #[inline(always)]
    pub const fn uc1ifg(&self) -> &Uc1ifg {
        &self.uc1ifg
    }
}
#[doc = "IE1 (rw) register accessor: Interrupt Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ie1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie1`] module"]
#[doc(alias = "IE1")]
pub type Ie1 = crate::Reg<ie1::Ie1Spec>;
#[doc = "Interrupt Enable 1"]
pub mod ie1;
#[doc = "IE2 (rw) register accessor: Interrupt Enable 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ie2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie2`] module"]
#[doc(alias = "IE2")]
pub type Ie2 = crate::Reg<ie2::Ie2Spec>;
#[doc = "Interrupt Enable 2"]
pub mod ie2;
#[doc = "IFG1 (rw) register accessor: Interrupt Flag 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ifg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifg1`] module"]
#[doc(alias = "IFG1")]
pub type Ifg1 = crate::Reg<ifg1::Ifg1Spec>;
#[doc = "Interrupt Flag 1"]
pub mod ifg1;
#[doc = "IFG2 (rw) register accessor: Interrupt Flag 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ifg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifg2`] module"]
#[doc(alias = "IFG2")]
pub type Ifg2 = crate::Reg<ifg2::Ifg2Spec>;
#[doc = "Interrupt Flag 2"]
pub mod ifg2;
#[doc = "UC1IE (rw) register accessor: USCI 1 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`uc1ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uc1ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uc1ie`] module"]
#[doc(alias = "UC1IE")]
pub type Uc1ie = crate::Reg<uc1ie::Uc1ieSpec>;
#[doc = "USCI 1 Interrupt Enable"]
pub mod uc1ie;
#[doc = "UC1IFG (rw) register accessor: ISCI 1 Interrupt Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`uc1ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uc1ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uc1ifg`] module"]
#[doc(alias = "UC1IFG")]
pub type Uc1ifg = crate::Reg<uc1ifg::Uc1ifgSpec>;
#[doc = "ISCI 1 Interrupt Flags"]
pub mod uc1ifg;
