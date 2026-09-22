#[doc = "Register `RTCCTL` reader"]
pub type R = crate::R<RtcctlSpec>;
#[doc = "Register `RTCCTL` writer"]
pub type W = crate::W<RtcctlSpec>;
#[doc = "Field `RTCIF` reader - Low-Power-Counter Interrupt Flag"]
pub type RtcifR = crate::BitReader;
#[doc = "Field `RTCIF` writer - Low-Power-Counter Interrupt Flag"]
pub type RtcifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCIE` reader - Low-Power-Counter Interrupt Enable"]
pub type RtcieR = crate::BitReader;
#[doc = "Field `RTCIE` writer - Low-Power-Counter Interrupt Enable"]
pub type RtcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSR` reader - Low-Power-Counter Software Reset"]
pub type RtcsrR = crate::BitReader;
#[doc = "Field `RTCSR` writer - Low-Power-Counter Software Reset"]
pub type RtcsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Low-Power-Counter Clock Pre-divider Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcps {
    #[doc = "0: Low-Power-Counter Clock Pre-divider Select: 0"]
    Rtcps0 = 0,
    #[doc = "1: Low-Power-Counter Clock Pre-divider Select: 1"]
    Rtcps1 = 1,
    #[doc = "2: Low-Power-Counter Clock Pre-divider Select: 2"]
    Rtcps2 = 2,
    #[doc = "3: Low-Power-Counter Clock Pre-divider Select: 3"]
    Rtcps3 = 3,
    #[doc = "4: Low-Power-Counter Clock Pre-divider Select: 4"]
    Rtcps4 = 4,
    #[doc = "5: Low-Power-Counter Clock Pre-divider Select: 5"]
    Rtcps5 = 5,
    #[doc = "6: Low-Power-Counter Clock Pre-divider Select: 6"]
    Rtcps6 = 6,
    #[doc = "7: Low-Power-Counter Clock Pre-divider Select: 7"]
    Rtcps7 = 7,
}
impl From<Rtcps> for u8 {
    #[inline(always)]
    fn from(variant: Rtcps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcps {
    type Ux = u8;
}
impl crate::IsEnum for Rtcps {}
#[doc = "Field `RTCPS` reader - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
pub type RtcpsR = crate::FieldReader<Rtcps>;
impl RtcpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcps {
        match self.bits {
            0 => Rtcps::Rtcps0,
            1 => Rtcps::Rtcps1,
            2 => Rtcps::Rtcps2,
            3 => Rtcps::Rtcps3,
            4 => Rtcps::Rtcps4,
            5 => Rtcps::Rtcps5,
            6 => Rtcps::Rtcps6,
            7 => Rtcps::Rtcps7,
            _ => unreachable!(),
        }
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 0"]
    #[inline(always)]
    pub fn is_rtcps_0(&self) -> bool {
        *self == Rtcps::Rtcps0
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 1"]
    #[inline(always)]
    pub fn is_rtcps_1(&self) -> bool {
        *self == Rtcps::Rtcps1
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 2"]
    #[inline(always)]
    pub fn is_rtcps_2(&self) -> bool {
        *self == Rtcps::Rtcps2
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 3"]
    #[inline(always)]
    pub fn is_rtcps_3(&self) -> bool {
        *self == Rtcps::Rtcps3
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 4"]
    #[inline(always)]
    pub fn is_rtcps_4(&self) -> bool {
        *self == Rtcps::Rtcps4
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 5"]
    #[inline(always)]
    pub fn is_rtcps_5(&self) -> bool {
        *self == Rtcps::Rtcps5
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 6"]
    #[inline(always)]
    pub fn is_rtcps_6(&self) -> bool {
        *self == Rtcps::Rtcps6
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 7"]
    #[inline(always)]
    pub fn is_rtcps_7(&self) -> bool {
        *self == Rtcps::Rtcps7
    }
}
#[doc = "Field `RTCPS` writer - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
pub type RtcpsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rtcps, crate::Safe>;
impl<'a, REG> RtcpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 0"]
    #[inline(always)]
    pub fn rtcps_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::Rtcps0)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 1"]
    #[inline(always)]
    pub fn rtcps_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::Rtcps1)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 2"]
    #[inline(always)]
    pub fn rtcps_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::Rtcps2)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 3"]
    #[inline(always)]
    pub fn rtcps_3(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::Rtcps3)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 4"]
    #[inline(always)]
    pub fn rtcps_4(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::Rtcps4)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 5"]
    #[inline(always)]
    pub fn rtcps_5(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::Rtcps5)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 6"]
    #[inline(always)]
    pub fn rtcps_6(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::Rtcps6)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 7"]
    #[inline(always)]
    pub fn rtcps_7(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::Rtcps7)
    }
}
#[doc = "Low-Power-Counter Clock Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcss {
    #[doc = "0: Low-Power-Counter Clock Source Select: 0"]
    Rtcss0 = 0,
    #[doc = "1: Low-Power-Counter Clock Source Select: 1"]
    Rtcss1 = 1,
    #[doc = "2: Low-Power-Counter Clock Source Select: 2"]
    Rtcss2 = 2,
    #[doc = "3: Low-Power-Counter Clock Source Select: 3"]
    Rtcss3 = 3,
}
impl From<Rtcss> for u8 {
    #[inline(always)]
    fn from(variant: Rtcss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcss {
    type Ux = u8;
}
impl crate::IsEnum for Rtcss {}
#[doc = "Field `RTCSS` reader - Low-Power-Counter Clock Source Select Bit: 0"]
pub type RtcssR = crate::FieldReader<Rtcss>;
impl RtcssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcss {
        match self.bits {
            0 => Rtcss::Rtcss0,
            1 => Rtcss::Rtcss1,
            2 => Rtcss::Rtcss2,
            3 => Rtcss::Rtcss3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low-Power-Counter Clock Source Select: 0"]
    #[inline(always)]
    pub fn is_rtcss_0(&self) -> bool {
        *self == Rtcss::Rtcss0
    }
    #[doc = "Low-Power-Counter Clock Source Select: 1"]
    #[inline(always)]
    pub fn is_rtcss_1(&self) -> bool {
        *self == Rtcss::Rtcss1
    }
    #[doc = "Low-Power-Counter Clock Source Select: 2"]
    #[inline(always)]
    pub fn is_rtcss_2(&self) -> bool {
        *self == Rtcss::Rtcss2
    }
    #[doc = "Low-Power-Counter Clock Source Select: 3"]
    #[inline(always)]
    pub fn is_rtcss_3(&self) -> bool {
        *self == Rtcss::Rtcss3
    }
}
#[doc = "Field `RTCSS` writer - Low-Power-Counter Clock Source Select Bit: 0"]
pub type RtcssW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtcss, crate::Safe>;
impl<'a, REG> RtcssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low-Power-Counter Clock Source Select: 0"]
    #[inline(always)]
    pub fn rtcss_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcss::Rtcss0)
    }
    #[doc = "Low-Power-Counter Clock Source Select: 1"]
    #[inline(always)]
    pub fn rtcss_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcss::Rtcss1)
    }
    #[doc = "Low-Power-Counter Clock Source Select: 2"]
    #[inline(always)]
    pub fn rtcss_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcss::Rtcss2)
    }
    #[doc = "Low-Power-Counter Clock Source Select: 3"]
    #[inline(always)]
    pub fn rtcss_3(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcss::Rtcss3)
    }
}
impl R {
    #[doc = "Bit 0 - Low-Power-Counter Interrupt Flag"]
    #[inline(always)]
    pub fn rtcif(&self) -> RtcifR {
        RtcifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low-Power-Counter Interrupt Enable"]
    #[inline(always)]
    pub fn rtcie(&self) -> RtcieR {
        RtcieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Low-Power-Counter Software Reset"]
    #[inline(always)]
    pub fn rtcsr(&self) -> RtcsrR {
        RtcsrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
    #[inline(always)]
    pub fn rtcps(&self) -> RtcpsR {
        RtcpsR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Low-Power-Counter Clock Source Select Bit: 0"]
    #[inline(always)]
    pub fn rtcss(&self) -> RtcssR {
        RtcssR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low-Power-Counter Interrupt Flag"]
    #[inline(always)]
    pub fn rtcif(&mut self) -> RtcifW<'_, RtcctlSpec> {
        RtcifW::new(self, 0)
    }
    #[doc = "Bit 1 - Low-Power-Counter Interrupt Enable"]
    #[inline(always)]
    pub fn rtcie(&mut self) -> RtcieW<'_, RtcctlSpec> {
        RtcieW::new(self, 1)
    }
    #[doc = "Bit 6 - Low-Power-Counter Software Reset"]
    #[inline(always)]
    pub fn rtcsr(&mut self) -> RtcsrW<'_, RtcctlSpec> {
        RtcsrW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
    #[inline(always)]
    pub fn rtcps(&mut self) -> RtcpsW<'_, RtcctlSpec> {
        RtcpsW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Low-Power-Counter Clock Source Select Bit: 0"]
    #[inline(always)]
    pub fn rtcss(&mut self) -> RtcssW<'_, RtcctlSpec> {
        RtcssW::new(self, 12)
    }
}
#[doc = "RTC control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcctlSpec;
impl crate::RegisterSpec for RtcctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcctl::R`](R) reader structure"]
impl crate::Readable for RtcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcctl::W`](W) writer structure"]
impl crate::Writable for RtcctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCTL to value 0"]
impl crate::Resettable for RtcctlSpec {}
