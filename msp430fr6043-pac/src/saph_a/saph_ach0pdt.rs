#[doc = "Register `SAPH_ACH0PDT` reader"]
pub type R = crate::R<SaphAch0pdtSpec>;
#[doc = "Register `SAPH_ACH0PDT` writer"]
pub type W = crate::W<SaphAch0pdtSpec>;
#[doc = "Field `CH0PDT` reader - DRV0 pull down trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
pub type Ch0pdtR = crate::FieldReader;
#[doc = "Field `CH0PDT` writer - DRV0 pull down trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
pub type Ch0pdtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DRV0 pull down trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
    #[inline(always)]
    pub fn ch0pdt(&self) -> Ch0pdtR {
        Ch0pdtR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DRV0 pull down trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
    #[inline(always)]
    pub fn ch0pdt(&mut self) -> Ch0pdtW<'_, SaphAch0pdtSpec> {
        Ch0pdtW::new(self, 0)
    }
}
#[doc = "Channel 0 Pull DownTrim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_ach0pdt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_ach0pdt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAch0pdtSpec;
impl crate::RegisterSpec for SaphAch0pdtSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_ach0pdt::R`](R) reader structure"]
impl crate::Readable for SaphAch0pdtSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_ach0pdt::W`](W) writer structure"]
impl crate::Writable for SaphAch0pdtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_ACH0PDT to value 0"]
impl crate::Resettable for SaphAch0pdtSpec {}
