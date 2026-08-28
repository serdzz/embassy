#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    taiv: Taiv,
    _reserved1: [u8; 0x30],
    tactl: Tactl,
    tacctl0: Tacctl0,
    tacctl1: Tacctl1,
    tacctl2: Tacctl2,
    _reserved5: [u8; 0x08],
    tar: Tar,
    taccr0: Taccr0,
    taccr1: Taccr1,
    taccr2: Taccr2,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer A Interrupt Vector Word"]
    #[inline(always)]
    pub const fn taiv(&self) -> &Taiv {
        &self.taiv
    }
    #[doc = "0x32 - Timer A Control"]
    #[inline(always)]
    pub const fn tactl(&self) -> &Tactl {
        &self.tactl
    }
    #[doc = "0x34 - Timer A Capture/Compare Control 0"]
    #[inline(always)]
    pub const fn tacctl0(&self) -> &Tacctl0 {
        &self.tacctl0
    }
    #[doc = "0x36 - Timer A Capture/Compare Control 1"]
    #[inline(always)]
    pub const fn tacctl1(&self) -> &Tacctl1 {
        &self.tacctl1
    }
    #[doc = "0x38 - Timer A Capture/Compare Control 2"]
    #[inline(always)]
    pub const fn tacctl2(&self) -> &Tacctl2 {
        &self.tacctl2
    }
    #[doc = "0x42 - Timer A Counter Register"]
    #[inline(always)]
    pub const fn tar(&self) -> &Tar {
        &self.tar
    }
    #[doc = "0x44 - Timer A Capture/Compare 0"]
    #[inline(always)]
    pub const fn taccr0(&self) -> &Taccr0 {
        &self.taccr0
    }
    #[doc = "0x46 - Timer A Capture/Compare 1"]
    #[inline(always)]
    pub const fn taccr1(&self) -> &Taccr1 {
        &self.taccr1
    }
    #[doc = "0x48 - Timer A Capture/Compare 2"]
    #[inline(always)]
    pub const fn taccr2(&self) -> &Taccr2 {
        &self.taccr2
    }
}
#[doc = "TAIV (rw) register accessor: Timer A Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`taiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`taiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taiv`] module"]
#[doc(alias = "TAIV")]
pub type Taiv = crate::Reg<taiv::TaivSpec>;
#[doc = "Timer A Interrupt Vector Word"]
pub mod taiv;
#[doc = "TACTL (rw) register accessor: Timer A Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tactl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tactl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tactl`] module"]
#[doc(alias = "TACTL")]
pub type Tactl = crate::Reg<tactl::TactlSpec>;
#[doc = "Timer A Control"]
pub mod tactl;
#[doc = "TACCTL0 (rw) register accessor: Timer A Capture/Compare Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tacctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tacctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tacctl0`] module"]
#[doc(alias = "TACCTL0")]
pub type Tacctl0 = crate::Reg<tacctl0::Tacctl0Spec>;
#[doc = "Timer A Capture/Compare Control 0"]
pub mod tacctl0;
#[doc = "TACCTL1 (rw) register accessor: Timer A Capture/Compare Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tacctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tacctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tacctl1`] module"]
#[doc(alias = "TACCTL1")]
pub type Tacctl1 = crate::Reg<tacctl1::Tacctl1Spec>;
#[doc = "Timer A Capture/Compare Control 1"]
pub mod tacctl1;
#[doc = "TACCTL2 (rw) register accessor: Timer A Capture/Compare Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tacctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tacctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tacctl2`] module"]
#[doc(alias = "TACCTL2")]
pub type Tacctl2 = crate::Reg<tacctl2::Tacctl2Spec>;
#[doc = "Timer A Capture/Compare Control 2"]
pub mod tacctl2;
#[doc = "TAR (rw) register accessor: Timer A Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tar`] module"]
#[doc(alias = "TAR")]
pub type Tar = crate::Reg<tar::TarSpec>;
#[doc = "Timer A Counter Register"]
pub mod tar;
#[doc = "TACCR0 (rw) register accessor: Timer A Capture/Compare 0\n\nYou can [`read`](crate::Reg::read) this register and get [`taccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`taccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taccr0`] module"]
#[doc(alias = "TACCR0")]
pub type Taccr0 = crate::Reg<taccr0::Taccr0Spec>;
#[doc = "Timer A Capture/Compare 0"]
pub mod taccr0;
#[doc = "TACCR1 (rw) register accessor: Timer A Capture/Compare 1\n\nYou can [`read`](crate::Reg::read) this register and get [`taccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`taccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taccr1`] module"]
#[doc(alias = "TACCR1")]
pub type Taccr1 = crate::Reg<taccr1::Taccr1Spec>;
#[doc = "Timer A Capture/Compare 1"]
pub mod taccr1;
#[doc = "TACCR2 (rw) register accessor: Timer A Capture/Compare 2\n\nYou can [`read`](crate::Reg::read) this register and get [`taccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`taccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taccr2`] module"]
#[doc(alias = "TACCR2")]
pub type Taccr2 = crate::Reg<taccr2::Taccr2Spec>;
#[doc = "Timer A Capture/Compare 2"]
pub mod taccr2;
