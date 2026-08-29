#[doc = "Register `DMA1SA` reader"]
pub type R = crate::R<Dma1saSpec>;
#[doc = "Register `DMA1SA` writer"]
pub type W = crate::W<Dma1saSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 1 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1sa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1sa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma1saSpec;
impl crate::RegisterSpec for Dma1saSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma1sa::R`](R) reader structure"]
impl crate::Readable for Dma1saSpec {}
#[doc = "`write(|w| ..)` method takes [`dma1sa::W`](W) writer structure"]
impl crate::Writable for Dma1saSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA1SA to value 0"]
impl crate::Resettable for Dma1saSpec {}
