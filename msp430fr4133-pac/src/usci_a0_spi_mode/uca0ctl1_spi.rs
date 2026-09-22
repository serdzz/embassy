#[doc = "Register `UCA0CTL1_SPI` reader"]
pub type R = crate::R<Uca0ctl1SpiSpec>;
#[doc = "Register `UCA0CTL1_SPI` writer"]
pub type W = crate::W<Uca0ctl1SpiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI A0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctl1_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctl1_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0ctl1SpiSpec;
impl crate::RegisterSpec for Uca0ctl1SpiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0ctl1_spi::R`](R) reader structure"]
impl crate::Readable for Uca0ctl1SpiSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0ctl1_spi::W`](W) writer structure"]
impl crate::Writable for Uca0ctl1SpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0CTL1_SPI to value 0"]
impl crate::Resettable for Uca0ctl1SpiSpec {}
