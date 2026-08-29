#[doc = "Register `DMACTL2` reader"]
pub type R = crate::R<Dmactl2Spec>;
#[doc = "Register `DMACTL2` writer"]
pub type W = crate::W<Dmactl2Spec>;
#[doc = "DMA trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma4tsel {
    #[doc = "0: DMA4TRIG0"]
    Dma4trig0 = 0,
    #[doc = "1: DMA4TRIG1"]
    Dma4trig1 = 1,
    #[doc = "2: DMA4TRIG2"]
    Dma4trig2 = 2,
    #[doc = "3: DMA4TRIG3"]
    Dma4trig3 = 3,
    #[doc = "4: DMA4TRIG4"]
    Dma4trig4 = 4,
    #[doc = "5: DMA4TRIG5"]
    Dma4trig5 = 5,
    #[doc = "6: DMA4TRIG6"]
    Dma4trig6 = 6,
    #[doc = "7: DMA4TRIG7"]
    Dma4trig7 = 7,
    #[doc = "8: DMA4TRIG8"]
    Dma4trig8 = 8,
    #[doc = "9: DMA4TRIG9"]
    Dma4trig9 = 9,
    #[doc = "10: DMA4TRIG10"]
    Dma4trig10 = 10,
    #[doc = "11: DMA4TRIG11"]
    Dma4trig11 = 11,
    #[doc = "12: DMA4TRIG12"]
    Dma4trig12 = 12,
    #[doc = "13: DMA4TRIG13"]
    Dma4trig13 = 13,
    #[doc = "14: DMA4TRIG14"]
    Dma4trig14 = 14,
    #[doc = "15: DMA4TRIG15"]
    Dma4trig15 = 15,
    #[doc = "16: DMA4TRIG16"]
    Dma4trig16 = 16,
    #[doc = "17: DMA4TRIG17"]
    Dma4trig17 = 17,
    #[doc = "18: DMA4TRIG18"]
    Dma4trig18 = 18,
    #[doc = "19: DMA4TRIG19"]
    Dma4trig19 = 19,
    #[doc = "20: DMA4TRIG20"]
    Dma4trig20 = 20,
    #[doc = "21: DMA4TRIG21"]
    Dma4trig21 = 21,
    #[doc = "22: DMA4TRIG22"]
    Dma4trig22 = 22,
    #[doc = "23: DMA4TRIG23"]
    Dma4trig23 = 23,
    #[doc = "24: DMA4TRIG24"]
    Dma4trig24 = 24,
    #[doc = "25: DMA4TRIG25"]
    Dma4trig25 = 25,
    #[doc = "26: DMA4TRIG26"]
    Dma4trig26 = 26,
    #[doc = "27: DMA4TRIG27"]
    Dma4trig27 = 27,
    #[doc = "28: DMA4TRIG28"]
    Dma4trig28 = 28,
    #[doc = "29: DMA4TRIG29"]
    Dma4trig29 = 29,
    #[doc = "30: DMA4TRIG30"]
    Dma4trig30 = 30,
    #[doc = "31: DMA4TRIG31"]
    Dma4trig31 = 31,
}
impl From<Dma4tsel> for u8 {
    #[inline(always)]
    fn from(variant: Dma4tsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma4tsel {
    type Ux = u8;
}
impl crate::IsEnum for Dma4tsel {}
#[doc = "Field `DMA4TSEL` reader - DMA trigger select"]
pub type Dma4tselR = crate::FieldReader<Dma4tsel>;
impl Dma4tselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma4tsel {
        match self.bits {
            0 => Dma4tsel::Dma4trig0,
            1 => Dma4tsel::Dma4trig1,
            2 => Dma4tsel::Dma4trig2,
            3 => Dma4tsel::Dma4trig3,
            4 => Dma4tsel::Dma4trig4,
            5 => Dma4tsel::Dma4trig5,
            6 => Dma4tsel::Dma4trig6,
            7 => Dma4tsel::Dma4trig7,
            8 => Dma4tsel::Dma4trig8,
            9 => Dma4tsel::Dma4trig9,
            10 => Dma4tsel::Dma4trig10,
            11 => Dma4tsel::Dma4trig11,
            12 => Dma4tsel::Dma4trig12,
            13 => Dma4tsel::Dma4trig13,
            14 => Dma4tsel::Dma4trig14,
            15 => Dma4tsel::Dma4trig15,
            16 => Dma4tsel::Dma4trig16,
            17 => Dma4tsel::Dma4trig17,
            18 => Dma4tsel::Dma4trig18,
            19 => Dma4tsel::Dma4trig19,
            20 => Dma4tsel::Dma4trig20,
            21 => Dma4tsel::Dma4trig21,
            22 => Dma4tsel::Dma4trig22,
            23 => Dma4tsel::Dma4trig23,
            24 => Dma4tsel::Dma4trig24,
            25 => Dma4tsel::Dma4trig25,
            26 => Dma4tsel::Dma4trig26,
            27 => Dma4tsel::Dma4trig27,
            28 => Dma4tsel::Dma4trig28,
            29 => Dma4tsel::Dma4trig29,
            30 => Dma4tsel::Dma4trig30,
            31 => Dma4tsel::Dma4trig31,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA4TRIG0"]
    #[inline(always)]
    pub fn is_dma4trig0(&self) -> bool {
        *self == Dma4tsel::Dma4trig0
    }
    #[doc = "DMA4TRIG1"]
    #[inline(always)]
    pub fn is_dma4trig1(&self) -> bool {
        *self == Dma4tsel::Dma4trig1
    }
    #[doc = "DMA4TRIG2"]
    #[inline(always)]
    pub fn is_dma4trig2(&self) -> bool {
        *self == Dma4tsel::Dma4trig2
    }
    #[doc = "DMA4TRIG3"]
    #[inline(always)]
    pub fn is_dma4trig3(&self) -> bool {
        *self == Dma4tsel::Dma4trig3
    }
    #[doc = "DMA4TRIG4"]
    #[inline(always)]
    pub fn is_dma4trig4(&self) -> bool {
        *self == Dma4tsel::Dma4trig4
    }
    #[doc = "DMA4TRIG5"]
    #[inline(always)]
    pub fn is_dma4trig5(&self) -> bool {
        *self == Dma4tsel::Dma4trig5
    }
    #[doc = "DMA4TRIG6"]
    #[inline(always)]
    pub fn is_dma4trig6(&self) -> bool {
        *self == Dma4tsel::Dma4trig6
    }
    #[doc = "DMA4TRIG7"]
    #[inline(always)]
    pub fn is_dma4trig7(&self) -> bool {
        *self == Dma4tsel::Dma4trig7
    }
    #[doc = "DMA4TRIG8"]
    #[inline(always)]
    pub fn is_dma4trig8(&self) -> bool {
        *self == Dma4tsel::Dma4trig8
    }
    #[doc = "DMA4TRIG9"]
    #[inline(always)]
    pub fn is_dma4trig9(&self) -> bool {
        *self == Dma4tsel::Dma4trig9
    }
    #[doc = "DMA4TRIG10"]
    #[inline(always)]
    pub fn is_dma4trig10(&self) -> bool {
        *self == Dma4tsel::Dma4trig10
    }
    #[doc = "DMA4TRIG11"]
    #[inline(always)]
    pub fn is_dma4trig11(&self) -> bool {
        *self == Dma4tsel::Dma4trig11
    }
    #[doc = "DMA4TRIG12"]
    #[inline(always)]
    pub fn is_dma4trig12(&self) -> bool {
        *self == Dma4tsel::Dma4trig12
    }
    #[doc = "DMA4TRIG13"]
    #[inline(always)]
    pub fn is_dma4trig13(&self) -> bool {
        *self == Dma4tsel::Dma4trig13
    }
    #[doc = "DMA4TRIG14"]
    #[inline(always)]
    pub fn is_dma4trig14(&self) -> bool {
        *self == Dma4tsel::Dma4trig14
    }
    #[doc = "DMA4TRIG15"]
    #[inline(always)]
    pub fn is_dma4trig15(&self) -> bool {
        *self == Dma4tsel::Dma4trig15
    }
    #[doc = "DMA4TRIG16"]
    #[inline(always)]
    pub fn is_dma4trig16(&self) -> bool {
        *self == Dma4tsel::Dma4trig16
    }
    #[doc = "DMA4TRIG17"]
    #[inline(always)]
    pub fn is_dma4trig17(&self) -> bool {
        *self == Dma4tsel::Dma4trig17
    }
    #[doc = "DMA4TRIG18"]
    #[inline(always)]
    pub fn is_dma4trig18(&self) -> bool {
        *self == Dma4tsel::Dma4trig18
    }
    #[doc = "DMA4TRIG19"]
    #[inline(always)]
    pub fn is_dma4trig19(&self) -> bool {
        *self == Dma4tsel::Dma4trig19
    }
    #[doc = "DMA4TRIG20"]
    #[inline(always)]
    pub fn is_dma4trig20(&self) -> bool {
        *self == Dma4tsel::Dma4trig20
    }
    #[doc = "DMA4TRIG21"]
    #[inline(always)]
    pub fn is_dma4trig21(&self) -> bool {
        *self == Dma4tsel::Dma4trig21
    }
    #[doc = "DMA4TRIG22"]
    #[inline(always)]
    pub fn is_dma4trig22(&self) -> bool {
        *self == Dma4tsel::Dma4trig22
    }
    #[doc = "DMA4TRIG23"]
    #[inline(always)]
    pub fn is_dma4trig23(&self) -> bool {
        *self == Dma4tsel::Dma4trig23
    }
    #[doc = "DMA4TRIG24"]
    #[inline(always)]
    pub fn is_dma4trig24(&self) -> bool {
        *self == Dma4tsel::Dma4trig24
    }
    #[doc = "DMA4TRIG25"]
    #[inline(always)]
    pub fn is_dma4trig25(&self) -> bool {
        *self == Dma4tsel::Dma4trig25
    }
    #[doc = "DMA4TRIG26"]
    #[inline(always)]
    pub fn is_dma4trig26(&self) -> bool {
        *self == Dma4tsel::Dma4trig26
    }
    #[doc = "DMA4TRIG27"]
    #[inline(always)]
    pub fn is_dma4trig27(&self) -> bool {
        *self == Dma4tsel::Dma4trig27
    }
    #[doc = "DMA4TRIG28"]
    #[inline(always)]
    pub fn is_dma4trig28(&self) -> bool {
        *self == Dma4tsel::Dma4trig28
    }
    #[doc = "DMA4TRIG29"]
    #[inline(always)]
    pub fn is_dma4trig29(&self) -> bool {
        *self == Dma4tsel::Dma4trig29
    }
    #[doc = "DMA4TRIG30"]
    #[inline(always)]
    pub fn is_dma4trig30(&self) -> bool {
        *self == Dma4tsel::Dma4trig30
    }
    #[doc = "DMA4TRIG31"]
    #[inline(always)]
    pub fn is_dma4trig31(&self) -> bool {
        *self == Dma4tsel::Dma4trig31
    }
}
#[doc = "Field `DMA4TSEL` writer - DMA trigger select"]
pub type Dma4tselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Dma4tsel, crate::Safe>;
impl<'a, REG> Dma4tselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA4TRIG0"]
    #[inline(always)]
    pub fn dma4trig0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig0)
    }
    #[doc = "DMA4TRIG1"]
    #[inline(always)]
    pub fn dma4trig1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig1)
    }
    #[doc = "DMA4TRIG2"]
    #[inline(always)]
    pub fn dma4trig2(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig2)
    }
    #[doc = "DMA4TRIG3"]
    #[inline(always)]
    pub fn dma4trig3(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig3)
    }
    #[doc = "DMA4TRIG4"]
    #[inline(always)]
    pub fn dma4trig4(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig4)
    }
    #[doc = "DMA4TRIG5"]
    #[inline(always)]
    pub fn dma4trig5(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig5)
    }
    #[doc = "DMA4TRIG6"]
    #[inline(always)]
    pub fn dma4trig6(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig6)
    }
    #[doc = "DMA4TRIG7"]
    #[inline(always)]
    pub fn dma4trig7(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig7)
    }
    #[doc = "DMA4TRIG8"]
    #[inline(always)]
    pub fn dma4trig8(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig8)
    }
    #[doc = "DMA4TRIG9"]
    #[inline(always)]
    pub fn dma4trig9(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig9)
    }
    #[doc = "DMA4TRIG10"]
    #[inline(always)]
    pub fn dma4trig10(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig10)
    }
    #[doc = "DMA4TRIG11"]
    #[inline(always)]
    pub fn dma4trig11(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig11)
    }
    #[doc = "DMA4TRIG12"]
    #[inline(always)]
    pub fn dma4trig12(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig12)
    }
    #[doc = "DMA4TRIG13"]
    #[inline(always)]
    pub fn dma4trig13(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig13)
    }
    #[doc = "DMA4TRIG14"]
    #[inline(always)]
    pub fn dma4trig14(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig14)
    }
    #[doc = "DMA4TRIG15"]
    #[inline(always)]
    pub fn dma4trig15(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig15)
    }
    #[doc = "DMA4TRIG16"]
    #[inline(always)]
    pub fn dma4trig16(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig16)
    }
    #[doc = "DMA4TRIG17"]
    #[inline(always)]
    pub fn dma4trig17(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig17)
    }
    #[doc = "DMA4TRIG18"]
    #[inline(always)]
    pub fn dma4trig18(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig18)
    }
    #[doc = "DMA4TRIG19"]
    #[inline(always)]
    pub fn dma4trig19(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig19)
    }
    #[doc = "DMA4TRIG20"]
    #[inline(always)]
    pub fn dma4trig20(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig20)
    }
    #[doc = "DMA4TRIG21"]
    #[inline(always)]
    pub fn dma4trig21(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig21)
    }
    #[doc = "DMA4TRIG22"]
    #[inline(always)]
    pub fn dma4trig22(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig22)
    }
    #[doc = "DMA4TRIG23"]
    #[inline(always)]
    pub fn dma4trig23(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig23)
    }
    #[doc = "DMA4TRIG24"]
    #[inline(always)]
    pub fn dma4trig24(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig24)
    }
    #[doc = "DMA4TRIG25"]
    #[inline(always)]
    pub fn dma4trig25(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig25)
    }
    #[doc = "DMA4TRIG26"]
    #[inline(always)]
    pub fn dma4trig26(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig26)
    }
    #[doc = "DMA4TRIG27"]
    #[inline(always)]
    pub fn dma4trig27(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig27)
    }
    #[doc = "DMA4TRIG28"]
    #[inline(always)]
    pub fn dma4trig28(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig28)
    }
    #[doc = "DMA4TRIG29"]
    #[inline(always)]
    pub fn dma4trig29(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig29)
    }
    #[doc = "DMA4TRIG30"]
    #[inline(always)]
    pub fn dma4trig30(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig30)
    }
    #[doc = "DMA4TRIG31"]
    #[inline(always)]
    pub fn dma4trig31(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4tsel::Dma4trig31)
    }
}
#[doc = "DMA trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma5tsel {
    #[doc = "0: DMA5TRIG0"]
    Dma5trig0 = 0,
    #[doc = "1: DMA5TRIG1"]
    Dma5trig1 = 1,
    #[doc = "2: DMA5TRIG2"]
    Dma5trig2 = 2,
    #[doc = "3: DMA5TRIG3"]
    Dma5trig3 = 3,
    #[doc = "4: DMA5TRIG4"]
    Dma5trig4 = 4,
    #[doc = "5: DMA5TRIG5"]
    Dma5trig5 = 5,
    #[doc = "6: DMA5TRIG6"]
    Dma5trig6 = 6,
    #[doc = "7: DMA5TRIG7"]
    Dma5trig7 = 7,
    #[doc = "8: DMA5TRIG8"]
    Dma5trig8 = 8,
    #[doc = "9: DMA5TRIG9"]
    Dma5trig9 = 9,
    #[doc = "10: DMA5TRIG10"]
    Dma5trig10 = 10,
    #[doc = "11: DMA5TRIG11"]
    Dma5trig11 = 11,
    #[doc = "12: DMA5TRIG12"]
    Dma5trig12 = 12,
    #[doc = "13: DMA5TRIG13"]
    Dma5trig13 = 13,
    #[doc = "14: DMA5TRIG14"]
    Dma5trig14 = 14,
    #[doc = "15: DMA5TRIG15"]
    Dma5trig15 = 15,
    #[doc = "16: DMA5TRIG16"]
    Dma5trig16 = 16,
    #[doc = "17: DMA5TRIG17"]
    Dma5trig17 = 17,
    #[doc = "18: DMA5TRIG18"]
    Dma5trig18 = 18,
    #[doc = "19: DMA5TRIG19"]
    Dma5trig19 = 19,
    #[doc = "20: DMA5TRIG20"]
    Dma5trig20 = 20,
    #[doc = "21: DMA5TRIG21"]
    Dma5trig21 = 21,
    #[doc = "22: DMA5TRIG22"]
    Dma5trig22 = 22,
    #[doc = "23: DMA5TRIG23"]
    Dma5trig23 = 23,
    #[doc = "24: DMA5TRIG24"]
    Dma5trig24 = 24,
    #[doc = "25: DMA5TRIG25"]
    Dma5trig25 = 25,
    #[doc = "26: DMA5TRIG26"]
    Dma5trig26 = 26,
    #[doc = "27: DMA5TRIG27"]
    Dma5trig27 = 27,
    #[doc = "28: DMA5TRIG28"]
    Dma5trig28 = 28,
    #[doc = "29: DMA5TRIG29"]
    Dma5trig29 = 29,
    #[doc = "30: DMA5TRIG30"]
    Dma5trig30 = 30,
    #[doc = "31: DMA5TRIG31"]
    Dma5trig31 = 31,
}
impl From<Dma5tsel> for u8 {
    #[inline(always)]
    fn from(variant: Dma5tsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma5tsel {
    type Ux = u8;
}
impl crate::IsEnum for Dma5tsel {}
#[doc = "Field `DMA5TSEL` reader - DMA trigger select"]
pub type Dma5tselR = crate::FieldReader<Dma5tsel>;
impl Dma5tselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma5tsel {
        match self.bits {
            0 => Dma5tsel::Dma5trig0,
            1 => Dma5tsel::Dma5trig1,
            2 => Dma5tsel::Dma5trig2,
            3 => Dma5tsel::Dma5trig3,
            4 => Dma5tsel::Dma5trig4,
            5 => Dma5tsel::Dma5trig5,
            6 => Dma5tsel::Dma5trig6,
            7 => Dma5tsel::Dma5trig7,
            8 => Dma5tsel::Dma5trig8,
            9 => Dma5tsel::Dma5trig9,
            10 => Dma5tsel::Dma5trig10,
            11 => Dma5tsel::Dma5trig11,
            12 => Dma5tsel::Dma5trig12,
            13 => Dma5tsel::Dma5trig13,
            14 => Dma5tsel::Dma5trig14,
            15 => Dma5tsel::Dma5trig15,
            16 => Dma5tsel::Dma5trig16,
            17 => Dma5tsel::Dma5trig17,
            18 => Dma5tsel::Dma5trig18,
            19 => Dma5tsel::Dma5trig19,
            20 => Dma5tsel::Dma5trig20,
            21 => Dma5tsel::Dma5trig21,
            22 => Dma5tsel::Dma5trig22,
            23 => Dma5tsel::Dma5trig23,
            24 => Dma5tsel::Dma5trig24,
            25 => Dma5tsel::Dma5trig25,
            26 => Dma5tsel::Dma5trig26,
            27 => Dma5tsel::Dma5trig27,
            28 => Dma5tsel::Dma5trig28,
            29 => Dma5tsel::Dma5trig29,
            30 => Dma5tsel::Dma5trig30,
            31 => Dma5tsel::Dma5trig31,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA5TRIG0"]
    #[inline(always)]
    pub fn is_dma5trig0(&self) -> bool {
        *self == Dma5tsel::Dma5trig0
    }
    #[doc = "DMA5TRIG1"]
    #[inline(always)]
    pub fn is_dma5trig1(&self) -> bool {
        *self == Dma5tsel::Dma5trig1
    }
    #[doc = "DMA5TRIG2"]
    #[inline(always)]
    pub fn is_dma5trig2(&self) -> bool {
        *self == Dma5tsel::Dma5trig2
    }
    #[doc = "DMA5TRIG3"]
    #[inline(always)]
    pub fn is_dma5trig3(&self) -> bool {
        *self == Dma5tsel::Dma5trig3
    }
    #[doc = "DMA5TRIG4"]
    #[inline(always)]
    pub fn is_dma5trig4(&self) -> bool {
        *self == Dma5tsel::Dma5trig4
    }
    #[doc = "DMA5TRIG5"]
    #[inline(always)]
    pub fn is_dma5trig5(&self) -> bool {
        *self == Dma5tsel::Dma5trig5
    }
    #[doc = "DMA5TRIG6"]
    #[inline(always)]
    pub fn is_dma5trig6(&self) -> bool {
        *self == Dma5tsel::Dma5trig6
    }
    #[doc = "DMA5TRIG7"]
    #[inline(always)]
    pub fn is_dma5trig7(&self) -> bool {
        *self == Dma5tsel::Dma5trig7
    }
    #[doc = "DMA5TRIG8"]
    #[inline(always)]
    pub fn is_dma5trig8(&self) -> bool {
        *self == Dma5tsel::Dma5trig8
    }
    #[doc = "DMA5TRIG9"]
    #[inline(always)]
    pub fn is_dma5trig9(&self) -> bool {
        *self == Dma5tsel::Dma5trig9
    }
    #[doc = "DMA5TRIG10"]
    #[inline(always)]
    pub fn is_dma5trig10(&self) -> bool {
        *self == Dma5tsel::Dma5trig10
    }
    #[doc = "DMA5TRIG11"]
    #[inline(always)]
    pub fn is_dma5trig11(&self) -> bool {
        *self == Dma5tsel::Dma5trig11
    }
    #[doc = "DMA5TRIG12"]
    #[inline(always)]
    pub fn is_dma5trig12(&self) -> bool {
        *self == Dma5tsel::Dma5trig12
    }
    #[doc = "DMA5TRIG13"]
    #[inline(always)]
    pub fn is_dma5trig13(&self) -> bool {
        *self == Dma5tsel::Dma5trig13
    }
    #[doc = "DMA5TRIG14"]
    #[inline(always)]
    pub fn is_dma5trig14(&self) -> bool {
        *self == Dma5tsel::Dma5trig14
    }
    #[doc = "DMA5TRIG15"]
    #[inline(always)]
    pub fn is_dma5trig15(&self) -> bool {
        *self == Dma5tsel::Dma5trig15
    }
    #[doc = "DMA5TRIG16"]
    #[inline(always)]
    pub fn is_dma5trig16(&self) -> bool {
        *self == Dma5tsel::Dma5trig16
    }
    #[doc = "DMA5TRIG17"]
    #[inline(always)]
    pub fn is_dma5trig17(&self) -> bool {
        *self == Dma5tsel::Dma5trig17
    }
    #[doc = "DMA5TRIG18"]
    #[inline(always)]
    pub fn is_dma5trig18(&self) -> bool {
        *self == Dma5tsel::Dma5trig18
    }
    #[doc = "DMA5TRIG19"]
    #[inline(always)]
    pub fn is_dma5trig19(&self) -> bool {
        *self == Dma5tsel::Dma5trig19
    }
    #[doc = "DMA5TRIG20"]
    #[inline(always)]
    pub fn is_dma5trig20(&self) -> bool {
        *self == Dma5tsel::Dma5trig20
    }
    #[doc = "DMA5TRIG21"]
    #[inline(always)]
    pub fn is_dma5trig21(&self) -> bool {
        *self == Dma5tsel::Dma5trig21
    }
    #[doc = "DMA5TRIG22"]
    #[inline(always)]
    pub fn is_dma5trig22(&self) -> bool {
        *self == Dma5tsel::Dma5trig22
    }
    #[doc = "DMA5TRIG23"]
    #[inline(always)]
    pub fn is_dma5trig23(&self) -> bool {
        *self == Dma5tsel::Dma5trig23
    }
    #[doc = "DMA5TRIG24"]
    #[inline(always)]
    pub fn is_dma5trig24(&self) -> bool {
        *self == Dma5tsel::Dma5trig24
    }
    #[doc = "DMA5TRIG25"]
    #[inline(always)]
    pub fn is_dma5trig25(&self) -> bool {
        *self == Dma5tsel::Dma5trig25
    }
    #[doc = "DMA5TRIG26"]
    #[inline(always)]
    pub fn is_dma5trig26(&self) -> bool {
        *self == Dma5tsel::Dma5trig26
    }
    #[doc = "DMA5TRIG27"]
    #[inline(always)]
    pub fn is_dma5trig27(&self) -> bool {
        *self == Dma5tsel::Dma5trig27
    }
    #[doc = "DMA5TRIG28"]
    #[inline(always)]
    pub fn is_dma5trig28(&self) -> bool {
        *self == Dma5tsel::Dma5trig28
    }
    #[doc = "DMA5TRIG29"]
    #[inline(always)]
    pub fn is_dma5trig29(&self) -> bool {
        *self == Dma5tsel::Dma5trig29
    }
    #[doc = "DMA5TRIG30"]
    #[inline(always)]
    pub fn is_dma5trig30(&self) -> bool {
        *self == Dma5tsel::Dma5trig30
    }
    #[doc = "DMA5TRIG31"]
    #[inline(always)]
    pub fn is_dma5trig31(&self) -> bool {
        *self == Dma5tsel::Dma5trig31
    }
}
#[doc = "Field `DMA5TSEL` writer - DMA trigger select"]
pub type Dma5tselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Dma5tsel, crate::Safe>;
impl<'a, REG> Dma5tselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA5TRIG0"]
    #[inline(always)]
    pub fn dma5trig0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig0)
    }
    #[doc = "DMA5TRIG1"]
    #[inline(always)]
    pub fn dma5trig1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig1)
    }
    #[doc = "DMA5TRIG2"]
    #[inline(always)]
    pub fn dma5trig2(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig2)
    }
    #[doc = "DMA5TRIG3"]
    #[inline(always)]
    pub fn dma5trig3(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig3)
    }
    #[doc = "DMA5TRIG4"]
    #[inline(always)]
    pub fn dma5trig4(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig4)
    }
    #[doc = "DMA5TRIG5"]
    #[inline(always)]
    pub fn dma5trig5(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig5)
    }
    #[doc = "DMA5TRIG6"]
    #[inline(always)]
    pub fn dma5trig6(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig6)
    }
    #[doc = "DMA5TRIG7"]
    #[inline(always)]
    pub fn dma5trig7(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig7)
    }
    #[doc = "DMA5TRIG8"]
    #[inline(always)]
    pub fn dma5trig8(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig8)
    }
    #[doc = "DMA5TRIG9"]
    #[inline(always)]
    pub fn dma5trig9(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig9)
    }
    #[doc = "DMA5TRIG10"]
    #[inline(always)]
    pub fn dma5trig10(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig10)
    }
    #[doc = "DMA5TRIG11"]
    #[inline(always)]
    pub fn dma5trig11(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig11)
    }
    #[doc = "DMA5TRIG12"]
    #[inline(always)]
    pub fn dma5trig12(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig12)
    }
    #[doc = "DMA5TRIG13"]
    #[inline(always)]
    pub fn dma5trig13(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig13)
    }
    #[doc = "DMA5TRIG14"]
    #[inline(always)]
    pub fn dma5trig14(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig14)
    }
    #[doc = "DMA5TRIG15"]
    #[inline(always)]
    pub fn dma5trig15(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig15)
    }
    #[doc = "DMA5TRIG16"]
    #[inline(always)]
    pub fn dma5trig16(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig16)
    }
    #[doc = "DMA5TRIG17"]
    #[inline(always)]
    pub fn dma5trig17(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig17)
    }
    #[doc = "DMA5TRIG18"]
    #[inline(always)]
    pub fn dma5trig18(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig18)
    }
    #[doc = "DMA5TRIG19"]
    #[inline(always)]
    pub fn dma5trig19(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig19)
    }
    #[doc = "DMA5TRIG20"]
    #[inline(always)]
    pub fn dma5trig20(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig20)
    }
    #[doc = "DMA5TRIG21"]
    #[inline(always)]
    pub fn dma5trig21(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig21)
    }
    #[doc = "DMA5TRIG22"]
    #[inline(always)]
    pub fn dma5trig22(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig22)
    }
    #[doc = "DMA5TRIG23"]
    #[inline(always)]
    pub fn dma5trig23(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig23)
    }
    #[doc = "DMA5TRIG24"]
    #[inline(always)]
    pub fn dma5trig24(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig24)
    }
    #[doc = "DMA5TRIG25"]
    #[inline(always)]
    pub fn dma5trig25(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig25)
    }
    #[doc = "DMA5TRIG26"]
    #[inline(always)]
    pub fn dma5trig26(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig26)
    }
    #[doc = "DMA5TRIG27"]
    #[inline(always)]
    pub fn dma5trig27(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig27)
    }
    #[doc = "DMA5TRIG28"]
    #[inline(always)]
    pub fn dma5trig28(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig28)
    }
    #[doc = "DMA5TRIG29"]
    #[inline(always)]
    pub fn dma5trig29(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig29)
    }
    #[doc = "DMA5TRIG30"]
    #[inline(always)]
    pub fn dma5trig30(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig30)
    }
    #[doc = "DMA5TRIG31"]
    #[inline(always)]
    pub fn dma5trig31(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5tsel::Dma5trig31)
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA trigger select"]
    #[inline(always)]
    pub fn dma4tsel(&self) -> Dma4tselR {
        Dma4tselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA trigger select"]
    #[inline(always)]
    pub fn dma5tsel(&self) -> Dma5tselR {
        Dma5tselR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA trigger select"]
    #[inline(always)]
    pub fn dma4tsel(&mut self) -> Dma4tselW<'_, Dmactl2Spec> {
        Dma4tselW::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA trigger select"]
    #[inline(always)]
    pub fn dma5tsel(&mut self) -> Dma5tselW<'_, Dmactl2Spec> {
        Dma5tselW::new(self, 8)
    }
}
#[doc = "DMA Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmactl2Spec;
impl crate::RegisterSpec for Dmactl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmactl2::R`](R) reader structure"]
impl crate::Readable for Dmactl2Spec {}
#[doc = "`write(|w| ..)` method takes [`dmactl2::W`](W) writer structure"]
impl crate::Writable for Dmactl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACTL2 to value 0"]
impl crate::Resettable for Dmactl2Spec {}
