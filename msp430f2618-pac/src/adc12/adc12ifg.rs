#[doc = "Register `ADC12IFG` reader"]
pub type R = crate::R<Adc12ifgSpec>;
#[doc = "Register `ADC12IFG` writer"]
pub type W = crate::W<Adc12ifgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC12 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12ifgSpec;
impl crate::RegisterSpec for Adc12ifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12ifg::R`](R) reader structure"]
impl crate::Readable for Adc12ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`adc12ifg::W`](W) writer structure"]
impl crate::Writable for Adc12ifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC12IFG to value 0"]
impl crate::Resettable for Adc12ifgSpec {}
