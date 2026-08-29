#[doc = "Register `DMA5SZ` reader"]
pub type R = crate::R<Dma5szSpec>;
#[doc = "Register `DMA5SZ` writer"]
pub type W = crate::W<Dma5szSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 5 Transfer Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dma5sz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma5sz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma5szSpec;
impl crate::RegisterSpec for Dma5szSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma5sz::R`](R) reader structure"]
impl crate::Readable for Dma5szSpec {}
#[doc = "`write(|w| ..)` method takes [`dma5sz::W`](W) writer structure"]
impl crate::Writable for Dma5szSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA5SZ to value 0"]
impl crate::Resettable for Dma5szSpec {}
