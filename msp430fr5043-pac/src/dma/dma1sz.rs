#[doc = "Register `DMA1SZ` reader"]
pub type R = crate::R<Dma1szSpec>;
#[doc = "Register `DMA1SZ` writer"]
pub type W = crate::W<Dma1szSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 1 Transfer Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1sz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1sz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma1szSpec;
impl crate::RegisterSpec for Dma1szSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma1sz::R`](R) reader structure"]
impl crate::Readable for Dma1szSpec {}
#[doc = "`write(|w| ..)` method takes [`dma1sz::W`](W) writer structure"]
impl crate::Writable for Dma1szSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA1SZ to value 0"]
impl crate::Resettable for Dma1szSpec {}
