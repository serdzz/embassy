#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adc12ctl0: Adc12ctl0,
    adc12ctl1: Adc12ctl1,
    adc12ctl2: Adc12ctl2,
    adc12ctl3: Adc12ctl3,
    adc12lo: Adc12lo,
    adc12hi: Adc12hi,
    adc12ifgr0: Adc12ifgr0,
    adc12ifgr1: Adc12ifgr1,
    adc12ifgr2: Adc12ifgr2,
    adc12ier0: Adc12ier0,
    adc12ier1: Adc12ier1,
    adc12ier2: Adc12ier2,
    adc12iv: Adc12iv,
    _reserved13: [u8; 0x06],
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
    adc12mctl16: Adc12mctl16,
    adc12mctl17: Adc12mctl17,
    adc12mctl18: Adc12mctl18,
    adc12mctl19: Adc12mctl19,
    adc12mctl20: Adc12mctl20,
    adc12mctl21: Adc12mctl21,
    adc12mctl22: Adc12mctl22,
    adc12mctl23: Adc12mctl23,
    adc12mctl24: Adc12mctl24,
    adc12mctl25: Adc12mctl25,
    adc12mctl26: Adc12mctl26,
    adc12mctl27: Adc12mctl27,
    adc12mctl28: Adc12mctl28,
    adc12mctl29: Adc12mctl29,
    adc12mctl30: Adc12mctl30,
    adc12mctl31: Adc12mctl31,
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
    adc12mem16: Adc12mem16,
    adc12mem17: Adc12mem17,
    adc12mem18: Adc12mem18,
    adc12mem19: Adc12mem19,
    adc12mem20: Adc12mem20,
    adc12mem21: Adc12mem21,
    adc12mem22: Adc12mem22,
    adc12mem23: Adc12mem23,
    adc12mem24: Adc12mem24,
    adc12mem25: Adc12mem25,
    adc12mem26: Adc12mem26,
    adc12mem27: Adc12mem27,
    adc12mem28: Adc12mem28,
    adc12mem29: Adc12mem29,
    adc12mem30: Adc12mem30,
    adc12mem31: Adc12mem31,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC12_B Control 0"]
    #[inline(always)]
    pub const fn adc12ctl0(&self) -> &Adc12ctl0 {
        &self.adc12ctl0
    }
    #[doc = "0x02 - ADC12_B Control 1"]
    #[inline(always)]
    pub const fn adc12ctl1(&self) -> &Adc12ctl1 {
        &self.adc12ctl1
    }
    #[doc = "0x04 - ADC12_B Control 2"]
    #[inline(always)]
    pub const fn adc12ctl2(&self) -> &Adc12ctl2 {
        &self.adc12ctl2
    }
    #[doc = "0x06 - ADC12_B Control 3"]
    #[inline(always)]
    pub const fn adc12ctl3(&self) -> &Adc12ctl3 {
        &self.adc12ctl3
    }
    #[doc = "0x08 - ADC12_B Window Comparator Low Threshold Register"]
    #[inline(always)]
    pub const fn adc12lo(&self) -> &Adc12lo {
        &self.adc12lo
    }
    #[doc = "0x0a - ADC12_B Window Comparator High Threshold Register"]
    #[inline(always)]
    pub const fn adc12hi(&self) -> &Adc12hi {
        &self.adc12hi
    }
    #[doc = "0x0c - ADC12_B Interrupt Flag 0"]
    #[inline(always)]
    pub const fn adc12ifgr0(&self) -> &Adc12ifgr0 {
        &self.adc12ifgr0
    }
    #[doc = "0x0e - ADC12_B Interrupt Flag 1"]
    #[inline(always)]
    pub const fn adc12ifgr1(&self) -> &Adc12ifgr1 {
        &self.adc12ifgr1
    }
    #[doc = "0x10 - ADC12_B Interrupt Flag 2"]
    #[inline(always)]
    pub const fn adc12ifgr2(&self) -> &Adc12ifgr2 {
        &self.adc12ifgr2
    }
    #[doc = "0x12 - ADC12_B Interrupt Enable 0"]
    #[inline(always)]
    pub const fn adc12ier0(&self) -> &Adc12ier0 {
        &self.adc12ier0
    }
    #[doc = "0x14 - ADC12_B Interrupt Enable 1"]
    #[inline(always)]
    pub const fn adc12ier1(&self) -> &Adc12ier1 {
        &self.adc12ier1
    }
    #[doc = "0x16 - ADC12_B Interrupt Enable 2"]
    #[inline(always)]
    pub const fn adc12ier2(&self) -> &Adc12ier2 {
        &self.adc12ier2
    }
    #[doc = "0x18 - ADC12_B Interrupt Vector"]
    #[inline(always)]
    pub const fn adc12iv(&self) -> &Adc12iv {
        &self.adc12iv
    }
    #[doc = "0x20 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl0(&self) -> &Adc12mctl0 {
        &self.adc12mctl0
    }
    #[doc = "0x22 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl1(&self) -> &Adc12mctl1 {
        &self.adc12mctl1
    }
    #[doc = "0x24 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl2(&self) -> &Adc12mctl2 {
        &self.adc12mctl2
    }
    #[doc = "0x26 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl3(&self) -> &Adc12mctl3 {
        &self.adc12mctl3
    }
    #[doc = "0x28 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl4(&self) -> &Adc12mctl4 {
        &self.adc12mctl4
    }
    #[doc = "0x2a - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl5(&self) -> &Adc12mctl5 {
        &self.adc12mctl5
    }
    #[doc = "0x2c - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl6(&self) -> &Adc12mctl6 {
        &self.adc12mctl6
    }
    #[doc = "0x2e - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl7(&self) -> &Adc12mctl7 {
        &self.adc12mctl7
    }
    #[doc = "0x30 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl8(&self) -> &Adc12mctl8 {
        &self.adc12mctl8
    }
    #[doc = "0x32 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl9(&self) -> &Adc12mctl9 {
        &self.adc12mctl9
    }
    #[doc = "0x34 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl10(&self) -> &Adc12mctl10 {
        &self.adc12mctl10
    }
    #[doc = "0x36 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl11(&self) -> &Adc12mctl11 {
        &self.adc12mctl11
    }
    #[doc = "0x38 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl12(&self) -> &Adc12mctl12 {
        &self.adc12mctl12
    }
    #[doc = "0x3a - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl13(&self) -> &Adc12mctl13 {
        &self.adc12mctl13
    }
    #[doc = "0x3c - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl14(&self) -> &Adc12mctl14 {
        &self.adc12mctl14
    }
    #[doc = "0x3e - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl15(&self) -> &Adc12mctl15 {
        &self.adc12mctl15
    }
    #[doc = "0x40 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl16(&self) -> &Adc12mctl16 {
        &self.adc12mctl16
    }
    #[doc = "0x42 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl17(&self) -> &Adc12mctl17 {
        &self.adc12mctl17
    }
    #[doc = "0x44 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl18(&self) -> &Adc12mctl18 {
        &self.adc12mctl18
    }
    #[doc = "0x46 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl19(&self) -> &Adc12mctl19 {
        &self.adc12mctl19
    }
    #[doc = "0x48 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl20(&self) -> &Adc12mctl20 {
        &self.adc12mctl20
    }
    #[doc = "0x4a - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl21(&self) -> &Adc12mctl21 {
        &self.adc12mctl21
    }
    #[doc = "0x4c - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl22(&self) -> &Adc12mctl22 {
        &self.adc12mctl22
    }
    #[doc = "0x4e - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl23(&self) -> &Adc12mctl23 {
        &self.adc12mctl23
    }
    #[doc = "0x50 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl24(&self) -> &Adc12mctl24 {
        &self.adc12mctl24
    }
    #[doc = "0x52 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl25(&self) -> &Adc12mctl25 {
        &self.adc12mctl25
    }
    #[doc = "0x54 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl26(&self) -> &Adc12mctl26 {
        &self.adc12mctl26
    }
    #[doc = "0x56 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl27(&self) -> &Adc12mctl27 {
        &self.adc12mctl27
    }
    #[doc = "0x58 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl28(&self) -> &Adc12mctl28 {
        &self.adc12mctl28
    }
    #[doc = "0x5a - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl29(&self) -> &Adc12mctl29 {
        &self.adc12mctl29
    }
    #[doc = "0x5c - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl30(&self) -> &Adc12mctl30 {
        &self.adc12mctl30
    }
    #[doc = "0x5e - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    #[inline(always)]
    pub const fn adc12mctl31(&self) -> &Adc12mctl31 {
        &self.adc12mctl31
    }
    #[doc = "0x60 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem0(&self) -> &Adc12mem0 {
        &self.adc12mem0
    }
    #[doc = "0x62 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem1(&self) -> &Adc12mem1 {
        &self.adc12mem1
    }
    #[doc = "0x64 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem2(&self) -> &Adc12mem2 {
        &self.adc12mem2
    }
    #[doc = "0x66 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem3(&self) -> &Adc12mem3 {
        &self.adc12mem3
    }
    #[doc = "0x68 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem4(&self) -> &Adc12mem4 {
        &self.adc12mem4
    }
    #[doc = "0x6a - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem5(&self) -> &Adc12mem5 {
        &self.adc12mem5
    }
    #[doc = "0x6c - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem6(&self) -> &Adc12mem6 {
        &self.adc12mem6
    }
    #[doc = "0x6e - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem7(&self) -> &Adc12mem7 {
        &self.adc12mem7
    }
    #[doc = "0x70 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem8(&self) -> &Adc12mem8 {
        &self.adc12mem8
    }
    #[doc = "0x72 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem9(&self) -> &Adc12mem9 {
        &self.adc12mem9
    }
    #[doc = "0x74 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem10(&self) -> &Adc12mem10 {
        &self.adc12mem10
    }
    #[doc = "0x76 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem11(&self) -> &Adc12mem11 {
        &self.adc12mem11
    }
    #[doc = "0x78 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem12(&self) -> &Adc12mem12 {
        &self.adc12mem12
    }
    #[doc = "0x7a - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem13(&self) -> &Adc12mem13 {
        &self.adc12mem13
    }
    #[doc = "0x7c - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem14(&self) -> &Adc12mem14 {
        &self.adc12mem14
    }
    #[doc = "0x7e - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem15(&self) -> &Adc12mem15 {
        &self.adc12mem15
    }
    #[doc = "0x80 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem16(&self) -> &Adc12mem16 {
        &self.adc12mem16
    }
    #[doc = "0x82 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem17(&self) -> &Adc12mem17 {
        &self.adc12mem17
    }
    #[doc = "0x84 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem18(&self) -> &Adc12mem18 {
        &self.adc12mem18
    }
    #[doc = "0x86 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem19(&self) -> &Adc12mem19 {
        &self.adc12mem19
    }
    #[doc = "0x88 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem20(&self) -> &Adc12mem20 {
        &self.adc12mem20
    }
    #[doc = "0x8a - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem21(&self) -> &Adc12mem21 {
        &self.adc12mem21
    }
    #[doc = "0x8c - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem22(&self) -> &Adc12mem22 {
        &self.adc12mem22
    }
    #[doc = "0x8e - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem23(&self) -> &Adc12mem23 {
        &self.adc12mem23
    }
    #[doc = "0x90 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem24(&self) -> &Adc12mem24 {
        &self.adc12mem24
    }
    #[doc = "0x92 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem25(&self) -> &Adc12mem25 {
        &self.adc12mem25
    }
    #[doc = "0x94 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem26(&self) -> &Adc12mem26 {
        &self.adc12mem26
    }
    #[doc = "0x96 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem27(&self) -> &Adc12mem27 {
        &self.adc12mem27
    }
    #[doc = "0x98 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem28(&self) -> &Adc12mem28 {
        &self.adc12mem28
    }
    #[doc = "0x9a - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem29(&self) -> &Adc12mem29 {
        &self.adc12mem29
    }
    #[doc = "0x9c - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem30(&self) -> &Adc12mem30 {
        &self.adc12mem30
    }
    #[doc = "0x9e - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    #[inline(always)]
    pub const fn adc12mem31(&self) -> &Adc12mem31 {
        &self.adc12mem31
    }
}
#[doc = "ADC12CTL0 (rw) register accessor: ADC12_B Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ctl0`] module"]
#[doc(alias = "ADC12CTL0")]
pub type Adc12ctl0 = crate::Reg<adc12ctl0::Adc12ctl0Spec>;
#[doc = "ADC12_B Control 0"]
pub mod adc12ctl0;
#[doc = "ADC12CTL1 (rw) register accessor: ADC12_B Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ctl1`] module"]
#[doc(alias = "ADC12CTL1")]
pub type Adc12ctl1 = crate::Reg<adc12ctl1::Adc12ctl1Spec>;
#[doc = "ADC12_B Control 1"]
pub mod adc12ctl1;
#[doc = "ADC12CTL2 (rw) register accessor: ADC12_B Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ctl2`] module"]
#[doc(alias = "ADC12CTL2")]
pub type Adc12ctl2 = crate::Reg<adc12ctl2::Adc12ctl2Spec>;
#[doc = "ADC12_B Control 2"]
pub mod adc12ctl2;
#[doc = "ADC12CTL3 (rw) register accessor: ADC12_B Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ctl3`] module"]
#[doc(alias = "ADC12CTL3")]
pub type Adc12ctl3 = crate::Reg<adc12ctl3::Adc12ctl3Spec>;
#[doc = "ADC12_B Control 3"]
pub mod adc12ctl3;
#[doc = "ADC12LO (rw) register accessor: ADC12_B Window Comparator Low Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12lo`] module"]
#[doc(alias = "ADC12LO")]
pub type Adc12lo = crate::Reg<adc12lo::Adc12loSpec>;
#[doc = "ADC12_B Window Comparator Low Threshold Register"]
pub mod adc12lo;
#[doc = "ADC12HI (rw) register accessor: ADC12_B Window Comparator High Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12hi`] module"]
#[doc(alias = "ADC12HI")]
pub type Adc12hi = crate::Reg<adc12hi::Adc12hiSpec>;
#[doc = "ADC12_B Window Comparator High Threshold Register"]
pub mod adc12hi;
#[doc = "ADC12IFGR0 (rw) register accessor: ADC12_B Interrupt Flag 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ifgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ifgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ifgr0`] module"]
#[doc(alias = "ADC12IFGR0")]
pub type Adc12ifgr0 = crate::Reg<adc12ifgr0::Adc12ifgr0Spec>;
#[doc = "ADC12_B Interrupt Flag 0"]
pub mod adc12ifgr0;
#[doc = "ADC12IFGR1 (rw) register accessor: ADC12_B Interrupt Flag 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ifgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ifgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ifgr1`] module"]
#[doc(alias = "ADC12IFGR1")]
pub type Adc12ifgr1 = crate::Reg<adc12ifgr1::Adc12ifgr1Spec>;
#[doc = "ADC12_B Interrupt Flag 1"]
pub mod adc12ifgr1;
#[doc = "ADC12IFGR2 (rw) register accessor: ADC12_B Interrupt Flag 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ifgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ifgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ifgr2`] module"]
#[doc(alias = "ADC12IFGR2")]
pub type Adc12ifgr2 = crate::Reg<adc12ifgr2::Adc12ifgr2Spec>;
#[doc = "ADC12_B Interrupt Flag 2"]
pub mod adc12ifgr2;
#[doc = "ADC12IER0 (rw) register accessor: ADC12_B Interrupt Enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ier0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ier0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ier0`] module"]
#[doc(alias = "ADC12IER0")]
pub type Adc12ier0 = crate::Reg<adc12ier0::Adc12ier0Spec>;
#[doc = "ADC12_B Interrupt Enable 0"]
pub mod adc12ier0;
#[doc = "ADC12IER1 (rw) register accessor: ADC12_B Interrupt Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ier1`] module"]
#[doc(alias = "ADC12IER1")]
pub type Adc12ier1 = crate::Reg<adc12ier1::Adc12ier1Spec>;
#[doc = "ADC12_B Interrupt Enable 1"]
pub mod adc12ier1;
#[doc = "ADC12IER2 (rw) register accessor: ADC12_B Interrupt Enable 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ier2`] module"]
#[doc(alias = "ADC12IER2")]
pub type Adc12ier2 = crate::Reg<adc12ier2::Adc12ier2Spec>;
#[doc = "ADC12_B Interrupt Enable 2"]
pub mod adc12ier2;
#[doc = "ADC12IV (rw) register accessor: ADC12_B Interrupt Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12iv`] module"]
#[doc(alias = "ADC12IV")]
pub type Adc12iv = crate::Reg<adc12iv::Adc12ivSpec>;
#[doc = "ADC12_B Interrupt Vector"]
pub mod adc12iv;
#[doc = "ADC12MCTL0 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl0`] module"]
#[doc(alias = "ADC12MCTL0")]
pub type Adc12mctl0 = crate::Reg<adc12mctl0::Adc12mctl0Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl0;
#[doc = "ADC12MCTL1 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl1`] module"]
#[doc(alias = "ADC12MCTL1")]
pub type Adc12mctl1 = crate::Reg<adc12mctl1::Adc12mctl1Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl1;
#[doc = "ADC12MCTL2 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl2`] module"]
#[doc(alias = "ADC12MCTL2")]
pub type Adc12mctl2 = crate::Reg<adc12mctl2::Adc12mctl2Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl2;
#[doc = "ADC12MCTL3 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl3`] module"]
#[doc(alias = "ADC12MCTL3")]
pub type Adc12mctl3 = crate::Reg<adc12mctl3::Adc12mctl3Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl3;
#[doc = "ADC12MCTL4 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl4`] module"]
#[doc(alias = "ADC12MCTL4")]
pub type Adc12mctl4 = crate::Reg<adc12mctl4::Adc12mctl4Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl4;
#[doc = "ADC12MCTL5 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl5`] module"]
#[doc(alias = "ADC12MCTL5")]
pub type Adc12mctl5 = crate::Reg<adc12mctl5::Adc12mctl5Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl5;
#[doc = "ADC12MCTL6 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl6`] module"]
#[doc(alias = "ADC12MCTL6")]
pub type Adc12mctl6 = crate::Reg<adc12mctl6::Adc12mctl6Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl6;
#[doc = "ADC12MCTL7 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl7`] module"]
#[doc(alias = "ADC12MCTL7")]
pub type Adc12mctl7 = crate::Reg<adc12mctl7::Adc12mctl7Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl7;
#[doc = "ADC12MCTL8 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl8`] module"]
#[doc(alias = "ADC12MCTL8")]
pub type Adc12mctl8 = crate::Reg<adc12mctl8::Adc12mctl8Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl8;
#[doc = "ADC12MCTL9 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl9`] module"]
#[doc(alias = "ADC12MCTL9")]
pub type Adc12mctl9 = crate::Reg<adc12mctl9::Adc12mctl9Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl9;
#[doc = "ADC12MCTL10 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl10`] module"]
#[doc(alias = "ADC12MCTL10")]
pub type Adc12mctl10 = crate::Reg<adc12mctl10::Adc12mctl10Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl10;
#[doc = "ADC12MCTL11 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl11`] module"]
#[doc(alias = "ADC12MCTL11")]
pub type Adc12mctl11 = crate::Reg<adc12mctl11::Adc12mctl11Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl11;
#[doc = "ADC12MCTL12 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl12`] module"]
#[doc(alias = "ADC12MCTL12")]
pub type Adc12mctl12 = crate::Reg<adc12mctl12::Adc12mctl12Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl12;
#[doc = "ADC12MCTL13 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl13`] module"]
#[doc(alias = "ADC12MCTL13")]
pub type Adc12mctl13 = crate::Reg<adc12mctl13::Adc12mctl13Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl13;
#[doc = "ADC12MCTL14 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl14`] module"]
#[doc(alias = "ADC12MCTL14")]
pub type Adc12mctl14 = crate::Reg<adc12mctl14::Adc12mctl14Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl14;
#[doc = "ADC12MCTL15 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl15`] module"]
#[doc(alias = "ADC12MCTL15")]
pub type Adc12mctl15 = crate::Reg<adc12mctl15::Adc12mctl15Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl15;
#[doc = "ADC12MCTL16 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl16`] module"]
#[doc(alias = "ADC12MCTL16")]
pub type Adc12mctl16 = crate::Reg<adc12mctl16::Adc12mctl16Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl16;
#[doc = "ADC12MCTL17 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl17`] module"]
#[doc(alias = "ADC12MCTL17")]
pub type Adc12mctl17 = crate::Reg<adc12mctl17::Adc12mctl17Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl17;
#[doc = "ADC12MCTL18 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl18`] module"]
#[doc(alias = "ADC12MCTL18")]
pub type Adc12mctl18 = crate::Reg<adc12mctl18::Adc12mctl18Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl18;
#[doc = "ADC12MCTL19 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl19`] module"]
#[doc(alias = "ADC12MCTL19")]
pub type Adc12mctl19 = crate::Reg<adc12mctl19::Adc12mctl19Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl19;
#[doc = "ADC12MCTL20 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl20`] module"]
#[doc(alias = "ADC12MCTL20")]
pub type Adc12mctl20 = crate::Reg<adc12mctl20::Adc12mctl20Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl20;
#[doc = "ADC12MCTL21 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl21`] module"]
#[doc(alias = "ADC12MCTL21")]
pub type Adc12mctl21 = crate::Reg<adc12mctl21::Adc12mctl21Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl21;
#[doc = "ADC12MCTL22 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl22`] module"]
#[doc(alias = "ADC12MCTL22")]
pub type Adc12mctl22 = crate::Reg<adc12mctl22::Adc12mctl22Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl22;
#[doc = "ADC12MCTL23 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl23`] module"]
#[doc(alias = "ADC12MCTL23")]
pub type Adc12mctl23 = crate::Reg<adc12mctl23::Adc12mctl23Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl23;
#[doc = "ADC12MCTL24 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl24`] module"]
#[doc(alias = "ADC12MCTL24")]
pub type Adc12mctl24 = crate::Reg<adc12mctl24::Adc12mctl24Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl24;
#[doc = "ADC12MCTL25 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl25`] module"]
#[doc(alias = "ADC12MCTL25")]
pub type Adc12mctl25 = crate::Reg<adc12mctl25::Adc12mctl25Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl25;
#[doc = "ADC12MCTL26 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl26`] module"]
#[doc(alias = "ADC12MCTL26")]
pub type Adc12mctl26 = crate::Reg<adc12mctl26::Adc12mctl26Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl26;
#[doc = "ADC12MCTL27 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl27`] module"]
#[doc(alias = "ADC12MCTL27")]
pub type Adc12mctl27 = crate::Reg<adc12mctl27::Adc12mctl27Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl27;
#[doc = "ADC12MCTL28 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl28`] module"]
#[doc(alias = "ADC12MCTL28")]
pub type Adc12mctl28 = crate::Reg<adc12mctl28::Adc12mctl28Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl28;
#[doc = "ADC12MCTL29 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl29`] module"]
#[doc(alias = "ADC12MCTL29")]
pub type Adc12mctl29 = crate::Reg<adc12mctl29::Adc12mctl29Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl29;
#[doc = "ADC12MCTL30 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl30`] module"]
#[doc(alias = "ADC12MCTL30")]
pub type Adc12mctl30 = crate::Reg<adc12mctl30::Adc12mctl30Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl30;
#[doc = "ADC12MCTL31 (rw) register accessor: ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl31`] module"]
#[doc(alias = "ADC12MCTL31")]
pub type Adc12mctl31 = crate::Reg<adc12mctl31::Adc12mctl31Spec>;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl31;
#[doc = "ADC12MEM0 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem0`] module"]
#[doc(alias = "ADC12MEM0")]
pub type Adc12mem0 = crate::Reg<adc12mem0::Adc12mem0Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem0;
#[doc = "ADC12MEM1 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem1`] module"]
#[doc(alias = "ADC12MEM1")]
pub type Adc12mem1 = crate::Reg<adc12mem1::Adc12mem1Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem1;
#[doc = "ADC12MEM2 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem2`] module"]
#[doc(alias = "ADC12MEM2")]
pub type Adc12mem2 = crate::Reg<adc12mem2::Adc12mem2Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem2;
#[doc = "ADC12MEM3 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem3`] module"]
#[doc(alias = "ADC12MEM3")]
pub type Adc12mem3 = crate::Reg<adc12mem3::Adc12mem3Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem3;
#[doc = "ADC12MEM4 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem4`] module"]
#[doc(alias = "ADC12MEM4")]
pub type Adc12mem4 = crate::Reg<adc12mem4::Adc12mem4Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem4;
#[doc = "ADC12MEM5 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem5`] module"]
#[doc(alias = "ADC12MEM5")]
pub type Adc12mem5 = crate::Reg<adc12mem5::Adc12mem5Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem5;
#[doc = "ADC12MEM6 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem6`] module"]
#[doc(alias = "ADC12MEM6")]
pub type Adc12mem6 = crate::Reg<adc12mem6::Adc12mem6Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem6;
#[doc = "ADC12MEM7 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem7`] module"]
#[doc(alias = "ADC12MEM7")]
pub type Adc12mem7 = crate::Reg<adc12mem7::Adc12mem7Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem7;
#[doc = "ADC12MEM8 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem8`] module"]
#[doc(alias = "ADC12MEM8")]
pub type Adc12mem8 = crate::Reg<adc12mem8::Adc12mem8Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem8;
#[doc = "ADC12MEM9 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem9`] module"]
#[doc(alias = "ADC12MEM9")]
pub type Adc12mem9 = crate::Reg<adc12mem9::Adc12mem9Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem9;
#[doc = "ADC12MEM10 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem10`] module"]
#[doc(alias = "ADC12MEM10")]
pub type Adc12mem10 = crate::Reg<adc12mem10::Adc12mem10Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem10;
#[doc = "ADC12MEM11 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem11`] module"]
#[doc(alias = "ADC12MEM11")]
pub type Adc12mem11 = crate::Reg<adc12mem11::Adc12mem11Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem11;
#[doc = "ADC12MEM12 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem12`] module"]
#[doc(alias = "ADC12MEM12")]
pub type Adc12mem12 = crate::Reg<adc12mem12::Adc12mem12Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem12;
#[doc = "ADC12MEM13 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem13`] module"]
#[doc(alias = "ADC12MEM13")]
pub type Adc12mem13 = crate::Reg<adc12mem13::Adc12mem13Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem13;
#[doc = "ADC12MEM14 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem14`] module"]
#[doc(alias = "ADC12MEM14")]
pub type Adc12mem14 = crate::Reg<adc12mem14::Adc12mem14Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem14;
#[doc = "ADC12MEM15 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem15`] module"]
#[doc(alias = "ADC12MEM15")]
pub type Adc12mem15 = crate::Reg<adc12mem15::Adc12mem15Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem15;
#[doc = "ADC12MEM16 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem16`] module"]
#[doc(alias = "ADC12MEM16")]
pub type Adc12mem16 = crate::Reg<adc12mem16::Adc12mem16Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem16;
#[doc = "ADC12MEM17 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem17`] module"]
#[doc(alias = "ADC12MEM17")]
pub type Adc12mem17 = crate::Reg<adc12mem17::Adc12mem17Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem17;
#[doc = "ADC12MEM18 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem18`] module"]
#[doc(alias = "ADC12MEM18")]
pub type Adc12mem18 = crate::Reg<adc12mem18::Adc12mem18Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem18;
#[doc = "ADC12MEM19 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem19`] module"]
#[doc(alias = "ADC12MEM19")]
pub type Adc12mem19 = crate::Reg<adc12mem19::Adc12mem19Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem19;
#[doc = "ADC12MEM20 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem20`] module"]
#[doc(alias = "ADC12MEM20")]
pub type Adc12mem20 = crate::Reg<adc12mem20::Adc12mem20Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem20;
#[doc = "ADC12MEM21 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem21`] module"]
#[doc(alias = "ADC12MEM21")]
pub type Adc12mem21 = crate::Reg<adc12mem21::Adc12mem21Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem21;
#[doc = "ADC12MEM22 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem22`] module"]
#[doc(alias = "ADC12MEM22")]
pub type Adc12mem22 = crate::Reg<adc12mem22::Adc12mem22Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem22;
#[doc = "ADC12MEM23 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem23`] module"]
#[doc(alias = "ADC12MEM23")]
pub type Adc12mem23 = crate::Reg<adc12mem23::Adc12mem23Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem23;
#[doc = "ADC12MEM24 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem24`] module"]
#[doc(alias = "ADC12MEM24")]
pub type Adc12mem24 = crate::Reg<adc12mem24::Adc12mem24Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem24;
#[doc = "ADC12MEM25 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem25`] module"]
#[doc(alias = "ADC12MEM25")]
pub type Adc12mem25 = crate::Reg<adc12mem25::Adc12mem25Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem25;
#[doc = "ADC12MEM26 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem26`] module"]
#[doc(alias = "ADC12MEM26")]
pub type Adc12mem26 = crate::Reg<adc12mem26::Adc12mem26Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem26;
#[doc = "ADC12MEM27 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem27`] module"]
#[doc(alias = "ADC12MEM27")]
pub type Adc12mem27 = crate::Reg<adc12mem27::Adc12mem27Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem27;
#[doc = "ADC12MEM28 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem28`] module"]
#[doc(alias = "ADC12MEM28")]
pub type Adc12mem28 = crate::Reg<adc12mem28::Adc12mem28Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem28;
#[doc = "ADC12MEM29 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem29`] module"]
#[doc(alias = "ADC12MEM29")]
pub type Adc12mem29 = crate::Reg<adc12mem29::Adc12mem29Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem29;
#[doc = "ADC12MEM30 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem30`] module"]
#[doc(alias = "ADC12MEM30")]
pub type Adc12mem30 = crate::Reg<adc12mem30::Adc12mem30Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem30;
#[doc = "ADC12MEM31 (rw) register accessor: ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem31`] module"]
#[doc(alias = "ADC12MEM31")]
pub type Adc12mem31 = crate::Reg<adc12mem31::Adc12mem31Spec>;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem31;
