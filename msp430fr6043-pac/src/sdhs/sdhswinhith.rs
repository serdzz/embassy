#[doc = "Register `SDHSWINHITH` reader"]
pub type R = crate::R<SdhswinhithSpec>;
#[doc = "Register `SDHSWINHITH` writer"]
pub type W = crate::W<SdhswinhithSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SDHS Window Comparator High Threshold Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhswinhith::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhswinhith::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhswinhithSpec;
impl crate::RegisterSpec for SdhswinhithSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhswinhith::R`](R) reader structure"]
impl crate::Readable for SdhswinhithSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhswinhith::W`](W) writer structure"]
impl crate::Writable for SdhswinhithSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSWINHITH to value 0"]
impl crate::Resettable for SdhswinhithSpec {}
