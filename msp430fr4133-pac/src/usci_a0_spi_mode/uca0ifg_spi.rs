#[doc = "Register `UCA0IFG_SPI` reader"]
pub type R = crate::R<Uca0ifgSpiSpec>;
#[doc = "Register `UCA0IFG_SPI` writer"]
pub type W = crate::W<Uca0ifgSpiSpec>;
#[doc = "Field `UCRXIFG` reader - SPI Receive Interrupt Flag"]
pub type UcrxifgR = crate::BitReader;
#[doc = "Field `UCRXIFG` writer - SPI Receive Interrupt Flag"]
pub type UcrxifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIFG` reader - SPI Transmit Interrupt Flag"]
pub type UctxifgR = crate::BitReader;
#[doc = "Field `UCTXIFG` writer - SPI Transmit Interrupt Flag"]
pub type UctxifgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI Receive Interrupt Flag"]
    #[inline(always)]
    pub fn ucrxifg(&self) -> UcrxifgR {
        UcrxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Transmit Interrupt Flag"]
    #[inline(always)]
    pub fn uctxifg(&self) -> UctxifgR {
        UctxifgR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Receive Interrupt Flag"]
    #[inline(always)]
    pub fn ucrxifg(&mut self) -> UcrxifgW<'_, Uca0ifgSpiSpec> {
        UcrxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - SPI Transmit Interrupt Flag"]
    #[inline(always)]
    pub fn uctxifg(&mut self) -> UctxifgW<'_, Uca0ifgSpiSpec> {
        UctxifgW::new(self, 1)
    }
}
#[doc = "USCI A0 Interrupt Flags Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ifg_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ifg_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0ifgSpiSpec;
impl crate::RegisterSpec for Uca0ifgSpiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0ifg_spi::R`](R) reader structure"]
impl crate::Readable for Uca0ifgSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0ifg_spi::W`](W) writer structure"]
impl crate::Writable for Uca0ifgSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0IFG_SPI to value 0"]
impl crate::Resettable for Uca0ifgSpiSpec {}
