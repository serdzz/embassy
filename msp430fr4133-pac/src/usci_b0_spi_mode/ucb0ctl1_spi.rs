#[doc = "Register `UCB0CTL1_SPI` reader"]
pub type R = crate::R<Ucb0ctl1SpiSpec>;
#[doc = "Register `UCB0CTL1_SPI` writer"]
pub type W = crate::W<Ucb0ctl1SpiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI B0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctl1_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctl1_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0ctl1SpiSpec;
impl crate::RegisterSpec for Ucb0ctl1SpiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0ctl1_spi::R`](R) reader structure"]
impl crate::Readable for Ucb0ctl1SpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0ctl1_spi::W`](W) writer structure"]
impl crate::Writable for Ucb0ctl1SpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0CTL1_SPI to value 0"]
impl crate::Resettable for Ucb0ctl1SpiSpec {}
