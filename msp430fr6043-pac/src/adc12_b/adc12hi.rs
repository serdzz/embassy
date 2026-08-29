#[doc = "Register `ADC12HI` reader"]
pub type R = crate::R<Adc12hiSpec>;
#[doc = "Register `ADC12HI` writer"]
pub type W = crate::W<Adc12hiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC12_B Window Comparator High Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12hi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12hi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12hiSpec;
impl crate::RegisterSpec for Adc12hiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12hi::R`](R) reader structure"]
impl crate::Readable for Adc12hiSpec {}
#[doc = "`write(|w| ..)` method takes [`adc12hi::W`](W) writer structure"]
impl crate::Writable for Adc12hiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC12HI to value 0"]
impl crate::Resettable for Adc12hiSpec {}
