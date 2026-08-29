#[doc = "Register `SAPH_ACH0TT` reader"]
pub type R = crate::R<SaphAch0ttSpec>;
#[doc = "Register `SAPH_ACH0TT` writer"]
pub type W = crate::W<SaphAch0ttSpec>;
#[doc = "Field `CH0TT` reader - SWG0 trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
pub type Ch0ttR = crate::FieldReader;
#[doc = "Field `CH0TT` writer - SWG0 trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
pub type Ch0ttW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - SWG0 trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
    #[inline(always)]
    pub fn ch0tt(&self) -> Ch0ttR {
        Ch0ttR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SWG0 trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
    #[inline(always)]
    pub fn ch0tt(&mut self) -> Ch0ttW<'_, SaphAch0ttSpec> {
        Ch0ttW::new(self, 0)
    }
}
#[doc = "Channel 0 Termination Trim\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_ach0tt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_ach0tt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAch0ttSpec;
impl crate::RegisterSpec for SaphAch0ttSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_ach0tt::R`](R) reader structure"]
impl crate::Readable for SaphAch0ttSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_ach0tt::W`](W) writer structure"]
impl crate::Writable for SaphAch0ttSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_ACH0TT to value 0"]
impl crate::Resettable for SaphAch0ttSpec {}
