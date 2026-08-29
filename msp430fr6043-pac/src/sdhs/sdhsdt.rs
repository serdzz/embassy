#[doc = "Register `SDHSDT` reader"]
pub type R = crate::R<SdhsdtSpec>;
#[doc = "Register `SDHSDT` writer"]
pub type W = crate::W<SdhsdtSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SDHS Data Converstion Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsdt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsdt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhsdtSpec;
impl crate::RegisterSpec for SdhsdtSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsdt::R`](R) reader structure"]
impl crate::Readable for SdhsdtSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhsdt::W`](W) writer structure"]
impl crate::Writable for SdhsdtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSDT to value 0"]
impl crate::Resettable for SdhsdtSpec {}
