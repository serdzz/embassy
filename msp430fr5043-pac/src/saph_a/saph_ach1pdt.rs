#[doc = "Register `SAPH_ACH1PDT` reader"]
pub type R = crate::R<SaphAch1pdtSpec>;
#[doc = "Register `SAPH_ACH1PDT` writer"]
pub type W = crate::W<SaphAch1pdtSpec>;
#[doc = "Field `CH1PDT` reader - DRV1 pull down trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
pub type Ch1pdtR = crate::FieldReader;
#[doc = "Field `CH1PDT` writer - DRV1 pull down trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
pub type Ch1pdtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DRV1 pull down trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
    #[inline(always)]
    pub fn ch1pdt(&self) -> Ch1pdtR {
        Ch1pdtR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DRV1 pull down trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation."]
    #[inline(always)]
    pub fn ch1pdt(&mut self) -> Ch1pdtW<'_, SaphAch1pdtSpec> {
        Ch1pdtW::new(self, 0)
    }
}
#[doc = "Channel 1 Pull DownTrim\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_ach1pdt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_ach1pdt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAch1pdtSpec;
impl crate::RegisterSpec for SaphAch1pdtSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_ach1pdt::R`](R) reader structure"]
impl crate::Readable for SaphAch1pdtSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_ach1pdt::W`](W) writer structure"]
impl crate::Writable for SaphAch1pdtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_ACH1PDT to value 0"]
impl crate::Resettable for SaphAch1pdtSpec {}
