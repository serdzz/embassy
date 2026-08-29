#[doc = "Register `SAPH_AATM_F` reader"]
pub type R = crate::R<SaphAatmFSpec>;
#[doc = "Register `SAPH_AATM_F` writer"]
pub type W = crate::W<SaphAatmFSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ASQ start to timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aatm_f::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aatm_f::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAatmFSpec;
impl crate::RegisterSpec for SaphAatmFSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aatm_f::R`](R) reader structure"]
impl crate::Readable for SaphAatmFSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_aatm_f::W`](W) writer structure"]
impl crate::Writable for SaphAatmFSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AATM_F to value 0"]
impl crate::Resettable for SaphAatmFSpec {}
