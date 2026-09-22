#[doc = "Register `ADCIFG` reader"]
pub type R = crate::R<AdcifgSpec>;
#[doc = "Register `ADCIFG` writer"]
pub type W = crate::W<AdcifgSpec>;
#[doc = "Field `ADCIFG0` reader - ADC Interrupt Flag"]
pub type Adcifg0R = crate::BitReader;
#[doc = "Field `ADCIFG0` writer - ADC Interrupt Flag"]
pub type Adcifg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCINIFG` reader - ADC Interrupt Flag for the inside of window of the Window comparator"]
pub type AdcinifgR = crate::BitReader;
#[doc = "Field `ADCINIFG` writer - ADC Interrupt Flag for the inside of window of the Window comparator"]
pub type AdcinifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCLOIFG` reader - ADC Interrupt Flag for lower threshold of the Window comparator"]
pub type AdcloifgR = crate::BitReader;
#[doc = "Field `ADCLOIFG` writer - ADC Interrupt Flag for lower threshold of the Window comparator"]
pub type AdcloifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCHIIFG` reader - ADC Interrupt Flag for upper threshold of the Window comparator"]
pub type AdchiifgR = crate::BitReader;
#[doc = "Field `ADCHIIFG` writer - ADC Interrupt Flag for upper threshold of the Window comparator"]
pub type AdchiifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCOVIFG` reader - ADC ADCMEM overflow Interrupt Flag"]
pub type AdcovifgR = crate::BitReader;
#[doc = "Field `ADCOVIFG` writer - ADC ADCMEM overflow Interrupt Flag"]
pub type AdcovifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCTOVIFG` reader - ADC conversion-time-overflow Interrupt Flag"]
pub type AdctovifgR = crate::BitReader;
#[doc = "Field `ADCTOVIFG` writer - ADC conversion-time-overflow Interrupt Flag"]
pub type AdctovifgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC Interrupt Flag"]
    #[inline(always)]
    pub fn adcifg0(&self) -> Adcifg0R {
        Adcifg0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Interrupt Flag for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adcinifg(&self) -> AdcinifgR {
        AdcinifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Interrupt Flag for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adcloifg(&self) -> AdcloifgR {
        AdcloifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Interrupt Flag for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adchiifg(&self) -> AdchiifgR {
        AdchiifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adcovifg(&self) -> AdcovifgR {
        AdcovifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adctovifg(&self) -> AdctovifgR {
        AdctovifgR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Interrupt Flag"]
    #[inline(always)]
    pub fn adcifg0(&mut self) -> Adcifg0W<'_, AdcifgSpec> {
        Adcifg0W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC Interrupt Flag for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adcinifg(&mut self) -> AdcinifgW<'_, AdcifgSpec> {
        AdcinifgW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC Interrupt Flag for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adcloifg(&mut self) -> AdcloifgW<'_, AdcifgSpec> {
        AdcloifgW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC Interrupt Flag for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adchiifg(&mut self) -> AdchiifgW<'_, AdcifgSpec> {
        AdchiifgW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adcovifg(&mut self) -> AdcovifgW<'_, AdcifgSpec> {
        AdcovifgW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adctovifg(&mut self) -> AdctovifgW<'_, AdcifgSpec> {
        AdctovifgW::new(self, 5)
    }
}
#[doc = "ADC Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`adcifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcifgSpec;
impl crate::RegisterSpec for AdcifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcifg::R`](R) reader structure"]
impl crate::Readable for AdcifgSpec {}
#[doc = "`write(|w| ..)` method takes [`adcifg::W`](W) writer structure"]
impl crate::Writable for AdcifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCIFG to value 0"]
impl crate::Resettable for AdcifgSpec {}
