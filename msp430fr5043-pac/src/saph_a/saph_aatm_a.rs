#[doc = "Register `SAPH_AATM_A` reader"]
pub type R = crate::R<SaphAatmASpec>;
#[doc = "Register `SAPH_AATM_A` writer"]
pub type W = crate::W<SaphAatmASpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "A-SEQ start to 1st ping\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aatm_a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aatm_a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAatmASpec;
impl crate::RegisterSpec for SaphAatmASpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aatm_a::R`](R) reader structure"]
impl crate::Readable for SaphAatmASpec {}
#[doc = "`write(|w| ..)` method takes [`saph_aatm_a::W`](W) writer structure"]
impl crate::Writable for SaphAatmASpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AATM_A to value 0"]
impl crate::Resettable for SaphAatmASpec {}
