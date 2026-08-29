#[doc = "Register `DMA5SA` reader"]
pub type R = crate::R<Dma5saSpec>;
#[doc = "Register `DMA5SA` writer"]
pub type W = crate::W<Dma5saSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 5 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma5sa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma5sa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma5saSpec;
impl crate::RegisterSpec for Dma5saSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma5sa::R`](R) reader structure"]
impl crate::Readable for Dma5saSpec {}
#[doc = "`write(|w| ..)` method takes [`dma5sa::W`](W) writer structure"]
impl crate::Writable for Dma5saSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA5SA to value 0"]
impl crate::Resettable for Dma5saSpec {}
