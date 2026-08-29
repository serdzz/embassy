#[doc = "Register `TA3CCR1` reader"]
pub type R = crate::R<Ta3ccr1Spec>;
#[doc = "Register `TA3CCR1` writer"]
pub type W = crate::W<Ta3ccr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer_A Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta3ccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta3ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta3ccr1Spec;
impl crate::RegisterSpec for Ta3ccr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta3ccr1::R`](R) reader structure"]
impl crate::Readable for Ta3ccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ta3ccr1::W`](W) writer structure"]
impl crate::Writable for Ta3ccr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA3CCR1 to value 0"]
impl crate::Resettable for Ta3ccr1Spec {}
