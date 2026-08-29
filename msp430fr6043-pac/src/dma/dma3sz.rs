#[doc = "Register `DMA3SZ` reader"]
pub type R = crate::R<Dma3szSpec>;
#[doc = "Register `DMA3SZ` writer"]
pub type W = crate::W<Dma3szSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 3 Transfer Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dma3sz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma3sz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma3szSpec;
impl crate::RegisterSpec for Dma3szSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma3sz::R`](R) reader structure"]
impl crate::Readable for Dma3szSpec {}
#[doc = "`write(|w| ..)` method takes [`dma3sz::W`](W) writer structure"]
impl crate::Writable for Dma3szSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA3SZ to value 0"]
impl crate::Resettable for Dma3szSpec {}
