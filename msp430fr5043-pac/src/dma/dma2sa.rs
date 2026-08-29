#[doc = "Register `DMA2SA` reader"]
pub type R = crate::R<Dma2saSpec>;
#[doc = "Register `DMA2SA` writer"]
pub type W = crate::W<Dma2saSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 2 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2sa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2sa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma2saSpec;
impl crate::RegisterSpec for Dma2saSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma2sa::R`](R) reader structure"]
impl crate::Readable for Dma2saSpec {}
#[doc = "`write(|w| ..)` method takes [`dma2sa::W`](W) writer structure"]
impl crate::Writable for Dma2saSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA2SA to value 0"]
impl crate::Resettable for Dma2saSpec {}
