#[doc = "Register `UCA2TXBUF_SPI` reader"]
pub type R = crate::R<Uca2txbufSpiSpec>;
#[doc = "Register `UCA2TXBUF_SPI` writer"]
pub type W = crate::W<Uca2txbufSpiSpec>;
#[doc = "Field `UCTXBUF` reader - Transmit data buffer"]
pub type UctxbufR = crate::FieldReader;
#[doc = "Field `UCTXBUF` writer - Transmit data buffer"]
pub type UctxbufW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&self) -> UctxbufR {
        UctxbufR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&mut self) -> UctxbufW<'_, Uca2txbufSpiSpec> {
        UctxbufW::new(self, 0)
    }
}
#[doc = "eUSCI_Ax Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2txbuf_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2txbuf_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca2txbufSpiSpec;
impl crate::RegisterSpec for Uca2txbufSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca2txbuf_spi::R`](R) reader structure"]
impl crate::Readable for Uca2txbufSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`uca2txbuf_spi::W`](W) writer structure"]
impl crate::Writable for Uca2txbufSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA2TXBUF_SPI to value 0"]
impl crate::Resettable for Uca2txbufSpiSpec {}
