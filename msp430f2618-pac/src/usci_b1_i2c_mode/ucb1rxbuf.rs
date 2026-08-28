#[doc = "Register `UCB1RXBUF` reader"]
pub type R = crate::R<Ucb1rxbufSpec>;
#[doc = "Register `UCB1RXBUF` writer"]
pub type W = crate::W<Ucb1rxbufSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI B1 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1rxbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1rxbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb1rxbufSpec;
impl crate::RegisterSpec for Ucb1rxbufSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb1rxbuf::R`](R) reader structure"]
impl crate::Readable for Ucb1rxbufSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb1rxbuf::W`](W) writer structure"]
impl crate::Writable for Ucb1rxbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB1RXBUF to value 0"]
impl crate::Resettable for Ucb1rxbufSpec {}
