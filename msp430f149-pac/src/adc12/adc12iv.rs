#[doc = "Register `ADC12IV` reader"]
pub type R = crate::R<Adc12ivSpec>;
#[doc = "Register `ADC12IV` writer"]
pub type W = crate::W<Adc12ivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC12 Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12ivSpec;
impl crate::RegisterSpec for Adc12ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12iv::R`](R) reader structure"]
impl crate::Readable for Adc12ivSpec {}
#[doc = "`write(|w| ..)` method takes [`adc12iv::W`](W) writer structure"]
impl crate::Writable for Adc12ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC12IV to value 0"]
impl crate::Resettable for Adc12ivSpec {}
