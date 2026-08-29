#[doc = "Register `SDHSDESCHI` reader"]
pub type R = crate::R<SdhsdeschiSpec>;
#[doc = "Register `SDHSDESCHI` writer"]
pub type W = crate::W<SdhsdeschiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SDHS Descriptor Register H.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsdeschi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsdeschi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhsdeschiSpec;
impl crate::RegisterSpec for SdhsdeschiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsdeschi::R`](R) reader structure"]
impl crate::Readable for SdhsdeschiSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhsdeschi::W`](W) writer structure"]
impl crate::Writable for SdhsdeschiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSDESCHI to value 0"]
impl crate::Resettable for SdhsdeschiSpec {}
