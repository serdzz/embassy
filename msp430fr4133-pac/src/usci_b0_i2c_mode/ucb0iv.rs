#[doc = "Register `UCB0IV` reader"]
pub type R = crate::R<Ucb0ivSpec>;
#[doc = "Register `UCB0IV` writer"]
pub type W = crate::W<Ucb0ivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI B0 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0ivSpec;
impl crate::RegisterSpec for Ucb0ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0iv::R`](R) reader structure"]
impl crate::Readable for Ucb0ivSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0iv::W`](W) writer structure"]
impl crate::Writable for Ucb0ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0IV to value 0"]
impl crate::Resettable for Ucb0ivSpec {}
