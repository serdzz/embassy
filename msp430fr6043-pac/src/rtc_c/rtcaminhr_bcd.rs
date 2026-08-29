#[doc = "Register `RTCAMINHR_BCD` reader"]
pub type R = crate::R<RtcaminhrBcdSpec>;
#[doc = "Register `RTCAMINHR_BCD` writer"]
pub type W = crate::W<RtcaminhrBcdSpec>;
#[doc = "Field `MINUTESLOWDIGIT` reader - Minutes low digit (0 to 9)"]
pub type MinuteslowdigitR = crate::FieldReader;
#[doc = "Field `MINUTESLOWDIGIT` writer - Minutes low digit (0 to 9)"]
pub type MinuteslowdigitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MINUTESHIGHDIGIT` reader - Minutes high digit (0 to 5)"]
pub type MinuteshighdigitR = crate::FieldReader;
#[doc = "Field `MINUTESHIGHDIGIT` writer - Minutes high digit (0 to 5)"]
pub type MinuteshighdigitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MINAE` reader - Alarm enable"]
pub type MinaeR = crate::BitReader;
#[doc = "Field `MINAE` writer - Alarm enable"]
pub type MinaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOURSLOWDIGIT` reader - Hours low digit (0 to 9)"]
pub type HourslowdigitR = crate::FieldReader;
#[doc = "Field `HOURSLOWDIGIT` writer - Hours low digit (0 to 9)"]
pub type HourslowdigitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOURSHIGHDIGIT` reader - Hours high digit (0 to 2)"]
pub type HourshighdigitR = crate::FieldReader;
#[doc = "Field `HOURSHIGHDIGIT` writer - Hours high digit (0 to 2)"]
pub type HourshighdigitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HOURAE` reader - Alarm enable"]
pub type HouraeR = crate::BitReader;
#[doc = "Field `HOURAE` writer - Alarm enable"]
pub type HouraeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Minutes low digit (0 to 9)"]
    #[inline(always)]
    pub fn minuteslowdigit(&self) -> MinuteslowdigitR {
        MinuteslowdigitR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Minutes high digit (0 to 5)"]
    #[inline(always)]
    pub fn minuteshighdigit(&self) -> MinuteshighdigitR {
        MinuteshighdigitR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn minae(&self) -> MinaeR {
        MinaeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Hours low digit (0 to 9)"]
    #[inline(always)]
    pub fn hourslowdigit(&self) -> HourslowdigitR {
        HourslowdigitR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Hours high digit (0 to 2)"]
    #[inline(always)]
    pub fn hourshighdigit(&self) -> HourshighdigitR {
        HourshighdigitR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn hourae(&self) -> HouraeR {
        HouraeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Minutes low digit (0 to 9)"]
    #[inline(always)]
    pub fn minuteslowdigit(&mut self) -> MinuteslowdigitW<'_, RtcaminhrBcdSpec> {
        MinuteslowdigitW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Minutes high digit (0 to 5)"]
    #[inline(always)]
    pub fn minuteshighdigit(&mut self) -> MinuteshighdigitW<'_, RtcaminhrBcdSpec> {
        MinuteshighdigitW::new(self, 4)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn minae(&mut self) -> MinaeW<'_, RtcaminhrBcdSpec> {
        MinaeW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Hours low digit (0 to 9)"]
    #[inline(always)]
    pub fn hourslowdigit(&mut self) -> HourslowdigitW<'_, RtcaminhrBcdSpec> {
        HourslowdigitW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Hours high digit (0 to 2)"]
    #[inline(always)]
    pub fn hourshighdigit(&mut self) -> HourshighdigitW<'_, RtcaminhrBcdSpec> {
        HourshighdigitW::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn hourae(&mut self) -> HouraeW<'_, RtcaminhrBcdSpec> {
        HouraeW::new(self, 15)
    }
}
#[doc = "Real-Time Clock Minutes, Hour Alarm - BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcaminhr_bcd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcaminhr_bcd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcaminhrBcdSpec;
impl crate::RegisterSpec for RtcaminhrBcdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcaminhr_bcd::R`](R) reader structure"]
impl crate::Readable for RtcaminhrBcdSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcaminhr_bcd::W`](W) writer structure"]
impl crate::Writable for RtcaminhrBcdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCAMINHR_BCD to value 0"]
impl crate::Resettable for RtcaminhrBcdSpec {}
