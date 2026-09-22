#[doc = "Register `ADCCTL0` reader"]
pub type R = crate::R<Adcctl0Spec>;
#[doc = "Register `ADCCTL0` writer"]
pub type W = crate::W<Adcctl0Spec>;
#[doc = "Field `ADCSC` reader - ADC Start Conversion"]
pub type AdcscR = crate::BitReader;
#[doc = "Field `ADCSC` writer - ADC Start Conversion"]
pub type AdcscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCENC` reader - ADC Enable Conversion"]
pub type AdcencR = crate::BitReader;
#[doc = "Field `ADCENC` writer - ADC Enable Conversion"]
pub type AdcencW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCON` reader - ADC On/enable"]
pub type AdconR = crate::BitReader;
#[doc = "Field `ADCON` writer - ADC On/enable"]
pub type AdconW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCMSC` reader - ADC Multiple SampleConversion"]
pub type AdcmscR = crate::BitReader;
#[doc = "Field `ADCMSC` writer - ADC Multiple SampleConversion"]
pub type AdcmscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC Sample Hold Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcsht {
    #[doc = "0: ADC Sample Hold Select 0"]
    Adcsht0 = 0,
    #[doc = "1: ADC Sample Hold Select 1"]
    Adcsht1 = 1,
    #[doc = "2: ADC Sample Hold Select 2"]
    Adcsht2 = 2,
    #[doc = "3: ADC Sample Hold Select 3"]
    Adcsht3 = 3,
    #[doc = "4: ADC Sample Hold Select 4"]
    Adcsht4 = 4,
    #[doc = "5: ADC Sample Hold Select 5"]
    Adcsht5 = 5,
    #[doc = "6: ADC Sample Hold Select 6"]
    Adcsht6 = 6,
    #[doc = "7: ADC Sample Hold Select 7"]
    Adcsht7 = 7,
    #[doc = "8: ADC Sample Hold Select 8"]
    Adcsht8 = 8,
    #[doc = "9: ADC Sample Hold Select 9"]
    Adcsht9 = 9,
    #[doc = "10: ADC Sample Hold Select 10"]
    Adcsht10 = 10,
    #[doc = "11: ADC Sample Hold Select 11"]
    Adcsht11 = 11,
    #[doc = "12: ADC Sample Hold Select 12"]
    Adcsht12 = 12,
    #[doc = "13: ADC Sample Hold Select 13"]
    Adcsht13 = 13,
    #[doc = "14: ADC Sample Hold Select 14"]
    Adcsht14 = 14,
    #[doc = "15: ADC Sample Hold Select 15"]
    Adcsht15 = 15,
}
impl From<Adcsht> for u8 {
    #[inline(always)]
    fn from(variant: Adcsht) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcsht {
    type Ux = u8;
}
impl crate::IsEnum for Adcsht {}
#[doc = "Field `ADCSHT` reader - ADC Sample Hold Select Bit: 0"]
pub type AdcshtR = crate::FieldReader<Adcsht>;
impl AdcshtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcsht {
        match self.bits {
            0 => Adcsht::Adcsht0,
            1 => Adcsht::Adcsht1,
            2 => Adcsht::Adcsht2,
            3 => Adcsht::Adcsht3,
            4 => Adcsht::Adcsht4,
            5 => Adcsht::Adcsht5,
            6 => Adcsht::Adcsht6,
            7 => Adcsht::Adcsht7,
            8 => Adcsht::Adcsht8,
            9 => Adcsht::Adcsht9,
            10 => Adcsht::Adcsht10,
            11 => Adcsht::Adcsht11,
            12 => Adcsht::Adcsht12,
            13 => Adcsht::Adcsht13,
            14 => Adcsht::Adcsht14,
            15 => Adcsht::Adcsht15,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC Sample Hold Select 0"]
    #[inline(always)]
    pub fn is_adcsht_0(&self) -> bool {
        *self == Adcsht::Adcsht0
    }
    #[doc = "ADC Sample Hold Select 1"]
    #[inline(always)]
    pub fn is_adcsht_1(&self) -> bool {
        *self == Adcsht::Adcsht1
    }
    #[doc = "ADC Sample Hold Select 2"]
    #[inline(always)]
    pub fn is_adcsht_2(&self) -> bool {
        *self == Adcsht::Adcsht2
    }
    #[doc = "ADC Sample Hold Select 3"]
    #[inline(always)]
    pub fn is_adcsht_3(&self) -> bool {
        *self == Adcsht::Adcsht3
    }
    #[doc = "ADC Sample Hold Select 4"]
    #[inline(always)]
    pub fn is_adcsht_4(&self) -> bool {
        *self == Adcsht::Adcsht4
    }
    #[doc = "ADC Sample Hold Select 5"]
    #[inline(always)]
    pub fn is_adcsht_5(&self) -> bool {
        *self == Adcsht::Adcsht5
    }
    #[doc = "ADC Sample Hold Select 6"]
    #[inline(always)]
    pub fn is_adcsht_6(&self) -> bool {
        *self == Adcsht::Adcsht6
    }
    #[doc = "ADC Sample Hold Select 7"]
    #[inline(always)]
    pub fn is_adcsht_7(&self) -> bool {
        *self == Adcsht::Adcsht7
    }
    #[doc = "ADC Sample Hold Select 8"]
    #[inline(always)]
    pub fn is_adcsht_8(&self) -> bool {
        *self == Adcsht::Adcsht8
    }
    #[doc = "ADC Sample Hold Select 9"]
    #[inline(always)]
    pub fn is_adcsht_9(&self) -> bool {
        *self == Adcsht::Adcsht9
    }
    #[doc = "ADC Sample Hold Select 10"]
    #[inline(always)]
    pub fn is_adcsht_10(&self) -> bool {
        *self == Adcsht::Adcsht10
    }
    #[doc = "ADC Sample Hold Select 11"]
    #[inline(always)]
    pub fn is_adcsht_11(&self) -> bool {
        *self == Adcsht::Adcsht11
    }
    #[doc = "ADC Sample Hold Select 12"]
    #[inline(always)]
    pub fn is_adcsht_12(&self) -> bool {
        *self == Adcsht::Adcsht12
    }
    #[doc = "ADC Sample Hold Select 13"]
    #[inline(always)]
    pub fn is_adcsht_13(&self) -> bool {
        *self == Adcsht::Adcsht13
    }
    #[doc = "ADC Sample Hold Select 14"]
    #[inline(always)]
    pub fn is_adcsht_14(&self) -> bool {
        *self == Adcsht::Adcsht14
    }
    #[doc = "ADC Sample Hold Select 15"]
    #[inline(always)]
    pub fn is_adcsht_15(&self) -> bool {
        *self == Adcsht::Adcsht15
    }
}
#[doc = "Field `ADCSHT` writer - ADC Sample Hold Select Bit: 0"]
pub type AdcshtW<'a, REG> = crate::FieldWriter<'a, REG, 4, Adcsht, crate::Safe>;
impl<'a, REG> AdcshtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC Sample Hold Select 0"]
    #[inline(always)]
    pub fn adcsht_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht0)
    }
    #[doc = "ADC Sample Hold Select 1"]
    #[inline(always)]
    pub fn adcsht_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht1)
    }
    #[doc = "ADC Sample Hold Select 2"]
    #[inline(always)]
    pub fn adcsht_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht2)
    }
    #[doc = "ADC Sample Hold Select 3"]
    #[inline(always)]
    pub fn adcsht_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht3)
    }
    #[doc = "ADC Sample Hold Select 4"]
    #[inline(always)]
    pub fn adcsht_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht4)
    }
    #[doc = "ADC Sample Hold Select 5"]
    #[inline(always)]
    pub fn adcsht_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht5)
    }
    #[doc = "ADC Sample Hold Select 6"]
    #[inline(always)]
    pub fn adcsht_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht6)
    }
    #[doc = "ADC Sample Hold Select 7"]
    #[inline(always)]
    pub fn adcsht_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht7)
    }
    #[doc = "ADC Sample Hold Select 8"]
    #[inline(always)]
    pub fn adcsht_8(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht8)
    }
    #[doc = "ADC Sample Hold Select 9"]
    #[inline(always)]
    pub fn adcsht_9(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht9)
    }
    #[doc = "ADC Sample Hold Select 10"]
    #[inline(always)]
    pub fn adcsht_10(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht10)
    }
    #[doc = "ADC Sample Hold Select 11"]
    #[inline(always)]
    pub fn adcsht_11(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht11)
    }
    #[doc = "ADC Sample Hold Select 12"]
    #[inline(always)]
    pub fn adcsht_12(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht12)
    }
    #[doc = "ADC Sample Hold Select 13"]
    #[inline(always)]
    pub fn adcsht_13(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht13)
    }
    #[doc = "ADC Sample Hold Select 14"]
    #[inline(always)]
    pub fn adcsht_14(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht14)
    }
    #[doc = "ADC Sample Hold Select 15"]
    #[inline(always)]
    pub fn adcsht_15(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht15)
    }
}
impl R {
    #[doc = "Bit 0 - ADC Start Conversion"]
    #[inline(always)]
    pub fn adcsc(&self) -> AdcscR {
        AdcscR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Enable Conversion"]
    #[inline(always)]
    pub fn adcenc(&self) -> AdcencR {
        AdcencR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC On/enable"]
    #[inline(always)]
    pub fn adcon(&self) -> AdconR {
        AdconR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC Multiple SampleConversion"]
    #[inline(always)]
    pub fn adcmsc(&self) -> AdcmscR {
        AdcmscR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADC Sample Hold Select Bit: 0"]
    #[inline(always)]
    pub fn adcsht(&self) -> AdcshtR {
        AdcshtR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Start Conversion"]
    #[inline(always)]
    pub fn adcsc(&mut self) -> AdcscW<'_, Adcctl0Spec> {
        AdcscW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC Enable Conversion"]
    #[inline(always)]
    pub fn adcenc(&mut self) -> AdcencW<'_, Adcctl0Spec> {
        AdcencW::new(self, 1)
    }
    #[doc = "Bit 4 - ADC On/enable"]
    #[inline(always)]
    pub fn adcon(&mut self) -> AdconW<'_, Adcctl0Spec> {
        AdconW::new(self, 4)
    }
    #[doc = "Bit 7 - ADC Multiple SampleConversion"]
    #[inline(always)]
    pub fn adcmsc(&mut self) -> AdcmscW<'_, Adcctl0Spec> {
        AdcmscW::new(self, 7)
    }
    #[doc = "Bits 8:11 - ADC Sample Hold Select Bit: 0"]
    #[inline(always)]
    pub fn adcsht(&mut self) -> AdcshtW<'_, Adcctl0Spec> {
        AdcshtW::new(self, 8)
    }
}
#[doc = "ADC Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adcctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcctl0Spec;
impl crate::RegisterSpec for Adcctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcctl0::R`](R) reader structure"]
impl crate::Readable for Adcctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`adcctl0::W`](W) writer structure"]
impl crate::Writable for Adcctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCCTL0 to value 0"]
impl crate::Resettable for Adcctl0Spec {}
