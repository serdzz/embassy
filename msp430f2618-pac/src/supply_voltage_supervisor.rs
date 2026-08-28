#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    svsctl: Svsctl,
}
impl RegisterBlock {
    #[doc = "0x01 - SVS Control"]
    #[inline(always)]
    pub const fn svsctl(&self) -> &Svsctl {
        &self.svsctl
    }
}
#[doc = "SVSCTL (rw) register accessor: SVS Control\n\nYou can [`read`](crate::Reg::read) this register and get [`svsctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svsctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@svsctl`] module"]
#[doc(alias = "SVSCTL")]
pub type Svsctl = crate::Reg<svsctl::SvsctlSpec>;
#[doc = "SVS Control"]
pub mod svsctl;
