#[doc = "Register `RTCTCMP` reader"]
pub type R = crate::R<RtctcmpSpec>;
#[doc = "Register `RTCTCMP` writer"]
pub type W = crate::W<RtctcmpSpec>;
#[doc = "Field `RTCTCMP` reader - Real-time clock temperature compensation"]
pub type RtctcmpR = crate::FieldReader;
#[doc = "Field `RTCTCMP` writer - Real-time clock temperature compensation"]
pub type RtctcmpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Real-time clock temperature compensation write OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtctcok {
    #[doc = "0: Write to RTCTCMPx is unsuccessful"]
    Rtctcok0 = 0,
    #[doc = "1: Write to RTCTCMPx is successful"]
    Rtctcok1 = 1,
}
impl From<Rtctcok> for bool {
    #[inline(always)]
    fn from(variant: Rtctcok) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCTCOK` reader - Real-time clock temperature compensation write OK"]
pub type RtctcokR = crate::BitReader<Rtctcok>;
impl RtctcokR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtctcok {
        match self.bits {
            false => Rtctcok::Rtctcok0,
            true => Rtctcok::Rtctcok1,
        }
    }
    #[doc = "Write to RTCTCMPx is unsuccessful"]
    #[inline(always)]
    pub fn is_rtctcok_0(&self) -> bool {
        *self == Rtctcok::Rtctcok0
    }
    #[doc = "Write to RTCTCMPx is successful"]
    #[inline(always)]
    pub fn is_rtctcok_1(&self) -> bool {
        *self == Rtctcok::Rtctcok1
    }
}
#[doc = "Field `RTCTCRDY` reader - Real-time clock temperature compensation ready"]
pub type RtctcrdyR = crate::BitReader;
#[doc = "Real-time clock temperature compensation sign\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtctcmps {
    #[doc = "0: Down calibration. Frequency adjusted down"]
    Down = 0,
    #[doc = "1: Up calibration. Frequency adjusted up"]
    Up = 1,
}
impl From<Rtctcmps> for bool {
    #[inline(always)]
    fn from(variant: Rtctcmps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCTCMPS` reader - Real-time clock temperature compensation sign"]
pub type RtctcmpsR = crate::BitReader<Rtctcmps>;
impl RtctcmpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtctcmps {
        match self.bits {
            false => Rtctcmps::Down,
            true => Rtctcmps::Up,
        }
    }
    #[doc = "Down calibration. Frequency adjusted down"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Rtctcmps::Down
    }
    #[doc = "Up calibration. Frequency adjusted up"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Rtctcmps::Up
    }
}
#[doc = "Field `RTCTCMPS` writer - Real-time clock temperature compensation sign"]
pub type RtctcmpsW<'a, REG> = crate::BitWriter<'a, REG, Rtctcmps>;
impl<'a, REG> RtctcmpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Down calibration. Frequency adjusted down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctcmps::Down)
    }
    #[doc = "Up calibration. Frequency adjusted up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctcmps::Up)
    }
}
impl R {
    #[doc = "Bits 0:7 - Real-time clock temperature compensation"]
    #[inline(always)]
    pub fn rtctcmp(&self) -> RtctcmpR {
        RtctcmpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 13 - Real-time clock temperature compensation write OK"]
    #[inline(always)]
    pub fn rtctcok(&self) -> RtctcokR {
        RtctcokR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Real-time clock temperature compensation ready"]
    #[inline(always)]
    pub fn rtctcrdy(&self) -> RtctcrdyR {
        RtctcrdyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Real-time clock temperature compensation sign"]
    #[inline(always)]
    pub fn rtctcmps(&self) -> RtctcmpsR {
        RtctcmpsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Real-time clock temperature compensation"]
    #[inline(always)]
    pub fn rtctcmp(&mut self) -> RtctcmpW<'_, RtctcmpSpec> {
        RtctcmpW::new(self, 0)
    }
    #[doc = "Bit 15 - Real-time clock temperature compensation sign"]
    #[inline(always)]
    pub fn rtctcmps(&mut self) -> RtctcmpsW<'_, RtctcmpSpec> {
        RtctcmpsW::new(self, 15)
    }
}
#[doc = "RTCTCMP Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtctcmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtctcmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtctcmpSpec;
impl crate::RegisterSpec for RtctcmpSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtctcmp::R`](R) reader structure"]
impl crate::Readable for RtctcmpSpec {}
#[doc = "`write(|w| ..)` method takes [`rtctcmp::W`](W) writer structure"]
impl crate::Writable for RtctcmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCTCMP to value 0"]
impl crate::Resettable for RtctcmpSpec {}
