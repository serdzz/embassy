#[doc = "Register `SAPH_ADESCLO` reader"]
pub type R = crate::R<SaphAdescloSpec>;
#[doc = "Register `SAPH_ADESCLO` writer"]
pub type W = crate::W<SaphAdescloSpec>;
#[doc = "Field `MINREV` reader - Minor Revision"]
pub type MinrevR = crate::FieldReader;
#[doc = "Field `MAJREV` reader - Major Revision"]
pub type MajrevR = crate::FieldReader;
#[doc = "Field `INSTNUM` reader - Instance Number"]
pub type InstnumR = crate::FieldReader;
#[doc = "Field `FEATUREVER` reader - Feature Version"]
pub type FeatureverR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Minor Revision"]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Major Revision"]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Instance Number"]
    #[inline(always)]
    pub fn instnum(&self) -> InstnumR {
        InstnumR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Feature Version"]
    #[inline(always)]
    pub fn featurever(&self) -> FeatureverR {
        FeatureverR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Module-Descriptor Low Word\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_adesclo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_adesclo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAdescloSpec;
impl crate::RegisterSpec for SaphAdescloSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_adesclo::R`](R) reader structure"]
impl crate::Readable for SaphAdescloSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_adesclo::W`](W) writer structure"]
impl crate::Writable for SaphAdescloSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_ADESCLO to value 0"]
impl crate::Resettable for SaphAdescloSpec {}
