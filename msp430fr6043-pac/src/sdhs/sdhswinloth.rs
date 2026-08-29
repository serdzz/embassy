#[doc = "Register `SDHSWINLOTH` reader"]
pub type R = crate::R<SdhswinlothSpec>;
#[doc = "Register `SDHSWINLOTH` writer"]
pub type W = crate::W<SdhswinlothSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SDHS Window Comparator Low Threshold Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhswinloth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhswinloth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhswinlothSpec;
impl crate::RegisterSpec for SdhswinlothSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhswinloth::R`](R) reader structure"]
impl crate::Readable for SdhswinlothSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhswinloth::W`](W) writer structure"]
impl crate::Writable for SdhswinlothSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSWINLOTH to value 0"]
impl crate::Resettable for SdhswinlothSpec {}
