#[doc = "Register `ADC12MCTL13` reader"]
pub type R = crate::R<Adc12mctl13Spec>;
#[doc = "Register `ADC12MCTL13` writer"]
pub type W = crate::W<Adc12mctl13Spec>;
#[doc = "ADC12 Input Channel Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Inch {
    #[doc = "0: ADC12 Input Channel 0"]
    Inch0 = 0,
    #[doc = "1: ADC12 Input Channel 1"]
    Inch1 = 1,
    #[doc = "2: ADC12 Input Channel 2"]
    Inch2 = 2,
    #[doc = "3: ADC12 Input Channel 3"]
    Inch3 = 3,
    #[doc = "4: ADC12 Input Channel 4"]
    Inch4 = 4,
    #[doc = "5: ADC12 Input Channel 5"]
    Inch5 = 5,
    #[doc = "6: ADC12 Input Channel 6"]
    Inch6 = 6,
    #[doc = "7: ADC12 Input Channel 7"]
    Inch7 = 7,
    #[doc = "8: ADC12 Input Channel 8"]
    Inch8 = 8,
    #[doc = "9: ADC12 Input Channel 9"]
    Inch9 = 9,
    #[doc = "10: ADC12 Input Channel 10"]
    Inch10 = 10,
    #[doc = "11: ADC12 Input Channel 11"]
    Inch11 = 11,
    #[doc = "12: ADC12 Input Channel 12"]
    Inch12 = 12,
    #[doc = "13: ADC12 Input Channel 13"]
    Inch13 = 13,
    #[doc = "14: ADC12 Input Channel 14"]
    Inch14 = 14,
    #[doc = "15: ADC12 Input Channel 15"]
    Inch15 = 15,
}
impl From<Inch> for u8 {
    #[inline(always)]
    fn from(variant: Inch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Inch {
    type Ux = u8;
}
impl crate::IsEnum for Inch {}
#[doc = "Field `INCH` reader - ADC12 Input Channel Select Bit 0"]
pub type InchR = crate::FieldReader<Inch>;
impl InchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inch {
        match self.bits {
            0 => Inch::Inch0,
            1 => Inch::Inch1,
            2 => Inch::Inch2,
            3 => Inch::Inch3,
            4 => Inch::Inch4,
            5 => Inch::Inch5,
            6 => Inch::Inch6,
            7 => Inch::Inch7,
            8 => Inch::Inch8,
            9 => Inch::Inch9,
            10 => Inch::Inch10,
            11 => Inch::Inch11,
            12 => Inch::Inch12,
            13 => Inch::Inch13,
            14 => Inch::Inch14,
            15 => Inch::Inch15,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC12 Input Channel 0"]
    #[inline(always)]
    pub fn is_inch_0(&self) -> bool {
        *self == Inch::Inch0
    }
    #[doc = "ADC12 Input Channel 1"]
    #[inline(always)]
    pub fn is_inch_1(&self) -> bool {
        *self == Inch::Inch1
    }
    #[doc = "ADC12 Input Channel 2"]
    #[inline(always)]
    pub fn is_inch_2(&self) -> bool {
        *self == Inch::Inch2
    }
    #[doc = "ADC12 Input Channel 3"]
    #[inline(always)]
    pub fn is_inch_3(&self) -> bool {
        *self == Inch::Inch3
    }
    #[doc = "ADC12 Input Channel 4"]
    #[inline(always)]
    pub fn is_inch_4(&self) -> bool {
        *self == Inch::Inch4
    }
    #[doc = "ADC12 Input Channel 5"]
    #[inline(always)]
    pub fn is_inch_5(&self) -> bool {
        *self == Inch::Inch5
    }
    #[doc = "ADC12 Input Channel 6"]
    #[inline(always)]
    pub fn is_inch_6(&self) -> bool {
        *self == Inch::Inch6
    }
    #[doc = "ADC12 Input Channel 7"]
    #[inline(always)]
    pub fn is_inch_7(&self) -> bool {
        *self == Inch::Inch7
    }
    #[doc = "ADC12 Input Channel 8"]
    #[inline(always)]
    pub fn is_inch_8(&self) -> bool {
        *self == Inch::Inch8
    }
    #[doc = "ADC12 Input Channel 9"]
    #[inline(always)]
    pub fn is_inch_9(&self) -> bool {
        *self == Inch::Inch9
    }
    #[doc = "ADC12 Input Channel 10"]
    #[inline(always)]
    pub fn is_inch_10(&self) -> bool {
        *self == Inch::Inch10
    }
    #[doc = "ADC12 Input Channel 11"]
    #[inline(always)]
    pub fn is_inch_11(&self) -> bool {
        *self == Inch::Inch11
    }
    #[doc = "ADC12 Input Channel 12"]
    #[inline(always)]
    pub fn is_inch_12(&self) -> bool {
        *self == Inch::Inch12
    }
    #[doc = "ADC12 Input Channel 13"]
    #[inline(always)]
    pub fn is_inch_13(&self) -> bool {
        *self == Inch::Inch13
    }
    #[doc = "ADC12 Input Channel 14"]
    #[inline(always)]
    pub fn is_inch_14(&self) -> bool {
        *self == Inch::Inch14
    }
    #[doc = "ADC12 Input Channel 15"]
    #[inline(always)]
    pub fn is_inch_15(&self) -> bool {
        *self == Inch::Inch15
    }
}
#[doc = "Field `INCH` writer - ADC12 Input Channel Select Bit 0"]
pub type InchW<'a, REG> = crate::FieldWriter<'a, REG, 4, Inch, crate::Safe>;
impl<'a, REG> InchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12 Input Channel 0"]
    #[inline(always)]
    pub fn inch_0(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch0)
    }
    #[doc = "ADC12 Input Channel 1"]
    #[inline(always)]
    pub fn inch_1(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch1)
    }
    #[doc = "ADC12 Input Channel 2"]
    #[inline(always)]
    pub fn inch_2(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch2)
    }
    #[doc = "ADC12 Input Channel 3"]
    #[inline(always)]
    pub fn inch_3(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch3)
    }
    #[doc = "ADC12 Input Channel 4"]
    #[inline(always)]
    pub fn inch_4(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch4)
    }
    #[doc = "ADC12 Input Channel 5"]
    #[inline(always)]
    pub fn inch_5(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch5)
    }
    #[doc = "ADC12 Input Channel 6"]
    #[inline(always)]
    pub fn inch_6(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch6)
    }
    #[doc = "ADC12 Input Channel 7"]
    #[inline(always)]
    pub fn inch_7(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch7)
    }
    #[doc = "ADC12 Input Channel 8"]
    #[inline(always)]
    pub fn inch_8(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch8)
    }
    #[doc = "ADC12 Input Channel 9"]
    #[inline(always)]
    pub fn inch_9(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch9)
    }
    #[doc = "ADC12 Input Channel 10"]
    #[inline(always)]
    pub fn inch_10(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch10)
    }
    #[doc = "ADC12 Input Channel 11"]
    #[inline(always)]
    pub fn inch_11(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch11)
    }
    #[doc = "ADC12 Input Channel 12"]
    #[inline(always)]
    pub fn inch_12(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch12)
    }
    #[doc = "ADC12 Input Channel 13"]
    #[inline(always)]
    pub fn inch_13(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch13)
    }
    #[doc = "ADC12 Input Channel 14"]
    #[inline(always)]
    pub fn inch_14(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch14)
    }
    #[doc = "ADC12 Input Channel 15"]
    #[inline(always)]
    pub fn inch_15(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch15)
    }
}
#[doc = "ADC12 Select Reference Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sref {
    #[doc = "0: ADC12 Select Reference 0"]
    Sref0 = 0,
    #[doc = "1: ADC12 Select Reference 1"]
    Sref1 = 1,
    #[doc = "2: ADC12 Select Reference 2"]
    Sref2 = 2,
    #[doc = "3: ADC12 Select Reference 3"]
    Sref3 = 3,
    #[doc = "4: ADC12 Select Reference 4"]
    Sref4 = 4,
    #[doc = "5: ADC12 Select Reference 5"]
    Sref5 = 5,
    #[doc = "6: ADC12 Select Reference 6"]
    Sref6 = 6,
    #[doc = "7: ADC12 Select Reference 7"]
    Sref7 = 7,
}
impl From<Sref> for u8 {
    #[inline(always)]
    fn from(variant: Sref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sref {
    type Ux = u8;
}
impl crate::IsEnum for Sref {}
#[doc = "Field `SREF` reader - ADC12 Select Reference Bit 0"]
pub type SrefR = crate::FieldReader<Sref>;
impl SrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sref {
        match self.bits {
            0 => Sref::Sref0,
            1 => Sref::Sref1,
            2 => Sref::Sref2,
            3 => Sref::Sref3,
            4 => Sref::Sref4,
            5 => Sref::Sref5,
            6 => Sref::Sref6,
            7 => Sref::Sref7,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC12 Select Reference 0"]
    #[inline(always)]
    pub fn is_sref_0(&self) -> bool {
        *self == Sref::Sref0
    }
    #[doc = "ADC12 Select Reference 1"]
    #[inline(always)]
    pub fn is_sref_1(&self) -> bool {
        *self == Sref::Sref1
    }
    #[doc = "ADC12 Select Reference 2"]
    #[inline(always)]
    pub fn is_sref_2(&self) -> bool {
        *self == Sref::Sref2
    }
    #[doc = "ADC12 Select Reference 3"]
    #[inline(always)]
    pub fn is_sref_3(&self) -> bool {
        *self == Sref::Sref3
    }
    #[doc = "ADC12 Select Reference 4"]
    #[inline(always)]
    pub fn is_sref_4(&self) -> bool {
        *self == Sref::Sref4
    }
    #[doc = "ADC12 Select Reference 5"]
    #[inline(always)]
    pub fn is_sref_5(&self) -> bool {
        *self == Sref::Sref5
    }
    #[doc = "ADC12 Select Reference 6"]
    #[inline(always)]
    pub fn is_sref_6(&self) -> bool {
        *self == Sref::Sref6
    }
    #[doc = "ADC12 Select Reference 7"]
    #[inline(always)]
    pub fn is_sref_7(&self) -> bool {
        *self == Sref::Sref7
    }
}
#[doc = "Field `SREF` writer - ADC12 Select Reference Bit 0"]
pub type SrefW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sref, crate::Safe>;
impl<'a, REG> SrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12 Select Reference 0"]
    #[inline(always)]
    pub fn sref_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref0)
    }
    #[doc = "ADC12 Select Reference 1"]
    #[inline(always)]
    pub fn sref_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref1)
    }
    #[doc = "ADC12 Select Reference 2"]
    #[inline(always)]
    pub fn sref_2(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref2)
    }
    #[doc = "ADC12 Select Reference 3"]
    #[inline(always)]
    pub fn sref_3(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref3)
    }
    #[doc = "ADC12 Select Reference 4"]
    #[inline(always)]
    pub fn sref_4(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref4)
    }
    #[doc = "ADC12 Select Reference 5"]
    #[inline(always)]
    pub fn sref_5(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref5)
    }
    #[doc = "ADC12 Select Reference 6"]
    #[inline(always)]
    pub fn sref_6(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref6)
    }
    #[doc = "ADC12 Select Reference 7"]
    #[inline(always)]
    pub fn sref_7(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref7)
    }
}
#[doc = "Field `EOS` reader - ADC12 End of Sequence"]
pub type EosR = crate::BitReader;
#[doc = "Field `EOS` writer - ADC12 End of Sequence"]
pub type EosW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - ADC12 Input Channel Select Bit 0"]
    #[inline(always)]
    pub fn inch(&self) -> InchR {
        InchR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - ADC12 Select Reference Bit 0"]
    #[inline(always)]
    pub fn sref(&self) -> SrefR {
        SrefR::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - ADC12 End of Sequence"]
    #[inline(always)]
    pub fn eos(&self) -> EosR {
        EosR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC12 Input Channel Select Bit 0"]
    #[inline(always)]
    pub fn inch(&mut self) -> InchW<'_, Adc12mctl13Spec> {
        InchW::new(self, 0)
    }
    #[doc = "Bits 4:6 - ADC12 Select Reference Bit 0"]
    #[inline(always)]
    pub fn sref(&mut self) -> SrefW<'_, Adc12mctl13Spec> {
        SrefW::new(self, 4)
    }
    #[doc = "Bit 7 - ADC12 End of Sequence"]
    #[inline(always)]
    pub fn eos(&mut self) -> EosW<'_, Adc12mctl13Spec> {
        EosW::new(self, 7)
    }
}
#[doc = "ADC12 Memory Control 13\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12mctl13Spec;
impl crate::RegisterSpec for Adc12mctl13Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adc12mctl13::R`](R) reader structure"]
impl crate::Readable for Adc12mctl13Spec {}
#[doc = "`write(|w| ..)` method takes [`adc12mctl13::W`](W) writer structure"]
impl crate::Writable for Adc12mctl13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC12MCTL13 to value 0"]
impl crate::Resettable for Adc12mctl13Spec {}
