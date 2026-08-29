#[doc = "Register `DMA5DA` reader"]
pub type R = crate::R<Dma5daSpec>;
#[doc = "Register `DMA5DA` writer"]
pub type W = crate::W<Dma5daSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 5 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma5da::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma5da::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma5daSpec;
impl crate::RegisterSpec for Dma5daSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma5da::R`](R) reader structure"]
impl crate::Readable for Dma5daSpec {}
#[doc = "`write(|w| ..)` method takes [`dma5da::W`](W) writer structure"]
impl crate::Writable for Dma5daSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA5DA to value 0"]
impl crate::Resettable for Dma5daSpec {}
