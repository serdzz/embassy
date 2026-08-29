#[doc = "Register `RTCCTL13` reader"]
pub type R = crate::R<Rtcctl13Spec>;
#[doc = "Register `RTCCTL13` writer"]
pub type W = crate::W<Rtcctl13Spec>;
#[doc = "Real-time clock time event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtctev {
    #[doc = "0: Minute changed"]
    Min = 0,
    #[doc = "1: Hour changed"]
    Hour = 1,
    #[doc = "2: Every day at midnight (00:00)"]
    _0000 = 2,
    #[doc = "3: Every day at noon (12:00)"]
    _1200 = 3,
}
impl From<Rtctev> for u8 {
    #[inline(always)]
    fn from(variant: Rtctev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtctev {
    type Ux = u8;
}
impl crate::IsEnum for Rtctev {}
#[doc = "Field `RTCTEV` reader - Real-time clock time event"]
pub type RtctevR = crate::FieldReader<Rtctev>;
impl RtctevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtctev {
        match self.bits {
            0 => Rtctev::Min,
            1 => Rtctev::Hour,
            2 => Rtctev::_0000,
            3 => Rtctev::_1200,
            _ => unreachable!(),
        }
    }
    #[doc = "Minute changed"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Rtctev::Min
    }
    #[doc = "Hour changed"]
    #[inline(always)]
    pub fn is_hour(&self) -> bool {
        *self == Rtctev::Hour
    }
    #[doc = "Every day at midnight (00:00)"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Rtctev::_0000
    }
    #[doc = "Every day at noon (12:00)"]
    #[inline(always)]
    pub fn is_1200(&self) -> bool {
        *self == Rtctev::_1200
    }
}
#[doc = "Field `RTCTEV` writer - Real-time clock time event"]
pub type RtctevW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtctev, crate::Safe>;
impl<'a, REG> RtctevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minute changed"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctev::Min)
    }
    #[doc = "Hour changed"]
    #[inline(always)]
    pub fn hour(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctev::Hour)
    }
    #[doc = "Every day at midnight (00:00)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctev::_0000)
    }
    #[doc = "Every day at noon (12:00)"]
    #[inline(always)]
    pub fn _1200(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctev::_1200)
    }
}
#[doc = "Real-time clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcssel {
    #[doc = "0: 32-kHz crystal oscillator clock"]
    Lfxt = 0,
    #[doc = "1: 32-kHz crystal oscillator clock"]
    Lfxt1 = 1,
    #[doc = "2: Output from RT1PS"]
    Rt1ps = 2,
    #[doc = "3: Output from RT1PS"]
    Rt1ps3 = 3,
}
impl From<Rtcssel> for u8 {
    #[inline(always)]
    fn from(variant: Rtcssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcssel {
    type Ux = u8;
}
impl crate::IsEnum for Rtcssel {}
#[doc = "Field `RTCSSEL` reader - Real-time clock source select"]
pub type RtcsselR = crate::FieldReader<Rtcssel>;
impl RtcsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcssel {
        match self.bits {
            0 => Rtcssel::Lfxt,
            1 => Rtcssel::Lfxt1,
            2 => Rtcssel::Rt1ps,
            3 => Rtcssel::Rt1ps3,
            _ => unreachable!(),
        }
    }
    #[doc = "32-kHz crystal oscillator clock"]
    #[inline(always)]
    pub fn is_lfxt(&self) -> bool {
        *self == Rtcssel::Lfxt
    }
    #[doc = "32-kHz crystal oscillator clock"]
    #[inline(always)]
    pub fn is_lfxt_1(&self) -> bool {
        *self == Rtcssel::Lfxt1
    }
    #[doc = "Output from RT1PS"]
    #[inline(always)]
    pub fn is_rt1ps(&self) -> bool {
        *self == Rtcssel::Rt1ps
    }
    #[doc = "Output from RT1PS"]
    #[inline(always)]
    pub fn is_rt1ps_3(&self) -> bool {
        *self == Rtcssel::Rt1ps3
    }
}
#[doc = "Field `RTCSSEL` writer - Real-time clock source select"]
pub type RtcsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtcssel, crate::Safe>;
impl<'a, REG> RtcsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32-kHz crystal oscillator clock"]
    #[inline(always)]
    pub fn lfxt(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcssel::Lfxt)
    }
    #[doc = "32-kHz crystal oscillator clock"]
    #[inline(always)]
    pub fn lfxt_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcssel::Lfxt1)
    }
    #[doc = "Output from RT1PS"]
    #[inline(always)]
    pub fn rt1ps(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcssel::Rt1ps)
    }
    #[doc = "Output from RT1PS"]
    #[inline(always)]
    pub fn rt1ps_3(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcssel::Rt1ps3)
    }
}
#[doc = "Real-time clock ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcrdy {
    #[doc = "0: RTC time values in transition"]
    Rtcrdy0 = 0,
    #[doc = "1: RTC time values safe for reading. This bit indicates when the real-time clock time values are safe for reading."]
    Rtcrdy1 = 1,
}
impl From<Rtcrdy> for bool {
    #[inline(always)]
    fn from(variant: Rtcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCRDY` reader - Real-time clock ready"]
pub type RtcrdyR = crate::BitReader<Rtcrdy>;
impl RtcrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcrdy {
        match self.bits {
            false => Rtcrdy::Rtcrdy0,
            true => Rtcrdy::Rtcrdy1,
        }
    }
    #[doc = "RTC time values in transition"]
    #[inline(always)]
    pub fn is_rtcrdy_0(&self) -> bool {
        *self == Rtcrdy::Rtcrdy0
    }
    #[doc = "RTC time values safe for reading. This bit indicates when the real-time clock time values are safe for reading."]
    #[inline(always)]
    pub fn is_rtcrdy_1(&self) -> bool {
        *self == Rtcrdy::Rtcrdy1
    }
}
#[doc = "RTCMODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcmode {
    #[doc = "1: Calendar mode. Always reads a value of 1."]
    Rtcmode1 = 1,
}
impl From<Rtcmode> for bool {
    #[inline(always)]
    fn from(variant: Rtcmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCMODE` reader - RTCMODE"]
pub type RtcmodeR = crate::BitReader<Rtcmode>;
impl RtcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rtcmode> {
        match self.bits {
            true => Some(Rtcmode::Rtcmode1),
            _ => None,
        }
    }
    #[doc = "Calendar mode. Always reads a value of 1."]
    #[inline(always)]
    pub fn is_rtcmode_1(&self) -> bool {
        *self == Rtcmode::Rtcmode1
    }
}
#[doc = "Real-time clock hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtchold {
    #[doc = "0: Real-time clock is operational"]
    Rtchold0 = 0,
    #[doc = "1: When set, the calendar is stopped as well as the prescale counters, RT0PS and RT1PS are don't care"]
    Rtchold1 = 1,
}
impl From<Rtchold> for bool {
    #[inline(always)]
    fn from(variant: Rtchold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCHOLD` reader - Real-time clock hold"]
pub type RtcholdR = crate::BitReader<Rtchold>;
impl RtcholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtchold {
        match self.bits {
            false => Rtchold::Rtchold0,
            true => Rtchold::Rtchold1,
        }
    }
    #[doc = "Real-time clock is operational"]
    #[inline(always)]
    pub fn is_rtchold_0(&self) -> bool {
        *self == Rtchold::Rtchold0
    }
    #[doc = "When set, the calendar is stopped as well as the prescale counters, RT0PS and RT1PS are don't care"]
    #[inline(always)]
    pub fn is_rtchold_1(&self) -> bool {
        *self == Rtchold::Rtchold1
    }
}
#[doc = "Field `RTCHOLD` writer - Real-time clock hold"]
pub type RtcholdW<'a, REG> = crate::BitWriter<'a, REG, Rtchold>;
impl<'a, REG> RtcholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Real-time clock is operational"]
    #[inline(always)]
    pub fn rtchold_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtchold::Rtchold0)
    }
    #[doc = "When set, the calendar is stopped as well as the prescale counters, RT0PS and RT1PS are don't care"]
    #[inline(always)]
    pub fn rtchold_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtchold::Rtchold1)
    }
}
#[doc = "Real-time clock BCD select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcbcd {
    #[doc = "0: Binary (hexadecimal) code selected"]
    Hex = 0,
    #[doc = "1: Binary coded decimal (BCD) code selected"]
    Bcd = 1,
}
impl From<Rtcbcd> for bool {
    #[inline(always)]
    fn from(variant: Rtcbcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCBCD` reader - Real-time clock BCD select"]
pub type RtcbcdR = crate::BitReader<Rtcbcd>;
impl RtcbcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcbcd {
        match self.bits {
            false => Rtcbcd::Hex,
            true => Rtcbcd::Bcd,
        }
    }
    #[doc = "Binary (hexadecimal) code selected"]
    #[inline(always)]
    pub fn is_hex(&self) -> bool {
        *self == Rtcbcd::Hex
    }
    #[doc = "Binary coded decimal (BCD) code selected"]
    #[inline(always)]
    pub fn is_bcd(&self) -> bool {
        *self == Rtcbcd::Bcd
    }
}
#[doc = "Field `RTCBCD` writer - Real-time clock BCD select"]
pub type RtcbcdW<'a, REG> = crate::BitWriter<'a, REG, Rtcbcd>;
impl<'a, REG> RtcbcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Binary (hexadecimal) code selected"]
    #[inline(always)]
    pub fn hex(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcbcd::Hex)
    }
    #[doc = "Binary coded decimal (BCD) code selected"]
    #[inline(always)]
    pub fn bcd(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcbcd::Bcd)
    }
}
#[doc = "Real-time clock calibration frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtccalf {
    #[doc = "0: No frequency output to RTCCLK pin"]
    None = 0,
    #[doc = "1: 512 Hz"]
    _512 = 1,
    #[doc = "2: 256 Hz"]
    _256 = 2,
    #[doc = "3: 1 Hz"]
    _1 = 3,
}
impl From<Rtccalf> for u8 {
    #[inline(always)]
    fn from(variant: Rtccalf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtccalf {
    type Ux = u8;
}
impl crate::IsEnum for Rtccalf {}
#[doc = "Field `RTCCALF` reader - Real-time clock calibration frequency"]
pub type RtccalfR = crate::FieldReader<Rtccalf>;
impl RtccalfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtccalf {
        match self.bits {
            0 => Rtccalf::None,
            1 => Rtccalf::_512,
            2 => Rtccalf::_256,
            3 => Rtccalf::_1,
            _ => unreachable!(),
        }
    }
    #[doc = "No frequency output to RTCCLK pin"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Rtccalf::None
    }
    #[doc = "512 Hz"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == Rtccalf::_512
    }
    #[doc = "256 Hz"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Rtccalf::_256
    }
    #[doc = "1 Hz"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtccalf::_1
    }
}
#[doc = "Field `RTCCALF` writer - Real-time clock calibration frequency"]
pub type RtccalfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtccalf, crate::Safe>;
impl<'a, REG> RtccalfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No frequency output to RTCCLK pin"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccalf::None)
    }
    #[doc = "512 Hz"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccalf::_512)
    }
    #[doc = "256 Hz"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccalf::_256)
    }
    #[doc = "1 Hz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccalf::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Real-time clock time event"]
    #[inline(always)]
    pub fn rtctev(&self) -> RtctevR {
        RtctevR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Real-time clock source select"]
    #[inline(always)]
    pub fn rtcssel(&self) -> RtcsselR {
        RtcsselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Real-time clock ready"]
    #[inline(always)]
    pub fn rtcrdy(&self) -> RtcrdyR {
        RtcrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTCMODE"]
    #[inline(always)]
    pub fn rtcmode(&self) -> RtcmodeR {
        RtcmodeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real-time clock hold"]
    #[inline(always)]
    pub fn rtchold(&self) -> RtcholdR {
        RtcholdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Real-time clock BCD select"]
    #[inline(always)]
    pub fn rtcbcd(&self) -> RtcbcdR {
        RtcbcdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Real-time clock calibration frequency"]
    #[inline(always)]
    pub fn rtccalf(&self) -> RtccalfR {
        RtccalfR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Real-time clock time event"]
    #[inline(always)]
    pub fn rtctev(&mut self) -> RtctevW<'_, Rtcctl13Spec> {
        RtctevW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Real-time clock source select"]
    #[inline(always)]
    pub fn rtcssel(&mut self) -> RtcsselW<'_, Rtcctl13Spec> {
        RtcsselW::new(self, 2)
    }
    #[doc = "Bit 6 - Real-time clock hold"]
    #[inline(always)]
    pub fn rtchold(&mut self) -> RtcholdW<'_, Rtcctl13Spec> {
        RtcholdW::new(self, 6)
    }
    #[doc = "Bit 7 - Real-time clock BCD select"]
    #[inline(always)]
    pub fn rtcbcd(&mut self) -> RtcbcdW<'_, Rtcctl13Spec> {
        RtcbcdW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Real-time clock calibration frequency"]
    #[inline(always)]
    pub fn rtccalf(&mut self) -> RtccalfW<'_, Rtcctl13Spec> {
        RtccalfW::new(self, 8)
    }
}
#[doc = "RTCCTL13 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctl13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctl13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtcctl13Spec;
impl crate::RegisterSpec for Rtcctl13Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcctl13::R`](R) reader structure"]
impl crate::Readable for Rtcctl13Spec {}
#[doc = "`write(|w| ..)` method takes [`rtcctl13::W`](W) writer structure"]
impl crate::Writable for Rtcctl13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCTL13 to value 0"]
impl crate::Resettable for Rtcctl13Spec {}
