#[doc = "Register `DMA2SZ` reader"]
pub type R = crate::R<Dma2szSpec>;
#[doc = "Register `DMA2SZ` writer"]
pub type W = crate::W<Dma2szSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 2 Transfer Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2sz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2sz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma2szSpec;
impl crate::RegisterSpec for Dma2szSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma2sz::R`](R) reader structure"]
impl crate::Readable for Dma2szSpec {}
#[doc = "`write(|w| ..)` method takes [`dma2sz::W`](W) writer structure"]
impl crate::Writable for Dma2szSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA2SZ to value 0"]
impl crate::Resettable for Dma2szSpec {}
