#[doc = "Register `UCB1TXBUF` reader"]
pub type R = crate::R<Ucb1txbufSpec>;
#[doc = "Register `UCB1TXBUF` writer"]
pub type W = crate::W<Ucb1txbufSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI B1 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1txbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1txbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb1txbufSpec;
impl crate::RegisterSpec for Ucb1txbufSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb1txbuf::R`](R) reader structure"]
impl crate::Readable for Ucb1txbufSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb1txbuf::W`](W) writer structure"]
impl crate::Writable for Ucb1txbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB1TXBUF to value 0"]
impl crate::Resettable for Ucb1txbufSpec {}
