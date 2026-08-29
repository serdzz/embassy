#[doc = "Register `DMA4DA` reader"]
pub type R = crate::R<Dma4daSpec>;
#[doc = "Register `DMA4DA` writer"]
pub type W = crate::W<Dma4daSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 4 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma4da::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma4da::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma4daSpec;
impl crate::RegisterSpec for Dma4daSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma4da::R`](R) reader structure"]
impl crate::Readable for Dma4daSpec {}
#[doc = "`write(|w| ..)` method takes [`dma4da::W`](W) writer structure"]
impl crate::Writable for Dma4daSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA4DA to value 0"]
impl crate::Resettable for Dma4daSpec {}
