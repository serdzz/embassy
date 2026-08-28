#[doc = "Register `DMA1SAL` reader"]
pub type R = crate::R<Dma1salSpec>;
#[doc = "Register `DMA1SAL` writer"]
pub type W = crate::W<Dma1salSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 1 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1sal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1sal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma1salSpec;
impl crate::RegisterSpec for Dma1salSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma1sal::R`](R) reader structure"]
impl crate::Readable for Dma1salSpec {}
#[doc = "`write(|w| ..)` method takes [`dma1sal::W`](W) writer structure"]
impl crate::Writable for Dma1salSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA1SAL to value 0"]
impl crate::Resettable for Dma1salSpec {}
