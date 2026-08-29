#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mtifpgcnf: Mtifpgcnf,
    mtifpgkval: Mtifpgkval,
    mtifpgctl: Mtifpgctl,
    mtifpgsr: Mtifpgsr,
    mtifpccnf: Mtifpccnf,
    mtifpcr: Mtifpcr,
    mtifpcctl: Mtifpcctl,
    mtifpcsr: Mtifpcsr,
    mtiftpctl: Mtiftpctl,
}
impl RegisterBlock {
    #[doc = "0x00 - Pulse Generator Configuration Register"]
    #[inline(always)]
    pub const fn mtifpgcnf(&self) -> &Mtifpgcnf {
        &self.mtifpgcnf
    }
    #[doc = "0x02 - Pulse Generator Value Register"]
    #[inline(always)]
    pub const fn mtifpgkval(&self) -> &Mtifpgkval {
        &self.mtifpgkval
    }
    #[doc = "0x04 - Pulse Generator Control Register"]
    #[inline(always)]
    pub const fn mtifpgctl(&self) -> &Mtifpgctl {
        &self.mtifpgctl
    }
    #[doc = "0x06 - Pulse Generator Status Register"]
    #[inline(always)]
    pub const fn mtifpgsr(&self) -> &Mtifpgsr {
        &self.mtifpgsr
    }
    #[doc = "0x08 - Pulse Counter Configuration Register"]
    #[inline(always)]
    pub const fn mtifpccnf(&self) -> &Mtifpccnf {
        &self.mtifpccnf
    }
    #[doc = "0x0a - Pulse Counter Value Register"]
    #[inline(always)]
    pub const fn mtifpcr(&self) -> &Mtifpcr {
        &self.mtifpcr
    }
    #[doc = "0x0c - Pulse Counter Control Register"]
    #[inline(always)]
    pub const fn mtifpcctl(&self) -> &Mtifpcctl {
        &self.mtifpcctl
    }
    #[doc = "0x0e - Pulse Counter Status Register"]
    #[inline(always)]
    pub const fn mtifpcsr(&self) -> &Mtifpcsr {
        &self.mtifpcsr
    }
    #[doc = "0x10 - Measurement Test Port Control Register"]
    #[inline(always)]
    pub const fn mtiftpctl(&self) -> &Mtiftpctl {
        &self.mtiftpctl
    }
}
#[doc = "MTIFPGCNF (rw) register accessor: Pulse Generator Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpgcnf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpgcnf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtifpgcnf`] module"]
#[doc(alias = "MTIFPGCNF")]
pub type Mtifpgcnf = crate::Reg<mtifpgcnf::MtifpgcnfSpec>;
#[doc = "Pulse Generator Configuration Register"]
pub mod mtifpgcnf;
#[doc = "MTIFPGKVAL (rw) register accessor: Pulse Generator Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpgkval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpgkval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtifpgkval`] module"]
#[doc(alias = "MTIFPGKVAL")]
pub type Mtifpgkval = crate::Reg<mtifpgkval::MtifpgkvalSpec>;
#[doc = "Pulse Generator Value Register"]
pub mod mtifpgkval;
#[doc = "MTIFPGCTL (rw) register accessor: Pulse Generator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtifpgctl`] module"]
#[doc(alias = "MTIFPGCTL")]
pub type Mtifpgctl = crate::Reg<mtifpgctl::MtifpgctlSpec>;
#[doc = "Pulse Generator Control Register"]
pub mod mtifpgctl;
#[doc = "MTIFPGSR (rw) register accessor: Pulse Generator Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpgsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpgsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtifpgsr`] module"]
#[doc(alias = "MTIFPGSR")]
pub type Mtifpgsr = crate::Reg<mtifpgsr::MtifpgsrSpec>;
#[doc = "Pulse Generator Status Register"]
pub mod mtifpgsr;
#[doc = "MTIFPCCNF (rw) register accessor: Pulse Counter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpccnf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpccnf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtifpccnf`] module"]
#[doc(alias = "MTIFPCCNF")]
pub type Mtifpccnf = crate::Reg<mtifpccnf::MtifpccnfSpec>;
#[doc = "Pulse Counter Configuration Register"]
pub mod mtifpccnf;
#[doc = "MTIFPCR (rw) register accessor: Pulse Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtifpcr`] module"]
#[doc(alias = "MTIFPCR")]
pub type Mtifpcr = crate::Reg<mtifpcr::MtifpcrSpec>;
#[doc = "Pulse Counter Value Register"]
pub mod mtifpcr;
#[doc = "MTIFPCCTL (rw) register accessor: Pulse Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpcctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpcctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtifpcctl`] module"]
#[doc(alias = "MTIFPCCTL")]
pub type Mtifpcctl = crate::Reg<mtifpcctl::MtifpcctlSpec>;
#[doc = "Pulse Counter Control Register"]
pub mod mtifpcctl;
#[doc = "MTIFPCSR (rw) register accessor: Pulse Counter Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtifpcsr`] module"]
#[doc(alias = "MTIFPCSR")]
pub type Mtifpcsr = crate::Reg<mtifpcsr::MtifpcsrSpec>;
#[doc = "Pulse Counter Status Register"]
pub mod mtifpcsr;
#[doc = "MTIFTPCTL (rw) register accessor: Measurement Test Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtiftpctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtiftpctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtiftpctl`] module"]
#[doc(alias = "MTIFTPCTL")]
pub type Mtiftpctl = crate::Reg<mtiftpctl::MtiftpctlSpec>;
#[doc = "Measurement Test Port Control Register"]
pub mod mtiftpctl;
