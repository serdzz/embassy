#[doc = "Register `TBCCR5` reader"]
pub type R = crate::R<Tbccr5Spec>;
#[doc = "Register `TBCCR5` writer"]
pub type W = crate::W<Tbccr5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer B Capture/Compare 5\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbccr5Spec;
impl crate::RegisterSpec for Tbccr5Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbccr5::R`](R) reader structure"]
impl crate::Readable for Tbccr5Spec {}
#[doc = "`write(|w| ..)` method takes [`tbccr5::W`](W) writer structure"]
impl crate::Writable for Tbccr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBCCR5 to value 0"]
impl crate::Resettable for Tbccr5Spec {}
