#[doc = "Register `TA4CCR0` reader"]
pub type R = crate::R<Ta4ccr0Spec>;
#[doc = "Register `TA4CCR0` writer"]
pub type W = crate::W<Ta4ccr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer_A Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta4ccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta4ccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta4ccr0Spec;
impl crate::RegisterSpec for Ta4ccr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta4ccr0::R`](R) reader structure"]
impl crate::Readable for Ta4ccr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ta4ccr0::W`](W) writer structure"]
impl crate::Writable for Ta4ccr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA4CCR0 to value 0"]
impl crate::Resettable for Ta4ccr0Spec {}
