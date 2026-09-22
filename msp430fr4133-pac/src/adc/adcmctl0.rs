#[doc = "Register `ADCMCTL0` reader"]
pub type R = crate::R<Adcmctl0Spec>;
#[doc = "Register `ADCMCTL0` writer"]
pub type W = crate::W<Adcmctl0Spec>;
#[doc = "ADC Input Channel Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcinch {
    #[doc = "0: ADC Input Channel 0"]
    Adcinch0 = 0,
    #[doc = "1: ADC Input Channel 1"]
    Adcinch1 = 1,
    #[doc = "2: ADC Input Channel 2"]
    Adcinch2 = 2,
    #[doc = "3: ADC Input Channel 3"]
    Adcinch3 = 3,
    #[doc = "4: ADC Input Channel 4"]
    Adcinch4 = 4,
    #[doc = "5: ADC Input Channel 5"]
    Adcinch5 = 5,
    #[doc = "6: ADC Input Channel 6"]
    Adcinch6 = 6,
    #[doc = "7: ADC Input Channel 7"]
    Adcinch7 = 7,
    #[doc = "8: ADC Input Channel 8"]
    Adcinch8 = 8,
    #[doc = "9: ADC Input Channel 9"]
    Adcinch9 = 9,
    #[doc = "10: ADC Input Channel 10"]
    Adcinch10 = 10,
    #[doc = "11: ADC Input Channel 11"]
    Adcinch11 = 11,
    #[doc = "12: ADC Input Channel 12"]
    Adcinch12 = 12,
    #[doc = "13: ADC Input Channel 13"]
    Adcinch13 = 13,
    #[doc = "14: ADC Input Channel 14"]
    Adcinch14 = 14,
    #[doc = "15: ADC Input Channel 15"]
    Adcinch15 = 15,
}
impl From<Adcinch> for u8 {
    #[inline(always)]
    fn from(variant: Adcinch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcinch {
    type Ux = u8;
}
impl crate::IsEnum for Adcinch {}
#[doc = "Field `ADCINCH` reader - ADC Input Channel Select Bit 0"]
pub type AdcinchR = crate::FieldReader<Adcinch>;
impl AdcinchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcinch {
        match self.bits {
            0 => Adcinch::Adcinch0,
            1 => Adcinch::Adcinch1,
            2 => Adcinch::Adcinch2,
            3 => Adcinch::Adcinch3,
            4 => Adcinch::Adcinch4,
            5 => Adcinch::Adcinch5,
            6 => Adcinch::Adcinch6,
            7 => Adcinch::Adcinch7,
            8 => Adcinch::Adcinch8,
            9 => Adcinch::Adcinch9,
            10 => Adcinch::Adcinch10,
            11 => Adcinch::Adcinch11,
            12 => Adcinch::Adcinch12,
            13 => Adcinch::Adcinch13,
            14 => Adcinch::Adcinch14,
            15 => Adcinch::Adcinch15,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC Input Channel 0"]
    #[inline(always)]
    pub fn is_adcinch_0(&self) -> bool {
        *self == Adcinch::Adcinch0
    }
    #[doc = "ADC Input Channel 1"]
    #[inline(always)]
    pub fn is_adcinch_1(&self) -> bool {
        *self == Adcinch::Adcinch1
    }
    #[doc = "ADC Input Channel 2"]
    #[inline(always)]
    pub fn is_adcinch_2(&self) -> bool {
        *self == Adcinch::Adcinch2
    }
    #[doc = "ADC Input Channel 3"]
    #[inline(always)]
    pub fn is_adcinch_3(&self) -> bool {
        *self == Adcinch::Adcinch3
    }
    #[doc = "ADC Input Channel 4"]
    #[inline(always)]
    pub fn is_adcinch_4(&self) -> bool {
        *self == Adcinch::Adcinch4
    }
    #[doc = "ADC Input Channel 5"]
    #[inline(always)]
    pub fn is_adcinch_5(&self) -> bool {
        *self == Adcinch::Adcinch5
    }
    #[doc = "ADC Input Channel 6"]
    #[inline(always)]
    pub fn is_adcinch_6(&self) -> bool {
        *self == Adcinch::Adcinch6
    }
    #[doc = "ADC Input Channel 7"]
    #[inline(always)]
    pub fn is_adcinch_7(&self) -> bool {
        *self == Adcinch::Adcinch7
    }
    #[doc = "ADC Input Channel 8"]
    #[inline(always)]
    pub fn is_adcinch_8(&self) -> bool {
        *self == Adcinch::Adcinch8
    }
    #[doc = "ADC Input Channel 9"]
    #[inline(always)]
    pub fn is_adcinch_9(&self) -> bool {
        *self == Adcinch::Adcinch9
    }
    #[doc = "ADC Input Channel 10"]
    #[inline(always)]
    pub fn is_adcinch_10(&self) -> bool {
        *self == Adcinch::Adcinch10
    }
    #[doc = "ADC Input Channel 11"]
    #[inline(always)]
    pub fn is_adcinch_11(&self) -> bool {
        *self == Adcinch::Adcinch11
    }
    #[doc = "ADC Input Channel 12"]
    #[inline(always)]
    pub fn is_adcinch_12(&self) -> bool {
        *self == Adcinch::Adcinch12
    }
    #[doc = "ADC Input Channel 13"]
    #[inline(always)]
    pub fn is_adcinch_13(&self) -> bool {
        *self == Adcinch::Adcinch13
    }
    #[doc = "ADC Input Channel 14"]
    #[inline(always)]
    pub fn is_adcinch_14(&self) -> bool {
        *self == Adcinch::Adcinch14
    }
    #[doc = "ADC Input Channel 15"]
    #[inline(always)]
    pub fn is_adcinch_15(&self) -> bool {
        *self == Adcinch::Adcinch15
    }
}
#[doc = "Field `ADCINCH` writer - ADC Input Channel Select Bit 0"]
pub type AdcinchW<'a, REG> = crate::FieldWriter<'a, REG, 4, Adcinch, crate::Safe>;
impl<'a, REG> AdcinchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC Input Channel 0"]
    #[inline(always)]
    pub fn adcinch_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch0)
    }
    #[doc = "ADC Input Channel 1"]
    #[inline(always)]
    pub fn adcinch_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch1)
    }
    #[doc = "ADC Input Channel 2"]
    #[inline(always)]
    pub fn adcinch_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch2)
    }
    #[doc = "ADC Input Channel 3"]
    #[inline(always)]
    pub fn adcinch_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch3)
    }
    #[doc = "ADC Input Channel 4"]
    #[inline(always)]
    pub fn adcinch_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch4)
    }
    #[doc = "ADC Input Channel 5"]
    #[inline(always)]
    pub fn adcinch_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch5)
    }
    #[doc = "ADC Input Channel 6"]
    #[inline(always)]
    pub fn adcinch_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch6)
    }
    #[doc = "ADC Input Channel 7"]
    #[inline(always)]
    pub fn adcinch_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch7)
    }
    #[doc = "ADC Input Channel 8"]
    #[inline(always)]
    pub fn adcinch_8(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch8)
    }
    #[doc = "ADC Input Channel 9"]
    #[inline(always)]
    pub fn adcinch_9(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch9)
    }
    #[doc = "ADC Input Channel 10"]
    #[inline(always)]
    pub fn adcinch_10(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch10)
    }
    #[doc = "ADC Input Channel 11"]
    #[inline(always)]
    pub fn adcinch_11(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch11)
    }
    #[doc = "ADC Input Channel 12"]
    #[inline(always)]
    pub fn adcinch_12(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch12)
    }
    #[doc = "ADC Input Channel 13"]
    #[inline(always)]
    pub fn adcinch_13(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch13)
    }
    #[doc = "ADC Input Channel 14"]
    #[inline(always)]
    pub fn adcinch_14(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch14)
    }
    #[doc = "ADC Input Channel 15"]
    #[inline(always)]
    pub fn adcinch_15(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch15)
    }
}
#[doc = "ADC Select Reference Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcsref {
    #[doc = "0: ADC Select Reference 0"]
    Adcsref0 = 0,
    #[doc = "1: ADC Select Reference 1"]
    Adcsref1 = 1,
    #[doc = "2: ADC Select Reference 2"]
    Adcsref2 = 2,
    #[doc = "3: ADC Select Reference 3"]
    Adcsref3 = 3,
    #[doc = "4: ADC Select Reference 4"]
    Adcsref4 = 4,
    #[doc = "5: ADC Select Reference 5"]
    Adcsref5 = 5,
    #[doc = "6: ADC Select Reference 6"]
    Adcsref6 = 6,
    #[doc = "7: ADC Select Reference 7"]
    Adcsref7 = 7,
}
impl From<Adcsref> for u8 {
    #[inline(always)]
    fn from(variant: Adcsref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcsref {
    type Ux = u8;
}
impl crate::IsEnum for Adcsref {}
#[doc = "Field `ADCSREF` reader - ADC Select Reference Bit 0"]
pub type AdcsrefR = crate::FieldReader<Adcsref>;
impl AdcsrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcsref {
        match self.bits {
            0 => Adcsref::Adcsref0,
            1 => Adcsref::Adcsref1,
            2 => Adcsref::Adcsref2,
            3 => Adcsref::Adcsref3,
            4 => Adcsref::Adcsref4,
            5 => Adcsref::Adcsref5,
            6 => Adcsref::Adcsref6,
            7 => Adcsref::Adcsref7,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC Select Reference 0"]
    #[inline(always)]
    pub fn is_adcsref_0(&self) -> bool {
        *self == Adcsref::Adcsref0
    }
    #[doc = "ADC Select Reference 1"]
    #[inline(always)]
    pub fn is_adcsref_1(&self) -> bool {
        *self == Adcsref::Adcsref1
    }
    #[doc = "ADC Select Reference 2"]
    #[inline(always)]
    pub fn is_adcsref_2(&self) -> bool {
        *self == Adcsref::Adcsref2
    }
    #[doc = "ADC Select Reference 3"]
    #[inline(always)]
    pub fn is_adcsref_3(&self) -> bool {
        *self == Adcsref::Adcsref3
    }
    #[doc = "ADC Select Reference 4"]
    #[inline(always)]
    pub fn is_adcsref_4(&self) -> bool {
        *self == Adcsref::Adcsref4
    }
    #[doc = "ADC Select Reference 5"]
    #[inline(always)]
    pub fn is_adcsref_5(&self) -> bool {
        *self == Adcsref::Adcsref5
    }
    #[doc = "ADC Select Reference 6"]
    #[inline(always)]
    pub fn is_adcsref_6(&self) -> bool {
        *self == Adcsref::Adcsref6
    }
    #[doc = "ADC Select Reference 7"]
    #[inline(always)]
    pub fn is_adcsref_7(&self) -> bool {
        *self == Adcsref::Adcsref7
    }
}
#[doc = "Field `ADCSREF` writer - ADC Select Reference Bit 0"]
pub type AdcsrefW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adcsref, crate::Safe>;
impl<'a, REG> AdcsrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC Select Reference 0"]
    #[inline(always)]
    pub fn adcsref_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref0)
    }
    #[doc = "ADC Select Reference 1"]
    #[inline(always)]
    pub fn adcsref_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref1)
    }
    #[doc = "ADC Select Reference 2"]
    #[inline(always)]
    pub fn adcsref_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref2)
    }
    #[doc = "ADC Select Reference 3"]
    #[inline(always)]
    pub fn adcsref_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref3)
    }
    #[doc = "ADC Select Reference 4"]
    #[inline(always)]
    pub fn adcsref_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref4)
    }
    #[doc = "ADC Select Reference 5"]
    #[inline(always)]
    pub fn adcsref_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref5)
    }
    #[doc = "ADC Select Reference 6"]
    #[inline(always)]
    pub fn adcsref_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref6)
    }
    #[doc = "ADC Select Reference 7"]
    #[inline(always)]
    pub fn adcsref_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref7)
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC Input Channel Select Bit 0"]
    #[inline(always)]
    pub fn adcinch(&self) -> AdcinchR {
        AdcinchR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - ADC Select Reference Bit 0"]
    #[inline(always)]
    pub fn adcsref(&self) -> AdcsrefR {
        AdcsrefR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC Input Channel Select Bit 0"]
    #[inline(always)]
    pub fn adcinch(&mut self) -> AdcinchW<'_, Adcmctl0Spec> {
        AdcinchW::new(self, 0)
    }
    #[doc = "Bits 4:6 - ADC Select Reference Bit 0"]
    #[inline(always)]
    pub fn adcsref(&mut self) -> AdcsrefW<'_, Adcmctl0Spec> {
        AdcsrefW::new(self, 4)
    }
}
#[doc = "ADC Memory Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcmctl0Spec;
impl crate::RegisterSpec for Adcmctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmctl0::R`](R) reader structure"]
impl crate::Readable for Adcmctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`adcmctl0::W`](W) writer structure"]
impl crate::Writable for Adcmctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCMCTL0 to value 0"]
impl crate::Resettable for Adcmctl0Spec {}
