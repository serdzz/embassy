#[doc = "Register `TB0CCR5` reader"]
pub type R = crate::R<Tb0ccr5Spec>;
#[doc = "Register `TB0CCR5` writer"]
pub type W = crate::W<Tb0ccr5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0ccr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0ccr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tb0ccr5Spec;
impl crate::RegisterSpec for Tb0ccr5Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tb0ccr5::R`](R) reader structure"]
impl crate::Readable for Tb0ccr5Spec {}
#[doc = "`write(|w| ..)` method takes [`tb0ccr5::W`](W) writer structure"]
impl crate::Writable for Tb0ccr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TB0CCR5 to value 0"]
impl crate::Resettable for Tb0ccr5Spec {}
