#[doc = "Register `SAPH_ACH0PUT` reader"]
pub type R = crate::R<SaphAch0putSpec>;
#[doc = "Register `SAPH_ACH0PUT` writer"]
pub type W = crate::W<SaphAch0putSpec>;
#[doc = "Field `CH0PUT` reader - DRV0 pull up trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
pub type Ch0putR = crate::FieldReader;
#[doc = "Field `CH0PUT` writer - DRV0 pull up trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
pub type Ch0putW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DRV0 pull up trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
    #[inline(always)]
    pub fn ch0put(&self) -> Ch0putR {
        Ch0putR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DRV0 pull up trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
    #[inline(always)]
    pub fn ch0put(&mut self) -> Ch0putW<'_, SaphAch0putSpec> {
        Ch0putW::new(self, 0)
    }
}
#[doc = "Channel 0 Pull UpTrim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_ach0put::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_ach0put::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAch0putSpec;
impl crate::RegisterSpec for SaphAch0putSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_ach0put::R`](R) reader structure"]
impl crate::Readable for SaphAch0putSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_ach0put::W`](W) writer structure"]
impl crate::Writable for SaphAch0putSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_ACH0PUT to value 0"]
impl crate::Resettable for SaphAch0putSpec {}
