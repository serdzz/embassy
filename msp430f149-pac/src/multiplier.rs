#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mpy: Mpy,
    mpys: Mpys,
    mac: Mac,
    macs: Macs,
    op2: Op2,
    reslo: Reslo,
    reshi: Reshi,
    sumext: Sumext,
}
impl RegisterBlock {
    #[doc = "0x00 - Multiply Unsigned/Operand 1"]
    #[inline(always)]
    pub const fn mpy(&self) -> &Mpy {
        &self.mpy
    }
    #[doc = "0x02 - Multiply Signed/Operand 1"]
    #[inline(always)]
    pub const fn mpys(&self) -> &Mpys {
        &self.mpys
    }
    #[doc = "0x04 - Multiply Unsigned and Accumulate/Operand 1"]
    #[inline(always)]
    pub const fn mac(&self) -> &Mac {
        &self.mac
    }
    #[doc = "0x06 - Multiply Signed and Accumulate/Operand 1"]
    #[inline(always)]
    pub const fn macs(&self) -> &Macs {
        &self.macs
    }
    #[doc = "0x08 - Operand 2"]
    #[inline(always)]
    pub const fn op2(&self) -> &Op2 {
        &self.op2
    }
    #[doc = "0x0a - Result Low Word"]
    #[inline(always)]
    pub const fn reslo(&self) -> &Reslo {
        &self.reslo
    }
    #[doc = "0x0c - Result High Word"]
    #[inline(always)]
    pub const fn reshi(&self) -> &Reshi {
        &self.reshi
    }
    #[doc = "0x0e - Sum Extend"]
    #[inline(always)]
    pub const fn sumext(&self) -> &Sumext {
        &self.sumext
    }
}
#[doc = "MPY (rw) register accessor: Multiply Unsigned/Operand 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mpy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpy`] module"]
#[doc(alias = "MPY")]
pub type Mpy = crate::Reg<mpy::MpySpec>;
#[doc = "Multiply Unsigned/Operand 1"]
pub mod mpy;
#[doc = "MPYS (rw) register accessor: Multiply Signed/Operand 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mpys::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpys::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpys`] module"]
#[doc(alias = "MPYS")]
pub type Mpys = crate::Reg<mpys::MpysSpec>;
#[doc = "Multiply Signed/Operand 1"]
pub mod mpys;
#[doc = "MAC (rw) register accessor: Multiply Unsigned and Accumulate/Operand 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac`] module"]
#[doc(alias = "MAC")]
pub type Mac = crate::Reg<mac::MacSpec>;
#[doc = "Multiply Unsigned and Accumulate/Operand 1"]
pub mod mac;
#[doc = "MACS (rw) register accessor: Multiply Signed and Accumulate/Operand 1\n\nYou can [`read`](crate::Reg::read) this register and get [`macs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macs`] module"]
#[doc(alias = "MACS")]
pub type Macs = crate::Reg<macs::MacsSpec>;
#[doc = "Multiply Signed and Accumulate/Operand 1"]
pub mod macs;
#[doc = "OP2 (rw) register accessor: Operand 2\n\nYou can [`read`](crate::Reg::read) this register and get [`op2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`op2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op2`] module"]
#[doc(alias = "OP2")]
pub type Op2 = crate::Reg<op2::Op2Spec>;
#[doc = "Operand 2"]
pub mod op2;
#[doc = "RESLO (rw) register accessor: Result Low Word\n\nYou can [`read`](crate::Reg::read) this register and get [`reslo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reslo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reslo`] module"]
#[doc(alias = "RESLO")]
pub type Reslo = crate::Reg<reslo::ResloSpec>;
#[doc = "Result Low Word"]
pub mod reslo;
#[doc = "RESHI (rw) register accessor: Result High Word\n\nYou can [`read`](crate::Reg::read) this register and get [`reshi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reshi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reshi`] module"]
#[doc(alias = "RESHI")]
pub type Reshi = crate::Reg<reshi::ReshiSpec>;
#[doc = "Result High Word"]
pub mod reshi;
#[doc = "SUMEXT (rw) register accessor: Sum Extend\n\nYou can [`read`](crate::Reg::read) this register and get [`sumext::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sumext::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sumext`] module"]
#[doc(alias = "SUMEXT")]
pub type Sumext = crate::Reg<sumext::SumextSpec>;
#[doc = "Sum Extend"]
pub mod sumext;
