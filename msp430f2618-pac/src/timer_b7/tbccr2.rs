#[doc = "Register `TBCCR2` reader"]
pub type R = crate::R<Tbccr2Spec>;
#[doc = "Register `TBCCR2` writer"]
pub type W = crate::W<Tbccr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer B Capture/Compare 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbccr2Spec;
impl crate::RegisterSpec for Tbccr2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbccr2::R`](R) reader structure"]
impl crate::Readable for Tbccr2Spec {}
#[doc = "`write(|w| ..)` method takes [`tbccr2::W`](W) writer structure"]
impl crate::Writable for Tbccr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBCCR2 to value 0"]
impl crate::Resettable for Tbccr2Spec {}
