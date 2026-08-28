#[doc = "Register `TBCCR1` reader"]
pub type R = crate::R<Tbccr1Spec>;
#[doc = "Register `TBCCR1` writer"]
pub type W = crate::W<Tbccr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer B Capture/Compare 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbccr1Spec;
impl crate::RegisterSpec for Tbccr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbccr1::R`](R) reader structure"]
impl crate::Readable for Tbccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`tbccr1::W`](W) writer structure"]
impl crate::Writable for Tbccr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBCCR1 to value 0"]
impl crate::Resettable for Tbccr1Spec {}
