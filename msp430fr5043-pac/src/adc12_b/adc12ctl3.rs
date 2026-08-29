#[doc = "Register `ADC12CTL3` reader"]
pub type R = crate::R<Adc12ctl3Spec>;
#[doc = "Register `ADC12CTL3` writer"]
pub type W = crate::W<Adc12ctl3Spec>;
#[doc = "conversion start address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc12cstartadd {
    #[doc = "0: Conversion start address ADC12MEM0"]
    Adc12mem0 = 0,
    #[doc = "1: Conversion start address ADC12MEM1"]
    Adc12mem1 = 1,
    #[doc = "2: Conversion start address ADC12MEM2"]
    Adc12mem2 = 2,
    #[doc = "3: Conversion start address ADC12MEM3"]
    Adc12mem3 = 3,
    #[doc = "4: Conversion start address ADC12MEM4"]
    Adc12mem4 = 4,
    #[doc = "5: Conversion start address ADC12MEM5"]
    Adc12mem5 = 5,
    #[doc = "6: Conversion start address ADC12MEM6"]
    Adc12mem6 = 6,
    #[doc = "7: Conversion start address ADC12MEM7"]
    Adc12mem7 = 7,
    #[doc = "8: Conversion start address ADC12MEM8"]
    Adc12mem8 = 8,
    #[doc = "9: Conversion start address ADC12MEM9"]
    Adc12mem9 = 9,
    #[doc = "10: Conversion start address ADC12MEM10"]
    Adc12mem10 = 10,
    #[doc = "11: Conversion start address ADC12MEM10"]
    Adc12mem11 = 11,
    #[doc = "12: Conversion start address ADC12MEM12"]
    Adc12mem12 = 12,
    #[doc = "13: Conversion start address ADC12MEM13"]
    Adc12mem13 = 13,
    #[doc = "14: Conversion start address ADC12MEM14"]
    Adc12mem14 = 14,
    #[doc = "15: Conversion start address ADC12MEM15"]
    Adc12mem15 = 15,
    #[doc = "16: Conversion start address ADC12MEM16"]
    Adc12mem16 = 16,
    #[doc = "17: Conversion start address ADC12MEM17"]
    Adc12mem17 = 17,
    #[doc = "18: Conversion start address ADC12MEM18"]
    Adc12mem18 = 18,
    #[doc = "19: Conversion start address ADC12MEM19"]
    Adc12mem19 = 19,
    #[doc = "20: Conversion start address ADC12MEM20"]
    Adc12mem20 = 20,
    #[doc = "21: Conversion start address ADC12MEM21"]
    Adc12mem21 = 21,
    #[doc = "22: Conversion start address ADC12MEM22"]
    Adc12mem22 = 22,
    #[doc = "23: Conversion start address ADC12MEM23"]
    Adc12mem23 = 23,
    #[doc = "24: Conversion start address ADC12MEM24"]
    Adc12mem24 = 24,
    #[doc = "25: Conversion start address ADC12MEM25"]
    Adc12mem25 = 25,
    #[doc = "26: Conversion start address ADC12MEM26"]
    Adc12mem26 = 26,
    #[doc = "27: Conversion start address ADC12MEM27"]
    Adc12mem27 = 27,
    #[doc = "28: Conversion start address ADC12MEM28"]
    Adc12mem28 = 28,
    #[doc = "29: Conversion start address ADC12MEM29"]
    Adc12mem29 = 29,
    #[doc = "30: Conversion start address ADC12MEM30"]
    Adc12mem30 = 30,
    #[doc = "31: Conversion start address ADC12MEM31"]
    Adc12mem31 = 31,
}
impl From<Adc12cstartadd> for u8 {
    #[inline(always)]
    fn from(variant: Adc12cstartadd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc12cstartadd {
    type Ux = u8;
}
impl crate::IsEnum for Adc12cstartadd {}
#[doc = "Field `ADC12CSTARTADD` reader - conversion start address"]
pub type Adc12cstartaddR = crate::FieldReader<Adc12cstartadd>;
impl Adc12cstartaddR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12cstartadd {
        match self.bits {
            0 => Adc12cstartadd::Adc12mem0,
            1 => Adc12cstartadd::Adc12mem1,
            2 => Adc12cstartadd::Adc12mem2,
            3 => Adc12cstartadd::Adc12mem3,
            4 => Adc12cstartadd::Adc12mem4,
            5 => Adc12cstartadd::Adc12mem5,
            6 => Adc12cstartadd::Adc12mem6,
            7 => Adc12cstartadd::Adc12mem7,
            8 => Adc12cstartadd::Adc12mem8,
            9 => Adc12cstartadd::Adc12mem9,
            10 => Adc12cstartadd::Adc12mem10,
            11 => Adc12cstartadd::Adc12mem11,
            12 => Adc12cstartadd::Adc12mem12,
            13 => Adc12cstartadd::Adc12mem13,
            14 => Adc12cstartadd::Adc12mem14,
            15 => Adc12cstartadd::Adc12mem15,
            16 => Adc12cstartadd::Adc12mem16,
            17 => Adc12cstartadd::Adc12mem17,
            18 => Adc12cstartadd::Adc12mem18,
            19 => Adc12cstartadd::Adc12mem19,
            20 => Adc12cstartadd::Adc12mem20,
            21 => Adc12cstartadd::Adc12mem21,
            22 => Adc12cstartadd::Adc12mem22,
            23 => Adc12cstartadd::Adc12mem23,
            24 => Adc12cstartadd::Adc12mem24,
            25 => Adc12cstartadd::Adc12mem25,
            26 => Adc12cstartadd::Adc12mem26,
            27 => Adc12cstartadd::Adc12mem27,
            28 => Adc12cstartadd::Adc12mem28,
            29 => Adc12cstartadd::Adc12mem29,
            30 => Adc12cstartadd::Adc12mem30,
            31 => Adc12cstartadd::Adc12mem31,
            _ => unreachable!(),
        }
    }
    #[doc = "Conversion start address ADC12MEM0"]
    #[inline(always)]
    pub fn is_adc12mem0(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem0
    }
    #[doc = "Conversion start address ADC12MEM1"]
    #[inline(always)]
    pub fn is_adc12mem1(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem1
    }
    #[doc = "Conversion start address ADC12MEM2"]
    #[inline(always)]
    pub fn is_adc12mem2(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem2
    }
    #[doc = "Conversion start address ADC12MEM3"]
    #[inline(always)]
    pub fn is_adc12mem3(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem3
    }
    #[doc = "Conversion start address ADC12MEM4"]
    #[inline(always)]
    pub fn is_adc12mem4(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem4
    }
    #[doc = "Conversion start address ADC12MEM5"]
    #[inline(always)]
    pub fn is_adc12mem5(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem5
    }
    #[doc = "Conversion start address ADC12MEM6"]
    #[inline(always)]
    pub fn is_adc12mem6(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem6
    }
    #[doc = "Conversion start address ADC12MEM7"]
    #[inline(always)]
    pub fn is_adc12mem7(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem7
    }
    #[doc = "Conversion start address ADC12MEM8"]
    #[inline(always)]
    pub fn is_adc12mem8(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem8
    }
    #[doc = "Conversion start address ADC12MEM9"]
    #[inline(always)]
    pub fn is_adc12mem9(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem9
    }
    #[doc = "Conversion start address ADC12MEM10"]
    #[inline(always)]
    pub fn is_adc12mem10(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem10
    }
    #[doc = "Conversion start address ADC12MEM10"]
    #[inline(always)]
    pub fn is_adc12mem11(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem11
    }
    #[doc = "Conversion start address ADC12MEM12"]
    #[inline(always)]
    pub fn is_adc12mem12(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem12
    }
    #[doc = "Conversion start address ADC12MEM13"]
    #[inline(always)]
    pub fn is_adc12mem13(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem13
    }
    #[doc = "Conversion start address ADC12MEM14"]
    #[inline(always)]
    pub fn is_adc12mem14(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem14
    }
    #[doc = "Conversion start address ADC12MEM15"]
    #[inline(always)]
    pub fn is_adc12mem15(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem15
    }
    #[doc = "Conversion start address ADC12MEM16"]
    #[inline(always)]
    pub fn is_adc12mem16(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem16
    }
    #[doc = "Conversion start address ADC12MEM17"]
    #[inline(always)]
    pub fn is_adc12mem17(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem17
    }
    #[doc = "Conversion start address ADC12MEM18"]
    #[inline(always)]
    pub fn is_adc12mem18(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem18
    }
    #[doc = "Conversion start address ADC12MEM19"]
    #[inline(always)]
    pub fn is_adc12mem19(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem19
    }
    #[doc = "Conversion start address ADC12MEM20"]
    #[inline(always)]
    pub fn is_adc12mem20(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem20
    }
    #[doc = "Conversion start address ADC12MEM21"]
    #[inline(always)]
    pub fn is_adc12mem21(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem21
    }
    #[doc = "Conversion start address ADC12MEM22"]
    #[inline(always)]
    pub fn is_adc12mem22(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem22
    }
    #[doc = "Conversion start address ADC12MEM23"]
    #[inline(always)]
    pub fn is_adc12mem23(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem23
    }
    #[doc = "Conversion start address ADC12MEM24"]
    #[inline(always)]
    pub fn is_adc12mem24(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem24
    }
    #[doc = "Conversion start address ADC12MEM25"]
    #[inline(always)]
    pub fn is_adc12mem25(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem25
    }
    #[doc = "Conversion start address ADC12MEM26"]
    #[inline(always)]
    pub fn is_adc12mem26(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem26
    }
    #[doc = "Conversion start address ADC12MEM27"]
    #[inline(always)]
    pub fn is_adc12mem27(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem27
    }
    #[doc = "Conversion start address ADC12MEM28"]
    #[inline(always)]
    pub fn is_adc12mem28(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem28
    }
    #[doc = "Conversion start address ADC12MEM29"]
    #[inline(always)]
    pub fn is_adc12mem29(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem29
    }
    #[doc = "Conversion start address ADC12MEM30"]
    #[inline(always)]
    pub fn is_adc12mem30(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem30
    }
    #[doc = "Conversion start address ADC12MEM31"]
    #[inline(always)]
    pub fn is_adc12mem31(&self) -> bool {
        *self == Adc12cstartadd::Adc12mem31
    }
}
#[doc = "Field `ADC12CSTARTADD` writer - conversion start address"]
pub type Adc12cstartaddW<'a, REG> = crate::FieldWriter<'a, REG, 5, Adc12cstartadd, crate::Safe>;
impl<'a, REG> Adc12cstartaddW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Conversion start address ADC12MEM0"]
    #[inline(always)]
    pub fn adc12mem0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem0)
    }
    #[doc = "Conversion start address ADC12MEM1"]
    #[inline(always)]
    pub fn adc12mem1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem1)
    }
    #[doc = "Conversion start address ADC12MEM2"]
    #[inline(always)]
    pub fn adc12mem2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem2)
    }
    #[doc = "Conversion start address ADC12MEM3"]
    #[inline(always)]
    pub fn adc12mem3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem3)
    }
    #[doc = "Conversion start address ADC12MEM4"]
    #[inline(always)]
    pub fn adc12mem4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem4)
    }
    #[doc = "Conversion start address ADC12MEM5"]
    #[inline(always)]
    pub fn adc12mem5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem5)
    }
    #[doc = "Conversion start address ADC12MEM6"]
    #[inline(always)]
    pub fn adc12mem6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem6)
    }
    #[doc = "Conversion start address ADC12MEM7"]
    #[inline(always)]
    pub fn adc12mem7(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem7)
    }
    #[doc = "Conversion start address ADC12MEM8"]
    #[inline(always)]
    pub fn adc12mem8(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem8)
    }
    #[doc = "Conversion start address ADC12MEM9"]
    #[inline(always)]
    pub fn adc12mem9(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem9)
    }
    #[doc = "Conversion start address ADC12MEM10"]
    #[inline(always)]
    pub fn adc12mem10(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem10)
    }
    #[doc = "Conversion start address ADC12MEM10"]
    #[inline(always)]
    pub fn adc12mem11(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem11)
    }
    #[doc = "Conversion start address ADC12MEM12"]
    #[inline(always)]
    pub fn adc12mem12(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem12)
    }
    #[doc = "Conversion start address ADC12MEM13"]
    #[inline(always)]
    pub fn adc12mem13(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem13)
    }
    #[doc = "Conversion start address ADC12MEM14"]
    #[inline(always)]
    pub fn adc12mem14(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem14)
    }
    #[doc = "Conversion start address ADC12MEM15"]
    #[inline(always)]
    pub fn adc12mem15(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem15)
    }
    #[doc = "Conversion start address ADC12MEM16"]
    #[inline(always)]
    pub fn adc12mem16(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem16)
    }
    #[doc = "Conversion start address ADC12MEM17"]
    #[inline(always)]
    pub fn adc12mem17(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem17)
    }
    #[doc = "Conversion start address ADC12MEM18"]
    #[inline(always)]
    pub fn adc12mem18(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem18)
    }
    #[doc = "Conversion start address ADC12MEM19"]
    #[inline(always)]
    pub fn adc12mem19(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem19)
    }
    #[doc = "Conversion start address ADC12MEM20"]
    #[inline(always)]
    pub fn adc12mem20(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem20)
    }
    #[doc = "Conversion start address ADC12MEM21"]
    #[inline(always)]
    pub fn adc12mem21(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem21)
    }
    #[doc = "Conversion start address ADC12MEM22"]
    #[inline(always)]
    pub fn adc12mem22(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem22)
    }
    #[doc = "Conversion start address ADC12MEM23"]
    #[inline(always)]
    pub fn adc12mem23(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem23)
    }
    #[doc = "Conversion start address ADC12MEM24"]
    #[inline(always)]
    pub fn adc12mem24(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem24)
    }
    #[doc = "Conversion start address ADC12MEM25"]
    #[inline(always)]
    pub fn adc12mem25(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem25)
    }
    #[doc = "Conversion start address ADC12MEM26"]
    #[inline(always)]
    pub fn adc12mem26(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem26)
    }
    #[doc = "Conversion start address ADC12MEM27"]
    #[inline(always)]
    pub fn adc12mem27(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem27)
    }
    #[doc = "Conversion start address ADC12MEM28"]
    #[inline(always)]
    pub fn adc12mem28(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem28)
    }
    #[doc = "Conversion start address ADC12MEM29"]
    #[inline(always)]
    pub fn adc12mem29(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem29)
    }
    #[doc = "Conversion start address ADC12MEM30"]
    #[inline(always)]
    pub fn adc12mem30(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem30)
    }
    #[doc = "Conversion start address ADC12MEM31"]
    #[inline(always)]
    pub fn adc12mem31(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12cstartadd::Adc12mem31)
    }
}
#[doc = "1/2 AVCC ADC input channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12batmap {
    #[doc = "0: external pin is selected for ADC input channel A31"]
    Adc12batmap0 = 0,
    #[doc = "1: ADC internal 1/2 x AVCC channel is selected for ADC input channel A31"]
    Adc12batmap1 = 1,
}
impl From<Adc12batmap> for bool {
    #[inline(always)]
    fn from(variant: Adc12batmap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12BATMAP` reader - 1/2 AVCC ADC input channel selection"]
pub type Adc12batmapR = crate::BitReader<Adc12batmap>;
impl Adc12batmapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12batmap {
        match self.bits {
            false => Adc12batmap::Adc12batmap0,
            true => Adc12batmap::Adc12batmap1,
        }
    }
    #[doc = "external pin is selected for ADC input channel A31"]
    #[inline(always)]
    pub fn is_adc12batmap_0(&self) -> bool {
        *self == Adc12batmap::Adc12batmap0
    }
    #[doc = "ADC internal 1/2 x AVCC channel is selected for ADC input channel A31"]
    #[inline(always)]
    pub fn is_adc12batmap_1(&self) -> bool {
        *self == Adc12batmap::Adc12batmap1
    }
}
#[doc = "Field `ADC12BATMAP` writer - 1/2 AVCC ADC input channel selection"]
pub type Adc12batmapW<'a, REG> = crate::BitWriter<'a, REG, Adc12batmap>;
impl<'a, REG> Adc12batmapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external pin is selected for ADC input channel A31"]
    #[inline(always)]
    pub fn adc12batmap_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12batmap::Adc12batmap0)
    }
    #[doc = "ADC internal 1/2 x AVCC channel is selected for ADC input channel A31"]
    #[inline(always)]
    pub fn adc12batmap_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12batmap::Adc12batmap1)
    }
}
#[doc = "temperature sensor ADC input channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12tcmap {
    #[doc = "0: external pin is selected for ADC input channel A30"]
    Adc12tcmap0 = 0,
    #[doc = "1: ADC internal temperature sensor channel is selected for ADC input channel A30"]
    Adc12tcmap1 = 1,
}
impl From<Adc12tcmap> for bool {
    #[inline(always)]
    fn from(variant: Adc12tcmap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12TCMAP` reader - temperature sensor ADC input channel selection"]
pub type Adc12tcmapR = crate::BitReader<Adc12tcmap>;
impl Adc12tcmapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12tcmap {
        match self.bits {
            false => Adc12tcmap::Adc12tcmap0,
            true => Adc12tcmap::Adc12tcmap1,
        }
    }
    #[doc = "external pin is selected for ADC input channel A30"]
    #[inline(always)]
    pub fn is_adc12tcmap_0(&self) -> bool {
        *self == Adc12tcmap::Adc12tcmap0
    }
    #[doc = "ADC internal temperature sensor channel is selected for ADC input channel A30"]
    #[inline(always)]
    pub fn is_adc12tcmap_1(&self) -> bool {
        *self == Adc12tcmap::Adc12tcmap1
    }
}
#[doc = "Field `ADC12TCMAP` writer - temperature sensor ADC input channel selection"]
pub type Adc12tcmapW<'a, REG> = crate::BitWriter<'a, REG, Adc12tcmap>;
impl<'a, REG> Adc12tcmapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external pin is selected for ADC input channel A30"]
    #[inline(always)]
    pub fn adc12tcmap_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12tcmap::Adc12tcmap0)
    }
    #[doc = "ADC internal temperature sensor channel is selected for ADC input channel A30"]
    #[inline(always)]
    pub fn adc12tcmap_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12tcmap::Adc12tcmap1)
    }
}
#[doc = "int ch 0 sel to ADC in ch A29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ich0map {
    #[doc = "0: external pin is selected for ADC input channel A29"]
    Adc12ich0map0 = 0,
    #[doc = "1: ADC input channel internal 0 is selected for ADC input channel A29, see device-specific data sheet for availability"]
    Adc12ich0map1 = 1,
}
impl From<Adc12ich0map> for bool {
    #[inline(always)]
    fn from(variant: Adc12ich0map) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12ICH0MAP` reader - int ch 0 sel to ADC in ch A29"]
pub type Adc12ich0mapR = crate::BitReader<Adc12ich0map>;
impl Adc12ich0mapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ich0map {
        match self.bits {
            false => Adc12ich0map::Adc12ich0map0,
            true => Adc12ich0map::Adc12ich0map1,
        }
    }
    #[doc = "external pin is selected for ADC input channel A29"]
    #[inline(always)]
    pub fn is_adc12ich0map_0(&self) -> bool {
        *self == Adc12ich0map::Adc12ich0map0
    }
    #[doc = "ADC input channel internal 0 is selected for ADC input channel A29, see device-specific data sheet for availability"]
    #[inline(always)]
    pub fn is_adc12ich0map_1(&self) -> bool {
        *self == Adc12ich0map::Adc12ich0map1
    }
}
#[doc = "Field `ADC12ICH0MAP` writer - int ch 0 sel to ADC in ch A29"]
pub type Adc12ich0mapW<'a, REG> = crate::BitWriter<'a, REG, Adc12ich0map>;
impl<'a, REG> Adc12ich0mapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external pin is selected for ADC input channel A29"]
    #[inline(always)]
    pub fn adc12ich0map_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ich0map::Adc12ich0map0)
    }
    #[doc = "ADC input channel internal 0 is selected for ADC input channel A29, see device-specific data sheet for availability"]
    #[inline(always)]
    pub fn adc12ich0map_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ich0map::Adc12ich0map1)
    }
}
#[doc = "int ch 1 sel to ADC in ch A28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ich1map {
    #[doc = "0: external pin is selected for ADC input channel A28"]
    Adc12ich1map0 = 0,
    #[doc = "1: ADC input channel internal 1 is selected for ADC input channel A28, see device-specific data sheet for availability"]
    Adc12ich1map1 = 1,
}
impl From<Adc12ich1map> for bool {
    #[inline(always)]
    fn from(variant: Adc12ich1map) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12ICH1MAP` reader - int ch 1 sel to ADC in ch A28"]
pub type Adc12ich1mapR = crate::BitReader<Adc12ich1map>;
impl Adc12ich1mapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ich1map {
        match self.bits {
            false => Adc12ich1map::Adc12ich1map0,
            true => Adc12ich1map::Adc12ich1map1,
        }
    }
    #[doc = "external pin is selected for ADC input channel A28"]
    #[inline(always)]
    pub fn is_adc12ich1map_0(&self) -> bool {
        *self == Adc12ich1map::Adc12ich1map0
    }
    #[doc = "ADC input channel internal 1 is selected for ADC input channel A28, see device-specific data sheet for availability"]
    #[inline(always)]
    pub fn is_adc12ich1map_1(&self) -> bool {
        *self == Adc12ich1map::Adc12ich1map1
    }
}
#[doc = "Field `ADC12ICH1MAP` writer - int ch 1 sel to ADC in ch A28"]
pub type Adc12ich1mapW<'a, REG> = crate::BitWriter<'a, REG, Adc12ich1map>;
impl<'a, REG> Adc12ich1mapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external pin is selected for ADC input channel A28"]
    #[inline(always)]
    pub fn adc12ich1map_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ich1map::Adc12ich1map0)
    }
    #[doc = "ADC input channel internal 1 is selected for ADC input channel A28, see device-specific data sheet for availability"]
    #[inline(always)]
    pub fn adc12ich1map_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ich1map::Adc12ich1map1)
    }
}
#[doc = "int ch 2 sel to ADC in ch A27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ich2map {
    #[doc = "0: external pin is selected for ADC input channel A27"]
    Adc12ich2map0 = 0,
    #[doc = "1: ADC input channel internal 2 is selected for ADC input channel A27, see device-specific data sheet for availability"]
    Adc12ich2map1 = 1,
}
impl From<Adc12ich2map> for bool {
    #[inline(always)]
    fn from(variant: Adc12ich2map) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12ICH2MAP` reader - int ch 2 sel to ADC in ch A27"]
pub type Adc12ich2mapR = crate::BitReader<Adc12ich2map>;
impl Adc12ich2mapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ich2map {
        match self.bits {
            false => Adc12ich2map::Adc12ich2map0,
            true => Adc12ich2map::Adc12ich2map1,
        }
    }
    #[doc = "external pin is selected for ADC input channel A27"]
    #[inline(always)]
    pub fn is_adc12ich2map_0(&self) -> bool {
        *self == Adc12ich2map::Adc12ich2map0
    }
    #[doc = "ADC input channel internal 2 is selected for ADC input channel A27, see device-specific data sheet for availability"]
    #[inline(always)]
    pub fn is_adc12ich2map_1(&self) -> bool {
        *self == Adc12ich2map::Adc12ich2map1
    }
}
#[doc = "Field `ADC12ICH2MAP` writer - int ch 2 sel to ADC in ch A27"]
pub type Adc12ich2mapW<'a, REG> = crate::BitWriter<'a, REG, Adc12ich2map>;
impl<'a, REG> Adc12ich2mapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external pin is selected for ADC input channel A27"]
    #[inline(always)]
    pub fn adc12ich2map_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ich2map::Adc12ich2map0)
    }
    #[doc = "ADC input channel internal 2 is selected for ADC input channel A27, see device-specific data sheet for availability"]
    #[inline(always)]
    pub fn adc12ich2map_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ich2map::Adc12ich2map1)
    }
}
#[doc = "int ch 3 sel to ADC in ch A26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ich3map {
    #[doc = "0: external pin is selected for ADC input channel A26"]
    Adc12ich3map0 = 0,
    #[doc = "1: ADC input channel internal 3 is selected for ADC input channel A26, see device-specific data sheet for availability"]
    Adc12ich3map1 = 1,
}
impl From<Adc12ich3map> for bool {
    #[inline(always)]
    fn from(variant: Adc12ich3map) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12ICH3MAP` reader - int ch 3 sel to ADC in ch A26"]
pub type Adc12ich3mapR = crate::BitReader<Adc12ich3map>;
impl Adc12ich3mapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ich3map {
        match self.bits {
            false => Adc12ich3map::Adc12ich3map0,
            true => Adc12ich3map::Adc12ich3map1,
        }
    }
    #[doc = "external pin is selected for ADC input channel A26"]
    #[inline(always)]
    pub fn is_adc12ich3map_0(&self) -> bool {
        *self == Adc12ich3map::Adc12ich3map0
    }
    #[doc = "ADC input channel internal 3 is selected for ADC input channel A26, see device-specific data sheet for availability"]
    #[inline(always)]
    pub fn is_adc12ich3map_1(&self) -> bool {
        *self == Adc12ich3map::Adc12ich3map1
    }
}
#[doc = "Field `ADC12ICH3MAP` writer - int ch 3 sel to ADC in ch A26"]
pub type Adc12ich3mapW<'a, REG> = crate::BitWriter<'a, REG, Adc12ich3map>;
impl<'a, REG> Adc12ich3mapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external pin is selected for ADC input channel A26"]
    #[inline(always)]
    pub fn adc12ich3map_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ich3map::Adc12ich3map0)
    }
    #[doc = "ADC input channel internal 3 is selected for ADC input channel A26, see device-specific data sheet for availability"]
    #[inline(always)]
    pub fn adc12ich3map_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ich3map::Adc12ich3map1)
    }
}
impl R {
    #[doc = "Bits 0:4 - conversion start address"]
    #[inline(always)]
    pub fn adc12cstartadd(&self) -> Adc12cstartaddR {
        Adc12cstartaddR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - 1/2 AVCC ADC input channel selection"]
    #[inline(always)]
    pub fn adc12batmap(&self) -> Adc12batmapR {
        Adc12batmapR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - temperature sensor ADC input channel selection"]
    #[inline(always)]
    pub fn adc12tcmap(&self) -> Adc12tcmapR {
        Adc12tcmapR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - int ch 0 sel to ADC in ch A29"]
    #[inline(always)]
    pub fn adc12ich0map(&self) -> Adc12ich0mapR {
        Adc12ich0mapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - int ch 1 sel to ADC in ch A28"]
    #[inline(always)]
    pub fn adc12ich1map(&self) -> Adc12ich1mapR {
        Adc12ich1mapR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - int ch 2 sel to ADC in ch A27"]
    #[inline(always)]
    pub fn adc12ich2map(&self) -> Adc12ich2mapR {
        Adc12ich2mapR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - int ch 3 sel to ADC in ch A26"]
    #[inline(always)]
    pub fn adc12ich3map(&self) -> Adc12ich3mapR {
        Adc12ich3mapR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - conversion start address"]
    #[inline(always)]
    pub fn adc12cstartadd(&mut self) -> Adc12cstartaddW<'_, Adc12ctl3Spec> {
        Adc12cstartaddW::new(self, 0)
    }
    #[doc = "Bit 6 - 1/2 AVCC ADC input channel selection"]
    #[inline(always)]
    pub fn adc12batmap(&mut self) -> Adc12batmapW<'_, Adc12ctl3Spec> {
        Adc12batmapW::new(self, 6)
    }
    #[doc = "Bit 7 - temperature sensor ADC input channel selection"]
    #[inline(always)]
    pub fn adc12tcmap(&mut self) -> Adc12tcmapW<'_, Adc12ctl3Spec> {
        Adc12tcmapW::new(self, 7)
    }
    #[doc = "Bit 8 - int ch 0 sel to ADC in ch A29"]
    #[inline(always)]
    pub fn adc12ich0map(&mut self) -> Adc12ich0mapW<'_, Adc12ctl3Spec> {
        Adc12ich0mapW::new(self, 8)
    }
    #[doc = "Bit 9 - int ch 1 sel to ADC in ch A28"]
    #[inline(always)]
    pub fn adc12ich1map(&mut self) -> Adc12ich1mapW<'_, Adc12ctl3Spec> {
        Adc12ich1mapW::new(self, 9)
    }
    #[doc = "Bit 10 - int ch 2 sel to ADC in ch A27"]
    #[inline(always)]
    pub fn adc12ich2map(&mut self) -> Adc12ich2mapW<'_, Adc12ctl3Spec> {
        Adc12ich2mapW::new(self, 10)
    }
    #[doc = "Bit 11 - int ch 3 sel to ADC in ch A26"]
    #[inline(always)]
    pub fn adc12ich3map(&mut self) -> Adc12ich3mapW<'_, Adc12ctl3Spec> {
        Adc12ich3mapW::new(self, 11)
    }
}
#[doc = "ADC12_B Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12ctl3Spec;
impl crate::RegisterSpec for Adc12ctl3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12ctl3::R`](R) reader structure"]
impl crate::Readable for Adc12ctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`adc12ctl3::W`](W) writer structure"]
impl crate::Writable for Adc12ctl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC12CTL3 to value 0"]
impl crate::Resettable for Adc12ctl3Spec {}
