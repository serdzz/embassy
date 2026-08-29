#[doc = "Register `RTCADOWDAY` reader"]
pub type R = crate::R<RtcadowdaySpec>;
#[doc = "Register `RTCADOWDAY` writer"]
pub type W = crate::W<RtcadowdaySpec>;
#[doc = "Field `DOW` reader - Day of week (0 to 6)"]
pub type DowR = crate::FieldReader;
#[doc = "Field `DOW` writer - Day of week (0 to 6)"]
pub type DowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DOWAE` reader - Alarm enable"]
pub type DowaeR = crate::BitReader;
#[doc = "Field `DOWAE` writer - Alarm enable"]
pub type DowaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAY` reader - Day of month (1 to 28, 29, 30, 31)"]
pub type DayR = crate::FieldReader;
#[doc = "Field `DAY` writer - Day of month (1 to 28, 29, 30, 31)"]
pub type DayW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DAYAE` reader - Alarm enable"]
pub type DayaeR = crate::BitReader;
#[doc = "Field `DAYAE` writer - Alarm enable"]
pub type DayaeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dow(&self) -> DowR {
        DowR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn dowae(&self) -> DowaeR {
        DowaeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Day of month (1 to 28, 29, 30, 31)"]
    #[inline(always)]
    pub fn day(&self) -> DayR {
        DayR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn dayae(&self) -> DayaeR {
        DayaeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dow(&mut self) -> DowW<'_, RtcadowdaySpec> {
        DowW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn dowae(&mut self) -> DowaeW<'_, RtcadowdaySpec> {
        DowaeW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Day of month (1 to 28, 29, 30, 31)"]
    #[inline(always)]
    pub fn day(&mut self) -> DayW<'_, RtcadowdaySpec> {
        DayW::new(self, 8)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn dayae(&mut self) -> DayaeW<'_, RtcadowdaySpec> {
        DayaeW::new(self, 15)
    }
}
#[doc = "RTCADOWDAY - Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcadowday::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcadowday::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcadowdaySpec;
impl crate::RegisterSpec for RtcadowdaySpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcadowday::R`](R) reader structure"]
impl crate::Readable for RtcadowdaySpec {}
#[doc = "`write(|w| ..)` method takes [`rtcadowday::W`](W) writer structure"]
impl crate::Writable for RtcadowdaySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCADOWDAY to value 0"]
impl crate::Resettable for RtcadowdaySpec {}
