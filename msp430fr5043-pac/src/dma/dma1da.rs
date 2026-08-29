#[doc = "Register `DMA1DA` reader"]
pub type R = crate::R<Dma1daSpec>;
#[doc = "Register `DMA1DA` writer"]
pub type W = crate::W<Dma1daSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 1 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1da::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1da::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma1daSpec;
impl crate::RegisterSpec for Dma1daSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma1da::R`](R) reader structure"]
impl crate::Readable for Dma1daSpec {}
#[doc = "`write(|w| ..)` method takes [`dma1da::W`](W) writer structure"]
impl crate::Writable for Dma1daSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA1DA to value 0"]
impl crate::Resettable for Dma1daSpec {}
