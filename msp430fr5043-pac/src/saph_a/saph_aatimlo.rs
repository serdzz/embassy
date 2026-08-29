#[doc = "Register `SAPH_AATIMLO` reader"]
pub type R = crate::R<SaphAatimloSpec>;
#[doc = "Register `SAPH_AATIMLO` writer"]
pub type W = crate::W<SaphAatimloSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Acquisition Timer Low Part\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aatimlo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aatimlo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAatimloSpec;
impl crate::RegisterSpec for SaphAatimloSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aatimlo::R`](R) reader structure"]
impl crate::Readable for SaphAatimloSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_aatimlo::W`](W) writer structure"]
impl crate::Writable for SaphAatimloSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AATIMLO to value 0"]
impl crate::Resettable for SaphAatimloSpec {}
