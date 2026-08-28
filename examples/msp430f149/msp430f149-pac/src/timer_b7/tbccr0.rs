#[doc = "Register `TBCCR0` reader"]
pub type R = crate::R<Tbccr0Spec>;
#[doc = "Register `TBCCR0` writer"]
pub type W = crate::W<Tbccr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer B Capture/Compare 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbccr0Spec;
impl crate::RegisterSpec for Tbccr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbccr0::R`](R) reader structure"]
impl crate::Readable for Tbccr0Spec {}
#[doc = "`write(|w| ..)` method takes [`tbccr0::W`](W) writer structure"]
impl crate::Writable for Tbccr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBCCR0 to value 0"]
impl crate::Resettable for Tbccr0Spec {}
