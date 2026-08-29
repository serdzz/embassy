#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    aesactl0: Aesactl0,
    aesactl1: Aesactl1,
    aesastat: Aesastat,
    aesakey: Aesakey,
    aesadin: Aesadin,
    aesadout: Aesadout,
    aesaxdin: Aesaxdin,
    aesaxin: Aesaxin,
}
impl RegisterBlock {
    #[doc = "0x00 - AES Accelerator Control Register 0"]
    #[inline(always)]
    pub const fn aesactl0(&self) -> &Aesactl0 {
        &self.aesactl0
    }
    #[doc = "0x02 - AES Accelerator Control Register 1"]
    #[inline(always)]
    pub const fn aesactl1(&self) -> &Aesactl1 {
        &self.aesactl1
    }
    #[doc = "0x04 - AES Accelerator Status Register"]
    #[inline(always)]
    pub const fn aesastat(&self) -> &Aesastat {
        &self.aesastat
    }
    #[doc = "0x06 - AES Accelerator Key Register"]
    #[inline(always)]
    pub const fn aesakey(&self) -> &Aesakey {
        &self.aesakey
    }
    #[doc = "0x08 - AES Accelerator Data In Register"]
    #[inline(always)]
    pub const fn aesadin(&self) -> &Aesadin {
        &self.aesadin
    }
    #[doc = "0x0a - AES Accelerator Data Out Register"]
    #[inline(always)]
    pub const fn aesadout(&self) -> &Aesadout {
        &self.aesadout
    }
    #[doc = "0x0c - AES Accelerator XORed Data In Register"]
    #[inline(always)]
    pub const fn aesaxdin(&self) -> &Aesaxdin {
        &self.aesaxdin
    }
    #[doc = "0x0e - AES Accelerator XORed Data In Register"]
    #[inline(always)]
    pub const fn aesaxin(&self) -> &Aesaxin {
        &self.aesaxin
    }
}
#[doc = "AESACTL0 (rw) register accessor: AES Accelerator Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`aesactl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesactl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesactl0`] module"]
#[doc(alias = "AESACTL0")]
pub type Aesactl0 = crate::Reg<aesactl0::Aesactl0Spec>;
#[doc = "AES Accelerator Control Register 0"]
pub mod aesactl0;
#[doc = "AESACTL1 (rw) register accessor: AES Accelerator Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`aesactl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesactl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesactl1`] module"]
#[doc(alias = "AESACTL1")]
pub type Aesactl1 = crate::Reg<aesactl1::Aesactl1Spec>;
#[doc = "AES Accelerator Control Register 1"]
pub mod aesactl1;
#[doc = "AESASTAT (rw) register accessor: AES Accelerator Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesastat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesastat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesastat`] module"]
#[doc(alias = "AESASTAT")]
pub type Aesastat = crate::Reg<aesastat::AesastatSpec>;
#[doc = "AES Accelerator Status Register"]
pub mod aesastat;
#[doc = "AESAKEY (rw) register accessor: AES Accelerator Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesakey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesakey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesakey`] module"]
#[doc(alias = "AESAKEY")]
pub type Aesakey = crate::Reg<aesakey::AesakeySpec>;
#[doc = "AES Accelerator Key Register"]
pub mod aesakey;
#[doc = "AESADIN (rw) register accessor: AES Accelerator Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadin`] module"]
#[doc(alias = "AESADIN")]
pub type Aesadin = crate::Reg<aesadin::AesadinSpec>;
#[doc = "AES Accelerator Data In Register"]
pub mod aesadin;
#[doc = "AESADOUT (rw) register accessor: AES Accelerator Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadout`] module"]
#[doc(alias = "AESADOUT")]
pub type Aesadout = crate::Reg<aesadout::AesadoutSpec>;
#[doc = "AES Accelerator Data Out Register"]
pub mod aesadout;
#[doc = "AESAXDIN (rw) register accessor: AES Accelerator XORed Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesaxdin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesaxdin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesaxdin`] module"]
#[doc(alias = "AESAXDIN")]
pub type Aesaxdin = crate::Reg<aesaxdin::AesaxdinSpec>;
#[doc = "AES Accelerator XORed Data In Register"]
pub mod aesaxdin;
#[doc = "AESAXIN (rw) register accessor: AES Accelerator XORed Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesaxin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesaxin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesaxin`] module"]
#[doc(alias = "AESAXIN")]
pub type Aesaxin = crate::Reg<aesaxin::AesaxinSpec>;
#[doc = "AES Accelerator XORed Data In Register"]
pub mod aesaxin;
