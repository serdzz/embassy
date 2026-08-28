#[doc = "Register `UCA1MCTL_SPI` reader"]
pub type R = crate::R<Uca1mctlSpiSpec>;
#[doc = "Register `UCA1MCTL_SPI` writer"]
pub type W = crate::W<Uca1mctlSpiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI A1 Modulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1mctl_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1mctl_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca1mctlSpiSpec;
impl crate::RegisterSpec for Uca1mctlSpiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1mctl_spi::R`](R) reader structure"]
impl crate::Readable for Uca1mctlSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`uca1mctl_spi::W`](W) writer structure"]
impl crate::Writable for Uca1mctlSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA1MCTL_SPI to value 0"]
impl crate::Resettable for Uca1mctlSpiSpec {}
