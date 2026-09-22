#[doc = "Register `UCB0CTL0` reader"]
pub type R = crate::R<Ucb0ctl0Spec>;
#[doc = "Register `UCB0CTL0` writer"]
pub type W = crate::W<Ucb0ctl0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI B0 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0ctl0Spec;
impl crate::RegisterSpec for Ucb0ctl0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0ctl0::R`](R) reader structure"]
impl crate::Readable for Ucb0ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ucb0ctl0::W`](W) writer structure"]
impl crate::Writable for Ucb0ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0CTL0 to value 0"]
impl crate::Resettable for Ucb0ctl0Spec {}
