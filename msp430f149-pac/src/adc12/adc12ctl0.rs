#[doc = "Register `ADC12CTL0` reader"]
pub type R = crate::R<Adc12ctl0Spec>;
#[doc = "Register `ADC12CTL0` writer"]
pub type W = crate::W<Adc12ctl0Spec>;
#[doc = "Field `ADC12SC` reader - ADC12 Start Conversion"]
pub type Adc12scR = crate::BitReader;
#[doc = "Field `ADC12SC` writer - ADC12 Start Conversion"]
pub type Adc12scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENC` reader - ADC12 Enable Conversion"]
pub type EncR = crate::BitReader;
#[doc = "Field `ENC` writer - ADC12 Enable Conversion"]
pub type EncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12TOVIE` reader - ADC12 Timer Overflow interrupt enable"]
pub type Adc12tovieR = crate::BitReader;
#[doc = "Field `ADC12TOVIE` writer - ADC12 Timer Overflow interrupt enable"]
pub type Adc12tovieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12OVIE` reader - ADC12 Overflow interrupt enable"]
pub type Adc12ovieR = crate::BitReader;
#[doc = "Field `ADC12OVIE` writer - ADC12 Overflow interrupt enable"]
pub type Adc12ovieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12ON` reader - ADC12 On/enable"]
pub type Adc12onR = crate::BitReader;
#[doc = "Field `ADC12ON` writer - ADC12 On/enable"]
pub type Adc12onW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFON` reader - ADC12 Reference on"]
pub type RefonR = crate::BitReader;
#[doc = "Field `REFON` writer - ADC12 Reference on"]
pub type RefonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF2_5V` reader - ADC12 Ref 0:1.5V / 1:2.5V"]
pub type Ref2_5vR = crate::BitReader;
#[doc = "Field `REF2_5V` writer - ADC12 Ref 0:1.5V / 1:2.5V"]
pub type Ref2_5vW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSC` reader - ADC12 Multiple SampleConversion"]
pub type MscR = crate::BitReader;
#[doc = "Field `MSC` writer - ADC12 Multiple SampleConversion"]
pub type MscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC12 Sample Hold 0 Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sht0 {
    #[doc = "0: ADC12 Sample Hold 0 Select Bit: 0"]
    Sht0_0 = 0,
    #[doc = "1: ADC12 Sample Hold 0 Select Bit: 1"]
    Sht0_1 = 1,
    #[doc = "2: ADC12 Sample Hold 0 Select Bit: 2"]
    Sht0_2 = 2,
    #[doc = "3: ADC12 Sample Hold 0 Select Bit: 3"]
    Sht0_3 = 3,
    #[doc = "4: ADC12 Sample Hold 0 Select Bit: 4"]
    Sht0_4 = 4,
    #[doc = "5: ADC12 Sample Hold 0 Select Bit: 5"]
    Sht0_5 = 5,
    #[doc = "6: ADC12 Sample Hold 0 Select Bit: 6"]
    Sht0_6 = 6,
    #[doc = "7: ADC12 Sample Hold 0 Select Bit: 7"]
    Sht0_7 = 7,
    #[doc = "8: ADC12 Sample Hold 0 Select Bit: 8"]
    Sht0_8 = 8,
    #[doc = "9: ADC12 Sample Hold 0 Select Bit: 9"]
    Sht0_9 = 9,
    #[doc = "10: ADC12 Sample Hold 0 Select Bit: 10"]
    Sht0_10 = 10,
    #[doc = "11: ADC12 Sample Hold 0 Select Bit: 11"]
    Sht0_11 = 11,
    #[doc = "12: ADC12 Sample Hold 0 Select Bit: 12"]
    Sht0_12 = 12,
    #[doc = "13: ADC12 Sample Hold 0 Select Bit: 13"]
    Sht0_13 = 13,
    #[doc = "14: ADC12 Sample Hold 0 Select Bit: 14"]
    Sht0_14 = 14,
    #[doc = "15: ADC12 Sample Hold 0 Select Bit: 15"]
    Sht0_15 = 15,
}
impl From<Sht0> for u8 {
    #[inline(always)]
    fn from(variant: Sht0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sht0 {
    type Ux = u8;
}
impl crate::IsEnum for Sht0 {}
#[doc = "Field `SHT0` reader - ADC12 Sample Hold 0 Select 0"]
pub type Sht0R = crate::FieldReader<Sht0>;
impl Sht0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sht0 {
        match self.bits {
            0 => Sht0::Sht0_0,
            1 => Sht0::Sht0_1,
            2 => Sht0::Sht0_2,
            3 => Sht0::Sht0_3,
            4 => Sht0::Sht0_4,
            5 => Sht0::Sht0_5,
            6 => Sht0::Sht0_6,
            7 => Sht0::Sht0_7,
            8 => Sht0::Sht0_8,
            9 => Sht0::Sht0_9,
            10 => Sht0::Sht0_10,
            11 => Sht0::Sht0_11,
            12 => Sht0::Sht0_12,
            13 => Sht0::Sht0_13,
            14 => Sht0::Sht0_14,
            15 => Sht0::Sht0_15,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 0"]
    #[inline(always)]
    pub fn is_sht0_0(&self) -> bool {
        *self == Sht0::Sht0_0
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 1"]
    #[inline(always)]
    pub fn is_sht0_1(&self) -> bool {
        *self == Sht0::Sht0_1
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 2"]
    #[inline(always)]
    pub fn is_sht0_2(&self) -> bool {
        *self == Sht0::Sht0_2
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 3"]
    #[inline(always)]
    pub fn is_sht0_3(&self) -> bool {
        *self == Sht0::Sht0_3
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 4"]
    #[inline(always)]
    pub fn is_sht0_4(&self) -> bool {
        *self == Sht0::Sht0_4
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 5"]
    #[inline(always)]
    pub fn is_sht0_5(&self) -> bool {
        *self == Sht0::Sht0_5
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 6"]
    #[inline(always)]
    pub fn is_sht0_6(&self) -> bool {
        *self == Sht0::Sht0_6
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 7"]
    #[inline(always)]
    pub fn is_sht0_7(&self) -> bool {
        *self == Sht0::Sht0_7
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 8"]
    #[inline(always)]
    pub fn is_sht0_8(&self) -> bool {
        *self == Sht0::Sht0_8
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 9"]
    #[inline(always)]
    pub fn is_sht0_9(&self) -> bool {
        *self == Sht0::Sht0_9
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 10"]
    #[inline(always)]
    pub fn is_sht0_10(&self) -> bool {
        *self == Sht0::Sht0_10
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 11"]
    #[inline(always)]
    pub fn is_sht0_11(&self) -> bool {
        *self == Sht0::Sht0_11
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 12"]
    #[inline(always)]
    pub fn is_sht0_12(&self) -> bool {
        *self == Sht0::Sht0_12
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 13"]
    #[inline(always)]
    pub fn is_sht0_13(&self) -> bool {
        *self == Sht0::Sht0_13
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 14"]
    #[inline(always)]
    pub fn is_sht0_14(&self) -> bool {
        *self == Sht0::Sht0_14
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 15"]
    #[inline(always)]
    pub fn is_sht0_15(&self) -> bool {
        *self == Sht0::Sht0_15
    }
}
#[doc = "Field `SHT0` writer - ADC12 Sample Hold 0 Select 0"]
pub type Sht0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Sht0, crate::Safe>;
impl<'a, REG> Sht0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12 Sample Hold 0 Select Bit: 0"]
    #[inline(always)]
    pub fn sht0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_0)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 1"]
    #[inline(always)]
    pub fn sht0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_1)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 2"]
    #[inline(always)]
    pub fn sht0_2(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_2)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 3"]
    #[inline(always)]
    pub fn sht0_3(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_3)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 4"]
    #[inline(always)]
    pub fn sht0_4(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_4)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 5"]
    #[inline(always)]
    pub fn sht0_5(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_5)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 6"]
    #[inline(always)]
    pub fn sht0_6(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_6)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 7"]
    #[inline(always)]
    pub fn sht0_7(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_7)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 8"]
    #[inline(always)]
    pub fn sht0_8(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_8)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 9"]
    #[inline(always)]
    pub fn sht0_9(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_9)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 10"]
    #[inline(always)]
    pub fn sht0_10(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_10)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 11"]
    #[inline(always)]
    pub fn sht0_11(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_11)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 12"]
    #[inline(always)]
    pub fn sht0_12(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_12)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 13"]
    #[inline(always)]
    pub fn sht0_13(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_13)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 14"]
    #[inline(always)]
    pub fn sht0_14(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_14)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 15"]
    #[inline(always)]
    pub fn sht0_15(self) -> &'a mut crate::W<REG> {
        self.variant(Sht0::Sht0_15)
    }
}
#[doc = "ADC12 Sample Hold 0 Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sht1 {
    #[doc = "0: ADC12 Sample Hold 1 Select Bit: 0"]
    Sht1_0 = 0,
    #[doc = "1: ADC12 Sample Hold 1 Select Bit: 1"]
    Sht1_1 = 1,
    #[doc = "2: ADC12 Sample Hold 1 Select Bit: 2"]
    Sht1_2 = 2,
    #[doc = "3: ADC12 Sample Hold 1 Select Bit: 3"]
    Sht1_3 = 3,
    #[doc = "4: ADC12 Sample Hold 1 Select Bit: 4"]
    Sht1_4 = 4,
    #[doc = "5: ADC12 Sample Hold 1 Select Bit: 5"]
    Sht1_5 = 5,
    #[doc = "6: ADC12 Sample Hold 1 Select Bit: 6"]
    Sht1_6 = 6,
    #[doc = "7: ADC12 Sample Hold 1 Select Bit: 7"]
    Sht1_7 = 7,
    #[doc = "8: ADC12 Sample Hold 1 Select Bit: 8"]
    Sht1_8 = 8,
    #[doc = "9: ADC12 Sample Hold 1 Select Bit: 9"]
    Sht1_9 = 9,
    #[doc = "10: ADC12 Sample Hold 1 Select Bit: 10"]
    Sht1_10 = 10,
    #[doc = "11: ADC12 Sample Hold 1 Select Bit: 11"]
    Sht1_11 = 11,
    #[doc = "12: ADC12 Sample Hold 1 Select Bit: 12"]
    Sht1_12 = 12,
    #[doc = "13: ADC12 Sample Hold 1 Select Bit: 13"]
    Sht1_13 = 13,
    #[doc = "14: ADC12 Sample Hold 1 Select Bit: 14"]
    Sht1_14 = 14,
    #[doc = "15: ADC12 Sample Hold 1 Select Bit: 15"]
    Sht1_15 = 15,
}
impl From<Sht1> for u8 {
    #[inline(always)]
    fn from(variant: Sht1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sht1 {
    type Ux = u8;
}
impl crate::IsEnum for Sht1 {}
#[doc = "Field `SHT1` reader - ADC12 Sample Hold 0 Select 0"]
pub type Sht1R = crate::FieldReader<Sht1>;
impl Sht1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sht1 {
        match self.bits {
            0 => Sht1::Sht1_0,
            1 => Sht1::Sht1_1,
            2 => Sht1::Sht1_2,
            3 => Sht1::Sht1_3,
            4 => Sht1::Sht1_4,
            5 => Sht1::Sht1_5,
            6 => Sht1::Sht1_6,
            7 => Sht1::Sht1_7,
            8 => Sht1::Sht1_8,
            9 => Sht1::Sht1_9,
            10 => Sht1::Sht1_10,
            11 => Sht1::Sht1_11,
            12 => Sht1::Sht1_12,
            13 => Sht1::Sht1_13,
            14 => Sht1::Sht1_14,
            15 => Sht1::Sht1_15,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 0"]
    #[inline(always)]
    pub fn is_sht1_0(&self) -> bool {
        *self == Sht1::Sht1_0
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 1"]
    #[inline(always)]
    pub fn is_sht1_1(&self) -> bool {
        *self == Sht1::Sht1_1
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 2"]
    #[inline(always)]
    pub fn is_sht1_2(&self) -> bool {
        *self == Sht1::Sht1_2
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 3"]
    #[inline(always)]
    pub fn is_sht1_3(&self) -> bool {
        *self == Sht1::Sht1_3
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 4"]
    #[inline(always)]
    pub fn is_sht1_4(&self) -> bool {
        *self == Sht1::Sht1_4
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 5"]
    #[inline(always)]
    pub fn is_sht1_5(&self) -> bool {
        *self == Sht1::Sht1_5
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 6"]
    #[inline(always)]
    pub fn is_sht1_6(&self) -> bool {
        *self == Sht1::Sht1_6
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 7"]
    #[inline(always)]
    pub fn is_sht1_7(&self) -> bool {
        *self == Sht1::Sht1_7
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 8"]
    #[inline(always)]
    pub fn is_sht1_8(&self) -> bool {
        *self == Sht1::Sht1_8
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 9"]
    #[inline(always)]
    pub fn is_sht1_9(&self) -> bool {
        *self == Sht1::Sht1_9
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 10"]
    #[inline(always)]
    pub fn is_sht1_10(&self) -> bool {
        *self == Sht1::Sht1_10
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 11"]
    #[inline(always)]
    pub fn is_sht1_11(&self) -> bool {
        *self == Sht1::Sht1_11
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 12"]
    #[inline(always)]
    pub fn is_sht1_12(&self) -> bool {
        *self == Sht1::Sht1_12
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 13"]
    #[inline(always)]
    pub fn is_sht1_13(&self) -> bool {
        *self == Sht1::Sht1_13
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 14"]
    #[inline(always)]
    pub fn is_sht1_14(&self) -> bool {
        *self == Sht1::Sht1_14
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 15"]
    #[inline(always)]
    pub fn is_sht1_15(&self) -> bool {
        *self == Sht1::Sht1_15
    }
}
#[doc = "Field `SHT1` writer - ADC12 Sample Hold 0 Select 0"]
pub type Sht1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Sht1, crate::Safe>;
impl<'a, REG> Sht1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12 Sample Hold 1 Select Bit: 0"]
    #[inline(always)]
    pub fn sht1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_0)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 1"]
    #[inline(always)]
    pub fn sht1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_1)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 2"]
    #[inline(always)]
    pub fn sht1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_2)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 3"]
    #[inline(always)]
    pub fn sht1_3(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_3)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 4"]
    #[inline(always)]
    pub fn sht1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_4)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 5"]
    #[inline(always)]
    pub fn sht1_5(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_5)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 6"]
    #[inline(always)]
    pub fn sht1_6(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_6)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 7"]
    #[inline(always)]
    pub fn sht1_7(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_7)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 8"]
    #[inline(always)]
    pub fn sht1_8(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_8)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 9"]
    #[inline(always)]
    pub fn sht1_9(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_9)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 10"]
    #[inline(always)]
    pub fn sht1_10(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_10)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 11"]
    #[inline(always)]
    pub fn sht1_11(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_11)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 12"]
    #[inline(always)]
    pub fn sht1_12(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_12)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 13"]
    #[inline(always)]
    pub fn sht1_13(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_13)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 14"]
    #[inline(always)]
    pub fn sht1_14(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_14)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 15"]
    #[inline(always)]
    pub fn sht1_15(self) -> &'a mut crate::W<REG> {
        self.variant(Sht1::Sht1_15)
    }
}
impl R {
    #[doc = "Bit 0 - ADC12 Start Conversion"]
    #[inline(always)]
    pub fn adc12sc(&self) -> Adc12scR {
        Adc12scR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC12 Enable Conversion"]
    #[inline(always)]
    pub fn enc(&self) -> EncR {
        EncR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC12 Timer Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12tovie(&self) -> Adc12tovieR {
        Adc12tovieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC12 Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12ovie(&self) -> Adc12ovieR {
        Adc12ovieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC12 On/enable"]
    #[inline(always)]
    pub fn adc12on(&self) -> Adc12onR {
        Adc12onR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC12 Reference on"]
    #[inline(always)]
    pub fn refon(&self) -> RefonR {
        RefonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC12 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    pub fn ref2_5v(&self) -> Ref2_5vR {
        Ref2_5vR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC12 Multiple SampleConversion"]
    #[inline(always)]
    pub fn msc(&self) -> MscR {
        MscR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADC12 Sample Hold 0 Select 0"]
    #[inline(always)]
    pub fn sht0(&self) -> Sht0R {
        Sht0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ADC12 Sample Hold 0 Select 0"]
    #[inline(always)]
    pub fn sht1(&self) -> Sht1R {
        Sht1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Start Conversion"]
    #[inline(always)]
    pub fn adc12sc(&mut self) -> Adc12scW<'_, Adc12ctl0Spec> {
        Adc12scW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC12 Enable Conversion"]
    #[inline(always)]
    pub fn enc(&mut self) -> EncW<'_, Adc12ctl0Spec> {
        EncW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC12 Timer Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12tovie(&mut self) -> Adc12tovieW<'_, Adc12ctl0Spec> {
        Adc12tovieW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC12 Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12ovie(&mut self) -> Adc12ovieW<'_, Adc12ctl0Spec> {
        Adc12ovieW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC12 On/enable"]
    #[inline(always)]
    pub fn adc12on(&mut self) -> Adc12onW<'_, Adc12ctl0Spec> {
        Adc12onW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC12 Reference on"]
    #[inline(always)]
    pub fn refon(&mut self) -> RefonW<'_, Adc12ctl0Spec> {
        RefonW::new(self, 5)
    }
    #[doc = "Bit 6 - ADC12 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    pub fn ref2_5v(&mut self) -> Ref2_5vW<'_, Adc12ctl0Spec> {
        Ref2_5vW::new(self, 6)
    }
    #[doc = "Bit 7 - ADC12 Multiple SampleConversion"]
    #[inline(always)]
    pub fn msc(&mut self) -> MscW<'_, Adc12ctl0Spec> {
        MscW::new(self, 7)
    }
    #[doc = "Bits 8:11 - ADC12 Sample Hold 0 Select 0"]
    #[inline(always)]
    pub fn sht0(&mut self) -> Sht0W<'_, Adc12ctl0Spec> {
        Sht0W::new(self, 8)
    }
    #[doc = "Bits 12:15 - ADC12 Sample Hold 0 Select 0"]
    #[inline(always)]
    pub fn sht1(&mut self) -> Sht1W<'_, Adc12ctl0Spec> {
        Sht1W::new(self, 12)
    }
}
#[doc = "ADC12 Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12ctl0Spec;
impl crate::RegisterSpec for Adc12ctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12ctl0::R`](R) reader structure"]
impl crate::Readable for Adc12ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc12ctl0::W`](W) writer structure"]
impl crate::Writable for Adc12ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC12CTL0 to value 0"]
impl crate::Resettable for Adc12ctl0Spec {}
