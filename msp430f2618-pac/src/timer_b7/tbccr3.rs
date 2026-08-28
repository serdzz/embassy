#[doc = "Register `TBCCR3` reader"]
pub type R = crate::R<Tbccr3Spec>;
#[doc = "Register `TBCCR3` writer"]
pub type W = crate::W<Tbccr3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer B Capture/Compare 3\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbccr3Spec;
impl crate::RegisterSpec for Tbccr3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbccr3::R`](R) reader structure"]
impl crate::Readable for Tbccr3Spec {}
#[doc = "`write(|w| ..)` method takes [`tbccr3::W`](W) writer structure"]
impl crate::Writable for Tbccr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBCCR3 to value 0"]
impl crate::Resettable for Tbccr3Spec {}
