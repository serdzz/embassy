#[doc = "Register `DMA0SA` reader"]
pub type R = crate::R<Dma0saSpec>;
#[doc = "Register `DMA0SA` writer"]
pub type W = crate::W<Dma0saSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 0 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0sa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0sa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma0saSpec;
impl crate::RegisterSpec for Dma0saSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma0sa::R`](R) reader structure"]
impl crate::Readable for Dma0saSpec {}
#[doc = "`write(|w| ..)` method takes [`dma0sa::W`](W) writer structure"]
impl crate::Writable for Dma0saSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA0SA to value 0"]
impl crate::Resettable for Dma0saSpec {}
