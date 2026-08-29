#[doc = "Register `DMA0DA` reader"]
pub type R = crate::R<Dma0daSpec>;
#[doc = "Register `DMA0DA` writer"]
pub type W = crate::W<Dma0daSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 0 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0da::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0da::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma0daSpec;
impl crate::RegisterSpec for Dma0daSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma0da::R`](R) reader structure"]
impl crate::Readable for Dma0daSpec {}
#[doc = "`write(|w| ..)` method takes [`dma0da::W`](W) writer structure"]
impl crate::Writable for Dma0daSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA0DA to value 0"]
impl crate::Resettable for Dma0daSpec {}
