#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lcdctl0: Lcdctl0,
    lcdctl1: Lcdctl1,
    lcdblkctl: Lcdblkctl,
    lcdmemctl: Lcdmemctl,
    lcdvctl: Lcdvctl,
    lcdpctl0: Lcdpctl0,
    lcdpctl1: Lcdpctl1,
    lcdpctl2: Lcdpctl2,
    _reserved8: [u8; 0x04],
    lcdcssel0: Lcdcssel0,
    lcdcssel1: Lcdcssel1,
    lcdcssel2: Lcdcssel2,
    _reserved11: [u8; 0x04],
    lcdiv: Lcdiv,
    lcdm0w: Lcdm0w,
    lcdm2w: Lcdm2w,
    lcdm4w: Lcdm4w,
    lcdm6w: Lcdm6w,
    lcdm8w: Lcdm8w,
    lcdm10w: Lcdm10w,
    lcdm12w: Lcdm12w,
    lcdm14w: Lcdm14w,
    lcdm16w: Lcdm16w,
    lcdm18w: Lcdm18w,
    lcdm20w: Lcdm20w,
    lcdm22w: Lcdm22w,
    lcdm24w: Lcdm24w,
    lcdm26w: Lcdm26w,
    lcdm28w: Lcdm28w,
    lcdm30w: Lcdm30w,
    _reserved_28_lcdbm0w: [u8; 0x02],
    _reserved_29_lcdbm2w: [u8; 0x02],
    _reserved_30_lcdbm4w: [u8; 0x02],
    _reserved_31_lcdbm6w: [u8; 0x02],
    lcdbm8w: Lcdbm8w,
    lcdbm10w: Lcdbm10w,
    lcdbm12w: Lcdbm12w,
    lcdbm14w: Lcdbm14w,
    lcdbm16w: Lcdbm16w,
    lcdbm18w: Lcdbm18w,
}
impl RegisterBlock {
    #[doc = "0x00 - LCD_E Control Register 0"]
    #[inline(always)]
    pub const fn lcdctl0(&self) -> &Lcdctl0 {
        &self.lcdctl0
    }
    #[doc = "0x02 - LCD_E Control Register 1"]
    #[inline(always)]
    pub const fn lcdctl1(&self) -> &Lcdctl1 {
        &self.lcdctl1
    }
    #[doc = "0x04 - LCD_E blinking control register"]
    #[inline(always)]
    pub const fn lcdblkctl(&self) -> &Lcdblkctl {
        &self.lcdblkctl
    }
    #[doc = "0x06 - LCD_E memory control register"]
    #[inline(always)]
    pub const fn lcdmemctl(&self) -> &Lcdmemctl {
        &self.lcdmemctl
    }
    #[doc = "0x08 - LCD_E Voltage Control Register"]
    #[inline(always)]
    pub const fn lcdvctl(&self) -> &Lcdvctl {
        &self.lcdvctl
    }
    #[doc = "0x0a - LCD_E Port Control Register 0"]
    #[inline(always)]
    pub const fn lcdpctl0(&self) -> &Lcdpctl0 {
        &self.lcdpctl0
    }
    #[doc = "0x0c - LCD_E Port Control Register 1"]
    #[inline(always)]
    pub const fn lcdpctl1(&self) -> &Lcdpctl1 {
        &self.lcdpctl1
    }
    #[doc = "0x0e - LCD_E Port Control Register 2"]
    #[inline(always)]
    pub const fn lcdpctl2(&self) -> &Lcdpctl2 {
        &self.lcdpctl2
    }
    #[doc = "0x14 - LCD_E COM/SEG select register 0"]
    #[inline(always)]
    pub const fn lcdcssel0(&self) -> &Lcdcssel0 {
        &self.lcdcssel0
    }
    #[doc = "0x16 - LCD_E COM/SEG select register 1"]
    #[inline(always)]
    pub const fn lcdcssel1(&self) -> &Lcdcssel1 {
        &self.lcdcssel1
    }
    #[doc = "0x18 - LCD_E COM/SEG select register 2"]
    #[inline(always)]
    pub const fn lcdcssel2(&self) -> &Lcdcssel2 {
        &self.lcdcssel2
    }
    #[doc = "0x1e - LCD_E Interrupt Vector Register"]
    #[inline(always)]
    pub const fn lcdiv(&self) -> &Lcdiv {
        &self.lcdiv
    }
    #[doc = "0x20 - LCD Memory 0/1"]
    #[inline(always)]
    pub const fn lcdm0w(&self) -> &Lcdm0w {
        &self.lcdm0w
    }
    #[doc = "0x22 - LCD Memory 2/3"]
    #[inline(always)]
    pub const fn lcdm2w(&self) -> &Lcdm2w {
        &self.lcdm2w
    }
    #[doc = "0x24 - LCD Memory 4/5"]
    #[inline(always)]
    pub const fn lcdm4w(&self) -> &Lcdm4w {
        &self.lcdm4w
    }
    #[doc = "0x26 - LCD Memory 6/7"]
    #[inline(always)]
    pub const fn lcdm6w(&self) -> &Lcdm6w {
        &self.lcdm6w
    }
    #[doc = "0x28 - LCD Memory 8/9"]
    #[inline(always)]
    pub const fn lcdm8w(&self) -> &Lcdm8w {
        &self.lcdm8w
    }
    #[doc = "0x2a - LCD Memory 10/11"]
    #[inline(always)]
    pub const fn lcdm10w(&self) -> &Lcdm10w {
        &self.lcdm10w
    }
    #[doc = "0x2c - LCD Memory 12/13"]
    #[inline(always)]
    pub const fn lcdm12w(&self) -> &Lcdm12w {
        &self.lcdm12w
    }
    #[doc = "0x2e - LCD Memory 14/15"]
    #[inline(always)]
    pub const fn lcdm14w(&self) -> &Lcdm14w {
        &self.lcdm14w
    }
    #[doc = "0x30 - LCD Memory 16/17"]
    #[inline(always)]
    pub const fn lcdm16w(&self) -> &Lcdm16w {
        &self.lcdm16w
    }
    #[doc = "0x32 - LCD Memory 18/19"]
    #[inline(always)]
    pub const fn lcdm18w(&self) -> &Lcdm18w {
        &self.lcdm18w
    }
    #[doc = "0x34 - LCD Memory 20/21"]
    #[inline(always)]
    pub const fn lcdm20w(&self) -> &Lcdm20w {
        &self.lcdm20w
    }
    #[doc = "0x36 - LCD Memory 22/23"]
    #[inline(always)]
    pub const fn lcdm22w(&self) -> &Lcdm22w {
        &self.lcdm22w
    }
    #[doc = "0x38 - LCD Memory 24/25"]
    #[inline(always)]
    pub const fn lcdm24w(&self) -> &Lcdm24w {
        &self.lcdm24w
    }
    #[doc = "0x3a - LCD Memory 26/27"]
    #[inline(always)]
    pub const fn lcdm26w(&self) -> &Lcdm26w {
        &self.lcdm26w
    }
    #[doc = "0x3c - LCD Memory 28/29"]
    #[inline(always)]
    pub const fn lcdm28w(&self) -> &Lcdm28w {
        &self.lcdm28w
    }
    #[doc = "0x3e - LCD Memory 30/31"]
    #[inline(always)]
    pub const fn lcdm30w(&self) -> &Lcdm30w {
        &self.lcdm30w
    }
    #[doc = "0x40 - LCD Memory 32/33"]
    #[inline(always)]
    pub const fn lcdm32w(&self) -> &Lcdm32w {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - LCD Blinking Memory 0/1"]
    #[inline(always)]
    pub const fn lcdbm0w(&self) -> &Lcdbm0w {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x42 - LCD Memory 34/35"]
    #[inline(always)]
    pub const fn lcdm34w(&self) -> &Lcdm34w {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x42 - LCD Blinking Memory 2/3"]
    #[inline(always)]
    pub const fn lcdbm2w(&self) -> &Lcdbm2w {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x44 - LCD Memory 36/37"]
    #[inline(always)]
    pub const fn lcdm36w(&self) -> &Lcdm36w {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x44 - LCD Blinking Memory 4/5"]
    #[inline(always)]
    pub const fn lcdbm4w(&self) -> &Lcdbm4w {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x46 - LCD Memory 38/39"]
    #[inline(always)]
    pub const fn lcdm38w(&self) -> &Lcdm38w {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(70).cast() }
    }
    #[doc = "0x46 - LCD Blinking Memory 6/7"]
    #[inline(always)]
    pub const fn lcdbm6w(&self) -> &Lcdbm6w {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(70).cast() }
    }
    #[doc = "0x48 - LCD Blinking Memory 8/9"]
    #[inline(always)]
    pub const fn lcdbm8w(&self) -> &Lcdbm8w {
        &self.lcdbm8w
    }
    #[doc = "0x4a - LCD Blinking Memory 10/11"]
    #[inline(always)]
    pub const fn lcdbm10w(&self) -> &Lcdbm10w {
        &self.lcdbm10w
    }
    #[doc = "0x4c - LCD Blinking Memory 12/13"]
    #[inline(always)]
    pub const fn lcdbm12w(&self) -> &Lcdbm12w {
        &self.lcdbm12w
    }
    #[doc = "0x4e - LCD Blinking Memory 14/15"]
    #[inline(always)]
    pub const fn lcdbm14w(&self) -> &Lcdbm14w {
        &self.lcdbm14w
    }
    #[doc = "0x50 - LCD Blinking Memory 16/17"]
    #[inline(always)]
    pub const fn lcdbm16w(&self) -> &Lcdbm16w {
        &self.lcdbm16w
    }
    #[doc = "0x52 - LCD Blinking Memory 18/19"]
    #[inline(always)]
    pub const fn lcdbm18w(&self) -> &Lcdbm18w {
        &self.lcdbm18w
    }
}
#[doc = "LCDCTL0 (rw) register accessor: LCD_E Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdctl0`] module"]
#[doc(alias = "LCDCTL0")]
pub type Lcdctl0 = crate::Reg<lcdctl0::Lcdctl0Spec>;
#[doc = "LCD_E Control Register 0"]
pub mod lcdctl0;
#[doc = "LCDCTL1 (rw) register accessor: LCD_E Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdctl1`] module"]
#[doc(alias = "LCDCTL1")]
pub type Lcdctl1 = crate::Reg<lcdctl1::Lcdctl1Spec>;
#[doc = "LCD_E Control Register 1"]
pub mod lcdctl1;
#[doc = "LCDBLKCTL (rw) register accessor: LCD_E blinking control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdblkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdblkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdblkctl`] module"]
#[doc(alias = "LCDBLKCTL")]
pub type Lcdblkctl = crate::Reg<lcdblkctl::LcdblkctlSpec>;
#[doc = "LCD_E blinking control register"]
pub mod lcdblkctl;
#[doc = "LCDMEMCTL (rw) register accessor: LCD_E memory control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdmemctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdmemctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdmemctl`] module"]
#[doc(alias = "LCDMEMCTL")]
pub type Lcdmemctl = crate::Reg<lcdmemctl::LcdmemctlSpec>;
#[doc = "LCD_E memory control register"]
pub mod lcdmemctl;
#[doc = "LCDVCTL (rw) register accessor: LCD_E Voltage Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdvctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdvctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdvctl`] module"]
#[doc(alias = "LCDVCTL")]
pub type Lcdvctl = crate::Reg<lcdvctl::LcdvctlSpec>;
#[doc = "LCD_E Voltage Control Register"]
pub mod lcdvctl;
#[doc = "LCDPCTL0 (rw) register accessor: LCD_E Port Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdpctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdpctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdpctl0`] module"]
#[doc(alias = "LCDPCTL0")]
pub type Lcdpctl0 = crate::Reg<lcdpctl0::Lcdpctl0Spec>;
#[doc = "LCD_E Port Control Register 0"]
pub mod lcdpctl0;
#[doc = "LCDPCTL1 (rw) register accessor: LCD_E Port Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdpctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdpctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdpctl1`] module"]
#[doc(alias = "LCDPCTL1")]
pub type Lcdpctl1 = crate::Reg<lcdpctl1::Lcdpctl1Spec>;
#[doc = "LCD_E Port Control Register 1"]
pub mod lcdpctl1;
#[doc = "LCDPCTL2 (rw) register accessor: LCD_E Port Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdpctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdpctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdpctl2`] module"]
#[doc(alias = "LCDPCTL2")]
pub type Lcdpctl2 = crate::Reg<lcdpctl2::Lcdpctl2Spec>;
#[doc = "LCD_E Port Control Register 2"]
pub mod lcdpctl2;
#[doc = "LCDCSSEL0 (rw) register accessor: LCD_E COM/SEG select register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcssel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcssel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcssel0`] module"]
#[doc(alias = "LCDCSSEL0")]
pub type Lcdcssel0 = crate::Reg<lcdcssel0::Lcdcssel0Spec>;
#[doc = "LCD_E COM/SEG select register 0"]
pub mod lcdcssel0;
#[doc = "LCDCSSEL1 (rw) register accessor: LCD_E COM/SEG select register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcssel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcssel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcssel1`] module"]
#[doc(alias = "LCDCSSEL1")]
pub type Lcdcssel1 = crate::Reg<lcdcssel1::Lcdcssel1Spec>;
#[doc = "LCD_E COM/SEG select register 1"]
pub mod lcdcssel1;
#[doc = "LCDCSSEL2 (rw) register accessor: LCD_E COM/SEG select register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcssel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcssel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcssel2`] module"]
#[doc(alias = "LCDCSSEL2")]
pub type Lcdcssel2 = crate::Reg<lcdcssel2::Lcdcssel2Spec>;
#[doc = "LCD_E COM/SEG select register 2"]
pub mod lcdcssel2;
#[doc = "LCDIV (rw) register accessor: LCD_E Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdiv`] module"]
#[doc(alias = "LCDIV")]
pub type Lcdiv = crate::Reg<lcdiv::LcdivSpec>;
#[doc = "LCD_E Interrupt Vector Register"]
pub mod lcdiv;
#[doc = "LCDM0W (rw) register accessor: LCD Memory 0/1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm0w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm0w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm0w`] module"]
#[doc(alias = "LCDM0W")]
pub type Lcdm0w = crate::Reg<lcdm0w::Lcdm0wSpec>;
#[doc = "LCD Memory 0/1"]
pub mod lcdm0w;
#[doc = "LCDM2W (rw) register accessor: LCD Memory 2/3\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm2w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm2w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm2w`] module"]
#[doc(alias = "LCDM2W")]
pub type Lcdm2w = crate::Reg<lcdm2w::Lcdm2wSpec>;
#[doc = "LCD Memory 2/3"]
pub mod lcdm2w;
#[doc = "LCDM4W (rw) register accessor: LCD Memory 4/5\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm4w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm4w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm4w`] module"]
#[doc(alias = "LCDM4W")]
pub type Lcdm4w = crate::Reg<lcdm4w::Lcdm4wSpec>;
#[doc = "LCD Memory 4/5"]
pub mod lcdm4w;
#[doc = "LCDM6W (rw) register accessor: LCD Memory 6/7\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm6w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm6w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm6w`] module"]
#[doc(alias = "LCDM6W")]
pub type Lcdm6w = crate::Reg<lcdm6w::Lcdm6wSpec>;
#[doc = "LCD Memory 6/7"]
pub mod lcdm6w;
#[doc = "LCDM8W (rw) register accessor: LCD Memory 8/9\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm8w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm8w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm8w`] module"]
#[doc(alias = "LCDM8W")]
pub type Lcdm8w = crate::Reg<lcdm8w::Lcdm8wSpec>;
#[doc = "LCD Memory 8/9"]
pub mod lcdm8w;
#[doc = "LCDM10W (rw) register accessor: LCD Memory 10/11\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm10w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm10w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm10w`] module"]
#[doc(alias = "LCDM10W")]
pub type Lcdm10w = crate::Reg<lcdm10w::Lcdm10wSpec>;
#[doc = "LCD Memory 10/11"]
pub mod lcdm10w;
#[doc = "LCDM12W (rw) register accessor: LCD Memory 12/13\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm12w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm12w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm12w`] module"]
#[doc(alias = "LCDM12W")]
pub type Lcdm12w = crate::Reg<lcdm12w::Lcdm12wSpec>;
#[doc = "LCD Memory 12/13"]
pub mod lcdm12w;
#[doc = "LCDM14W (rw) register accessor: LCD Memory 14/15\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm14w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm14w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm14w`] module"]
#[doc(alias = "LCDM14W")]
pub type Lcdm14w = crate::Reg<lcdm14w::Lcdm14wSpec>;
#[doc = "LCD Memory 14/15"]
pub mod lcdm14w;
#[doc = "LCDM16W (rw) register accessor: LCD Memory 16/17\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm16w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm16w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm16w`] module"]
#[doc(alias = "LCDM16W")]
pub type Lcdm16w = crate::Reg<lcdm16w::Lcdm16wSpec>;
#[doc = "LCD Memory 16/17"]
pub mod lcdm16w;
#[doc = "LCDM18W (rw) register accessor: LCD Memory 18/19\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm18w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm18w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm18w`] module"]
#[doc(alias = "LCDM18W")]
pub type Lcdm18w = crate::Reg<lcdm18w::Lcdm18wSpec>;
#[doc = "LCD Memory 18/19"]
pub mod lcdm18w;
#[doc = "LCDM20W (rw) register accessor: LCD Memory 20/21\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm20w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm20w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm20w`] module"]
#[doc(alias = "LCDM20W")]
pub type Lcdm20w = crate::Reg<lcdm20w::Lcdm20wSpec>;
#[doc = "LCD Memory 20/21"]
pub mod lcdm20w;
#[doc = "LCDM22W (rw) register accessor: LCD Memory 22/23\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm22w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm22w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm22w`] module"]
#[doc(alias = "LCDM22W")]
pub type Lcdm22w = crate::Reg<lcdm22w::Lcdm22wSpec>;
#[doc = "LCD Memory 22/23"]
pub mod lcdm22w;
#[doc = "LCDM24W (rw) register accessor: LCD Memory 24/25\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm24w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm24w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm24w`] module"]
#[doc(alias = "LCDM24W")]
pub type Lcdm24w = crate::Reg<lcdm24w::Lcdm24wSpec>;
#[doc = "LCD Memory 24/25"]
pub mod lcdm24w;
#[doc = "LCDM26W (rw) register accessor: LCD Memory 26/27\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm26w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm26w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm26w`] module"]
#[doc(alias = "LCDM26W")]
pub type Lcdm26w = crate::Reg<lcdm26w::Lcdm26wSpec>;
#[doc = "LCD Memory 26/27"]
pub mod lcdm26w;
#[doc = "LCDM28W (rw) register accessor: LCD Memory 28/29\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm28w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm28w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm28w`] module"]
#[doc(alias = "LCDM28W")]
pub type Lcdm28w = crate::Reg<lcdm28w::Lcdm28wSpec>;
#[doc = "LCD Memory 28/29"]
pub mod lcdm28w;
#[doc = "LCDM30W (rw) register accessor: LCD Memory 30/31\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm30w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm30w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm30w`] module"]
#[doc(alias = "LCDM30W")]
pub type Lcdm30w = crate::Reg<lcdm30w::Lcdm30wSpec>;
#[doc = "LCD Memory 30/31"]
pub mod lcdm30w;
#[doc = "LCDBM0W (rw) register accessor: LCD Blinking Memory 0/1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm0w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm0w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdbm0w`] module"]
#[doc(alias = "LCDBM0W")]
pub type Lcdbm0w = crate::Reg<lcdbm0w::Lcdbm0wSpec>;
#[doc = "LCD Blinking Memory 0/1"]
pub mod lcdbm0w;
#[doc = "LCDM32W (rw) register accessor: LCD Memory 32/33\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm32w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm32w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm32w`] module"]
#[doc(alias = "LCDM32W")]
pub type Lcdm32w = crate::Reg<lcdm32w::Lcdm32wSpec>;
#[doc = "LCD Memory 32/33"]
pub mod lcdm32w;
#[doc = "LCDBM2W (rw) register accessor: LCD Blinking Memory 2/3\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm2w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm2w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdbm2w`] module"]
#[doc(alias = "LCDBM2W")]
pub type Lcdbm2w = crate::Reg<lcdbm2w::Lcdbm2wSpec>;
#[doc = "LCD Blinking Memory 2/3"]
pub mod lcdbm2w;
#[doc = "LCDM34W (rw) register accessor: LCD Memory 34/35\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm34w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm34w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm34w`] module"]
#[doc(alias = "LCDM34W")]
pub type Lcdm34w = crate::Reg<lcdm34w::Lcdm34wSpec>;
#[doc = "LCD Memory 34/35"]
pub mod lcdm34w;
#[doc = "LCDBM4W (rw) register accessor: LCD Blinking Memory 4/5\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm4w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm4w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdbm4w`] module"]
#[doc(alias = "LCDBM4W")]
pub type Lcdbm4w = crate::Reg<lcdbm4w::Lcdbm4wSpec>;
#[doc = "LCD Blinking Memory 4/5"]
pub mod lcdbm4w;
#[doc = "LCDM36W (rw) register accessor: LCD Memory 36/37\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm36w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm36w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm36w`] module"]
#[doc(alias = "LCDM36W")]
pub type Lcdm36w = crate::Reg<lcdm36w::Lcdm36wSpec>;
#[doc = "LCD Memory 36/37"]
pub mod lcdm36w;
#[doc = "LCDBM6W (rw) register accessor: LCD Blinking Memory 6/7\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm6w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm6w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdbm6w`] module"]
#[doc(alias = "LCDBM6W")]
pub type Lcdbm6w = crate::Reg<lcdbm6w::Lcdbm6wSpec>;
#[doc = "LCD Blinking Memory 6/7"]
pub mod lcdbm6w;
#[doc = "LCDM38W (rw) register accessor: LCD Memory 38/39\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm38w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm38w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm38w`] module"]
#[doc(alias = "LCDM38W")]
pub type Lcdm38w = crate::Reg<lcdm38w::Lcdm38wSpec>;
#[doc = "LCD Memory 38/39"]
pub mod lcdm38w;
#[doc = "LCDBM8W (rw) register accessor: LCD Blinking Memory 8/9\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm8w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm8w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdbm8w`] module"]
#[doc(alias = "LCDBM8W")]
pub type Lcdbm8w = crate::Reg<lcdbm8w::Lcdbm8wSpec>;
#[doc = "LCD Blinking Memory 8/9"]
pub mod lcdbm8w;
#[doc = "LCDBM10W (rw) register accessor: LCD Blinking Memory 10/11\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm10w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm10w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdbm10w`] module"]
#[doc(alias = "LCDBM10W")]
pub type Lcdbm10w = crate::Reg<lcdbm10w::Lcdbm10wSpec>;
#[doc = "LCD Blinking Memory 10/11"]
pub mod lcdbm10w;
#[doc = "LCDBM12W (rw) register accessor: LCD Blinking Memory 12/13\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm12w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm12w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdbm12w`] module"]
#[doc(alias = "LCDBM12W")]
pub type Lcdbm12w = crate::Reg<lcdbm12w::Lcdbm12wSpec>;
#[doc = "LCD Blinking Memory 12/13"]
pub mod lcdbm12w;
#[doc = "LCDBM14W (rw) register accessor: LCD Blinking Memory 14/15\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm14w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm14w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdbm14w`] module"]
#[doc(alias = "LCDBM14W")]
pub type Lcdbm14w = crate::Reg<lcdbm14w::Lcdbm14wSpec>;
#[doc = "LCD Blinking Memory 14/15"]
pub mod lcdbm14w;
#[doc = "LCDBM16W (rw) register accessor: LCD Blinking Memory 16/17\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm16w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm16w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdbm16w`] module"]
#[doc(alias = "LCDBM16W")]
pub type Lcdbm16w = crate::Reg<lcdbm16w::Lcdbm16wSpec>;
#[doc = "LCD Blinking Memory 16/17"]
pub mod lcdbm16w;
#[doc = "LCDBM18W (rw) register accessor: LCD Blinking Memory 18/19\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm18w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm18w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdbm18w`] module"]
#[doc(alias = "LCDBM18W")]
pub type Lcdbm18w = crate::Reg<lcdbm18w::Lcdbm18wSpec>;
#[doc = "LCD Blinking Memory 18/19"]
pub mod lcdbm18w;
