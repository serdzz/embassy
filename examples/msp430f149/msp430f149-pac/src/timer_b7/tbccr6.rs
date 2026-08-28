#[doc = "Register `TBCCR6` reader"]
pub type R = crate::R<Tbccr6Spec>;
#[doc = "Register `TBCCR6` writer"]
pub type W = crate::W<Tbccr6Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer B Capture/Compare 6\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbccr6Spec;
impl crate::RegisterSpec for Tbccr6Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbccr6::R`](R) reader structure"]
impl crate::Readable for Tbccr6Spec {}
#[doc = "`write(|w| ..)` method takes [`tbccr6::W`](W) writer structure"]
impl crate::Writable for Tbccr6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBCCR6 to value 0"]
impl crate::Resettable for Tbccr6Spec {}
