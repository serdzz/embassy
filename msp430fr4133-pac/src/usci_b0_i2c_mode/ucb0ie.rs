#[doc = "Register `UCB0IE` reader"]
pub type R = crate::R<Ucb0ieSpec>;
#[doc = "Register `UCB0IE` writer"]
pub type W = crate::W<Ucb0ieSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI B0 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0ieSpec;
impl crate::RegisterSpec for Ucb0ieSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0ie::R`](R) reader structure"]
impl crate::Readable for Ucb0ieSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0ie::W`](W) writer structure"]
impl crate::Writable for Ucb0ieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0IE to value 0"]
impl crate::Resettable for Ucb0ieSpec {}
