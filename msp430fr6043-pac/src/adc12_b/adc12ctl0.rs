#[doc = "Register `ADC12CTL0` reader"]
pub type R = crate::R<Adc12ctl0Spec>;
#[doc = "Register `ADC12CTL0` writer"]
pub type W = crate::W<Adc12ctl0Spec>;
#[doc = "start conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12sc {
    #[doc = "0: No sample-and-conversion-start"]
    Adc12sc0 = 0,
    #[doc = "1: Start sample-and-conversion"]
    Adc12sc1 = 1,
}
impl From<Adc12sc> for bool {
    #[inline(always)]
    fn from(variant: Adc12sc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12SC` reader - start conversion"]
pub type Adc12scR = crate::BitReader<Adc12sc>;
impl Adc12scR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12sc {
        match self.bits {
            false => Adc12sc::Adc12sc0,
            true => Adc12sc::Adc12sc1,
        }
    }
    #[doc = "No sample-and-conversion-start"]
    #[inline(always)]
    pub fn is_adc12sc_0(&self) -> bool {
        *self == Adc12sc::Adc12sc0
    }
    #[doc = "Start sample-and-conversion"]
    #[inline(always)]
    pub fn is_adc12sc_1(&self) -> bool {
        *self == Adc12sc::Adc12sc1
    }
}
#[doc = "Field `ADC12SC` writer - start conversion"]
pub type Adc12scW<'a, REG> = crate::BitWriter<'a, REG, Adc12sc>;
impl<'a, REG> Adc12scW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No sample-and-conversion-start"]
    #[inline(always)]
    pub fn adc12sc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sc::Adc12sc0)
    }
    #[doc = "Start sample-and-conversion"]
    #[inline(always)]
    pub fn adc12sc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sc::Adc12sc1)
    }
}
#[doc = "enable conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12enc {
    #[doc = "0: ADC12_B disabled"]
    Adc12enc0 = 0,
    #[doc = "1: ADC12_B enabled"]
    Adc12enc1 = 1,
}
impl From<Adc12enc> for bool {
    #[inline(always)]
    fn from(variant: Adc12enc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12ENC` reader - enable conversion"]
pub type Adc12encR = crate::BitReader<Adc12enc>;
impl Adc12encR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12enc {
        match self.bits {
            false => Adc12enc::Adc12enc0,
            true => Adc12enc::Adc12enc1,
        }
    }
    #[doc = "ADC12_B disabled"]
    #[inline(always)]
    pub fn is_adc12enc_0(&self) -> bool {
        *self == Adc12enc::Adc12enc0
    }
    #[doc = "ADC12_B enabled"]
    #[inline(always)]
    pub fn is_adc12enc_1(&self) -> bool {
        *self == Adc12enc::Adc12enc1
    }
}
#[doc = "Field `ADC12ENC` writer - enable conversion"]
pub type Adc12encW<'a, REG> = crate::BitWriter<'a, REG, Adc12enc>;
impl<'a, REG> Adc12encW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC12_B disabled"]
    #[inline(always)]
    pub fn adc12enc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12enc::Adc12enc0)
    }
    #[doc = "ADC12_B enabled"]
    #[inline(always)]
    pub fn adc12enc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12enc::Adc12enc1)
    }
}
#[doc = "ADC on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12on {
    #[doc = "0: ADC12_B off"]
    Adc12on0 = 0,
    #[doc = "1: ADC12_B on"]
    Adc12on1 = 1,
}
impl From<Adc12on> for bool {
    #[inline(always)]
    fn from(variant: Adc12on) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12ON` reader - ADC on"]
pub type Adc12onR = crate::BitReader<Adc12on>;
impl Adc12onR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12on {
        match self.bits {
            false => Adc12on::Adc12on0,
            true => Adc12on::Adc12on1,
        }
    }
    #[doc = "ADC12_B off"]
    #[inline(always)]
    pub fn is_adc12on_0(&self) -> bool {
        *self == Adc12on::Adc12on0
    }
    #[doc = "ADC12_B on"]
    #[inline(always)]
    pub fn is_adc12on_1(&self) -> bool {
        *self == Adc12on::Adc12on1
    }
}
#[doc = "Field `ADC12ON` writer - ADC on"]
pub type Adc12onW<'a, REG> = crate::BitWriter<'a, REG, Adc12on>;
impl<'a, REG> Adc12onW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC12_B off"]
    #[inline(always)]
    pub fn adc12on_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12on::Adc12on0)
    }
    #[doc = "ADC12_B on"]
    #[inline(always)]
    pub fn adc12on_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12on::Adc12on1)
    }
}
#[doc = "sample-and-hold time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12msc {
    #[doc = "0: The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert."]
    Adc12msc0 = 0,
    #[doc = "1: The incidence of a positive(or for devices first rising edge of the) SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed."]
    Adc12msc1 = 1,
}
impl From<Adc12msc> for bool {
    #[inline(always)]
    fn from(variant: Adc12msc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12MSC` reader - sample-and-hold time."]
pub type Adc12mscR = crate::BitReader<Adc12msc>;
impl Adc12mscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12msc {
        match self.bits {
            false => Adc12msc::Adc12msc0,
            true => Adc12msc::Adc12msc1,
        }
    }
    #[doc = "The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert."]
    #[inline(always)]
    pub fn is_adc12msc_0(&self) -> bool {
        *self == Adc12msc::Adc12msc0
    }
    #[doc = "The incidence of a positive(or for devices first rising edge of the) SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed."]
    #[inline(always)]
    pub fn is_adc12msc_1(&self) -> bool {
        *self == Adc12msc::Adc12msc1
    }
}
#[doc = "Field `ADC12MSC` writer - sample-and-hold time."]
pub type Adc12mscW<'a, REG> = crate::BitWriter<'a, REG, Adc12msc>;
impl<'a, REG> Adc12mscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert."]
    #[inline(always)]
    pub fn adc12msc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12msc::Adc12msc0)
    }
    #[doc = "The incidence of a positive(or for devices first rising edge of the) SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed."]
    #[inline(always)]
    pub fn adc12msc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12msc::Adc12msc1)
    }
}
#[doc = "sample-and-hold time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc12sht0 {
    #[doc = "0: 4 ADC12CLK cycles"]
    Adc12sht0_0 = 0,
    #[doc = "1: 8 ADC12CLK cycles"]
    Adc12sht0_1 = 1,
    #[doc = "2: 16 ADC12CLK cycles"]
    Adc12sht0_2 = 2,
    #[doc = "3: 32 ADC12CLK cycles"]
    Adc12sht0_3 = 3,
    #[doc = "4: 64 ADC12CLK cycles"]
    Adc12sht0_4 = 4,
    #[doc = "5: 96 ADC12CLK cycles"]
    Adc12sht0_5 = 5,
    #[doc = "6: 128 ADC12CLK cycles"]
    Adc12sht0_6 = 6,
    #[doc = "7: 192 ADC12CLK cycles"]
    Adc12sht0_7 = 7,
    #[doc = "8: 256 ADC12CLK cycles"]
    Adc12sht0_8 = 8,
    #[doc = "9: 384 ADC12CLK cycles"]
    Adc12sht0_9 = 9,
    #[doc = "10: 512 ADC12CLK cycles"]
    Adc12sht0_10 = 10,
    #[doc = "11: Reserved"]
    Adc12sht0_11 = 11,
    #[doc = "12: Reserved"]
    Adc12sht0_12 = 12,
    #[doc = "13: Reserved"]
    Adc12sht0_13 = 13,
    #[doc = "14: Reserved"]
    Adc12sht0_14 = 14,
    #[doc = "15: Reserved"]
    Adc12sht0_15 = 15,
}
impl From<Adc12sht0> for u8 {
    #[inline(always)]
    fn from(variant: Adc12sht0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc12sht0 {
    type Ux = u8;
}
impl crate::IsEnum for Adc12sht0 {}
#[doc = "Field `ADC12SHT0` reader - sample-and-hold time."]
pub type Adc12sht0R = crate::FieldReader<Adc12sht0>;
impl Adc12sht0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12sht0 {
        match self.bits {
            0 => Adc12sht0::Adc12sht0_0,
            1 => Adc12sht0::Adc12sht0_1,
            2 => Adc12sht0::Adc12sht0_2,
            3 => Adc12sht0::Adc12sht0_3,
            4 => Adc12sht0::Adc12sht0_4,
            5 => Adc12sht0::Adc12sht0_5,
            6 => Adc12sht0::Adc12sht0_6,
            7 => Adc12sht0::Adc12sht0_7,
            8 => Adc12sht0::Adc12sht0_8,
            9 => Adc12sht0::Adc12sht0_9,
            10 => Adc12sht0::Adc12sht0_10,
            11 => Adc12sht0::Adc12sht0_11,
            12 => Adc12sht0::Adc12sht0_12,
            13 => Adc12sht0::Adc12sht0_13,
            14 => Adc12sht0::Adc12sht0_14,
            15 => Adc12sht0::Adc12sht0_15,
            _ => unreachable!(),
        }
    }
    #[doc = "4 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht0_0(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_0
    }
    #[doc = "8 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht0_1(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_1
    }
    #[doc = "16 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht0_2(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_2
    }
    #[doc = "32 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht0_3(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_3
    }
    #[doc = "64 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht0_4(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_4
    }
    #[doc = "96 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht0_5(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_5
    }
    #[doc = "128 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht0_6(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_6
    }
    #[doc = "192 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht0_7(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_7
    }
    #[doc = "256 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht0_8(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_8
    }
    #[doc = "384 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht0_9(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_9
    }
    #[doc = "512 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht0_10(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_10
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adc12sht0_11(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_11
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adc12sht0_12(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_12
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adc12sht0_13(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_13
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adc12sht0_14(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_14
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adc12sht0_15(&self) -> bool {
        *self == Adc12sht0::Adc12sht0_15
    }
}
#[doc = "Field `ADC12SHT0` writer - sample-and-hold time."]
pub type Adc12sht0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Adc12sht0, crate::Safe>;
impl<'a, REG> Adc12sht0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_0)
    }
    #[doc = "8 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_1)
    }
    #[doc = "16 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_2)
    }
    #[doc = "32 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_3)
    }
    #[doc = "64 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_4)
    }
    #[doc = "96 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_5)
    }
    #[doc = "128 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_6)
    }
    #[doc = "192 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_7)
    }
    #[doc = "256 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_8(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_8)
    }
    #[doc = "384 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_9(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_9)
    }
    #[doc = "512 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_10(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_10)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht0_11(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_11)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht0_12(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_12)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht0_13(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_13)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht0_14(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_14)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht0_15(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht0::Adc12sht0_15)
    }
}
#[doc = "sample-and-hold time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc12sht1 {
    #[doc = "0: 4 ADC12CLK cycles"]
    Adc12sht1_0 = 0,
    #[doc = "1: 8 ADC12CLK cycles"]
    Adc12sht1_1 = 1,
    #[doc = "2: 16 ADC12CLK cycles"]
    Adc12sht1_2 = 2,
    #[doc = "3: 32 ADC12CLK cycles"]
    Adc12sht1_3 = 3,
    #[doc = "4: 64 ADC12CLK cycles"]
    Adc12sht1_4 = 4,
    #[doc = "5: 96 ADC12CLK cycles"]
    Adc12sht1_5 = 5,
    #[doc = "6: 128 ADC12CLK cycles"]
    Adc12sht1_6 = 6,
    #[doc = "7: 192 ADC12CLK cycles"]
    Adc12sht1_7 = 7,
    #[doc = "8: 256 ADC12CLK cycles"]
    Adc12sht1_8 = 8,
    #[doc = "9: 384 ADC12CLK cycles"]
    Adc12sht1_9 = 9,
    #[doc = "10: 512 ADC12CLK cycles"]
    Adc12sht1_10 = 10,
    #[doc = "11: Reserved"]
    Adc12sht1_11 = 11,
    #[doc = "12: Reserved"]
    Adc12sht1_12 = 12,
    #[doc = "13: Reserved"]
    Adc12sht1_13 = 13,
    #[doc = "14: Reserved"]
    Adc12sht1_14 = 14,
    #[doc = "15: Reserved"]
    Adc12sht1_15 = 15,
}
impl From<Adc12sht1> for u8 {
    #[inline(always)]
    fn from(variant: Adc12sht1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc12sht1 {
    type Ux = u8;
}
impl crate::IsEnum for Adc12sht1 {}
#[doc = "Field `ADC12SHT1` reader - sample-and-hold time."]
pub type Adc12sht1R = crate::FieldReader<Adc12sht1>;
impl Adc12sht1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12sht1 {
        match self.bits {
            0 => Adc12sht1::Adc12sht1_0,
            1 => Adc12sht1::Adc12sht1_1,
            2 => Adc12sht1::Adc12sht1_2,
            3 => Adc12sht1::Adc12sht1_3,
            4 => Adc12sht1::Adc12sht1_4,
            5 => Adc12sht1::Adc12sht1_5,
            6 => Adc12sht1::Adc12sht1_6,
            7 => Adc12sht1::Adc12sht1_7,
            8 => Adc12sht1::Adc12sht1_8,
            9 => Adc12sht1::Adc12sht1_9,
            10 => Adc12sht1::Adc12sht1_10,
            11 => Adc12sht1::Adc12sht1_11,
            12 => Adc12sht1::Adc12sht1_12,
            13 => Adc12sht1::Adc12sht1_13,
            14 => Adc12sht1::Adc12sht1_14,
            15 => Adc12sht1::Adc12sht1_15,
            _ => unreachable!(),
        }
    }
    #[doc = "4 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht1_0(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_0
    }
    #[doc = "8 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht1_1(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_1
    }
    #[doc = "16 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht1_2(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_2
    }
    #[doc = "32 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht1_3(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_3
    }
    #[doc = "64 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht1_4(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_4
    }
    #[doc = "96 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht1_5(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_5
    }
    #[doc = "128 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht1_6(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_6
    }
    #[doc = "192 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht1_7(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_7
    }
    #[doc = "256 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht1_8(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_8
    }
    #[doc = "384 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht1_9(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_9
    }
    #[doc = "512 ADC12CLK cycles"]
    #[inline(always)]
    pub fn is_adc12sht1_10(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_10
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adc12sht1_11(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_11
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adc12sht1_12(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_12
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adc12sht1_13(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_13
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adc12sht1_14(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_14
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adc12sht1_15(&self) -> bool {
        *self == Adc12sht1::Adc12sht1_15
    }
}
#[doc = "Field `ADC12SHT1` writer - sample-and-hold time."]
pub type Adc12sht1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Adc12sht1, crate::Safe>;
impl<'a, REG> Adc12sht1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_0)
    }
    #[doc = "8 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_1)
    }
    #[doc = "16 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_2)
    }
    #[doc = "32 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_3)
    }
    #[doc = "64 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_4)
    }
    #[doc = "96 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_5)
    }
    #[doc = "128 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_6)
    }
    #[doc = "192 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_7)
    }
    #[doc = "256 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_8(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_8)
    }
    #[doc = "384 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_9(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_9)
    }
    #[doc = "512 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_10(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_10)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht1_11(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_11)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht1_12(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_12)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht1_13(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_13)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht1_14(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_14)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht1_15(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sht1::Adc12sht1_15)
    }
}
impl R {
    #[doc = "Bit 0 - start conversion"]
    #[inline(always)]
    pub fn adc12sc(&self) -> Adc12scR {
        Adc12scR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable conversion"]
    #[inline(always)]
    pub fn adc12enc(&self) -> Adc12encR {
        Adc12encR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC on"]
    #[inline(always)]
    pub fn adc12on(&self) -> Adc12onR {
        Adc12onR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - sample-and-hold time."]
    #[inline(always)]
    pub fn adc12msc(&self) -> Adc12mscR {
        Adc12mscR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - sample-and-hold time."]
    #[inline(always)]
    pub fn adc12sht0(&self) -> Adc12sht0R {
        Adc12sht0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - sample-and-hold time."]
    #[inline(always)]
    pub fn adc12sht1(&self) -> Adc12sht1R {
        Adc12sht1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - start conversion"]
    #[inline(always)]
    pub fn adc12sc(&mut self) -> Adc12scW<'_, Adc12ctl0Spec> {
        Adc12scW::new(self, 0)
    }
    #[doc = "Bit 1 - enable conversion"]
    #[inline(always)]
    pub fn adc12enc(&mut self) -> Adc12encW<'_, Adc12ctl0Spec> {
        Adc12encW::new(self, 1)
    }
    #[doc = "Bit 4 - ADC on"]
    #[inline(always)]
    pub fn adc12on(&mut self) -> Adc12onW<'_, Adc12ctl0Spec> {
        Adc12onW::new(self, 4)
    }
    #[doc = "Bit 7 - sample-and-hold time."]
    #[inline(always)]
    pub fn adc12msc(&mut self) -> Adc12mscW<'_, Adc12ctl0Spec> {
        Adc12mscW::new(self, 7)
    }
    #[doc = "Bits 8:11 - sample-and-hold time."]
    #[inline(always)]
    pub fn adc12sht0(&mut self) -> Adc12sht0W<'_, Adc12ctl0Spec> {
        Adc12sht0W::new(self, 8)
    }
    #[doc = "Bits 12:15 - sample-and-hold time."]
    #[inline(always)]
    pub fn adc12sht1(&mut self) -> Adc12sht1W<'_, Adc12ctl0Spec> {
        Adc12sht1W::new(self, 12)
    }
}
#[doc = "ADC12_B Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
