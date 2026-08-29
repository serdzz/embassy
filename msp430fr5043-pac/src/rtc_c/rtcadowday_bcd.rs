#[doc = "Register `RTCADOWDAY_BCD` reader"]
pub type R = crate::R<RtcadowdayBcdSpec>;
#[doc = "Register `RTCADOWDAY_BCD` writer"]
pub type W = crate::W<RtcadowdayBcdSpec>;
#[doc = "Field `DOW` reader - Day of week (0 to 6)"]
pub type DowR = crate::FieldReader;
#[doc = "Field `DOW` writer - Day of week (0 to 6)"]
pub type DowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DOWAE` reader - Alarm enable"]
pub type DowaeR = crate::BitReader;
#[doc = "Field `DOWAE` writer - Alarm enable"]
pub type DowaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAY_LD` reader - Day of month low digit (0 to 9)"]
pub type DayLdR = crate::FieldReader;
#[doc = "Field `DAY_LD` writer - Day of month low digit (0 to 9)"]
pub type DayLdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAY_HD` reader - Day of month high digit (0 to 3)"]
pub type DayHdR = crate::FieldReader;
#[doc = "Field `DAY_HD` writer - Day of month high digit (0 to 3)"]
pub type DayHdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "Bits 8:11 - Day of month low digit (0 to 9)"]
    #[inline(always)]
    pub fn day_ld(&self) -> DayLdR {
        DayLdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Day of month high digit (0 to 3)"]
    #[inline(always)]
    pub fn day_hd(&self) -> DayHdR {
        DayHdR::new(((self.bits >> 12) & 3) as u8)
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
    pub fn dow(&mut self) -> DowW<'_, RtcadowdayBcdSpec> {
        DowW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn dowae(&mut self) -> DowaeW<'_, RtcadowdayBcdSpec> {
        DowaeW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Day of month low digit (0 to 9)"]
    #[inline(always)]
    pub fn day_ld(&mut self) -> DayLdW<'_, RtcadowdayBcdSpec> {
        DayLdW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Day of month high digit (0 to 3)"]
    #[inline(always)]
    pub fn day_hd(&mut self) -> DayHdW<'_, RtcadowdayBcdSpec> {
        DayHdW::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn dayae(&mut self) -> DayaeW<'_, RtcadowdayBcdSpec> {
        DayaeW::new(self, 15)
    }
}
#[doc = "Real-Time Clock Day of Week, Day of Month Alarm - BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcadowday_bcd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcadowday_bcd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcadowdayBcdSpec;
impl crate::RegisterSpec for RtcadowdayBcdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcadowday_bcd::R`](R) reader structure"]
impl crate::Readable for RtcadowdayBcdSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcadowday_bcd::W`](W) writer structure"]
impl crate::Writable for RtcadowdayBcdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCADOWDAY_BCD to value 0"]
impl crate::Resettable for RtcadowdayBcdSpec {}
