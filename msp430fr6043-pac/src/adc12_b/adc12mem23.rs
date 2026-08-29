#[doc = "Register `ADC12MEM23` reader"]
pub type R = crate::R<Adc12mem23Spec>;
#[doc = "Register `ADC12MEM23` writer"]
pub type W = crate::W<Adc12mem23Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mem23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mem23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12mem23Spec;
impl crate::RegisterSpec for Adc12mem23Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12mem23::R`](R) reader structure"]
impl crate::Readable for Adc12mem23Spec {}
#[doc = "`write(|w| ..)` method takes [`adc12mem23::W`](W) writer structure"]
impl crate::Writable for Adc12mem23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC12MEM23 to value 0"]
impl crate::Resettable for Adc12mem23Spec {}
