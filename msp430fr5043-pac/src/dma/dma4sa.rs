#[doc = "Register `DMA4SA` reader"]
pub type R = crate::R<Dma4saSpec>;
#[doc = "Register `DMA4SA` writer"]
pub type W = crate::W<Dma4saSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 4 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma4sa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma4sa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma4saSpec;
impl crate::RegisterSpec for Dma4saSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma4sa::R`](R) reader structure"]
impl crate::Readable for Dma4saSpec {}
#[doc = "`write(|w| ..)` method takes [`dma4sa::W`](W) writer structure"]
impl crate::Writable for Dma4saSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA4SA to value 0"]
impl crate::Resettable for Dma4saSpec {}
