#[doc = "Register `DMACTL0` reader"]
pub type R = crate::R<Dmactl0Spec>;
#[doc = "Register `DMACTL0` writer"]
pub type W = crate::W<Dmactl0Spec>;
#[doc = "DMA trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma0tsel {
    #[doc = "0: DMA0TRIG0"]
    Dma0trig0 = 0,
    #[doc = "1: DMA0TRIG1"]
    Dma0trig1 = 1,
    #[doc = "2: DMA0TRIG2"]
    Dma0trig2 = 2,
    #[doc = "3: DMA0TRIG3"]
    Dma0trig3 = 3,
    #[doc = "4: DMA0TRIG4"]
    Dma0trig4 = 4,
    #[doc = "5: DMA0TRIG5"]
    Dma0trig5 = 5,
    #[doc = "6: DMA0TRIG6"]
    Dma0trig6 = 6,
    #[doc = "7: DMA0TRIG7"]
    Dma0trig7 = 7,
    #[doc = "8: DMA0TRIG8"]
    Dma0trig8 = 8,
    #[doc = "9: DMA0TRIG9"]
    Dma0trig9 = 9,
    #[doc = "10: DMA0TRIG10"]
    Dma0trig10 = 10,
    #[doc = "11: DMA0TRIG11"]
    Dma0trig11 = 11,
    #[doc = "12: DMA0TRIG12"]
    Dma0trig12 = 12,
    #[doc = "13: DMA0TRIG13"]
    Dma0trig13 = 13,
    #[doc = "14: DMA0TRIG14"]
    Dma0trig14 = 14,
    #[doc = "15: DMA0TRIG15"]
    Dma0trig15 = 15,
    #[doc = "16: DMA0TRIG16"]
    Dma0trig16 = 16,
    #[doc = "17: DMA0TRIG17"]
    Dma0trig17 = 17,
    #[doc = "18: DMA0TRIG18"]
    Dma0trig18 = 18,
    #[doc = "19: DMA0TRIG19"]
    Dma0trig19 = 19,
    #[doc = "20: DMA0TRIG20"]
    Dma0trig20 = 20,
    #[doc = "21: DMA0TRIG21"]
    Dma0trig21 = 21,
    #[doc = "22: DMA0TRIG22"]
    Dma0trig22 = 22,
    #[doc = "23: DMA0TRIG23"]
    Dma0trig23 = 23,
    #[doc = "24: DMA0TRIG24"]
    Dma0trig24 = 24,
    #[doc = "25: DMA0TRIG25"]
    Dma0trig25 = 25,
    #[doc = "26: DMA0TRIG26"]
    Dma0trig26 = 26,
    #[doc = "27: DMA0TRIG27"]
    Dma0trig27 = 27,
    #[doc = "28: DMA0TRIG28"]
    Dma0trig28 = 28,
    #[doc = "29: DMA0TRIG29"]
    Dma0trig29 = 29,
    #[doc = "30: DMA0TRIG30"]
    Dma0trig30 = 30,
    #[doc = "31: DMA0TRIG31"]
    Dma0trig31 = 31,
}
impl From<Dma0tsel> for u8 {
    #[inline(always)]
    fn from(variant: Dma0tsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma0tsel {
    type Ux = u8;
}
impl crate::IsEnum for Dma0tsel {}
#[doc = "Field `DMA0TSEL` reader - DMA trigger select"]
pub type Dma0tselR = crate::FieldReader<Dma0tsel>;
impl Dma0tselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma0tsel {
        match self.bits {
            0 => Dma0tsel::Dma0trig0,
            1 => Dma0tsel::Dma0trig1,
            2 => Dma0tsel::Dma0trig2,
            3 => Dma0tsel::Dma0trig3,
            4 => Dma0tsel::Dma0trig4,
            5 => Dma0tsel::Dma0trig5,
            6 => Dma0tsel::Dma0trig6,
            7 => Dma0tsel::Dma0trig7,
            8 => Dma0tsel::Dma0trig8,
            9 => Dma0tsel::Dma0trig9,
            10 => Dma0tsel::Dma0trig10,
            11 => Dma0tsel::Dma0trig11,
            12 => Dma0tsel::Dma0trig12,
            13 => Dma0tsel::Dma0trig13,
            14 => Dma0tsel::Dma0trig14,
            15 => Dma0tsel::Dma0trig15,
            16 => Dma0tsel::Dma0trig16,
            17 => Dma0tsel::Dma0trig17,
            18 => Dma0tsel::Dma0trig18,
            19 => Dma0tsel::Dma0trig19,
            20 => Dma0tsel::Dma0trig20,
            21 => Dma0tsel::Dma0trig21,
            22 => Dma0tsel::Dma0trig22,
            23 => Dma0tsel::Dma0trig23,
            24 => Dma0tsel::Dma0trig24,
            25 => Dma0tsel::Dma0trig25,
            26 => Dma0tsel::Dma0trig26,
            27 => Dma0tsel::Dma0trig27,
            28 => Dma0tsel::Dma0trig28,
            29 => Dma0tsel::Dma0trig29,
            30 => Dma0tsel::Dma0trig30,
            31 => Dma0tsel::Dma0trig31,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA0TRIG0"]
    #[inline(always)]
    pub fn is_dma0trig0(&self) -> bool {
        *self == Dma0tsel::Dma0trig0
    }
    #[doc = "DMA0TRIG1"]
    #[inline(always)]
    pub fn is_dma0trig1(&self) -> bool {
        *self == Dma0tsel::Dma0trig1
    }
    #[doc = "DMA0TRIG2"]
    #[inline(always)]
    pub fn is_dma0trig2(&self) -> bool {
        *self == Dma0tsel::Dma0trig2
    }
    #[doc = "DMA0TRIG3"]
    #[inline(always)]
    pub fn is_dma0trig3(&self) -> bool {
        *self == Dma0tsel::Dma0trig3
    }
    #[doc = "DMA0TRIG4"]
    #[inline(always)]
    pub fn is_dma0trig4(&self) -> bool {
        *self == Dma0tsel::Dma0trig4
    }
    #[doc = "DMA0TRIG5"]
    #[inline(always)]
    pub fn is_dma0trig5(&self) -> bool {
        *self == Dma0tsel::Dma0trig5
    }
    #[doc = "DMA0TRIG6"]
    #[inline(always)]
    pub fn is_dma0trig6(&self) -> bool {
        *self == Dma0tsel::Dma0trig6
    }
    #[doc = "DMA0TRIG7"]
    #[inline(always)]
    pub fn is_dma0trig7(&self) -> bool {
        *self == Dma0tsel::Dma0trig7
    }
    #[doc = "DMA0TRIG8"]
    #[inline(always)]
    pub fn is_dma0trig8(&self) -> bool {
        *self == Dma0tsel::Dma0trig8
    }
    #[doc = "DMA0TRIG9"]
    #[inline(always)]
    pub fn is_dma0trig9(&self) -> bool {
        *self == Dma0tsel::Dma0trig9
    }
    #[doc = "DMA0TRIG10"]
    #[inline(always)]
    pub fn is_dma0trig10(&self) -> bool {
        *self == Dma0tsel::Dma0trig10
    }
    #[doc = "DMA0TRIG11"]
    #[inline(always)]
    pub fn is_dma0trig11(&self) -> bool {
        *self == Dma0tsel::Dma0trig11
    }
    #[doc = "DMA0TRIG12"]
    #[inline(always)]
    pub fn is_dma0trig12(&self) -> bool {
        *self == Dma0tsel::Dma0trig12
    }
    #[doc = "DMA0TRIG13"]
    #[inline(always)]
    pub fn is_dma0trig13(&self) -> bool {
        *self == Dma0tsel::Dma0trig13
    }
    #[doc = "DMA0TRIG14"]
    #[inline(always)]
    pub fn is_dma0trig14(&self) -> bool {
        *self == Dma0tsel::Dma0trig14
    }
    #[doc = "DMA0TRIG15"]
    #[inline(always)]
    pub fn is_dma0trig15(&self) -> bool {
        *self == Dma0tsel::Dma0trig15
    }
    #[doc = "DMA0TRIG16"]
    #[inline(always)]
    pub fn is_dma0trig16(&self) -> bool {
        *self == Dma0tsel::Dma0trig16
    }
    #[doc = "DMA0TRIG17"]
    #[inline(always)]
    pub fn is_dma0trig17(&self) -> bool {
        *self == Dma0tsel::Dma0trig17
    }
    #[doc = "DMA0TRIG18"]
    #[inline(always)]
    pub fn is_dma0trig18(&self) -> bool {
        *self == Dma0tsel::Dma0trig18
    }
    #[doc = "DMA0TRIG19"]
    #[inline(always)]
    pub fn is_dma0trig19(&self) -> bool {
        *self == Dma0tsel::Dma0trig19
    }
    #[doc = "DMA0TRIG20"]
    #[inline(always)]
    pub fn is_dma0trig20(&self) -> bool {
        *self == Dma0tsel::Dma0trig20
    }
    #[doc = "DMA0TRIG21"]
    #[inline(always)]
    pub fn is_dma0trig21(&self) -> bool {
        *self == Dma0tsel::Dma0trig21
    }
    #[doc = "DMA0TRIG22"]
    #[inline(always)]
    pub fn is_dma0trig22(&self) -> bool {
        *self == Dma0tsel::Dma0trig22
    }
    #[doc = "DMA0TRIG23"]
    #[inline(always)]
    pub fn is_dma0trig23(&self) -> bool {
        *self == Dma0tsel::Dma0trig23
    }
    #[doc = "DMA0TRIG24"]
    #[inline(always)]
    pub fn is_dma0trig24(&self) -> bool {
        *self == Dma0tsel::Dma0trig24
    }
    #[doc = "DMA0TRIG25"]
    #[inline(always)]
    pub fn is_dma0trig25(&self) -> bool {
        *self == Dma0tsel::Dma0trig25
    }
    #[doc = "DMA0TRIG26"]
    #[inline(always)]
    pub fn is_dma0trig26(&self) -> bool {
        *self == Dma0tsel::Dma0trig26
    }
    #[doc = "DMA0TRIG27"]
    #[inline(always)]
    pub fn is_dma0trig27(&self) -> bool {
        *self == Dma0tsel::Dma0trig27
    }
    #[doc = "DMA0TRIG28"]
    #[inline(always)]
    pub fn is_dma0trig28(&self) -> bool {
        *self == Dma0tsel::Dma0trig28
    }
    #[doc = "DMA0TRIG29"]
    #[inline(always)]
    pub fn is_dma0trig29(&self) -> bool {
        *self == Dma0tsel::Dma0trig29
    }
    #[doc = "DMA0TRIG30"]
    #[inline(always)]
    pub fn is_dma0trig30(&self) -> bool {
        *self == Dma0tsel::Dma0trig30
    }
    #[doc = "DMA0TRIG31"]
    #[inline(always)]
    pub fn is_dma0trig31(&self) -> bool {
        *self == Dma0tsel::Dma0trig31
    }
}
#[doc = "Field `DMA0TSEL` writer - DMA trigger select"]
pub type Dma0tselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Dma0tsel, crate::Safe>;
impl<'a, REG> Dma0tselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA0TRIG0"]
    #[inline(always)]
    pub fn dma0trig0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig0)
    }
    #[doc = "DMA0TRIG1"]
    #[inline(always)]
    pub fn dma0trig1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig1)
    }
    #[doc = "DMA0TRIG2"]
    #[inline(always)]
    pub fn dma0trig2(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig2)
    }
    #[doc = "DMA0TRIG3"]
    #[inline(always)]
    pub fn dma0trig3(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig3)
    }
    #[doc = "DMA0TRIG4"]
    #[inline(always)]
    pub fn dma0trig4(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig4)
    }
    #[doc = "DMA0TRIG5"]
    #[inline(always)]
    pub fn dma0trig5(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig5)
    }
    #[doc = "DMA0TRIG6"]
    #[inline(always)]
    pub fn dma0trig6(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig6)
    }
    #[doc = "DMA0TRIG7"]
    #[inline(always)]
    pub fn dma0trig7(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig7)
    }
    #[doc = "DMA0TRIG8"]
    #[inline(always)]
    pub fn dma0trig8(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig8)
    }
    #[doc = "DMA0TRIG9"]
    #[inline(always)]
    pub fn dma0trig9(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig9)
    }
    #[doc = "DMA0TRIG10"]
    #[inline(always)]
    pub fn dma0trig10(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig10)
    }
    #[doc = "DMA0TRIG11"]
    #[inline(always)]
    pub fn dma0trig11(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig11)
    }
    #[doc = "DMA0TRIG12"]
    #[inline(always)]
    pub fn dma0trig12(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig12)
    }
    #[doc = "DMA0TRIG13"]
    #[inline(always)]
    pub fn dma0trig13(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig13)
    }
    #[doc = "DMA0TRIG14"]
    #[inline(always)]
    pub fn dma0trig14(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig14)
    }
    #[doc = "DMA0TRIG15"]
    #[inline(always)]
    pub fn dma0trig15(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig15)
    }
    #[doc = "DMA0TRIG16"]
    #[inline(always)]
    pub fn dma0trig16(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig16)
    }
    #[doc = "DMA0TRIG17"]
    #[inline(always)]
    pub fn dma0trig17(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig17)
    }
    #[doc = "DMA0TRIG18"]
    #[inline(always)]
    pub fn dma0trig18(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig18)
    }
    #[doc = "DMA0TRIG19"]
    #[inline(always)]
    pub fn dma0trig19(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig19)
    }
    #[doc = "DMA0TRIG20"]
    #[inline(always)]
    pub fn dma0trig20(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig20)
    }
    #[doc = "DMA0TRIG21"]
    #[inline(always)]
    pub fn dma0trig21(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig21)
    }
    #[doc = "DMA0TRIG22"]
    #[inline(always)]
    pub fn dma0trig22(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig22)
    }
    #[doc = "DMA0TRIG23"]
    #[inline(always)]
    pub fn dma0trig23(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig23)
    }
    #[doc = "DMA0TRIG24"]
    #[inline(always)]
    pub fn dma0trig24(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig24)
    }
    #[doc = "DMA0TRIG25"]
    #[inline(always)]
    pub fn dma0trig25(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig25)
    }
    #[doc = "DMA0TRIG26"]
    #[inline(always)]
    pub fn dma0trig26(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig26)
    }
    #[doc = "DMA0TRIG27"]
    #[inline(always)]
    pub fn dma0trig27(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig27)
    }
    #[doc = "DMA0TRIG28"]
    #[inline(always)]
    pub fn dma0trig28(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig28)
    }
    #[doc = "DMA0TRIG29"]
    #[inline(always)]
    pub fn dma0trig29(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig29)
    }
    #[doc = "DMA0TRIG30"]
    #[inline(always)]
    pub fn dma0trig30(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig30)
    }
    #[doc = "DMA0TRIG31"]
    #[inline(always)]
    pub fn dma0trig31(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0trig31)
    }
}
#[doc = "DMA trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma1tsel {
    #[doc = "0: DMA1TRIG0"]
    Dma1trig0 = 0,
    #[doc = "1: DMA1TRIG1"]
    Dma1trig1 = 1,
    #[doc = "2: DMA1TRIG2"]
    Dma1trig2 = 2,
    #[doc = "3: DMA1TRIG3"]
    Dma1trig3 = 3,
    #[doc = "4: DMA1TRIG4"]
    Dma1trig4 = 4,
    #[doc = "5: DMA1TRIG5"]
    Dma1trig5 = 5,
    #[doc = "6: DMA1TRIG6"]
    Dma1trig6 = 6,
    #[doc = "7: DMA1TRIG7"]
    Dma1trig7 = 7,
    #[doc = "8: DMA1TRIG8"]
    Dma1trig8 = 8,
    #[doc = "9: DMA1TRIG9"]
    Dma1trig9 = 9,
    #[doc = "10: DMA1TRIG10"]
    Dma1trig10 = 10,
    #[doc = "11: DMA1TRIG11"]
    Dma1trig11 = 11,
    #[doc = "12: DMA1TRIG12"]
    Dma1trig12 = 12,
    #[doc = "13: DMA1TRIG13"]
    Dma1trig13 = 13,
    #[doc = "14: DMA1TRIG14"]
    Dma1trig14 = 14,
    #[doc = "15: DMA1TRIG15"]
    Dma1trig15 = 15,
    #[doc = "16: DMA1TRIG16"]
    Dma1trig16 = 16,
    #[doc = "17: DMA1TRIG17"]
    Dma1trig17 = 17,
    #[doc = "18: DMA1TRIG18"]
    Dma1trig18 = 18,
    #[doc = "19: DMA1TRIG19"]
    Dma1trig19 = 19,
    #[doc = "20: DMA1TRIG20"]
    Dma1trig20 = 20,
    #[doc = "21: DMA1TRIG21"]
    Dma1trig21 = 21,
    #[doc = "22: DMA1TRIG22"]
    Dma1trig22 = 22,
    #[doc = "23: DMA1TRIG23"]
    Dma1trig23 = 23,
    #[doc = "24: DMA1TRIG24"]
    Dma1trig24 = 24,
    #[doc = "25: DMA1TRIG25"]
    Dma1trig25 = 25,
    #[doc = "26: DMA1TRIG26"]
    Dma1trig26 = 26,
    #[doc = "27: DMA1TRIG27"]
    Dma1trig27 = 27,
    #[doc = "28: DMA1TRIG28"]
    Dma1trig28 = 28,
    #[doc = "29: DMA1TRIG29"]
    Dma1trig29 = 29,
    #[doc = "30: DMA1TRIG30"]
    Dma1trig30 = 30,
    #[doc = "31: DMA1TRIG31"]
    Dma1trig31 = 31,
}
impl From<Dma1tsel> for u8 {
    #[inline(always)]
    fn from(variant: Dma1tsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma1tsel {
    type Ux = u8;
}
impl crate::IsEnum for Dma1tsel {}
#[doc = "Field `DMA1TSEL` reader - DMA trigger select"]
pub type Dma1tselR = crate::FieldReader<Dma1tsel>;
impl Dma1tselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1tsel {
        match self.bits {
            0 => Dma1tsel::Dma1trig0,
            1 => Dma1tsel::Dma1trig1,
            2 => Dma1tsel::Dma1trig2,
            3 => Dma1tsel::Dma1trig3,
            4 => Dma1tsel::Dma1trig4,
            5 => Dma1tsel::Dma1trig5,
            6 => Dma1tsel::Dma1trig6,
            7 => Dma1tsel::Dma1trig7,
            8 => Dma1tsel::Dma1trig8,
            9 => Dma1tsel::Dma1trig9,
            10 => Dma1tsel::Dma1trig10,
            11 => Dma1tsel::Dma1trig11,
            12 => Dma1tsel::Dma1trig12,
            13 => Dma1tsel::Dma1trig13,
            14 => Dma1tsel::Dma1trig14,
            15 => Dma1tsel::Dma1trig15,
            16 => Dma1tsel::Dma1trig16,
            17 => Dma1tsel::Dma1trig17,
            18 => Dma1tsel::Dma1trig18,
            19 => Dma1tsel::Dma1trig19,
            20 => Dma1tsel::Dma1trig20,
            21 => Dma1tsel::Dma1trig21,
            22 => Dma1tsel::Dma1trig22,
            23 => Dma1tsel::Dma1trig23,
            24 => Dma1tsel::Dma1trig24,
            25 => Dma1tsel::Dma1trig25,
            26 => Dma1tsel::Dma1trig26,
            27 => Dma1tsel::Dma1trig27,
            28 => Dma1tsel::Dma1trig28,
            29 => Dma1tsel::Dma1trig29,
            30 => Dma1tsel::Dma1trig30,
            31 => Dma1tsel::Dma1trig31,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA1TRIG0"]
    #[inline(always)]
    pub fn is_dma1trig0(&self) -> bool {
        *self == Dma1tsel::Dma1trig0
    }
    #[doc = "DMA1TRIG1"]
    #[inline(always)]
    pub fn is_dma1trig1(&self) -> bool {
        *self == Dma1tsel::Dma1trig1
    }
    #[doc = "DMA1TRIG2"]
    #[inline(always)]
    pub fn is_dma1trig2(&self) -> bool {
        *self == Dma1tsel::Dma1trig2
    }
    #[doc = "DMA1TRIG3"]
    #[inline(always)]
    pub fn is_dma1trig3(&self) -> bool {
        *self == Dma1tsel::Dma1trig3
    }
    #[doc = "DMA1TRIG4"]
    #[inline(always)]
    pub fn is_dma1trig4(&self) -> bool {
        *self == Dma1tsel::Dma1trig4
    }
    #[doc = "DMA1TRIG5"]
    #[inline(always)]
    pub fn is_dma1trig5(&self) -> bool {
        *self == Dma1tsel::Dma1trig5
    }
    #[doc = "DMA1TRIG6"]
    #[inline(always)]
    pub fn is_dma1trig6(&self) -> bool {
        *self == Dma1tsel::Dma1trig6
    }
    #[doc = "DMA1TRIG7"]
    #[inline(always)]
    pub fn is_dma1trig7(&self) -> bool {
        *self == Dma1tsel::Dma1trig7
    }
    #[doc = "DMA1TRIG8"]
    #[inline(always)]
    pub fn is_dma1trig8(&self) -> bool {
        *self == Dma1tsel::Dma1trig8
    }
    #[doc = "DMA1TRIG9"]
    #[inline(always)]
    pub fn is_dma1trig9(&self) -> bool {
        *self == Dma1tsel::Dma1trig9
    }
    #[doc = "DMA1TRIG10"]
    #[inline(always)]
    pub fn is_dma1trig10(&self) -> bool {
        *self == Dma1tsel::Dma1trig10
    }
    #[doc = "DMA1TRIG11"]
    #[inline(always)]
    pub fn is_dma1trig11(&self) -> bool {
        *self == Dma1tsel::Dma1trig11
    }
    #[doc = "DMA1TRIG12"]
    #[inline(always)]
    pub fn is_dma1trig12(&self) -> bool {
        *self == Dma1tsel::Dma1trig12
    }
    #[doc = "DMA1TRIG13"]
    #[inline(always)]
    pub fn is_dma1trig13(&self) -> bool {
        *self == Dma1tsel::Dma1trig13
    }
    #[doc = "DMA1TRIG14"]
    #[inline(always)]
    pub fn is_dma1trig14(&self) -> bool {
        *self == Dma1tsel::Dma1trig14
    }
    #[doc = "DMA1TRIG15"]
    #[inline(always)]
    pub fn is_dma1trig15(&self) -> bool {
        *self == Dma1tsel::Dma1trig15
    }
    #[doc = "DMA1TRIG16"]
    #[inline(always)]
    pub fn is_dma1trig16(&self) -> bool {
        *self == Dma1tsel::Dma1trig16
    }
    #[doc = "DMA1TRIG17"]
    #[inline(always)]
    pub fn is_dma1trig17(&self) -> bool {
        *self == Dma1tsel::Dma1trig17
    }
    #[doc = "DMA1TRIG18"]
    #[inline(always)]
    pub fn is_dma1trig18(&self) -> bool {
        *self == Dma1tsel::Dma1trig18
    }
    #[doc = "DMA1TRIG19"]
    #[inline(always)]
    pub fn is_dma1trig19(&self) -> bool {
        *self == Dma1tsel::Dma1trig19
    }
    #[doc = "DMA1TRIG20"]
    #[inline(always)]
    pub fn is_dma1trig20(&self) -> bool {
        *self == Dma1tsel::Dma1trig20
    }
    #[doc = "DMA1TRIG21"]
    #[inline(always)]
    pub fn is_dma1trig21(&self) -> bool {
        *self == Dma1tsel::Dma1trig21
    }
    #[doc = "DMA1TRIG22"]
    #[inline(always)]
    pub fn is_dma1trig22(&self) -> bool {
        *self == Dma1tsel::Dma1trig22
    }
    #[doc = "DMA1TRIG23"]
    #[inline(always)]
    pub fn is_dma1trig23(&self) -> bool {
        *self == Dma1tsel::Dma1trig23
    }
    #[doc = "DMA1TRIG24"]
    #[inline(always)]
    pub fn is_dma1trig24(&self) -> bool {
        *self == Dma1tsel::Dma1trig24
    }
    #[doc = "DMA1TRIG25"]
    #[inline(always)]
    pub fn is_dma1trig25(&self) -> bool {
        *self == Dma1tsel::Dma1trig25
    }
    #[doc = "DMA1TRIG26"]
    #[inline(always)]
    pub fn is_dma1trig26(&self) -> bool {
        *self == Dma1tsel::Dma1trig26
    }
    #[doc = "DMA1TRIG27"]
    #[inline(always)]
    pub fn is_dma1trig27(&self) -> bool {
        *self == Dma1tsel::Dma1trig27
    }
    #[doc = "DMA1TRIG28"]
    #[inline(always)]
    pub fn is_dma1trig28(&self) -> bool {
        *self == Dma1tsel::Dma1trig28
    }
    #[doc = "DMA1TRIG29"]
    #[inline(always)]
    pub fn is_dma1trig29(&self) -> bool {
        *self == Dma1tsel::Dma1trig29
    }
    #[doc = "DMA1TRIG30"]
    #[inline(always)]
    pub fn is_dma1trig30(&self) -> bool {
        *self == Dma1tsel::Dma1trig30
    }
    #[doc = "DMA1TRIG31"]
    #[inline(always)]
    pub fn is_dma1trig31(&self) -> bool {
        *self == Dma1tsel::Dma1trig31
    }
}
#[doc = "Field `DMA1TSEL` writer - DMA trigger select"]
pub type Dma1tselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Dma1tsel, crate::Safe>;
impl<'a, REG> Dma1tselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA1TRIG0"]
    #[inline(always)]
    pub fn dma1trig0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig0)
    }
    #[doc = "DMA1TRIG1"]
    #[inline(always)]
    pub fn dma1trig1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig1)
    }
    #[doc = "DMA1TRIG2"]
    #[inline(always)]
    pub fn dma1trig2(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig2)
    }
    #[doc = "DMA1TRIG3"]
    #[inline(always)]
    pub fn dma1trig3(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig3)
    }
    #[doc = "DMA1TRIG4"]
    #[inline(always)]
    pub fn dma1trig4(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig4)
    }
    #[doc = "DMA1TRIG5"]
    #[inline(always)]
    pub fn dma1trig5(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig5)
    }
    #[doc = "DMA1TRIG6"]
    #[inline(always)]
    pub fn dma1trig6(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig6)
    }
    #[doc = "DMA1TRIG7"]
    #[inline(always)]
    pub fn dma1trig7(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig7)
    }
    #[doc = "DMA1TRIG8"]
    #[inline(always)]
    pub fn dma1trig8(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig8)
    }
    #[doc = "DMA1TRIG9"]
    #[inline(always)]
    pub fn dma1trig9(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig9)
    }
    #[doc = "DMA1TRIG10"]
    #[inline(always)]
    pub fn dma1trig10(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig10)
    }
    #[doc = "DMA1TRIG11"]
    #[inline(always)]
    pub fn dma1trig11(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig11)
    }
    #[doc = "DMA1TRIG12"]
    #[inline(always)]
    pub fn dma1trig12(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig12)
    }
    #[doc = "DMA1TRIG13"]
    #[inline(always)]
    pub fn dma1trig13(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig13)
    }
    #[doc = "DMA1TRIG14"]
    #[inline(always)]
    pub fn dma1trig14(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig14)
    }
    #[doc = "DMA1TRIG15"]
    #[inline(always)]
    pub fn dma1trig15(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig15)
    }
    #[doc = "DMA1TRIG16"]
    #[inline(always)]
    pub fn dma1trig16(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig16)
    }
    #[doc = "DMA1TRIG17"]
    #[inline(always)]
    pub fn dma1trig17(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig17)
    }
    #[doc = "DMA1TRIG18"]
    #[inline(always)]
    pub fn dma1trig18(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig18)
    }
    #[doc = "DMA1TRIG19"]
    #[inline(always)]
    pub fn dma1trig19(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig19)
    }
    #[doc = "DMA1TRIG20"]
    #[inline(always)]
    pub fn dma1trig20(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig20)
    }
    #[doc = "DMA1TRIG21"]
    #[inline(always)]
    pub fn dma1trig21(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig21)
    }
    #[doc = "DMA1TRIG22"]
    #[inline(always)]
    pub fn dma1trig22(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig22)
    }
    #[doc = "DMA1TRIG23"]
    #[inline(always)]
    pub fn dma1trig23(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig23)
    }
    #[doc = "DMA1TRIG24"]
    #[inline(always)]
    pub fn dma1trig24(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig24)
    }
    #[doc = "DMA1TRIG25"]
    #[inline(always)]
    pub fn dma1trig25(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig25)
    }
    #[doc = "DMA1TRIG26"]
    #[inline(always)]
    pub fn dma1trig26(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig26)
    }
    #[doc = "DMA1TRIG27"]
    #[inline(always)]
    pub fn dma1trig27(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig27)
    }
    #[doc = "DMA1TRIG28"]
    #[inline(always)]
    pub fn dma1trig28(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig28)
    }
    #[doc = "DMA1TRIG29"]
    #[inline(always)]
    pub fn dma1trig29(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig29)
    }
    #[doc = "DMA1TRIG30"]
    #[inline(always)]
    pub fn dma1trig30(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig30)
    }
    #[doc = "DMA1TRIG31"]
    #[inline(always)]
    pub fn dma1trig31(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1trig31)
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA trigger select"]
    #[inline(always)]
    pub fn dma0tsel(&self) -> Dma0tselR {
        Dma0tselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA trigger select"]
    #[inline(always)]
    pub fn dma1tsel(&self) -> Dma1tselR {
        Dma1tselR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA trigger select"]
    #[inline(always)]
    pub fn dma0tsel(&mut self) -> Dma0tselW<'_, Dmactl0Spec> {
        Dma0tselW::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA trigger select"]
    #[inline(always)]
    pub fn dma1tsel(&mut self) -> Dma1tselW<'_, Dmactl0Spec> {
        Dma1tselW::new(self, 8)
    }
}
#[doc = "DMA Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmactl0Spec;
impl crate::RegisterSpec for Dmactl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmactl0::R`](R) reader structure"]
impl crate::Readable for Dmactl0Spec {}
#[doc = "`write(|w| ..)` method takes [`dmactl0::W`](W) writer structure"]
impl crate::Writable for Dmactl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACTL0 to value 0"]
impl crate::Resettable for Dmactl0Spec {}
