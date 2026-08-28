#[doc = "Register `UCB1RXBUF_SPI` reader"]
pub type R = crate::R<Ucb1rxbufSpiSpec>;
#[doc = "Register `UCB1RXBUF_SPI` writer"]
pub type W = crate::W<Ucb1rxbufSpiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI B1 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1rxbuf_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1rxbuf_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb1rxbufSpiSpec;
impl crate::RegisterSpec for Ucb1rxbufSpiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb1rxbuf_spi::R`](R) reader structure"]
impl crate::Readable for Ucb1rxbufSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb1rxbuf_spi::W`](W) writer structure"]
impl crate::Writable for Ucb1rxbufSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB1RXBUF_SPI to value 0"]
impl crate::Resettable for Ucb1rxbufSpiSpec {}
