#[doc = "Register `DMA2DAL` reader"]
pub type R = crate::R<Dma2dalSpec>;
#[doc = "Register `DMA2DAL` writer"]
pub type W = crate::W<Dma2dalSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 2 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2dal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2dal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma2dalSpec;
impl crate::RegisterSpec for Dma2dalSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma2dal::R`](R) reader structure"]
impl crate::Readable for Dma2dalSpec {}
#[doc = "`write(|w| ..)` method takes [`dma2dal::W`](W) writer structure"]
impl crate::Writable for Dma2dalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA2DAL to value 0"]
impl crate::Resettable for Dma2dalSpec {}
