#[doc = "Register `TACCR1` reader"]
pub type R = crate::R<Taccr1Spec>;
#[doc = "Register `TACCR1` writer"]
pub type W = crate::W<Taccr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer A Capture/Compare 1\n\nYou can [`read`](crate::Reg::read) this register and get [`taccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`taccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Taccr1Spec;
impl crate::RegisterSpec for Taccr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`taccr1::R`](R) reader structure"]
impl crate::Readable for Taccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`taccr1::W`](W) writer structure"]
impl crate::Writable for Taccr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TACCR1 to value 0"]
impl crate::Resettable for Taccr1Spec {}
