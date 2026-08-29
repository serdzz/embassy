#[doc = "Register `RTCDATE` reader"]
pub type R = crate::R<RtcdateSpec>;
#[doc = "Register `RTCDATE` writer"]
pub type W = crate::W<RtcdateSpec>;
#[doc = "Field `DAY` reader - Day of month (1 to 28, 29, 30, 31)"]
pub type DayR = crate::FieldReader;
#[doc = "Field `DAY` writer - Day of month (1 to 28, 29, 30, 31)"]
pub type DayW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MONTH` reader - Month (1 to 12)"]
pub type MonthR = crate::FieldReader;
#[doc = "Field `MONTH` writer - Month (1 to 12)"]
pub type MonthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - Day of month (1 to 28, 29, 30, 31)"]
    #[inline(always)]
    pub fn day(&self) -> DayR {
        DayR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Month (1 to 12)"]
    #[inline(always)]
    pub fn month(&self) -> MonthR {
        MonthR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Day of month (1 to 28, 29, 30, 31)"]
    #[inline(always)]
    pub fn day(&mut self) -> DayW<'_, RtcdateSpec> {
        DayW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Month (1 to 12)"]
    #[inline(always)]
    pub fn month(&mut self) -> MonthW<'_, RtcdateSpec> {
        MonthW::new(self, 8)
    }
}
#[doc = "RTCDATE - Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcdate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcdate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcdateSpec;
impl crate::RegisterSpec for RtcdateSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcdate::R`](R) reader structure"]
impl crate::Readable for RtcdateSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcdate::W`](W) writer structure"]
impl crate::Writable for RtcdateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCDATE to value 0"]
impl crate::Resettable for RtcdateSpec {}
