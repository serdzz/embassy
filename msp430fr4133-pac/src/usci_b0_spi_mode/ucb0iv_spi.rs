#[doc = "Register `UCB0IV_SPI` reader"]
pub type R = crate::R<Ucb0ivSpiSpec>;
#[doc = "Register `UCB0IV_SPI` writer"]
pub type W = crate::W<Ucb0ivSpiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI B0 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0iv_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0iv_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0ivSpiSpec;
impl crate::RegisterSpec for Ucb0ivSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0iv_spi::R`](R) reader structure"]
impl crate::Readable for Ucb0ivSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0iv_spi::W`](W) writer structure"]
impl crate::Writable for Ucb0ivSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0IV_SPI to value 0"]
impl crate::Resettable for Ucb0ivSpiSpec {}
