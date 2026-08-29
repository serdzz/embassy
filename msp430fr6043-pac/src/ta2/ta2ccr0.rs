#[doc = "Register `TA2CCR0` reader"]
pub type R = crate::R<Ta2ccr0Spec>;
#[doc = "Register `TA2CCR0` writer"]
pub type W = crate::W<Ta2ccr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer_A Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta2ccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta2ccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta2ccr0Spec;
impl crate::RegisterSpec for Ta2ccr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta2ccr0::R`](R) reader structure"]
impl crate::Readable for Ta2ccr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ta2ccr0::W`](W) writer structure"]
impl crate::Writable for Ta2ccr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA2CCR0 to value 0"]
impl crate::Resettable for Ta2ccr0Spec {}
