#[doc = "Register `UCB0TXBUF_SPI` reader"]
pub type R = crate::R<Ucb0txbufSpiSpec>;
#[doc = "Register `UCB0TXBUF_SPI` writer"]
pub type W = crate::W<Ucb0txbufSpiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI B0 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0txbuf_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0txbuf_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0txbufSpiSpec;
impl crate::RegisterSpec for Ucb0txbufSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0txbuf_spi::R`](R) reader structure"]
impl crate::Readable for Ucb0txbufSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0txbuf_spi::W`](W) writer structure"]
impl crate::Writable for Ucb0txbufSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0TXBUF_SPI to value 0"]
impl crate::Resettable for Ucb0txbufSpiSpec {}
