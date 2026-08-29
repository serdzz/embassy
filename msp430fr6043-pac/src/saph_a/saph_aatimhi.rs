#[doc = "Register `SAPH_AATIMHI` reader"]
pub type R = crate::R<SaphAatimhiSpec>;
#[doc = "Register `SAPH_AATIMHI` writer"]
pub type W = crate::W<SaphAatimhiSpec>;
#[doc = "Field `ATIMHI` reader - ASQ Timer Counter high part. The reading this register returns the counter value \\[19:16\\]."]
pub type AtimhiR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - ASQ Timer Counter high part. The reading this register returns the counter value \\[19:16\\]."]
    #[inline(always)]
    pub fn atimhi(&self) -> AtimhiR {
        AtimhiR::new((self.bits & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Acquisition Timer High Part\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aatimhi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aatimhi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAatimhiSpec;
impl crate::RegisterSpec for SaphAatimhiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aatimhi::R`](R) reader structure"]
impl crate::Readable for SaphAatimhiSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_aatimhi::W`](W) writer structure"]
impl crate::Writable for SaphAatimhiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AATIMHI to value 0"]
impl crate::Resettable for SaphAatimhiSpec {}
