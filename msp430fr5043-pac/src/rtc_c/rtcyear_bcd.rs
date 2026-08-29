#[doc = "Register `RTCYEAR_BCD` reader"]
pub type R = crate::R<RtcyearBcdSpec>;
#[doc = "Register `RTCYEAR_BCD` writer"]
pub type W = crate::W<RtcyearBcdSpec>;
#[doc = "Field `YEAR` reader - Year lowest digit (0 to 9)"]
pub type YearR = crate::FieldReader;
#[doc = "Field `YEAR` writer - Year lowest digit (0 to 9)"]
pub type YearW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DECADE` reader - Decade (0 to 9)"]
pub type DecadeR = crate::FieldReader;
#[doc = "Field `DECADE` writer - Decade (0 to 9)"]
pub type DecadeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CENTURYLOWDIGIT` reader - Century low digit (0 to 9)"]
pub type CenturylowdigitR = crate::FieldReader;
#[doc = "Field `CENTURYLOWDIGIT` writer - Century low digit (0 to 9)"]
pub type CenturylowdigitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CENTURYHIGHDIGIT` reader - Century high digit (0 to 4)"]
pub type CenturyhighdigitR = crate::FieldReader;
#[doc = "Field `CENTURYHIGHDIGIT` writer - Century high digit (0 to 4)"]
pub type CenturyhighdigitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Year lowest digit (0 to 9)"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Decade (0 to 9)"]
    #[inline(always)]
    pub fn decade(&self) -> DecadeR {
        DecadeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Century low digit (0 to 9)"]
    #[inline(always)]
    pub fn centurylowdigit(&self) -> CenturylowdigitR {
        CenturylowdigitR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Century high digit (0 to 4)"]
    #[inline(always)]
    pub fn centuryhighdigit(&self) -> CenturyhighdigitR {
        CenturyhighdigitR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Year lowest digit (0 to 9)"]
    #[inline(always)]
    pub fn year(&mut self) -> YearW<'_, RtcyearBcdSpec> {
        YearW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Decade (0 to 9)"]
    #[inline(always)]
    pub fn decade(&mut self) -> DecadeW<'_, RtcyearBcdSpec> {
        DecadeW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Century low digit (0 to 9)"]
    #[inline(always)]
    pub fn centurylowdigit(&mut self) -> CenturylowdigitW<'_, RtcyearBcdSpec> {
        CenturylowdigitW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Century high digit (0 to 4)"]
    #[inline(always)]
    pub fn centuryhighdigit(&mut self) -> CenturyhighdigitW<'_, RtcyearBcdSpec> {
        CenturyhighdigitW::new(self, 12)
    }
}
#[doc = "Real-Time Clock Year Register - BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcyear_bcd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcyear_bcd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcyearBcdSpec;
impl crate::RegisterSpec for RtcyearBcdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcyear_bcd::R`](R) reader structure"]
impl crate::Readable for RtcyearBcdSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcyear_bcd::W`](W) writer structure"]
impl crate::Writable for RtcyearBcdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCYEAR_BCD to value 0"]
impl crate::Resettable for RtcyearBcdSpec {}
