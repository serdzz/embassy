#[doc = "Register `ADCIE` reader"]
pub type R = crate::R<AdcieSpec>;
#[doc = "Register `ADCIE` writer"]
pub type W = crate::W<AdcieSpec>;
#[doc = "Field `ADCIE0` reader - ADC Interrupt enable"]
pub type Adcie0R = crate::BitReader;
#[doc = "Field `ADCIE0` writer - ADC Interrupt enable"]
pub type Adcie0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCINIE` reader - ADC Interrupt enable for the inside of window of the Window comparator"]
pub type AdcinieR = crate::BitReader;
#[doc = "Field `ADCINIE` writer - ADC Interrupt enable for the inside of window of the Window comparator"]
pub type AdcinieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCLOIE` reader - ADC Interrupt enable for lower threshold of the Window comparator"]
pub type AdcloieR = crate::BitReader;
#[doc = "Field `ADCLOIE` writer - ADC Interrupt enable for lower threshold of the Window comparator"]
pub type AdcloieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCHIIE` reader - ADC Interrupt enable for upper threshold of the Window comparator"]
pub type AdchiieR = crate::BitReader;
#[doc = "Field `ADCHIIE` writer - ADC Interrupt enable for upper threshold of the Window comparator"]
pub type AdchiieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCOVIE` reader - ADC ADCMEM overflow Interrupt enable"]
pub type AdcovieR = crate::BitReader;
#[doc = "Field `ADCOVIE` writer - ADC ADCMEM overflow Interrupt enable"]
pub type AdcovieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCTOVIE` reader - ADC conversion-time-overflow Interrupt enable"]
pub type AdctovieR = crate::BitReader;
#[doc = "Field `ADCTOVIE` writer - ADC conversion-time-overflow Interrupt enable"]
pub type AdctovieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC Interrupt enable"]
    #[inline(always)]
    pub fn adcie0(&self) -> Adcie0R {
        Adcie0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Interrupt enable for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adcinie(&self) -> AdcinieR {
        AdcinieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Interrupt enable for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adcloie(&self) -> AdcloieR {
        AdcloieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Interrupt enable for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adchiie(&self) -> AdchiieR {
        AdchiieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt enable"]
    #[inline(always)]
    pub fn adcovie(&self) -> AdcovieR {
        AdcovieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt enable"]
    #[inline(always)]
    pub fn adctovie(&self) -> AdctovieR {
        AdctovieR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Interrupt enable"]
    #[inline(always)]
    pub fn adcie0(&mut self) -> Adcie0W<'_, AdcieSpec> {
        Adcie0W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC Interrupt enable for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adcinie(&mut self) -> AdcinieW<'_, AdcieSpec> {
        AdcinieW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC Interrupt enable for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adcloie(&mut self) -> AdcloieW<'_, AdcieSpec> {
        AdcloieW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC Interrupt enable for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adchiie(&mut self) -> AdchiieW<'_, AdcieSpec> {
        AdchiieW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt enable"]
    #[inline(always)]
    pub fn adcovie(&mut self) -> AdcovieW<'_, AdcieSpec> {
        AdcovieW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt enable"]
    #[inline(always)]
    pub fn adctovie(&mut self) -> AdctovieW<'_, AdcieSpec> {
        AdctovieW::new(self, 5)
    }
}
#[doc = "ADC Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`adcie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcieSpec;
impl crate::RegisterSpec for AdcieSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcie::R`](R) reader structure"]
impl crate::Readable for AdcieSpec {}
#[doc = "`write(|w| ..)` method takes [`adcie::W`](W) writer structure"]
impl crate::Writable for AdcieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCIE to value 0"]
impl crate::Resettable for AdcieSpec {}
