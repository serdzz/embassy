#[doc = "Register `TACCR2` reader"]
pub type R = crate::R<Taccr2Spec>;
#[doc = "Register `TACCR2` writer"]
pub type W = crate::W<Taccr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer A Capture/Compare 2\n\nYou can [`read`](crate::Reg::read) this register and get [`taccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`taccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Taccr2Spec;
impl crate::RegisterSpec for Taccr2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`taccr2::R`](R) reader structure"]
impl crate::Readable for Taccr2Spec {}
#[doc = "`write(|w| ..)` method takes [`taccr2::W`](W) writer structure"]
impl crate::Writable for Taccr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TACCR2 to value 0"]
impl crate::Resettable for Taccr2Spec {}
