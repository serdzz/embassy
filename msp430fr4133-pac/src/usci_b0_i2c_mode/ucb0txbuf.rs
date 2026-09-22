#[doc = "Register `UCB0TXBUF` reader"]
pub type R = crate::R<Ucb0txbufSpec>;
#[doc = "Register `UCB0TXBUF` writer"]
pub type W = crate::W<Ucb0txbufSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI B0 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0txbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0txbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0txbufSpec;
impl crate::RegisterSpec for Ucb0txbufSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0txbuf::R`](R) reader structure"]
impl crate::Readable for Ucb0txbufSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0txbuf::W`](W) writer structure"]
impl crate::Writable for Ucb0txbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0TXBUF to value 0"]
impl crate::Resettable for Ucb0txbufSpec {}
