#[doc = "Register `DMA4SZ` reader"]
pub type R = crate::R<Dma4szSpec>;
#[doc = "Register `DMA4SZ` writer"]
pub type W = crate::W<Dma4szSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 4 Transfer Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dma4sz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma4sz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma4szSpec;
impl crate::RegisterSpec for Dma4szSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma4sz::R`](R) reader structure"]
impl crate::Readable for Dma4szSpec {}
#[doc = "`write(|w| ..)` method takes [`dma4sz::W`](W) writer structure"]
impl crate::Writable for Dma4szSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA4SZ to value 0"]
impl crate::Resettable for Dma4szSpec {}
