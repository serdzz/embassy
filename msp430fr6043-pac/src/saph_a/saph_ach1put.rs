#[doc = "Register `SAPH_ACH1PUT` reader"]
pub type R = crate::R<SaphAch1putSpec>;
#[doc = "Register `SAPH_ACH1PUT` writer"]
pub type W = crate::W<SaphAch1putSpec>;
#[doc = "Field `CH1PUT` reader - DRV1 pull up trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
pub type Ch1putR = crate::FieldReader;
#[doc = "Field `CH1PUT` writer - DRV1 pull up trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
pub type Ch1putW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DRV1 pull up trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
    #[inline(always)]
    pub fn ch1put(&self) -> Ch1putR {
        Ch1putR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DRV1 pull up trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
    #[inline(always)]
    pub fn ch1put(&mut self) -> Ch1putW<'_, SaphAch1putSpec> {
        Ch1putW::new(self, 0)
    }
}
#[doc = "Channel 1 Pull UpTrim\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_ach1put::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_ach1put::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAch1putSpec;
impl crate::RegisterSpec for SaphAch1putSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_ach1put::R`](R) reader structure"]
impl crate::Readable for SaphAch1putSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_ach1put::W`](W) writer structure"]
impl crate::Writable for SaphAch1putSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_ACH1PUT to value 0"]
impl crate::Resettable for SaphAch1putSpec {}
