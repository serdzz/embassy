#[doc = "Register `DMAIV` reader"]
pub type R = crate::R<DmaivSpec>;
#[doc = "Register `DMAIV` writer"]
pub type W = crate::W<DmaivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaivSpec;
impl crate::RegisterSpec for DmaivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmaiv::R`](R) reader structure"]
impl crate::Readable for DmaivSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaiv::W`](W) writer structure"]
impl crate::Writable for DmaivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAIV to value 0"]
impl crate::Resettable for DmaivSpec {}
