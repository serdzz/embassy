#[doc = "Register `UCA2RXBUF_SPI` reader"]
pub type R = crate::R<Uca2rxbufSpiSpec>;
#[doc = "Register `UCA2RXBUF_SPI` writer"]
pub type W = crate::W<Uca2rxbufSpiSpec>;
#[doc = "Field `UCRXBUF` reader - Receive data buffer"]
pub type UcrxbufR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive data buffer"]
    #[inline(always)]
    pub fn ucrxbuf(&self) -> UcrxbufR {
        UcrxbufR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "eUSCI_Ax Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2rxbuf_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2rxbuf_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca2rxbufSpiSpec;
impl crate::RegisterSpec for Uca2rxbufSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca2rxbuf_spi::R`](R) reader structure"]
impl crate::Readable for Uca2rxbufSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`uca2rxbuf_spi::W`](W) writer structure"]
impl crate::Writable for Uca2rxbufSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA2RXBUF_SPI to value 0"]
impl crate::Resettable for Uca2rxbufSpiSpec {}
