#[doc = "Register `DMACTL1` reader"]
pub type R = crate::R<Dmactl1Spec>;
#[doc = "Register `DMACTL1` writer"]
pub type W = crate::W<Dmactl1Spec>;
#[doc = "DMA trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma2tsel {
    #[doc = "0: DMA2TRIG0"]
    Dma2trig0 = 0,
    #[doc = "1: DMA2TRIG1"]
    Dma2trig1 = 1,
    #[doc = "2: DMA2TRIG2"]
    Dma2trig2 = 2,
    #[doc = "3: DMA2TRIG3"]
    Dma2trig3 = 3,
    #[doc = "4: DMA2TRIG4"]
    Dma2trig4 = 4,
    #[doc = "5: DMA2TRIG5"]
    Dma2trig5 = 5,
    #[doc = "6: DMA2TRIG6"]
    Dma2trig6 = 6,
    #[doc = "7: DMA2TRIG7"]
    Dma2trig7 = 7,
    #[doc = "8: DMA2TRIG8"]
    Dma2trig8 = 8,
    #[doc = "9: DMA2TRIG9"]
    Dma2trig9 = 9,
    #[doc = "10: DMA2TRIG10"]
    Dma2trig10 = 10,
    #[doc = "11: DMA2TRIG11"]
    Dma2trig11 = 11,
    #[doc = "12: DMA2TRIG12"]
    Dma2trig12 = 12,
    #[doc = "13: DMA2TRIG13"]
    Dma2trig13 = 13,
    #[doc = "14: DMA2TRIG14"]
    Dma2trig14 = 14,
    #[doc = "15: DMA2TRIG15"]
    Dma2trig15 = 15,
    #[doc = "16: DMA2TRIG16"]
    Dma2trig16 = 16,
    #[doc = "17: DMA2TRIG17"]
    Dma2trig17 = 17,
    #[doc = "18: DMA2TRIG18"]
    Dma2trig18 = 18,
    #[doc = "19: DMA2TRIG19"]
    Dma2trig19 = 19,
    #[doc = "20: DMA2TRIG20"]
    Dma2trig20 = 20,
    #[doc = "21: DMA2TRIG21"]
    Dma2trig21 = 21,
    #[doc = "22: DMA2TRIG22"]
    Dma2trig22 = 22,
    #[doc = "23: DMA2TRIG23"]
    Dma2trig23 = 23,
    #[doc = "24: DMA2TRIG24"]
    Dma2trig24 = 24,
    #[doc = "25: DMA2TRIG25"]
    Dma2trig25 = 25,
    #[doc = "26: DMA2TRIG26"]
    Dma2trig26 = 26,
    #[doc = "27: DMA2TRIG27"]
    Dma2trig27 = 27,
    #[doc = "28: DMA2TRIG28"]
    Dma2trig28 = 28,
    #[doc = "29: DMA2TRIG29"]
    Dma2trig29 = 29,
    #[doc = "30: DMA2TRIG30"]
    Dma2trig30 = 30,
    #[doc = "31: DMA2TRIG31"]
    Dma2trig31 = 31,
}
impl From<Dma2tsel> for u8 {
    #[inline(always)]
    fn from(variant: Dma2tsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma2tsel {
    type Ux = u8;
}
impl crate::IsEnum for Dma2tsel {}
#[doc = "Field `DMA2TSEL` reader - DMA trigger select"]
pub type Dma2tselR = crate::FieldReader<Dma2tsel>;
impl Dma2tselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma2tsel {
        match self.bits {
            0 => Dma2tsel::Dma2trig0,
            1 => Dma2tsel::Dma2trig1,
            2 => Dma2tsel::Dma2trig2,
            3 => Dma2tsel::Dma2trig3,
            4 => Dma2tsel::Dma2trig4,
            5 => Dma2tsel::Dma2trig5,
            6 => Dma2tsel::Dma2trig6,
            7 => Dma2tsel::Dma2trig7,
            8 => Dma2tsel::Dma2trig8,
            9 => Dma2tsel::Dma2trig9,
            10 => Dma2tsel::Dma2trig10,
            11 => Dma2tsel::Dma2trig11,
            12 => Dma2tsel::Dma2trig12,
            13 => Dma2tsel::Dma2trig13,
            14 => Dma2tsel::Dma2trig14,
            15 => Dma2tsel::Dma2trig15,
            16 => Dma2tsel::Dma2trig16,
            17 => Dma2tsel::Dma2trig17,
            18 => Dma2tsel::Dma2trig18,
            19 => Dma2tsel::Dma2trig19,
            20 => Dma2tsel::Dma2trig20,
            21 => Dma2tsel::Dma2trig21,
            22 => Dma2tsel::Dma2trig22,
            23 => Dma2tsel::Dma2trig23,
            24 => Dma2tsel::Dma2trig24,
            25 => Dma2tsel::Dma2trig25,
            26 => Dma2tsel::Dma2trig26,
            27 => Dma2tsel::Dma2trig27,
            28 => Dma2tsel::Dma2trig28,
            29 => Dma2tsel::Dma2trig29,
            30 => Dma2tsel::Dma2trig30,
            31 => Dma2tsel::Dma2trig31,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA2TRIG0"]
    #[inline(always)]
    pub fn is_dma2trig0(&self) -> bool {
        *self == Dma2tsel::Dma2trig0
    }
    #[doc = "DMA2TRIG1"]
    #[inline(always)]
    pub fn is_dma2trig1(&self) -> bool {
        *self == Dma2tsel::Dma2trig1
    }
    #[doc = "DMA2TRIG2"]
    #[inline(always)]
    pub fn is_dma2trig2(&self) -> bool {
        *self == Dma2tsel::Dma2trig2
    }
    #[doc = "DMA2TRIG3"]
    #[inline(always)]
    pub fn is_dma2trig3(&self) -> bool {
        *self == Dma2tsel::Dma2trig3
    }
    #[doc = "DMA2TRIG4"]
    #[inline(always)]
    pub fn is_dma2trig4(&self) -> bool {
        *self == Dma2tsel::Dma2trig4
    }
    #[doc = "DMA2TRIG5"]
    #[inline(always)]
    pub fn is_dma2trig5(&self) -> bool {
        *self == Dma2tsel::Dma2trig5
    }
    #[doc = "DMA2TRIG6"]
    #[inline(always)]
    pub fn is_dma2trig6(&self) -> bool {
        *self == Dma2tsel::Dma2trig6
    }
    #[doc = "DMA2TRIG7"]
    #[inline(always)]
    pub fn is_dma2trig7(&self) -> bool {
        *self == Dma2tsel::Dma2trig7
    }
    #[doc = "DMA2TRIG8"]
    #[inline(always)]
    pub fn is_dma2trig8(&self) -> bool {
        *self == Dma2tsel::Dma2trig8
    }
    #[doc = "DMA2TRIG9"]
    #[inline(always)]
    pub fn is_dma2trig9(&self) -> bool {
        *self == Dma2tsel::Dma2trig9
    }
    #[doc = "DMA2TRIG10"]
    #[inline(always)]
    pub fn is_dma2trig10(&self) -> bool {
        *self == Dma2tsel::Dma2trig10
    }
    #[doc = "DMA2TRIG11"]
    #[inline(always)]
    pub fn is_dma2trig11(&self) -> bool {
        *self == Dma2tsel::Dma2trig11
    }
    #[doc = "DMA2TRIG12"]
    #[inline(always)]
    pub fn is_dma2trig12(&self) -> bool {
        *self == Dma2tsel::Dma2trig12
    }
    #[doc = "DMA2TRIG13"]
    #[inline(always)]
    pub fn is_dma2trig13(&self) -> bool {
        *self == Dma2tsel::Dma2trig13
    }
    #[doc = "DMA2TRIG14"]
    #[inline(always)]
    pub fn is_dma2trig14(&self) -> bool {
        *self == Dma2tsel::Dma2trig14
    }
    #[doc = "DMA2TRIG15"]
    #[inline(always)]
    pub fn is_dma2trig15(&self) -> bool {
        *self == Dma2tsel::Dma2trig15
    }
    #[doc = "DMA2TRIG16"]
    #[inline(always)]
    pub fn is_dma2trig16(&self) -> bool {
        *self == Dma2tsel::Dma2trig16
    }
    #[doc = "DMA2TRIG17"]
    #[inline(always)]
    pub fn is_dma2trig17(&self) -> bool {
        *self == Dma2tsel::Dma2trig17
    }
    #[doc = "DMA2TRIG18"]
    #[inline(always)]
    pub fn is_dma2trig18(&self) -> bool {
        *self == Dma2tsel::Dma2trig18
    }
    #[doc = "DMA2TRIG19"]
    #[inline(always)]
    pub fn is_dma2trig19(&self) -> bool {
        *self == Dma2tsel::Dma2trig19
    }
    #[doc = "DMA2TRIG20"]
    #[inline(always)]
    pub fn is_dma2trig20(&self) -> bool {
        *self == Dma2tsel::Dma2trig20
    }
    #[doc = "DMA2TRIG21"]
    #[inline(always)]
    pub fn is_dma2trig21(&self) -> bool {
        *self == Dma2tsel::Dma2trig21
    }
    #[doc = "DMA2TRIG22"]
    #[inline(always)]
    pub fn is_dma2trig22(&self) -> bool {
        *self == Dma2tsel::Dma2trig22
    }
    #[doc = "DMA2TRIG23"]
    #[inline(always)]
    pub fn is_dma2trig23(&self) -> bool {
        *self == Dma2tsel::Dma2trig23
    }
    #[doc = "DMA2TRIG24"]
    #[inline(always)]
    pub fn is_dma2trig24(&self) -> bool {
        *self == Dma2tsel::Dma2trig24
    }
    #[doc = "DMA2TRIG25"]
    #[inline(always)]
    pub fn is_dma2trig25(&self) -> bool {
        *self == Dma2tsel::Dma2trig25
    }
    #[doc = "DMA2TRIG26"]
    #[inline(always)]
    pub fn is_dma2trig26(&self) -> bool {
        *self == Dma2tsel::Dma2trig26
    }
    #[doc = "DMA2TRIG27"]
    #[inline(always)]
    pub fn is_dma2trig27(&self) -> bool {
        *self == Dma2tsel::Dma2trig27
    }
    #[doc = "DMA2TRIG28"]
    #[inline(always)]
    pub fn is_dma2trig28(&self) -> bool {
        *self == Dma2tsel::Dma2trig28
    }
    #[doc = "DMA2TRIG29"]
    #[inline(always)]
    pub fn is_dma2trig29(&self) -> bool {
        *self == Dma2tsel::Dma2trig29
    }
    #[doc = "DMA2TRIG30"]
    #[inline(always)]
    pub fn is_dma2trig30(&self) -> bool {
        *self == Dma2tsel::Dma2trig30
    }
    #[doc = "DMA2TRIG31"]
    #[inline(always)]
    pub fn is_dma2trig31(&self) -> bool {
        *self == Dma2tsel::Dma2trig31
    }
}
#[doc = "Field `DMA2TSEL` writer - DMA trigger select"]
pub type Dma2tselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Dma2tsel, crate::Safe>;
impl<'a, REG> Dma2tselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA2TRIG0"]
    #[inline(always)]
    pub fn dma2trig0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig0)
    }
    #[doc = "DMA2TRIG1"]
    #[inline(always)]
    pub fn dma2trig1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig1)
    }
    #[doc = "DMA2TRIG2"]
    #[inline(always)]
    pub fn dma2trig2(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig2)
    }
    #[doc = "DMA2TRIG3"]
    #[inline(always)]
    pub fn dma2trig3(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig3)
    }
    #[doc = "DMA2TRIG4"]
    #[inline(always)]
    pub fn dma2trig4(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig4)
    }
    #[doc = "DMA2TRIG5"]
    #[inline(always)]
    pub fn dma2trig5(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig5)
    }
    #[doc = "DMA2TRIG6"]
    #[inline(always)]
    pub fn dma2trig6(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig6)
    }
    #[doc = "DMA2TRIG7"]
    #[inline(always)]
    pub fn dma2trig7(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig7)
    }
    #[doc = "DMA2TRIG8"]
    #[inline(always)]
    pub fn dma2trig8(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig8)
    }
    #[doc = "DMA2TRIG9"]
    #[inline(always)]
    pub fn dma2trig9(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig9)
    }
    #[doc = "DMA2TRIG10"]
    #[inline(always)]
    pub fn dma2trig10(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig10)
    }
    #[doc = "DMA2TRIG11"]
    #[inline(always)]
    pub fn dma2trig11(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig11)
    }
    #[doc = "DMA2TRIG12"]
    #[inline(always)]
    pub fn dma2trig12(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig12)
    }
    #[doc = "DMA2TRIG13"]
    #[inline(always)]
    pub fn dma2trig13(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig13)
    }
    #[doc = "DMA2TRIG14"]
    #[inline(always)]
    pub fn dma2trig14(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig14)
    }
    #[doc = "DMA2TRIG15"]
    #[inline(always)]
    pub fn dma2trig15(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig15)
    }
    #[doc = "DMA2TRIG16"]
    #[inline(always)]
    pub fn dma2trig16(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig16)
    }
    #[doc = "DMA2TRIG17"]
    #[inline(always)]
    pub fn dma2trig17(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig17)
    }
    #[doc = "DMA2TRIG18"]
    #[inline(always)]
    pub fn dma2trig18(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig18)
    }
    #[doc = "DMA2TRIG19"]
    #[inline(always)]
    pub fn dma2trig19(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig19)
    }
    #[doc = "DMA2TRIG20"]
    #[inline(always)]
    pub fn dma2trig20(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig20)
    }
    #[doc = "DMA2TRIG21"]
    #[inline(always)]
    pub fn dma2trig21(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig21)
    }
    #[doc = "DMA2TRIG22"]
    #[inline(always)]
    pub fn dma2trig22(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig22)
    }
    #[doc = "DMA2TRIG23"]
    #[inline(always)]
    pub fn dma2trig23(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig23)
    }
    #[doc = "DMA2TRIG24"]
    #[inline(always)]
    pub fn dma2trig24(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig24)
    }
    #[doc = "DMA2TRIG25"]
    #[inline(always)]
    pub fn dma2trig25(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig25)
    }
    #[doc = "DMA2TRIG26"]
    #[inline(always)]
    pub fn dma2trig26(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig26)
    }
    #[doc = "DMA2TRIG27"]
    #[inline(always)]
    pub fn dma2trig27(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig27)
    }
    #[doc = "DMA2TRIG28"]
    #[inline(always)]
    pub fn dma2trig28(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig28)
    }
    #[doc = "DMA2TRIG29"]
    #[inline(always)]
    pub fn dma2trig29(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig29)
    }
    #[doc = "DMA2TRIG30"]
    #[inline(always)]
    pub fn dma2trig30(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig30)
    }
    #[doc = "DMA2TRIG31"]
    #[inline(always)]
    pub fn dma2trig31(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2trig31)
    }
}
#[doc = "DMA trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma3tsel {
    #[doc = "0: DMA3TRIG0"]
    Dma3trig0 = 0,
    #[doc = "1: DMA3TRIG1"]
    Dma3trig1 = 1,
    #[doc = "2: DMA3TRIG2"]
    Dma3trig2 = 2,
    #[doc = "3: DMA3TRIG3"]
    Dma3trig3 = 3,
    #[doc = "4: DMA3TRIG4"]
    Dma3trig4 = 4,
    #[doc = "5: DMA3TRIG5"]
    Dma3trig5 = 5,
    #[doc = "6: DMA3TRIG6"]
    Dma3trig6 = 6,
    #[doc = "7: DMA3TRIG7"]
    Dma3trig7 = 7,
    #[doc = "8: DMA3TRIG8"]
    Dma3trig8 = 8,
    #[doc = "9: DMA3TRIG9"]
    Dma3trig9 = 9,
    #[doc = "10: DMA3TRIG10"]
    Dma3trig10 = 10,
    #[doc = "11: DMA3TRIG11"]
    Dma3trig11 = 11,
    #[doc = "12: DMA3TRIG12"]
    Dma3trig12 = 12,
    #[doc = "13: DMA3TRIG13"]
    Dma3trig13 = 13,
    #[doc = "14: DMA3TRIG14"]
    Dma3trig14 = 14,
    #[doc = "15: DMA3TRIG15"]
    Dma3trig15 = 15,
    #[doc = "16: DMA3TRIG16"]
    Dma3trig16 = 16,
    #[doc = "17: DMA3TRIG17"]
    Dma3trig17 = 17,
    #[doc = "18: DMA3TRIG18"]
    Dma3trig18 = 18,
    #[doc = "19: DMA3TRIG19"]
    Dma3trig19 = 19,
    #[doc = "20: DMA3TRIG20"]
    Dma3trig20 = 20,
    #[doc = "21: DMA3TRIG21"]
    Dma3trig21 = 21,
    #[doc = "22: DMA3TRIG22"]
    Dma3trig22 = 22,
    #[doc = "23: DMA3TRIG23"]
    Dma3trig23 = 23,
    #[doc = "24: DMA3TRIG24"]
    Dma3trig24 = 24,
    #[doc = "25: DMA3TRIG25"]
    Dma3trig25 = 25,
    #[doc = "26: DMA3TRIG26"]
    Dma3trig26 = 26,
    #[doc = "27: DMA3TRIG27"]
    Dma3trig27 = 27,
    #[doc = "28: DMA3TRIG28"]
    Dma3trig28 = 28,
    #[doc = "29: DMA3TRIG29"]
    Dma3trig29 = 29,
    #[doc = "30: DMA3TRIG30"]
    Dma3trig30 = 30,
    #[doc = "31: DMA3TRIG31"]
    Dma3trig31 = 31,
}
impl From<Dma3tsel> for u8 {
    #[inline(always)]
    fn from(variant: Dma3tsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma3tsel {
    type Ux = u8;
}
impl crate::IsEnum for Dma3tsel {}
#[doc = "Field `DMA3TSEL` reader - DMA trigger select"]
pub type Dma3tselR = crate::FieldReader<Dma3tsel>;
impl Dma3tselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma3tsel {
        match self.bits {
            0 => Dma3tsel::Dma3trig0,
            1 => Dma3tsel::Dma3trig1,
            2 => Dma3tsel::Dma3trig2,
            3 => Dma3tsel::Dma3trig3,
            4 => Dma3tsel::Dma3trig4,
            5 => Dma3tsel::Dma3trig5,
            6 => Dma3tsel::Dma3trig6,
            7 => Dma3tsel::Dma3trig7,
            8 => Dma3tsel::Dma3trig8,
            9 => Dma3tsel::Dma3trig9,
            10 => Dma3tsel::Dma3trig10,
            11 => Dma3tsel::Dma3trig11,
            12 => Dma3tsel::Dma3trig12,
            13 => Dma3tsel::Dma3trig13,
            14 => Dma3tsel::Dma3trig14,
            15 => Dma3tsel::Dma3trig15,
            16 => Dma3tsel::Dma3trig16,
            17 => Dma3tsel::Dma3trig17,
            18 => Dma3tsel::Dma3trig18,
            19 => Dma3tsel::Dma3trig19,
            20 => Dma3tsel::Dma3trig20,
            21 => Dma3tsel::Dma3trig21,
            22 => Dma3tsel::Dma3trig22,
            23 => Dma3tsel::Dma3trig23,
            24 => Dma3tsel::Dma3trig24,
            25 => Dma3tsel::Dma3trig25,
            26 => Dma3tsel::Dma3trig26,
            27 => Dma3tsel::Dma3trig27,
            28 => Dma3tsel::Dma3trig28,
            29 => Dma3tsel::Dma3trig29,
            30 => Dma3tsel::Dma3trig30,
            31 => Dma3tsel::Dma3trig31,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA3TRIG0"]
    #[inline(always)]
    pub fn is_dma3trig0(&self) -> bool {
        *self == Dma3tsel::Dma3trig0
    }
    #[doc = "DMA3TRIG1"]
    #[inline(always)]
    pub fn is_dma3trig1(&self) -> bool {
        *self == Dma3tsel::Dma3trig1
    }
    #[doc = "DMA3TRIG2"]
    #[inline(always)]
    pub fn is_dma3trig2(&self) -> bool {
        *self == Dma3tsel::Dma3trig2
    }
    #[doc = "DMA3TRIG3"]
    #[inline(always)]
    pub fn is_dma3trig3(&self) -> bool {
        *self == Dma3tsel::Dma3trig3
    }
    #[doc = "DMA3TRIG4"]
    #[inline(always)]
    pub fn is_dma3trig4(&self) -> bool {
        *self == Dma3tsel::Dma3trig4
    }
    #[doc = "DMA3TRIG5"]
    #[inline(always)]
    pub fn is_dma3trig5(&self) -> bool {
        *self == Dma3tsel::Dma3trig5
    }
    #[doc = "DMA3TRIG6"]
    #[inline(always)]
    pub fn is_dma3trig6(&self) -> bool {
        *self == Dma3tsel::Dma3trig6
    }
    #[doc = "DMA3TRIG7"]
    #[inline(always)]
    pub fn is_dma3trig7(&self) -> bool {
        *self == Dma3tsel::Dma3trig7
    }
    #[doc = "DMA3TRIG8"]
    #[inline(always)]
    pub fn is_dma3trig8(&self) -> bool {
        *self == Dma3tsel::Dma3trig8
    }
    #[doc = "DMA3TRIG9"]
    #[inline(always)]
    pub fn is_dma3trig9(&self) -> bool {
        *self == Dma3tsel::Dma3trig9
    }
    #[doc = "DMA3TRIG10"]
    #[inline(always)]
    pub fn is_dma3trig10(&self) -> bool {
        *self == Dma3tsel::Dma3trig10
    }
    #[doc = "DMA3TRIG11"]
    #[inline(always)]
    pub fn is_dma3trig11(&self) -> bool {
        *self == Dma3tsel::Dma3trig11
    }
    #[doc = "DMA3TRIG12"]
    #[inline(always)]
    pub fn is_dma3trig12(&self) -> bool {
        *self == Dma3tsel::Dma3trig12
    }
    #[doc = "DMA3TRIG13"]
    #[inline(always)]
    pub fn is_dma3trig13(&self) -> bool {
        *self == Dma3tsel::Dma3trig13
    }
    #[doc = "DMA3TRIG14"]
    #[inline(always)]
    pub fn is_dma3trig14(&self) -> bool {
        *self == Dma3tsel::Dma3trig14
    }
    #[doc = "DMA3TRIG15"]
    #[inline(always)]
    pub fn is_dma3trig15(&self) -> bool {
        *self == Dma3tsel::Dma3trig15
    }
    #[doc = "DMA3TRIG16"]
    #[inline(always)]
    pub fn is_dma3trig16(&self) -> bool {
        *self == Dma3tsel::Dma3trig16
    }
    #[doc = "DMA3TRIG17"]
    #[inline(always)]
    pub fn is_dma3trig17(&self) -> bool {
        *self == Dma3tsel::Dma3trig17
    }
    #[doc = "DMA3TRIG18"]
    #[inline(always)]
    pub fn is_dma3trig18(&self) -> bool {
        *self == Dma3tsel::Dma3trig18
    }
    #[doc = "DMA3TRIG19"]
    #[inline(always)]
    pub fn is_dma3trig19(&self) -> bool {
        *self == Dma3tsel::Dma3trig19
    }
    #[doc = "DMA3TRIG20"]
    #[inline(always)]
    pub fn is_dma3trig20(&self) -> bool {
        *self == Dma3tsel::Dma3trig20
    }
    #[doc = "DMA3TRIG21"]
    #[inline(always)]
    pub fn is_dma3trig21(&self) -> bool {
        *self == Dma3tsel::Dma3trig21
    }
    #[doc = "DMA3TRIG22"]
    #[inline(always)]
    pub fn is_dma3trig22(&self) -> bool {
        *self == Dma3tsel::Dma3trig22
    }
    #[doc = "DMA3TRIG23"]
    #[inline(always)]
    pub fn is_dma3trig23(&self) -> bool {
        *self == Dma3tsel::Dma3trig23
    }
    #[doc = "DMA3TRIG24"]
    #[inline(always)]
    pub fn is_dma3trig24(&self) -> bool {
        *self == Dma3tsel::Dma3trig24
    }
    #[doc = "DMA3TRIG25"]
    #[inline(always)]
    pub fn is_dma3trig25(&self) -> bool {
        *self == Dma3tsel::Dma3trig25
    }
    #[doc = "DMA3TRIG26"]
    #[inline(always)]
    pub fn is_dma3trig26(&self) -> bool {
        *self == Dma3tsel::Dma3trig26
    }
    #[doc = "DMA3TRIG27"]
    #[inline(always)]
    pub fn is_dma3trig27(&self) -> bool {
        *self == Dma3tsel::Dma3trig27
    }
    #[doc = "DMA3TRIG28"]
    #[inline(always)]
    pub fn is_dma3trig28(&self) -> bool {
        *self == Dma3tsel::Dma3trig28
    }
    #[doc = "DMA3TRIG29"]
    #[inline(always)]
    pub fn is_dma3trig29(&self) -> bool {
        *self == Dma3tsel::Dma3trig29
    }
    #[doc = "DMA3TRIG30"]
    #[inline(always)]
    pub fn is_dma3trig30(&self) -> bool {
        *self == Dma3tsel::Dma3trig30
    }
    #[doc = "DMA3TRIG31"]
    #[inline(always)]
    pub fn is_dma3trig31(&self) -> bool {
        *self == Dma3tsel::Dma3trig31
    }
}
#[doc = "Field `DMA3TSEL` writer - DMA trigger select"]
pub type Dma3tselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Dma3tsel, crate::Safe>;
impl<'a, REG> Dma3tselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA3TRIG0"]
    #[inline(always)]
    pub fn dma3trig0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig0)
    }
    #[doc = "DMA3TRIG1"]
    #[inline(always)]
    pub fn dma3trig1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig1)
    }
    #[doc = "DMA3TRIG2"]
    #[inline(always)]
    pub fn dma3trig2(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig2)
    }
    #[doc = "DMA3TRIG3"]
    #[inline(always)]
    pub fn dma3trig3(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig3)
    }
    #[doc = "DMA3TRIG4"]
    #[inline(always)]
    pub fn dma3trig4(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig4)
    }
    #[doc = "DMA3TRIG5"]
    #[inline(always)]
    pub fn dma3trig5(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig5)
    }
    #[doc = "DMA3TRIG6"]
    #[inline(always)]
    pub fn dma3trig6(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig6)
    }
    #[doc = "DMA3TRIG7"]
    #[inline(always)]
    pub fn dma3trig7(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig7)
    }
    #[doc = "DMA3TRIG8"]
    #[inline(always)]
    pub fn dma3trig8(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig8)
    }
    #[doc = "DMA3TRIG9"]
    #[inline(always)]
    pub fn dma3trig9(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig9)
    }
    #[doc = "DMA3TRIG10"]
    #[inline(always)]
    pub fn dma3trig10(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig10)
    }
    #[doc = "DMA3TRIG11"]
    #[inline(always)]
    pub fn dma3trig11(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig11)
    }
    #[doc = "DMA3TRIG12"]
    #[inline(always)]
    pub fn dma3trig12(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig12)
    }
    #[doc = "DMA3TRIG13"]
    #[inline(always)]
    pub fn dma3trig13(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig13)
    }
    #[doc = "DMA3TRIG14"]
    #[inline(always)]
    pub fn dma3trig14(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig14)
    }
    #[doc = "DMA3TRIG15"]
    #[inline(always)]
    pub fn dma3trig15(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig15)
    }
    #[doc = "DMA3TRIG16"]
    #[inline(always)]
    pub fn dma3trig16(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig16)
    }
    #[doc = "DMA3TRIG17"]
    #[inline(always)]
    pub fn dma3trig17(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig17)
    }
    #[doc = "DMA3TRIG18"]
    #[inline(always)]
    pub fn dma3trig18(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig18)
    }
    #[doc = "DMA3TRIG19"]
    #[inline(always)]
    pub fn dma3trig19(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig19)
    }
    #[doc = "DMA3TRIG20"]
    #[inline(always)]
    pub fn dma3trig20(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig20)
    }
    #[doc = "DMA3TRIG21"]
    #[inline(always)]
    pub fn dma3trig21(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig21)
    }
    #[doc = "DMA3TRIG22"]
    #[inline(always)]
    pub fn dma3trig22(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig22)
    }
    #[doc = "DMA3TRIG23"]
    #[inline(always)]
    pub fn dma3trig23(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig23)
    }
    #[doc = "DMA3TRIG24"]
    #[inline(always)]
    pub fn dma3trig24(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig24)
    }
    #[doc = "DMA3TRIG25"]
    #[inline(always)]
    pub fn dma3trig25(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig25)
    }
    #[doc = "DMA3TRIG26"]
    #[inline(always)]
    pub fn dma3trig26(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig26)
    }
    #[doc = "DMA3TRIG27"]
    #[inline(always)]
    pub fn dma3trig27(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig27)
    }
    #[doc = "DMA3TRIG28"]
    #[inline(always)]
    pub fn dma3trig28(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig28)
    }
    #[doc = "DMA3TRIG29"]
    #[inline(always)]
    pub fn dma3trig29(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig29)
    }
    #[doc = "DMA3TRIG30"]
    #[inline(always)]
    pub fn dma3trig30(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig30)
    }
    #[doc = "DMA3TRIG31"]
    #[inline(always)]
    pub fn dma3trig31(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3tsel::Dma3trig31)
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA trigger select"]
    #[inline(always)]
    pub fn dma2tsel(&self) -> Dma2tselR {
        Dma2tselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA trigger select"]
    #[inline(always)]
    pub fn dma3tsel(&self) -> Dma3tselR {
        Dma3tselR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA trigger select"]
    #[inline(always)]
    pub fn dma2tsel(&mut self) -> Dma2tselW<'_, Dmactl1Spec> {
        Dma2tselW::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA trigger select"]
    #[inline(always)]
    pub fn dma3tsel(&mut self) -> Dma3tselW<'_, Dmactl1Spec> {
        Dma3tselW::new(self, 8)
    }
}
#[doc = "DMA Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmactl1Spec;
impl crate::RegisterSpec for Dmactl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmactl1::R`](R) reader structure"]
impl crate::Readable for Dmactl1Spec {}
#[doc = "`write(|w| ..)` method takes [`dmactl1::W`](W) writer structure"]
impl crate::Writable for Dmactl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACTL1 to value 0"]
impl crate::Resettable for Dmactl1Spec {}
