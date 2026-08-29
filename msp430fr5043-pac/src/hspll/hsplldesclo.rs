#[doc = "Register `HSPLLDESCLO` reader"]
pub type R = crate::R<HsplldescloSpec>;
#[doc = "Register `HSPLLDESCLO` writer"]
pub type W = crate::W<HsplldescloSpec>;
#[doc = "Field `MINREV` reader - Minor Revision"]
pub type MinrevR = crate::FieldReader;
#[doc = "Field `MAJREV` reader - Major Revision"]
pub type MajrevR = crate::FieldReader;
#[doc = "Field `INSTNUM` reader - Instance Number within the device."]
pub type InstnumR = crate::FieldReader;
#[doc = "Field `FEATUREVER` reader - Feature Set for the module"]
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
    #[doc = "Bits 8:11 - Instance Number within the device."]
    #[inline(always)]
    pub fn instnum(&self) -> InstnumR {
        InstnumR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Feature Set for the module"]
    #[inline(always)]
    pub fn featurever(&self) -> FeatureverR {
        FeatureverR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "HSPLL Descriptor Register L.\n\nYou can [`read`](crate::Reg::read) this register and get [`hsplldesclo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsplldesclo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsplldescloSpec;
impl crate::RegisterSpec for HsplldescloSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hsplldesclo::R`](R) reader structure"]
impl crate::Readable for HsplldescloSpec {}
#[doc = "`write(|w| ..)` method takes [`hsplldesclo::W`](W) writer structure"]
impl crate::Writable for HsplldescloSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSPLLDESCLO to value 0"]
impl crate::Resettable for HsplldescloSpec {}
