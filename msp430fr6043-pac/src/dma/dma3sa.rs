#[doc = "Register `DMA3SA` reader"]
pub type R = crate::R<Dma3saSpec>;
#[doc = "Register `DMA3SA` writer"]
pub type W = crate::W<Dma3saSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 3 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma3sa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma3sa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma3saSpec;
impl crate::RegisterSpec for Dma3saSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma3sa::R`](R) reader structure"]
impl crate::Readable for Dma3saSpec {}
#[doc = "`write(|w| ..)` method takes [`dma3sa::W`](W) writer structure"]
impl crate::Writable for Dma3saSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA3SA to value 0"]
impl crate::Resettable for Dma3saSpec {}
