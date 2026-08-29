#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mpuctl0: Mpuctl0,
    mpuctl1: Mpuctl1,
    mpusegb2: Mpusegb2,
    mpusegb1: Mpusegb1,
    mpusam: Mpusam,
    mpuipc0: Mpuipc0,
    mpuipsegb2: Mpuipsegb2,
    mpuipsegb1: Mpuipsegb1,
}
impl RegisterBlock {
    #[doc = "0x00 - Memory Protection Unit Control 0"]
    #[inline(always)]
    pub const fn mpuctl0(&self) -> &Mpuctl0 {
        &self.mpuctl0
    }
    #[doc = "0x02 - Memory Protection Unit Control 1"]
    #[inline(always)]
    pub const fn mpuctl1(&self) -> &Mpuctl1 {
        &self.mpuctl1
    }
    #[doc = "0x04 - Memory Protection Unit Segmentation Border 2 Register"]
    #[inline(always)]
    pub const fn mpusegb2(&self) -> &Mpusegb2 {
        &self.mpusegb2
    }
    #[doc = "0x06 - Memory Protection Unit Segmentation Border 1 Register"]
    #[inline(always)]
    pub const fn mpusegb1(&self) -> &Mpusegb1 {
        &self.mpusegb1
    }
    #[doc = "0x08 - Memory Protection Unit Segmentation Access Management Register"]
    #[inline(always)]
    pub const fn mpusam(&self) -> &Mpusam {
        &self.mpusam
    }
    #[doc = "0x0a - Memory Protection Unit IP Control 0 Register"]
    #[inline(always)]
    pub const fn mpuipc0(&self) -> &Mpuipc0 {
        &self.mpuipc0
    }
    #[doc = "0x0c - Memory Protection Unit IP Encapsulation Segment Border 2 Register"]
    #[inline(always)]
    pub const fn mpuipsegb2(&self) -> &Mpuipsegb2 {
        &self.mpuipsegb2
    }
    #[doc = "0x0e - Memory Protection Unit IP Encapsulation Segment Border 1 Register"]
    #[inline(always)]
    pub const fn mpuipsegb1(&self) -> &Mpuipsegb1 {
        &self.mpuipsegb1
    }
}
#[doc = "MPUCTL0 (rw) register accessor: Memory Protection Unit Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mpuctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpuctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpuctl0`] module"]
#[doc(alias = "MPUCTL0")]
pub type Mpuctl0 = crate::Reg<mpuctl0::Mpuctl0Spec>;
#[doc = "Memory Protection Unit Control 0"]
pub mod mpuctl0;
#[doc = "MPUCTL1 (rw) register accessor: Memory Protection Unit Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mpuctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpuctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpuctl1`] module"]
#[doc(alias = "MPUCTL1")]
pub type Mpuctl1 = crate::Reg<mpuctl1::Mpuctl1Spec>;
#[doc = "Memory Protection Unit Control 1"]
pub mod mpuctl1;
#[doc = "MPUSEGB2 (rw) register accessor: Memory Protection Unit Segmentation Border 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpusegb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpusegb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpusegb2`] module"]
#[doc(alias = "MPUSEGB2")]
pub type Mpusegb2 = crate::Reg<mpusegb2::Mpusegb2Spec>;
#[doc = "Memory Protection Unit Segmentation Border 2 Register"]
pub mod mpusegb2;
#[doc = "MPUSEGB1 (rw) register accessor: Memory Protection Unit Segmentation Border 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpusegb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpusegb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpusegb1`] module"]
#[doc(alias = "MPUSEGB1")]
pub type Mpusegb1 = crate::Reg<mpusegb1::Mpusegb1Spec>;
#[doc = "Memory Protection Unit Segmentation Border 1 Register"]
pub mod mpusegb1;
#[doc = "MPUSAM (rw) register accessor: Memory Protection Unit Segmentation Access Management Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpusam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpusam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpusam`] module"]
#[doc(alias = "MPUSAM")]
pub type Mpusam = crate::Reg<mpusam::MpusamSpec>;
#[doc = "Memory Protection Unit Segmentation Access Management Register"]
pub mod mpusam;
#[doc = "MPUIPC0 (rw) register accessor: Memory Protection Unit IP Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpuipc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpuipc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpuipc0`] module"]
#[doc(alias = "MPUIPC0")]
pub type Mpuipc0 = crate::Reg<mpuipc0::Mpuipc0Spec>;
#[doc = "Memory Protection Unit IP Control 0 Register"]
pub mod mpuipc0;
#[doc = "MPUIPSEGB2 (rw) register accessor: Memory Protection Unit IP Encapsulation Segment Border 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpuipsegb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpuipsegb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpuipsegb2`] module"]
#[doc(alias = "MPUIPSEGB2")]
pub type Mpuipsegb2 = crate::Reg<mpuipsegb2::Mpuipsegb2Spec>;
#[doc = "Memory Protection Unit IP Encapsulation Segment Border 2 Register"]
pub mod mpuipsegb2;
#[doc = "MPUIPSEGB1 (rw) register accessor: Memory Protection Unit IP Encapsulation Segment Border 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpuipsegb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpuipsegb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpuipsegb1`] module"]
#[doc(alias = "MPUIPSEGB1")]
pub type Mpuipsegb1 = crate::Reg<mpuipsegb1::Mpuipsegb1Spec>;
#[doc = "Memory Protection Unit IP Encapsulation Segment Border 1 Register"]
pub mod mpuipsegb1;
