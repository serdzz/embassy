#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    captio1ctl: Captio1ctl,
}
impl RegisterBlock {
    #[doc = "0x00 - Capacitive Touch IO 0 Control Register"]
    #[inline(always)]
    pub const fn captio1ctl(&self) -> &Captio1ctl {
        &self.captio1ctl
    }
}
#[doc = "CAPTIO1CTL (rw) register accessor: Capacitive Touch IO 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`captio1ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`captio1ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@captio1ctl`] module"]
#[doc(alias = "CAPTIO1CTL")]
pub type Captio1ctl = crate::Reg<captio1ctl::Captio1ctlSpec>;
#[doc = "Capacitive Touch IO 0 Control Register"]
pub mod captio1ctl;
