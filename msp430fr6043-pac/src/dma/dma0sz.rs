#[doc = "Register `DMA0SZ` reader"]
pub type R = crate::R<Dma0szSpec>;
#[doc = "Register `DMA0SZ` writer"]
pub type W = crate::W<Dma0szSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 0 Transfer Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0sz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0sz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma0szSpec;
impl crate::RegisterSpec for Dma0szSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma0sz::R`](R) reader structure"]
impl crate::Readable for Dma0szSpec {}
#[doc = "`write(|w| ..)` method takes [`dma0sz::W`](W) writer structure"]
impl crate::Writable for Dma0szSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA0SZ to value 0"]
impl crate::Resettable for Dma0szSpec {}
