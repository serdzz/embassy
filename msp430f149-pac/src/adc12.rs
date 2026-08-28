#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adc12mctl0: Adc12mctl0,
    adc12mctl1: Adc12mctl1,
    adc12mctl2: Adc12mctl2,
    adc12mctl3: Adc12mctl3,
    adc12mctl4: Adc12mctl4,
    adc12mctl5: Adc12mctl5,
    adc12mctl6: Adc12mctl6,
    adc12mctl7: Adc12mctl7,
    adc12mctl8: Adc12mctl8,
    adc12mctl9: Adc12mctl9,
    adc12mctl10: Adc12mctl10,
    adc12mctl11: Adc12mctl11,
    adc12mctl12: Adc12mctl12,
    adc12mctl13: Adc12mctl13,
    adc12mctl14: Adc12mctl14,
    adc12mctl15: Adc12mctl15,
    _reserved16: [u8; 0xb0],
    adc12mem0: Adc12mem0,
    adc12mem1: Adc12mem1,
    adc12mem2: Adc12mem2,
    adc12mem3: Adc12mem3,
    adc12mem4: Adc12mem4,
    adc12mem5: Adc12mem5,
    adc12mem6: Adc12mem6,
    adc12mem7: Adc12mem7,
    adc12mem8: Adc12mem8,
    adc12mem9: Adc12mem9,
    adc12mem10: Adc12mem10,
    adc12mem11: Adc12mem11,
    adc12mem12: Adc12mem12,
    adc12mem13: Adc12mem13,
    adc12mem14: Adc12mem14,
    adc12mem15: Adc12mem15,
    _reserved32: [u8; 0x40],
    adc12ctl0: Adc12ctl0,
    adc12ctl1: Adc12ctl1,
    adc12ifg: Adc12ifg,
    adc12ie: Adc12ie,
    adc12iv: Adc12iv,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC12 Memory Control 0"]
    #[inline(always)]
    pub const fn adc12mctl0(&self) -> &Adc12mctl0 {
        &self.adc12mctl0
    }
    #[doc = "0x01 - ADC12 Memory Control 1"]
    #[inline(always)]
    pub const fn adc12mctl1(&self) -> &Adc12mctl1 {
        &self.adc12mctl1
    }
    #[doc = "0x02 - ADC12 Memory Control 2"]
    #[inline(always)]
    pub const fn adc12mctl2(&self) -> &Adc12mctl2 {
        &self.adc12mctl2
    }
    #[doc = "0x03 - ADC12 Memory Control 3"]
    #[inline(always)]
    pub const fn adc12mctl3(&self) -> &Adc12mctl3 {
        &self.adc12mctl3
    }
    #[doc = "0x04 - ADC12 Memory Control 4"]
    #[inline(always)]
    pub const fn adc12mctl4(&self) -> &Adc12mctl4 {
        &self.adc12mctl4
    }
    #[doc = "0x05 - ADC12 Memory Control 5"]
    #[inline(always)]
    pub const fn adc12mctl5(&self) -> &Adc12mctl5 {
        &self.adc12mctl5
    }
    #[doc = "0x06 - ADC12 Memory Control 6"]
    #[inline(always)]
    pub const fn adc12mctl6(&self) -> &Adc12mctl6 {
        &self.adc12mctl6
    }
    #[doc = "0x07 - ADC12 Memory Control 7"]
    #[inline(always)]
    pub const fn adc12mctl7(&self) -> &Adc12mctl7 {
        &self.adc12mctl7
    }
    #[doc = "0x08 - ADC12 Memory Control 8"]
    #[inline(always)]
    pub const fn adc12mctl8(&self) -> &Adc12mctl8 {
        &self.adc12mctl8
    }
    #[doc = "0x09 - ADC12 Memory Control 9"]
    #[inline(always)]
    pub const fn adc12mctl9(&self) -> &Adc12mctl9 {
        &self.adc12mctl9
    }
    #[doc = "0x0a - ADC12 Memory Control 10"]
    #[inline(always)]
    pub const fn adc12mctl10(&self) -> &Adc12mctl10 {
        &self.adc12mctl10
    }
    #[doc = "0x0b - ADC12 Memory Control 11"]
    #[inline(always)]
    pub const fn adc12mctl11(&self) -> &Adc12mctl11 {
        &self.adc12mctl11
    }
    #[doc = "0x0c - ADC12 Memory Control 12"]
    #[inline(always)]
    pub const fn adc12mctl12(&self) -> &Adc12mctl12 {
        &self.adc12mctl12
    }
    #[doc = "0x0d - ADC12 Memory Control 13"]
    #[inline(always)]
    pub const fn adc12mctl13(&self) -> &Adc12mctl13 {
        &self.adc12mctl13
    }
    #[doc = "0x0e - ADC12 Memory Control 14"]
    #[inline(always)]
    pub const fn adc12mctl14(&self) -> &Adc12mctl14 {
        &self.adc12mctl14
    }
    #[doc = "0x0f - ADC12 Memory Control 15"]
    #[inline(always)]
    pub const fn adc12mctl15(&self) -> &Adc12mctl15 {
        &self.adc12mctl15
    }
    #[doc = "0xc0 - ADC12 Conversion Memory 0"]
    #[inline(always)]
    pub const fn adc12mem0(&self) -> &Adc12mem0 {
        &self.adc12mem0
    }
    #[doc = "0xc2 - ADC12 Conversion Memory 1"]
    #[inline(always)]
    pub const fn adc12mem1(&self) -> &Adc12mem1 {
        &self.adc12mem1
    }
    #[doc = "0xc4 - ADC12 Conversion Memory 2"]
    #[inline(always)]
    pub const fn adc12mem2(&self) -> &Adc12mem2 {
        &self.adc12mem2
    }
    #[doc = "0xc6 - ADC12 Conversion Memory 3"]
    #[inline(always)]
    pub const fn adc12mem3(&self) -> &Adc12mem3 {
        &self.adc12mem3
    }
    #[doc = "0xc8 - ADC12 Conversion Memory 4"]
    #[inline(always)]
    pub const fn adc12mem4(&self) -> &Adc12mem4 {
        &self.adc12mem4
    }
    #[doc = "0xca - ADC12 Conversion Memory 5"]
    #[inline(always)]
    pub const fn adc12mem5(&self) -> &Adc12mem5 {
        &self.adc12mem5
    }
    #[doc = "0xcc - ADC12 Conversion Memory 6"]
    #[inline(always)]
    pub const fn adc12mem6(&self) -> &Adc12mem6 {
        &self.adc12mem6
    }
    #[doc = "0xce - ADC12 Conversion Memory 7"]
    #[inline(always)]
    pub const fn adc12mem7(&self) -> &Adc12mem7 {
        &self.adc12mem7
    }
    #[doc = "0xd0 - ADC12 Conversion Memory 8"]
    #[inline(always)]
    pub const fn adc12mem8(&self) -> &Adc12mem8 {
        &self.adc12mem8
    }
    #[doc = "0xd2 - ADC12 Conversion Memory 9"]
    #[inline(always)]
    pub const fn adc12mem9(&self) -> &Adc12mem9 {
        &self.adc12mem9
    }
    #[doc = "0xd4 - ADC12 Conversion Memory 10"]
    #[inline(always)]
    pub const fn adc12mem10(&self) -> &Adc12mem10 {
        &self.adc12mem10
    }
    #[doc = "0xd6 - ADC12 Conversion Memory 11"]
    #[inline(always)]
    pub const fn adc12mem11(&self) -> &Adc12mem11 {
        &self.adc12mem11
    }
    #[doc = "0xd8 - ADC12 Conversion Memory 12"]
    #[inline(always)]
    pub const fn adc12mem12(&self) -> &Adc12mem12 {
        &self.adc12mem12
    }
    #[doc = "0xda - ADC12 Conversion Memory 13"]
    #[inline(always)]
    pub const fn adc12mem13(&self) -> &Adc12mem13 {
        &self.adc12mem13
    }
    #[doc = "0xdc - ADC12 Conversion Memory 14"]
    #[inline(always)]
    pub const fn adc12mem14(&self) -> &Adc12mem14 {
        &self.adc12mem14
    }
    #[doc = "0xde - ADC12 Conversion Memory 15"]
    #[inline(always)]
    pub const fn adc12mem15(&self) -> &Adc12mem15 {
        &self.adc12mem15
    }
    #[doc = "0x120 - ADC12 Control 0"]
    #[inline(always)]
    pub const fn adc12ctl0(&self) -> &Adc12ctl0 {
        &self.adc12ctl0
    }
    #[doc = "0x122 - ADC12 Control 1"]
    #[inline(always)]
    pub const fn adc12ctl1(&self) -> &Adc12ctl1 {
        &self.adc12ctl1
    }
    #[doc = "0x124 - ADC12 Interrupt Flag"]
    #[inline(always)]
    pub const fn adc12ifg(&self) -> &Adc12ifg {
        &self.adc12ifg
    }
    #[doc = "0x126 - ADC12 Interrupt Enable"]
    #[inline(always)]
    pub const fn adc12ie(&self) -> &Adc12ie {
        &self.adc12ie
    }
    #[doc = "0x128 - ADC12 Interrupt Vector Word"]
    #[inline(always)]
    pub const fn adc12iv(&self) -> &Adc12iv {
        &self.adc12iv
    }
}
#[doc = "ADC12MCTL0 (rw) register accessor: ADC12 Memory Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl0`] module"]
#[doc(alias = "ADC12MCTL0")]
pub type Adc12mctl0 = crate::Reg<adc12mctl0::Adc12mctl0Spec>;
#[doc = "ADC12 Memory Control 0"]
pub mod adc12mctl0;
#[doc = "ADC12MCTL1 (rw) register accessor: ADC12 Memory Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl1`] module"]
#[doc(alias = "ADC12MCTL1")]
pub type Adc12mctl1 = crate::Reg<adc12mctl1::Adc12mctl1Spec>;
#[doc = "ADC12 Memory Control 1"]
pub mod adc12mctl1;
#[doc = "ADC12MCTL2 (rw) register accessor: ADC12 Memory Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl2`] module"]
#[doc(alias = "ADC12MCTL2")]
pub type Adc12mctl2 = crate::Reg<adc12mctl2::Adc12mctl2Spec>;
#[doc = "ADC12 Memory Control 2"]
pub mod adc12mctl2;
#[doc = "ADC12MCTL3 (rw) register accessor: ADC12 Memory Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl3`] module"]
#[doc(alias = "ADC12MCTL3")]
pub type Adc12mctl3 = crate::Reg<adc12mctl3::Adc12mctl3Spec>;
#[doc = "ADC12 Memory Control 3"]
pub mod adc12mctl3;
#[doc = "ADC12MCTL4 (rw) register accessor: ADC12 Memory Control 4\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl4`] module"]
#[doc(alias = "ADC12MCTL4")]
pub type Adc12mctl4 = crate::Reg<adc12mctl4::Adc12mctl4Spec>;
#[doc = "ADC12 Memory Control 4"]
pub mod adc12mctl4;
#[doc = "ADC12MCTL5 (rw) register accessor: ADC12 Memory Control 5\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl5`] module"]
#[doc(alias = "ADC12MCTL5")]
pub type Adc12mctl5 = crate::Reg<adc12mctl5::Adc12mctl5Spec>;
#[doc = "ADC12 Memory Control 5"]
pub mod adc12mctl5;
#[doc = "ADC12MCTL6 (rw) register accessor: ADC12 Memory Control 6\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl6`] module"]
#[doc(alias = "ADC12MCTL6")]
pub type Adc12mctl6 = crate::Reg<adc12mctl6::Adc12mctl6Spec>;
#[doc = "ADC12 Memory Control 6"]
pub mod adc12mctl6;
#[doc = "ADC12MCTL7 (rw) register accessor: ADC12 Memory Control 7\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl7`] module"]
#[doc(alias = "ADC12MCTL7")]
pub type Adc12mctl7 = crate::Reg<adc12mctl7::Adc12mctl7Spec>;
#[doc = "ADC12 Memory Control 7"]
pub mod adc12mctl7;
#[doc = "ADC12MCTL8 (rw) register accessor: ADC12 Memory Control 8\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl8`] module"]
#[doc(alias = "ADC12MCTL8")]
pub type Adc12mctl8 = crate::Reg<adc12mctl8::Adc12mctl8Spec>;
#[doc = "ADC12 Memory Control 8"]
pub mod adc12mctl8;
#[doc = "ADC12MCTL9 (rw) register accessor: ADC12 Memory Control 9\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl9`] module"]
#[doc(alias = "ADC12MCTL9")]
pub type Adc12mctl9 = crate::Reg<adc12mctl9::Adc12mctl9Spec>;
#[doc = "ADC12 Memory Control 9"]
pub mod adc12mctl9;
#[doc = "ADC12MCTL10 (rw) register accessor: ADC12 Memory Control 10\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl10`] module"]
#[doc(alias = "ADC12MCTL10")]
pub type Adc12mctl10 = crate::Reg<adc12mctl10::Adc12mctl10Spec>;
#[doc = "ADC12 Memory Control 10"]
pub mod adc12mctl10;
#[doc = "ADC12MCTL11 (rw) register accessor: ADC12 Memory Control 11\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl11`] module"]
#[doc(alias = "ADC12MCTL11")]
pub type Adc12mctl11 = crate::Reg<adc12mctl11::Adc12mctl11Spec>;
#[doc = "ADC12 Memory Control 11"]
pub mod adc12mctl11;
#[doc = "ADC12MCTL12 (rw) register accessor: ADC12 Memory Control 12\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl12`] module"]
#[doc(alias = "ADC12MCTL12")]
pub type Adc12mctl12 = crate::Reg<adc12mctl12::Adc12mctl12Spec>;
#[doc = "ADC12 Memory Control 12"]
pub mod adc12mctl12;
#[doc = "ADC12MCTL13 (rw) register accessor: ADC12 Memory Control 13\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl13`] module"]
#[doc(alias = "ADC12MCTL13")]
pub type Adc12mctl13 = crate::Reg<adc12mctl13::Adc12mctl13Spec>;
#[doc = "ADC12 Memory Control 13"]
pub mod adc12mctl13;
#[doc = "ADC12MCTL14 (rw) register accessor: ADC12 Memory Control 14\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl14`] module"]
#[doc(alias = "ADC12MCTL14")]
pub type Adc12mctl14 = crate::Reg<adc12mctl14::Adc12mctl14Spec>;
#[doc = "ADC12 Memory Control 14"]
pub mod adc12mctl14;
#[doc = "ADC12MCTL15 (rw) register accessor: ADC12 Memory Control 15\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl15`] module"]
#[doc(alias = "ADC12MCTL15")]
pub type Adc12mctl15 = crate::Reg<adc12mctl15::Adc12mctl15Spec>;
#[doc = "ADC12 Memory Control 15"]
pub mod adc12mctl15;
#[doc = "ADC12MEM0 (rw) register accessor: ADC12 Conversion Memory 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem0`] module"]
#[doc(alias = "ADC12MEM0")]
pub type Adc12mem0 = crate::Reg<adc12mem0::Adc12mem0Spec>;
#[doc = "ADC12 Conversion Memory 0"]
pub mod adc12mem0;
#[doc = "ADC12MEM1 (rw) register accessor: ADC12 Conversion Memory 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem1`] module"]
#[doc(alias = "ADC12MEM1")]
pub type Adc12mem1 = crate::Reg<adc12mem1::Adc12mem1Spec>;
#[doc = "ADC12 Conversion Memory 1"]
pub mod adc12mem1;
#[doc = "ADC12MEM2 (rw) register accessor: ADC12 Conversion Memory 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem2`] module"]
#[doc(alias = "ADC12MEM2")]
pub type Adc12mem2 = crate::Reg<adc12mem2::Adc12mem2Spec>;
#[doc = "ADC12 Conversion Memory 2"]
pub mod adc12mem2;
#[doc = "ADC12MEM3 (rw) register accessor: ADC12 Conversion Memory 3\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem3`] module"]
#[doc(alias = "ADC12MEM3")]
pub type Adc12mem3 = crate::Reg<adc12mem3::Adc12mem3Spec>;
#[doc = "ADC12 Conversion Memory 3"]
pub mod adc12mem3;
#[doc = "ADC12MEM4 (rw) register accessor: ADC12 Conversion Memory 4\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem4`] module"]
#[doc(alias = "ADC12MEM4")]
pub type Adc12mem4 = crate::Reg<adc12mem4::Adc12mem4Spec>;
#[doc = "ADC12 Conversion Memory 4"]
pub mod adc12mem4;
#[doc = "ADC12MEM5 (rw) register accessor: ADC12 Conversion Memory 5\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem5`] module"]
#[doc(alias = "ADC12MEM5")]
pub type Adc12mem5 = crate::Reg<adc12mem5::Adc12mem5Spec>;
#[doc = "ADC12 Conversion Memory 5"]
pub mod adc12mem5;
#[doc = "ADC12MEM6 (rw) register accessor: ADC12 Conversion Memory 6\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem6`] module"]
#[doc(alias = "ADC12MEM6")]
pub type Adc12mem6 = crate::Reg<adc12mem6::Adc12mem6Spec>;
#[doc = "ADC12 Conversion Memory 6"]
pub mod adc12mem6;
#[doc = "ADC12MEM7 (rw) register accessor: ADC12 Conversion Memory 7\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem7`] module"]
#[doc(alias = "ADC12MEM7")]
pub type Adc12mem7 = crate::Reg<adc12mem7::Adc12mem7Spec>;
#[doc = "ADC12 Conversion Memory 7"]
pub mod adc12mem7;
#[doc = "ADC12MEM8 (rw) register accessor: ADC12 Conversion Memory 8\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem8`] module"]
#[doc(alias = "ADC12MEM8")]
pub type Adc12mem8 = crate::Reg<adc12mem8::Adc12mem8Spec>;
#[doc = "ADC12 Conversion Memory 8"]
pub mod adc12mem8;
#[doc = "ADC12MEM9 (rw) register accessor: ADC12 Conversion Memory 9\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem9`] module"]
#[doc(alias = "ADC12MEM9")]
pub type Adc12mem9 = crate::Reg<adc12mem9::Adc12mem9Spec>;
#[doc = "ADC12 Conversion Memory 9"]
pub mod adc12mem9;
#[doc = "ADC12MEM10 (rw) register accessor: ADC12 Conversion Memory 10\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem10`] module"]
#[doc(alias = "ADC12MEM10")]
pub type Adc12mem10 = crate::Reg<adc12mem10::Adc12mem10Spec>;
#[doc = "ADC12 Conversion Memory 10"]
pub mod adc12mem10;
#[doc = "ADC12MEM11 (rw) register accessor: ADC12 Conversion Memory 11\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem11`] module"]
#[doc(alias = "ADC12MEM11")]
pub type Adc12mem11 = crate::Reg<adc12mem11::Adc12mem11Spec>;
#[doc = "ADC12 Conversion Memory 11"]
pub mod adc12mem11;
#[doc = "ADC12MEM12 (rw) register accessor: ADC12 Conversion Memory 12\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem12`] module"]
#[doc(alias = "ADC12MEM12")]
pub type Adc12mem12 = crate::Reg<adc12mem12::Adc12mem12Spec>;
#[doc = "ADC12 Conversion Memory 12"]
pub mod adc12mem12;
#[doc = "ADC12MEM13 (rw) register accessor: ADC12 Conversion Memory 13\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem13`] module"]
#[doc(alias = "ADC12MEM13")]
pub type Adc12mem13 = crate::Reg<adc12mem13::Adc12mem13Spec>;
#[doc = "ADC12 Conversion Memory 13"]
pub mod adc12mem13;
#[doc = "ADC12MEM14 (rw) register accessor: ADC12 Conversion Memory 14\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem14`] module"]
#[doc(alias = "ADC12MEM14")]
pub type Adc12mem14 = crate::Reg<adc12mem14::Adc12mem14Spec>;
#[doc = "ADC12 Conversion Memory 14"]
pub mod adc12mem14;
#[doc = "ADC12MEM15 (rw) register accessor: ADC12 Conversion Memory 15\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem15`] module"]
#[doc(alias = "ADC12MEM15")]
pub type Adc12mem15 = crate::Reg<adc12mem15::Adc12mem15Spec>;
#[doc = "ADC12 Conversion Memory 15"]
pub mod adc12mem15;
#[doc = "ADC12CTL0 (rw) register accessor: ADC12 Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ctl0`] module"]
#[doc(alias = "ADC12CTL0")]
pub type Adc12ctl0 = crate::Reg<adc12ctl0::Adc12ctl0Spec>;
#[doc = "ADC12 Control 0"]
pub mod adc12ctl0;
#[doc = "ADC12CTL1 (rw) register accessor: ADC12 Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ctl1`] module"]
#[doc(alias = "ADC12CTL1")]
pub type Adc12ctl1 = crate::Reg<adc12ctl1::Adc12ctl1Spec>;
#[doc = "ADC12 Control 1"]
pub mod adc12ctl1;
#[doc = "ADC12IFG (rw) register accessor: ADC12 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ifg`] module"]
#[doc(alias = "ADC12IFG")]
pub type Adc12ifg = crate::Reg<adc12ifg::Adc12ifgSpec>;
#[doc = "ADC12 Interrupt Flag"]
pub mod adc12ifg;
#[doc = "ADC12IE (rw) register accessor: ADC12 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ie`] module"]
#[doc(alias = "ADC12IE")]
pub type Adc12ie = crate::Reg<adc12ie::Adc12ieSpec>;
#[doc = "ADC12 Interrupt Enable"]
pub mod adc12ie;
#[doc = "ADC12IV (rw) register accessor: ADC12 Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12iv`] module"]
#[doc(alias = "ADC12IV")]
pub type Adc12iv = crate::Reg<adc12iv::Adc12ivSpec>;
#[doc = "ADC12 Interrupt Vector Word"]
pub mod adc12iv;
