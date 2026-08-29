#[doc = "Register `RTCAMINHR` reader"]
pub type R = crate::R<RtcaminhrSpec>;
#[doc = "Register `RTCAMINHR` writer"]
pub type W = crate::W<RtcaminhrSpec>;
#[doc = "Field `MIN` reader - Minutes (0 to 59)"]
pub type MinR = crate::FieldReader;
#[doc = "Field `MIN` writer - Minutes (0 to 59)"]
pub type MinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MINAE` reader - Alarm enable"]
pub type MinaeR = crate::BitReader;
#[doc = "Field `MINAE` writer - Alarm enable"]
pub type MinaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOUR` reader - Hours (0 to 23)"]
pub type HourR = crate::FieldReader;
#[doc = "Field `HOUR` writer - Hours (0 to 23)"]
pub type HourW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HOURAE` reader - Alarm enable"]
pub type HouraeR = crate::BitReader;
#[doc = "Field `HOURAE` writer - Alarm enable"]
pub type HouraeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn min(&self) -> MinR {
        MinR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn minae(&self) -> MinaeR {
        MinaeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Hours (0 to 23)"]
    #[inline(always)]
    pub fn hour(&self) -> HourR {
        HourR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn hourae(&self) -> HouraeR {
        HouraeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn min(&mut self) -> MinW<'_, RtcaminhrSpec> {
        MinW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn minae(&mut self) -> MinaeW<'_, RtcaminhrSpec> {
        MinaeW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Hours (0 to 23)"]
    #[inline(always)]
    pub fn hour(&mut self) -> HourW<'_, RtcaminhrSpec> {
        HourW::new(self, 8)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn hourae(&mut self) -> HouraeW<'_, RtcaminhrSpec> {
        HouraeW::new(self, 15)
    }
}
#[doc = "RTCMINHR - Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcaminhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcaminhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcaminhrSpec;
impl crate::RegisterSpec for RtcaminhrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcaminhr::R`](R) reader structure"]
impl crate::Readable for RtcaminhrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcaminhr::W`](W) writer structure"]
impl crate::Writable for RtcaminhrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCAMINHR to value 0"]
impl crate::Resettable for RtcaminhrSpec {}
