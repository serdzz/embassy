#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lcdcctl0: Lcdcctl0,
    lcdcctl1: Lcdcctl1,
    lcdcblkctl: Lcdcblkctl,
    lcdcmemctl: Lcdcmemctl,
    lcdcvctl: Lcdcvctl,
    lcdcpctl0: Lcdcpctl0,
    lcdcpctl1: Lcdcpctl1,
    lcdcpctl2: Lcdcpctl2,
    lcdcpctl3: Lcdcpctl3,
    lcdccpctl: Lcdccpctl,
    _reserved10: [u8; 0x0a],
    lcdciv: Lcdciv,
    lcdm1: Lcdm1,
    lcdm2: Lcdm2,
    lcdm3: Lcdm3,
    lcdm4: Lcdm4,
    lcdm5: Lcdm5,
    lcdm6: Lcdm6,
    lcdm7: Lcdm7,
    lcdm8: Lcdm8,
    lcdm9: Lcdm9,
    lcdm10: Lcdm10,
    lcdm11: Lcdm11,
    lcdm12: Lcdm12,
    lcdm13: Lcdm13,
    lcdm14: Lcdm14,
    lcdm15: Lcdm15,
    lcdm16: Lcdm16,
    lcdm17: Lcdm17,
    lcdm18: Lcdm18,
    lcdm19: Lcdm19,
    lcdm20: Lcdm20,
    lcdm21: Lcdm21,
    lcdm22: Lcdm22,
    lcdm23: Lcdm23,
    lcdm24: Lcdm24,
    lcdm25: Lcdm25,
    lcdm26: Lcdm26,
    lcdm27: Lcdm27,
    lcdm28: Lcdm28,
    lcdm29: Lcdm29,
    lcdm30: Lcdm30,
    lcdm31: Lcdm31,
    lcdm32: Lcdm32,
    lcdm33_lcdbm1: Lcdm33Lcdbm1,
    lcdm34_lcdbm2: Lcdm34Lcdbm2,
    lcdm35_lcdbm3: Lcdm35Lcdbm3,
    lcdm36_lcdbm4: Lcdm36Lcdbm4,
    lcdm37_lcdbm5: Lcdm37Lcdbm5,
    lcdm38_lcdbm6: Lcdm38Lcdbm6,
    lcdm39_lcdbm7: Lcdm39Lcdbm7,
    lcdm40_lcdbm8: Lcdm40Lcdbm8,
    lcdm41_lcdbm9: Lcdm41Lcdbm9,
    lcdm42_lcdbm10: Lcdm42Lcdbm10,
    lcdm43_lcdbm11: Lcdm43Lcdbm11,
    lcdm44_lcdbm12: Lcdm44Lcdbm12,
    lcdm45_lcdbm13: Lcdm45Lcdbm13,
    lcdm46_lcdbm14: Lcdm46Lcdbm14,
    lcdm47_lcdbm15: Lcdm47Lcdbm15,
    lcdm48_lcdbm16: Lcdm48Lcdbm16,
    lcdm49_lcdbm17: Lcdm49Lcdbm17,
    lcdm50_lcdbm18: Lcdm50Lcdbm18,
    lcdm51_lcdbm19: Lcdm51Lcdbm19,
    lcdm52_lcdbm20: Lcdm52Lcdbm20,
}
impl RegisterBlock {
    #[doc = "0x00 - LCD_C control 0"]
    #[inline(always)]
    pub const fn lcdcctl0(&self) -> &Lcdcctl0 {
        &self.lcdcctl0
    }
    #[doc = "0x02 - LCD_C control 1"]
    #[inline(always)]
    pub const fn lcdcctl1(&self) -> &Lcdcctl1 {
        &self.lcdcctl1
    }
    #[doc = "0x04 - LCD_C blinking control"]
    #[inline(always)]
    pub const fn lcdcblkctl(&self) -> &Lcdcblkctl {
        &self.lcdcblkctl
    }
    #[doc = "0x06 - LCD_C memory control"]
    #[inline(always)]
    pub const fn lcdcmemctl(&self) -> &Lcdcmemctl {
        &self.lcdcmemctl
    }
    #[doc = "0x08 - LCD_C Voltage Control Register"]
    #[inline(always)]
    pub const fn lcdcvctl(&self) -> &Lcdcvctl {
        &self.lcdcvctl
    }
    #[doc = "0x0a - LCD_C port control 0"]
    #[inline(always)]
    pub const fn lcdcpctl0(&self) -> &Lcdcpctl0 {
        &self.lcdcpctl0
    }
    #[doc = "0x0c - LCD_C port control 1"]
    #[inline(always)]
    pub const fn lcdcpctl1(&self) -> &Lcdcpctl1 {
        &self.lcdcpctl1
    }
    #[doc = "0x0e - LCD_C port control 2 (256 segments)"]
    #[inline(always)]
    pub const fn lcdcpctl2(&self) -> &Lcdcpctl2 {
        &self.lcdcpctl2
    }
    #[doc = "0x10 - LCD_C port control 3 (384 segments)"]
    #[inline(always)]
    pub const fn lcdcpctl3(&self) -> &Lcdcpctl3 {
        &self.lcdcpctl3
    }
    #[doc = "0x12 - LCD_C charge pump control"]
    #[inline(always)]
    pub const fn lcdccpctl(&self) -> &Lcdccpctl {
        &self.lcdccpctl
    }
    #[doc = "0x1e - LCD_C interrupt vector"]
    #[inline(always)]
    pub const fn lcdciv(&self) -> &Lcdciv {
        &self.lcdciv
    }
    #[doc = "0x20 - LCD memory 1"]
    #[inline(always)]
    pub const fn lcdm1(&self) -> &Lcdm1 {
        &self.lcdm1
    }
    #[doc = "0x21 - LCD memory 2"]
    #[inline(always)]
    pub const fn lcdm2(&self) -> &Lcdm2 {
        &self.lcdm2
    }
    #[doc = "0x22 - LCD memory 3"]
    #[inline(always)]
    pub const fn lcdm3(&self) -> &Lcdm3 {
        &self.lcdm3
    }
    #[doc = "0x23 - LCD memory 4"]
    #[inline(always)]
    pub const fn lcdm4(&self) -> &Lcdm4 {
        &self.lcdm4
    }
    #[doc = "0x24 - LCD memory 5"]
    #[inline(always)]
    pub const fn lcdm5(&self) -> &Lcdm5 {
        &self.lcdm5
    }
    #[doc = "0x25 - LCD memory 6"]
    #[inline(always)]
    pub const fn lcdm6(&self) -> &Lcdm6 {
        &self.lcdm6
    }
    #[doc = "0x26 - LCD memory 7"]
    #[inline(always)]
    pub const fn lcdm7(&self) -> &Lcdm7 {
        &self.lcdm7
    }
    #[doc = "0x27 - LCD memory 8"]
    #[inline(always)]
    pub const fn lcdm8(&self) -> &Lcdm8 {
        &self.lcdm8
    }
    #[doc = "0x28 - LCD memory 9"]
    #[inline(always)]
    pub const fn lcdm9(&self) -> &Lcdm9 {
        &self.lcdm9
    }
    #[doc = "0x29 - LCD memory 10"]
    #[inline(always)]
    pub const fn lcdm10(&self) -> &Lcdm10 {
        &self.lcdm10
    }
    #[doc = "0x2a - LCD memory 11"]
    #[inline(always)]
    pub const fn lcdm11(&self) -> &Lcdm11 {
        &self.lcdm11
    }
    #[doc = "0x2b - LCD memory 12"]
    #[inline(always)]
    pub const fn lcdm12(&self) -> &Lcdm12 {
        &self.lcdm12
    }
    #[doc = "0x2c - LCD memory 13"]
    #[inline(always)]
    pub const fn lcdm13(&self) -> &Lcdm13 {
        &self.lcdm13
    }
    #[doc = "0x2d - LCD memory 14"]
    #[inline(always)]
    pub const fn lcdm14(&self) -> &Lcdm14 {
        &self.lcdm14
    }
    #[doc = "0x2e - LCD memory 15"]
    #[inline(always)]
    pub const fn lcdm15(&self) -> &Lcdm15 {
        &self.lcdm15
    }
    #[doc = "0x2f - LCD memory 16"]
    #[inline(always)]
    pub const fn lcdm16(&self) -> &Lcdm16 {
        &self.lcdm16
    }
    #[doc = "0x30 - LCD memory 17"]
    #[inline(always)]
    pub const fn lcdm17(&self) -> &Lcdm17 {
        &self.lcdm17
    }
    #[doc = "0x31 - LCD memory 18"]
    #[inline(always)]
    pub const fn lcdm18(&self) -> &Lcdm18 {
        &self.lcdm18
    }
    #[doc = "0x32 - LCD memory 19"]
    #[inline(always)]
    pub const fn lcdm19(&self) -> &Lcdm19 {
        &self.lcdm19
    }
    #[doc = "0x33 - LCD memory 20"]
    #[inline(always)]
    pub const fn lcdm20(&self) -> &Lcdm20 {
        &self.lcdm20
    }
    #[doc = "0x34 - LCD memory 21"]
    #[inline(always)]
    pub const fn lcdm21(&self) -> &Lcdm21 {
        &self.lcdm21
    }
    #[doc = "0x35 - LCD memory 22"]
    #[inline(always)]
    pub const fn lcdm22(&self) -> &Lcdm22 {
        &self.lcdm22
    }
    #[doc = "0x36 - LCD memory 23"]
    #[inline(always)]
    pub const fn lcdm23(&self) -> &Lcdm23 {
        &self.lcdm23
    }
    #[doc = "0x37 - LCD memory 24"]
    #[inline(always)]
    pub const fn lcdm24(&self) -> &Lcdm24 {
        &self.lcdm24
    }
    #[doc = "0x38 - LCD memory 25"]
    #[inline(always)]
    pub const fn lcdm25(&self) -> &Lcdm25 {
        &self.lcdm25
    }
    #[doc = "0x39 - LCD memory 26"]
    #[inline(always)]
    pub const fn lcdm26(&self) -> &Lcdm26 {
        &self.lcdm26
    }
    #[doc = "0x3a - LCD memory 27"]
    #[inline(always)]
    pub const fn lcdm27(&self) -> &Lcdm27 {
        &self.lcdm27
    }
    #[doc = "0x3b - LCD memory 28"]
    #[inline(always)]
    pub const fn lcdm28(&self) -> &Lcdm28 {
        &self.lcdm28
    }
    #[doc = "0x3c - LCD memory 29"]
    #[inline(always)]
    pub const fn lcdm29(&self) -> &Lcdm29 {
        &self.lcdm29
    }
    #[doc = "0x3d - LCD memory 30"]
    #[inline(always)]
    pub const fn lcdm30(&self) -> &Lcdm30 {
        &self.lcdm30
    }
    #[doc = "0x3e - LCD memory 31"]
    #[inline(always)]
    pub const fn lcdm31(&self) -> &Lcdm31 {
        &self.lcdm31
    }
    #[doc = "0x3f - LCD memory 32"]
    #[inline(always)]
    pub const fn lcdm32(&self) -> &Lcdm32 {
        &self.lcdm32
    }
    #[doc = "0x40 - LCD memory 33 / LCD blinking memory 1"]
    #[inline(always)]
    pub const fn lcdm33_lcdbm1(&self) -> &Lcdm33Lcdbm1 {
        &self.lcdm33_lcdbm1
    }
    #[doc = "0x41 - LCD memory 34 / LCD blinking memory 2"]
    #[inline(always)]
    pub const fn lcdm34_lcdbm2(&self) -> &Lcdm34Lcdbm2 {
        &self.lcdm34_lcdbm2
    }
    #[doc = "0x42 - LCD memory 35 / LCD blinking memory 3"]
    #[inline(always)]
    pub const fn lcdm35_lcdbm3(&self) -> &Lcdm35Lcdbm3 {
        &self.lcdm35_lcdbm3
    }
    #[doc = "0x43 - LCD memory 36 / LCD blinking memory 4"]
    #[inline(always)]
    pub const fn lcdm36_lcdbm4(&self) -> &Lcdm36Lcdbm4 {
        &self.lcdm36_lcdbm4
    }
    #[doc = "0x44 - LCD memory 37 / LCD blinking memory 5"]
    #[inline(always)]
    pub const fn lcdm37_lcdbm5(&self) -> &Lcdm37Lcdbm5 {
        &self.lcdm37_lcdbm5
    }
    #[doc = "0x45 - LCD memory 38 / LCD blinking memory 6"]
    #[inline(always)]
    pub const fn lcdm38_lcdbm6(&self) -> &Lcdm38Lcdbm6 {
        &self.lcdm38_lcdbm6
    }
    #[doc = "0x46 - LCD memory 39 / LCD blinking memory 7"]
    #[inline(always)]
    pub const fn lcdm39_lcdbm7(&self) -> &Lcdm39Lcdbm7 {
        &self.lcdm39_lcdbm7
    }
    #[doc = "0x47 - LCD memory 40 / LCD blinking memory 8"]
    #[inline(always)]
    pub const fn lcdm40_lcdbm8(&self) -> &Lcdm40Lcdbm8 {
        &self.lcdm40_lcdbm8
    }
    #[doc = "0x48 - LCD memory 41 / LCD blinking memory 9"]
    #[inline(always)]
    pub const fn lcdm41_lcdbm9(&self) -> &Lcdm41Lcdbm9 {
        &self.lcdm41_lcdbm9
    }
    #[doc = "0x49 - LCD memory 42 / LCD blinking memory 10"]
    #[inline(always)]
    pub const fn lcdm42_lcdbm10(&self) -> &Lcdm42Lcdbm10 {
        &self.lcdm42_lcdbm10
    }
    #[doc = "0x4a - LCD memory 43 / LCD blinking memory 11"]
    #[inline(always)]
    pub const fn lcdm43_lcdbm11(&self) -> &Lcdm43Lcdbm11 {
        &self.lcdm43_lcdbm11
    }
    #[doc = "0x4b - LCD memory 44 / LCD blinking memory 11"]
    #[inline(always)]
    pub const fn lcdm44_lcdbm12(&self) -> &Lcdm44Lcdbm12 {
        &self.lcdm44_lcdbm12
    }
    #[doc = "0x4c - LCD memory 45 / LCD blinking memory 13"]
    #[inline(always)]
    pub const fn lcdm45_lcdbm13(&self) -> &Lcdm45Lcdbm13 {
        &self.lcdm45_lcdbm13
    }
    #[doc = "0x4d - LCD memory 46 / LCD blinking memory 14"]
    #[inline(always)]
    pub const fn lcdm46_lcdbm14(&self) -> &Lcdm46Lcdbm14 {
        &self.lcdm46_lcdbm14
    }
    #[doc = "0x4e - LCD memory 47 / LCD blinking memory 15"]
    #[inline(always)]
    pub const fn lcdm47_lcdbm15(&self) -> &Lcdm47Lcdbm15 {
        &self.lcdm47_lcdbm15
    }
    #[doc = "0x4f - LCD memory 48 / LCD blinking memory 16"]
    #[inline(always)]
    pub const fn lcdm48_lcdbm16(&self) -> &Lcdm48Lcdbm16 {
        &self.lcdm48_lcdbm16
    }
    #[doc = "0x50 - LCD memory 49 / LCD blinking memory 17"]
    #[inline(always)]
    pub const fn lcdm49_lcdbm17(&self) -> &Lcdm49Lcdbm17 {
        &self.lcdm49_lcdbm17
    }
    #[doc = "0x51 - LCD memory 50 / LCD blinking memory 18"]
    #[inline(always)]
    pub const fn lcdm50_lcdbm18(&self) -> &Lcdm50Lcdbm18 {
        &self.lcdm50_lcdbm18
    }
    #[doc = "0x52 - LCD memory 51 / LCD blinking memory 19"]
    #[inline(always)]
    pub const fn lcdm51_lcdbm19(&self) -> &Lcdm51Lcdbm19 {
        &self.lcdm51_lcdbm19
    }
    #[doc = "0x53 - LCD memory 52 / LCD blinking memory 20"]
    #[inline(always)]
    pub const fn lcdm52_lcdbm20(&self) -> &Lcdm52Lcdbm20 {
        &self.lcdm52_lcdbm20
    }
}
#[doc = "LCDM1 (rw) register accessor: LCD memory 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm1`] module"]
#[doc(alias = "LCDM1")]
pub type Lcdm1 = crate::Reg<lcdm1::Lcdm1Spec>;
#[doc = "LCD memory 1"]
pub mod lcdm1;
#[doc = "LCDM2 (rw) register accessor: LCD memory 2\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm2`] module"]
#[doc(alias = "LCDM2")]
pub type Lcdm2 = crate::Reg<lcdm2::Lcdm2Spec>;
#[doc = "LCD memory 2"]
pub mod lcdm2;
#[doc = "LCDM3 (rw) register accessor: LCD memory 3\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm3`] module"]
#[doc(alias = "LCDM3")]
pub type Lcdm3 = crate::Reg<lcdm3::Lcdm3Spec>;
#[doc = "LCD memory 3"]
pub mod lcdm3;
#[doc = "LCDM4 (rw) register accessor: LCD memory 4\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm4`] module"]
#[doc(alias = "LCDM4")]
pub type Lcdm4 = crate::Reg<lcdm4::Lcdm4Spec>;
#[doc = "LCD memory 4"]
pub mod lcdm4;
#[doc = "LCDM5 (rw) register accessor: LCD memory 5\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm5`] module"]
#[doc(alias = "LCDM5")]
pub type Lcdm5 = crate::Reg<lcdm5::Lcdm5Spec>;
#[doc = "LCD memory 5"]
pub mod lcdm5;
#[doc = "LCDM6 (rw) register accessor: LCD memory 6\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm6`] module"]
#[doc(alias = "LCDM6")]
pub type Lcdm6 = crate::Reg<lcdm6::Lcdm6Spec>;
#[doc = "LCD memory 6"]
pub mod lcdm6;
#[doc = "LCDM7 (rw) register accessor: LCD memory 7\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm7`] module"]
#[doc(alias = "LCDM7")]
pub type Lcdm7 = crate::Reg<lcdm7::Lcdm7Spec>;
#[doc = "LCD memory 7"]
pub mod lcdm7;
#[doc = "LCDM8 (rw) register accessor: LCD memory 8\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm8`] module"]
#[doc(alias = "LCDM8")]
pub type Lcdm8 = crate::Reg<lcdm8::Lcdm8Spec>;
#[doc = "LCD memory 8"]
pub mod lcdm8;
#[doc = "LCDM9 (rw) register accessor: LCD memory 9\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm9`] module"]
#[doc(alias = "LCDM9")]
pub type Lcdm9 = crate::Reg<lcdm9::Lcdm9Spec>;
#[doc = "LCD memory 9"]
pub mod lcdm9;
#[doc = "LCDM10 (rw) register accessor: LCD memory 10\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm10`] module"]
#[doc(alias = "LCDM10")]
pub type Lcdm10 = crate::Reg<lcdm10::Lcdm10Spec>;
#[doc = "LCD memory 10"]
pub mod lcdm10;
#[doc = "LCDM11 (rw) register accessor: LCD memory 11\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm11`] module"]
#[doc(alias = "LCDM11")]
pub type Lcdm11 = crate::Reg<lcdm11::Lcdm11Spec>;
#[doc = "LCD memory 11"]
pub mod lcdm11;
#[doc = "LCDM12 (rw) register accessor: LCD memory 12\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm12`] module"]
#[doc(alias = "LCDM12")]
pub type Lcdm12 = crate::Reg<lcdm12::Lcdm12Spec>;
#[doc = "LCD memory 12"]
pub mod lcdm12;
#[doc = "LCDM13 (rw) register accessor: LCD memory 13\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm13`] module"]
#[doc(alias = "LCDM13")]
pub type Lcdm13 = crate::Reg<lcdm13::Lcdm13Spec>;
#[doc = "LCD memory 13"]
pub mod lcdm13;
#[doc = "LCDM14 (rw) register accessor: LCD memory 14\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm14`] module"]
#[doc(alias = "LCDM14")]
pub type Lcdm14 = crate::Reg<lcdm14::Lcdm14Spec>;
#[doc = "LCD memory 14"]
pub mod lcdm14;
#[doc = "LCDM15 (rw) register accessor: LCD memory 15\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm15`] module"]
#[doc(alias = "LCDM15")]
pub type Lcdm15 = crate::Reg<lcdm15::Lcdm15Spec>;
#[doc = "LCD memory 15"]
pub mod lcdm15;
#[doc = "LCDM16 (rw) register accessor: LCD memory 16\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm16`] module"]
#[doc(alias = "LCDM16")]
pub type Lcdm16 = crate::Reg<lcdm16::Lcdm16Spec>;
#[doc = "LCD memory 16"]
pub mod lcdm16;
#[doc = "LCDM17 (rw) register accessor: LCD memory 17\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm17`] module"]
#[doc(alias = "LCDM17")]
pub type Lcdm17 = crate::Reg<lcdm17::Lcdm17Spec>;
#[doc = "LCD memory 17"]
pub mod lcdm17;
#[doc = "LCDM18 (rw) register accessor: LCD memory 18\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm18`] module"]
#[doc(alias = "LCDM18")]
pub type Lcdm18 = crate::Reg<lcdm18::Lcdm18Spec>;
#[doc = "LCD memory 18"]
pub mod lcdm18;
#[doc = "LCDM19 (rw) register accessor: LCD memory 19\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm19`] module"]
#[doc(alias = "LCDM19")]
pub type Lcdm19 = crate::Reg<lcdm19::Lcdm19Spec>;
#[doc = "LCD memory 19"]
pub mod lcdm19;
#[doc = "LCDM20 (rw) register accessor: LCD memory 20\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm20`] module"]
#[doc(alias = "LCDM20")]
pub type Lcdm20 = crate::Reg<lcdm20::Lcdm20Spec>;
#[doc = "LCD memory 20"]
pub mod lcdm20;
#[doc = "LCDM21 (rw) register accessor: LCD memory 21\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm21`] module"]
#[doc(alias = "LCDM21")]
pub type Lcdm21 = crate::Reg<lcdm21::Lcdm21Spec>;
#[doc = "LCD memory 21"]
pub mod lcdm21;
#[doc = "LCDM22 (rw) register accessor: LCD memory 22\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm22`] module"]
#[doc(alias = "LCDM22")]
pub type Lcdm22 = crate::Reg<lcdm22::Lcdm22Spec>;
#[doc = "LCD memory 22"]
pub mod lcdm22;
#[doc = "LCDM23 (rw) register accessor: LCD memory 23\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm23`] module"]
#[doc(alias = "LCDM23")]
pub type Lcdm23 = crate::Reg<lcdm23::Lcdm23Spec>;
#[doc = "LCD memory 23"]
pub mod lcdm23;
#[doc = "LCDM24 (rw) register accessor: LCD memory 24\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm24`] module"]
#[doc(alias = "LCDM24")]
pub type Lcdm24 = crate::Reg<lcdm24::Lcdm24Spec>;
#[doc = "LCD memory 24"]
pub mod lcdm24;
#[doc = "LCDM25 (rw) register accessor: LCD memory 25\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm25`] module"]
#[doc(alias = "LCDM25")]
pub type Lcdm25 = crate::Reg<lcdm25::Lcdm25Spec>;
#[doc = "LCD memory 25"]
pub mod lcdm25;
#[doc = "LCDM26 (rw) register accessor: LCD memory 26\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm26`] module"]
#[doc(alias = "LCDM26")]
pub type Lcdm26 = crate::Reg<lcdm26::Lcdm26Spec>;
#[doc = "LCD memory 26"]
pub mod lcdm26;
#[doc = "LCDM27 (rw) register accessor: LCD memory 27\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm27`] module"]
#[doc(alias = "LCDM27")]
pub type Lcdm27 = crate::Reg<lcdm27::Lcdm27Spec>;
#[doc = "LCD memory 27"]
pub mod lcdm27;
#[doc = "LCDM28 (rw) register accessor: LCD memory 28\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm28`] module"]
#[doc(alias = "LCDM28")]
pub type Lcdm28 = crate::Reg<lcdm28::Lcdm28Spec>;
#[doc = "LCD memory 28"]
pub mod lcdm28;
#[doc = "LCDM29 (rw) register accessor: LCD memory 29\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm29`] module"]
#[doc(alias = "LCDM29")]
pub type Lcdm29 = crate::Reg<lcdm29::Lcdm29Spec>;
#[doc = "LCD memory 29"]
pub mod lcdm29;
#[doc = "LCDM30 (rw) register accessor: LCD memory 30\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm30`] module"]
#[doc(alias = "LCDM30")]
pub type Lcdm30 = crate::Reg<lcdm30::Lcdm30Spec>;
#[doc = "LCD memory 30"]
pub mod lcdm30;
#[doc = "LCDM31 (rw) register accessor: LCD memory 31\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm31`] module"]
#[doc(alias = "LCDM31")]
pub type Lcdm31 = crate::Reg<lcdm31::Lcdm31Spec>;
#[doc = "LCD memory 31"]
pub mod lcdm31;
#[doc = "LCDM32 (rw) register accessor: LCD memory 32\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm32`] module"]
#[doc(alias = "LCDM32")]
pub type Lcdm32 = crate::Reg<lcdm32::Lcdm32Spec>;
#[doc = "LCD memory 32"]
pub mod lcdm32;
#[doc = "LCDM33_LCDBM1 (rw) register accessor: LCD memory 33 / LCD blinking memory 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm33_lcdbm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm33_lcdbm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm33_lcdbm1`] module"]
#[doc(alias = "LCDM33_LCDBM1")]
pub type Lcdm33Lcdbm1 = crate::Reg<lcdm33_lcdbm1::Lcdm33Lcdbm1Spec>;
#[doc = "LCD memory 33 / LCD blinking memory 1"]
pub mod lcdm33_lcdbm1;
#[doc = "LCDM34_LCDBM2 (rw) register accessor: LCD memory 34 / LCD blinking memory 2\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm34_lcdbm2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm34_lcdbm2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm34_lcdbm2`] module"]
#[doc(alias = "LCDM34_LCDBM2")]
pub type Lcdm34Lcdbm2 = crate::Reg<lcdm34_lcdbm2::Lcdm34Lcdbm2Spec>;
#[doc = "LCD memory 34 / LCD blinking memory 2"]
pub mod lcdm34_lcdbm2;
#[doc = "LCDM35_LCDBM3 (rw) register accessor: LCD memory 35 / LCD blinking memory 3\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm35_lcdbm3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm35_lcdbm3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm35_lcdbm3`] module"]
#[doc(alias = "LCDM35_LCDBM3")]
pub type Lcdm35Lcdbm3 = crate::Reg<lcdm35_lcdbm3::Lcdm35Lcdbm3Spec>;
#[doc = "LCD memory 35 / LCD blinking memory 3"]
pub mod lcdm35_lcdbm3;
#[doc = "LCDM36_LCDBM4 (rw) register accessor: LCD memory 36 / LCD blinking memory 4\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm36_lcdbm4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm36_lcdbm4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm36_lcdbm4`] module"]
#[doc(alias = "LCDM36_LCDBM4")]
pub type Lcdm36Lcdbm4 = crate::Reg<lcdm36_lcdbm4::Lcdm36Lcdbm4Spec>;
#[doc = "LCD memory 36 / LCD blinking memory 4"]
pub mod lcdm36_lcdbm4;
#[doc = "LCDM37_LCDBM5 (rw) register accessor: LCD memory 37 / LCD blinking memory 5\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm37_lcdbm5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm37_lcdbm5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm37_lcdbm5`] module"]
#[doc(alias = "LCDM37_LCDBM5")]
pub type Lcdm37Lcdbm5 = crate::Reg<lcdm37_lcdbm5::Lcdm37Lcdbm5Spec>;
#[doc = "LCD memory 37 / LCD blinking memory 5"]
pub mod lcdm37_lcdbm5;
#[doc = "LCDM38_LCDBM6 (rw) register accessor: LCD memory 38 / LCD blinking memory 6\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm38_lcdbm6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm38_lcdbm6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm38_lcdbm6`] module"]
#[doc(alias = "LCDM38_LCDBM6")]
pub type Lcdm38Lcdbm6 = crate::Reg<lcdm38_lcdbm6::Lcdm38Lcdbm6Spec>;
#[doc = "LCD memory 38 / LCD blinking memory 6"]
pub mod lcdm38_lcdbm6;
#[doc = "LCDM39_LCDBM7 (rw) register accessor: LCD memory 39 / LCD blinking memory 7\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm39_lcdbm7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm39_lcdbm7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm39_lcdbm7`] module"]
#[doc(alias = "LCDM39_LCDBM7")]
pub type Lcdm39Lcdbm7 = crate::Reg<lcdm39_lcdbm7::Lcdm39Lcdbm7Spec>;
#[doc = "LCD memory 39 / LCD blinking memory 7"]
pub mod lcdm39_lcdbm7;
#[doc = "LCDM40_LCDBM8 (rw) register accessor: LCD memory 40 / LCD blinking memory 8\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm40_lcdbm8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm40_lcdbm8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm40_lcdbm8`] module"]
#[doc(alias = "LCDM40_LCDBM8")]
pub type Lcdm40Lcdbm8 = crate::Reg<lcdm40_lcdbm8::Lcdm40Lcdbm8Spec>;
#[doc = "LCD memory 40 / LCD blinking memory 8"]
pub mod lcdm40_lcdbm8;
#[doc = "LCDM41_LCDBM9 (rw) register accessor: LCD memory 41 / LCD blinking memory 9\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm41_lcdbm9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm41_lcdbm9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm41_lcdbm9`] module"]
#[doc(alias = "LCDM41_LCDBM9")]
pub type Lcdm41Lcdbm9 = crate::Reg<lcdm41_lcdbm9::Lcdm41Lcdbm9Spec>;
#[doc = "LCD memory 41 / LCD blinking memory 9"]
pub mod lcdm41_lcdbm9;
#[doc = "LCDM42_LCDBM10 (rw) register accessor: LCD memory 42 / LCD blinking memory 10\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm42_lcdbm10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm42_lcdbm10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm42_lcdbm10`] module"]
#[doc(alias = "LCDM42_LCDBM10")]
pub type Lcdm42Lcdbm10 = crate::Reg<lcdm42_lcdbm10::Lcdm42Lcdbm10Spec>;
#[doc = "LCD memory 42 / LCD blinking memory 10"]
pub mod lcdm42_lcdbm10;
#[doc = "LCDM43_LCDBM11 (rw) register accessor: LCD memory 43 / LCD blinking memory 11\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm43_lcdbm11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm43_lcdbm11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm43_lcdbm11`] module"]
#[doc(alias = "LCDM43_LCDBM11")]
pub type Lcdm43Lcdbm11 = crate::Reg<lcdm43_lcdbm11::Lcdm43Lcdbm11Spec>;
#[doc = "LCD memory 43 / LCD blinking memory 11"]
pub mod lcdm43_lcdbm11;
#[doc = "LCDM44_LCDBM12 (rw) register accessor: LCD memory 44 / LCD blinking memory 11\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm44_lcdbm12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm44_lcdbm12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm44_lcdbm12`] module"]
#[doc(alias = "LCDM44_LCDBM12")]
pub type Lcdm44Lcdbm12 = crate::Reg<lcdm44_lcdbm12::Lcdm44Lcdbm12Spec>;
#[doc = "LCD memory 44 / LCD blinking memory 11"]
pub mod lcdm44_lcdbm12;
#[doc = "LCDM45_LCDBM13 (rw) register accessor: LCD memory 45 / LCD blinking memory 13\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm45_lcdbm13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm45_lcdbm13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm45_lcdbm13`] module"]
#[doc(alias = "LCDM45_LCDBM13")]
pub type Lcdm45Lcdbm13 = crate::Reg<lcdm45_lcdbm13::Lcdm45Lcdbm13Spec>;
#[doc = "LCD memory 45 / LCD blinking memory 13"]
pub mod lcdm45_lcdbm13;
#[doc = "LCDM46_LCDBM14 (rw) register accessor: LCD memory 46 / LCD blinking memory 14\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm46_lcdbm14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm46_lcdbm14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm46_lcdbm14`] module"]
#[doc(alias = "LCDM46_LCDBM14")]
pub type Lcdm46Lcdbm14 = crate::Reg<lcdm46_lcdbm14::Lcdm46Lcdbm14Spec>;
#[doc = "LCD memory 46 / LCD blinking memory 14"]
pub mod lcdm46_lcdbm14;
#[doc = "LCDM47_LCDBM15 (rw) register accessor: LCD memory 47 / LCD blinking memory 15\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm47_lcdbm15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm47_lcdbm15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm47_lcdbm15`] module"]
#[doc(alias = "LCDM47_LCDBM15")]
pub type Lcdm47Lcdbm15 = crate::Reg<lcdm47_lcdbm15::Lcdm47Lcdbm15Spec>;
#[doc = "LCD memory 47 / LCD blinking memory 15"]
pub mod lcdm47_lcdbm15;
#[doc = "LCDM48_LCDBM16 (rw) register accessor: LCD memory 48 / LCD blinking memory 16\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm48_lcdbm16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm48_lcdbm16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm48_lcdbm16`] module"]
#[doc(alias = "LCDM48_LCDBM16")]
pub type Lcdm48Lcdbm16 = crate::Reg<lcdm48_lcdbm16::Lcdm48Lcdbm16Spec>;
#[doc = "LCD memory 48 / LCD blinking memory 16"]
pub mod lcdm48_lcdbm16;
#[doc = "LCDM49_LCDBM17 (rw) register accessor: LCD memory 49 / LCD blinking memory 17\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm49_lcdbm17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm49_lcdbm17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm49_lcdbm17`] module"]
#[doc(alias = "LCDM49_LCDBM17")]
pub type Lcdm49Lcdbm17 = crate::Reg<lcdm49_lcdbm17::Lcdm49Lcdbm17Spec>;
#[doc = "LCD memory 49 / LCD blinking memory 17"]
pub mod lcdm49_lcdbm17;
#[doc = "LCDM50_LCDBM18 (rw) register accessor: LCD memory 50 / LCD blinking memory 18\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm50_lcdbm18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm50_lcdbm18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm50_lcdbm18`] module"]
#[doc(alias = "LCDM50_LCDBM18")]
pub type Lcdm50Lcdbm18 = crate::Reg<lcdm50_lcdbm18::Lcdm50Lcdbm18Spec>;
#[doc = "LCD memory 50 / LCD blinking memory 18"]
pub mod lcdm50_lcdbm18;
#[doc = "LCDM51_LCDBM19 (rw) register accessor: LCD memory 51 / LCD blinking memory 19\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm51_lcdbm19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm51_lcdbm19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm51_lcdbm19`] module"]
#[doc(alias = "LCDM51_LCDBM19")]
pub type Lcdm51Lcdbm19 = crate::Reg<lcdm51_lcdbm19::Lcdm51Lcdbm19Spec>;
#[doc = "LCD memory 51 / LCD blinking memory 19"]
pub mod lcdm51_lcdbm19;
#[doc = "LCDM52_LCDBM20 (rw) register accessor: LCD memory 52 / LCD blinking memory 20\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm52_lcdbm20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm52_lcdbm20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdm52_lcdbm20`] module"]
#[doc(alias = "LCDM52_LCDBM20")]
pub type Lcdm52Lcdbm20 = crate::Reg<lcdm52_lcdbm20::Lcdm52Lcdbm20Spec>;
#[doc = "LCD memory 52 / LCD blinking memory 20"]
pub mod lcdm52_lcdbm20;
#[doc = "LCDCCTL0 (rw) register accessor: LCD_C control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcctl0`] module"]
#[doc(alias = "LCDCCTL0")]
pub type Lcdcctl0 = crate::Reg<lcdcctl0::Lcdcctl0Spec>;
#[doc = "LCD_C control 0"]
pub mod lcdcctl0;
#[doc = "LCDCCTL1 (rw) register accessor: LCD_C control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcctl1`] module"]
#[doc(alias = "LCDCCTL1")]
pub type Lcdcctl1 = crate::Reg<lcdcctl1::Lcdcctl1Spec>;
#[doc = "LCD_C control 1"]
pub mod lcdcctl1;
#[doc = "LCDCBLKCTL (rw) register accessor: LCD_C blinking control\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcblkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcblkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcblkctl`] module"]
#[doc(alias = "LCDCBLKCTL")]
pub type Lcdcblkctl = crate::Reg<lcdcblkctl::LcdcblkctlSpec>;
#[doc = "LCD_C blinking control"]
pub mod lcdcblkctl;
#[doc = "LCDCMEMCTL (rw) register accessor: LCD_C memory control\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcmemctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcmemctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcmemctl`] module"]
#[doc(alias = "LCDCMEMCTL")]
pub type Lcdcmemctl = crate::Reg<lcdcmemctl::LcdcmemctlSpec>;
#[doc = "LCD_C memory control"]
pub mod lcdcmemctl;
#[doc = "LCDCVCTL (rw) register accessor: LCD_C Voltage Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcvctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcvctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcvctl`] module"]
#[doc(alias = "LCDCVCTL")]
pub type Lcdcvctl = crate::Reg<lcdcvctl::LcdcvctlSpec>;
#[doc = "LCD_C Voltage Control Register"]
pub mod lcdcvctl;
#[doc = "LCDCPCTL0 (rw) register accessor: LCD_C port control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcpctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcpctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcpctl0`] module"]
#[doc(alias = "LCDCPCTL0")]
pub type Lcdcpctl0 = crate::Reg<lcdcpctl0::Lcdcpctl0Spec>;
#[doc = "LCD_C port control 0"]
pub mod lcdcpctl0;
#[doc = "LCDCPCTL1 (rw) register accessor: LCD_C port control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcpctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcpctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcpctl1`] module"]
#[doc(alias = "LCDCPCTL1")]
pub type Lcdcpctl1 = crate::Reg<lcdcpctl1::Lcdcpctl1Spec>;
#[doc = "LCD_C port control 1"]
pub mod lcdcpctl1;
#[doc = "LCDCPCTL2 (rw) register accessor: LCD_C port control 2 (256 segments)\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcpctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcpctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcpctl2`] module"]
#[doc(alias = "LCDCPCTL2")]
pub type Lcdcpctl2 = crate::Reg<lcdcpctl2::Lcdcpctl2Spec>;
#[doc = "LCD_C port control 2 (256 segments)"]
pub mod lcdcpctl2;
#[doc = "LCDCPCTL3 (rw) register accessor: LCD_C port control 3 (384 segments)\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcpctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcpctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcpctl3`] module"]
#[doc(alias = "LCDCPCTL3")]
pub type Lcdcpctl3 = crate::Reg<lcdcpctl3::Lcdcpctl3Spec>;
#[doc = "LCD_C port control 3 (384 segments)"]
pub mod lcdcpctl3;
#[doc = "LCDCCPCTL (rw) register accessor: LCD_C charge pump control\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdccpctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdccpctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdccpctl`] module"]
#[doc(alias = "LCDCCPCTL")]
pub type Lcdccpctl = crate::Reg<lcdccpctl::LcdccpctlSpec>;
#[doc = "LCD_C charge pump control"]
pub mod lcdccpctl;
#[doc = "LCDCIV (rw) register accessor: LCD_C interrupt vector\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdciv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdciv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdciv`] module"]
#[doc(alias = "LCDCIV")]
pub type Lcdciv = crate::Reg<lcdciv::LcdcivSpec>;
#[doc = "LCD_C interrupt vector"]
pub mod lcdciv;
