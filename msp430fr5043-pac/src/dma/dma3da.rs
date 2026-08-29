#[doc = "Register `DMA3DA` reader"]
pub type R = crate::R<Dma3daSpec>;
#[doc = "Register `DMA3DA` writer"]
pub type W = crate::W<Dma3daSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 3 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma3da::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma3da::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma3daSpec;
impl crate::RegisterSpec for Dma3daSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma3da::R`](R) reader structure"]
impl crate::Readable for Dma3daSpec {}
#[doc = "`write(|w| ..)` method takes [`dma3da::W`](W) writer structure"]
impl crate::Writable for Dma3daSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA3DA to value 0"]
impl crate::Resettable for Dma3daSpec {}
