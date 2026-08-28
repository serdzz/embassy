#[doc = "Register `TACCR0` reader"]
pub type R = crate::R<Taccr0Spec>;
#[doc = "Register `TACCR0` writer"]
pub type W = crate::W<Taccr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer A Capture/Compare 0\n\nYou can [`read`](crate::Reg::read) this register and get [`taccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`taccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Taccr0Spec;
impl crate::RegisterSpec for Taccr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`taccr0::R`](R) reader structure"]
impl crate::Readable for Taccr0Spec {}
#[doc = "`write(|w| ..)` method takes [`taccr0::W`](W) writer structure"]
impl crate::Writable for Taccr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TACCR0 to value 0"]
impl crate::Resettable for Taccr0Spec {}
