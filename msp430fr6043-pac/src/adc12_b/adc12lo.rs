#[doc = "Register `ADC12LO` reader"]
pub type R = crate::R<Adc12loSpec>;
#[doc = "Register `ADC12LO` writer"]
pub type W = crate::W<Adc12loSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC12_B Window Comparator Low Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12lo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12lo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12loSpec;
impl crate::RegisterSpec for Adc12loSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12lo::R`](R) reader structure"]
impl crate::Readable for Adc12loSpec {}
#[doc = "`write(|w| ..)` method takes [`adc12lo::W`](W) writer structure"]
impl crate::Writable for Adc12loSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC12LO to value 0"]
impl crate::Resettable for Adc12loSpec {}
