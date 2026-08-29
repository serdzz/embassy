#[doc = "Register `SAPH_AATM_E` reader"]
pub type R = crate::R<SaphAatmESpec>;
#[doc = "Register `SAPH_AATM_E` writer"]
pub type W = crate::W<SaphAatmESpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ASQ start to restart\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aatm_e::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aatm_e::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAatmESpec;
impl crate::RegisterSpec for SaphAatmESpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aatm_e::R`](R) reader structure"]
impl crate::Readable for SaphAatmESpec {}
#[doc = "`write(|w| ..)` method takes [`saph_aatm_e::W`](W) writer structure"]
impl crate::Writable for SaphAatmESpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AATM_E to value 0"]
impl crate::Resettable for SaphAatmESpec {}
