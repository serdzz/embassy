#[doc = "Register `DMA0SAL` reader"]
pub type R = crate::R<Dma0salSpec>;
#[doc = "Register `DMA0SAL` writer"]
pub type W = crate::W<Dma0salSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 0 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0sal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0sal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma0salSpec;
impl crate::RegisterSpec for Dma0salSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma0sal::R`](R) reader structure"]
impl crate::Readable for Dma0salSpec {}
#[doc = "`write(|w| ..)` method takes [`dma0sal::W`](W) writer structure"]
impl crate::Writable for Dma0salSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA0SAL to value 0"]
impl crate::Resettable for Dma0salSpec {}
