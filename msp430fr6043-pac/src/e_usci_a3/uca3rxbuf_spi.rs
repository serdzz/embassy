#[doc = "Register `UCA3RXBUF_SPI` reader"]
pub type R = crate::R<Uca3rxbufSpiSpec>;
#[doc = "Register `UCA3RXBUF_SPI` writer"]
pub type W = crate::W<Uca3rxbufSpiSpec>;
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
#[doc = "eUSCI_Ax Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3rxbuf_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3rxbuf_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca3rxbufSpiSpec;
impl crate::RegisterSpec for Uca3rxbufSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca3rxbuf_spi::R`](R) reader structure"]
impl crate::Readable for Uca3rxbufSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`uca3rxbuf_spi::W`](W) writer structure"]
impl crate::Writable for Uca3rxbufSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA3RXBUF_SPI to value 0"]
impl crate::Resettable for Uca3rxbufSpiSpec {}
