#[doc = "Register `DMA2DA` reader"]
pub type R = crate::R<Dma2daSpec>;
#[doc = "Register `DMA2DA` writer"]
pub type W = crate::W<Dma2daSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 2 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2da::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2da::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma2daSpec;
impl crate::RegisterSpec for Dma2daSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma2da::R`](R) reader structure"]
impl crate::Readable for Dma2daSpec {}
#[doc = "`write(|w| ..)` method takes [`dma2da::W`](W) writer structure"]
impl crate::Writable for Dma2daSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA2DA to value 0"]
impl crate::Resettable for Dma2daSpec {}
