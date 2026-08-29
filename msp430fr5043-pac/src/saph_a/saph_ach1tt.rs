#[doc = "Register `SAPH_ACH1TT` reader"]
pub type R = crate::R<SaphAch1ttSpec>;
#[doc = "Register `SAPH_ACH1TT` writer"]
pub type W = crate::W<SaphAch1ttSpec>;
#[doc = "Field `CH1TT` reader - SWG1 trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
pub type Ch1ttR = crate::FieldReader;
#[doc = "Field `CH1TT` writer - SWG1 trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
pub type Ch1ttW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - SWG1 trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
    #[inline(always)]
    pub fn ch1tt(&self) -> Ch1ttR {
        Ch1ttR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SWG1 trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
    #[inline(always)]
    pub fn ch1tt(&mut self) -> Ch1ttW<'_, SaphAch1ttSpec> {
        Ch1ttW::new(self, 0)
    }
}
#[doc = "Channel 1 Termination Trim\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_ach1tt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_ach1tt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAch1ttSpec;
impl crate::RegisterSpec for SaphAch1ttSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_ach1tt::R`](R) reader structure"]
impl crate::Readable for SaphAch1ttSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_ach1tt::W`](W) writer structure"]
impl crate::Writable for SaphAch1ttSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_ACH1TT to value 0"]
impl crate::Resettable for SaphAch1ttSpec {}
