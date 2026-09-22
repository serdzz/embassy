#[doc = "Register `UCB0RXBUF` reader"]
pub type R = crate::R<Ucb0rxbufSpec>;
#[doc = "Register `UCB0RXBUF` writer"]
pub type W = crate::W<Ucb0rxbufSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI B0 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0rxbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0rxbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0rxbufSpec;
impl crate::RegisterSpec for Ucb0rxbufSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0rxbuf::R`](R) reader structure"]
impl crate::Readable for Ucb0rxbufSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0rxbuf::W`](W) writer structure"]
impl crate::Writable for Ucb0rxbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0RXBUF to value 0"]
impl crate::Resettable for Ucb0rxbufSpec {}
