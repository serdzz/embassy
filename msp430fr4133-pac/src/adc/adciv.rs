#[doc = "Register `ADCIV` reader"]
pub type R = crate::R<AdcivSpec>;
#[doc = "Register `ADCIV` writer"]
pub type W = crate::W<AdcivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`adciv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adciv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcivSpec;
impl crate::RegisterSpec for AdcivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adciv::R`](R) reader structure"]
impl crate::Readable for AdcivSpec {}
#[doc = "`write(|w| ..)` method takes [`adciv::W`](W) writer structure"]
impl crate::Writable for AdcivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCIV to value 0"]
impl crate::Resettable for AdcivSpec {}
