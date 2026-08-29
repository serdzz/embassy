#[doc = "Register `HSPLLUSSXTLCTL` reader"]
pub type R = crate::R<HspllussxtlctlSpec>;
#[doc = "Register `HSPLLUSSXTLCTL` writer"]
pub type W = crate::W<HspllussxtlctlSpec>;
#[doc = "USSXT Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ussxten {
    #[doc = "0: Disable USSXT Oscillator"]
    Ussxten0 = 0,
    #[doc = "1: Enable USSXT Oscillator"]
    Ussxten1 = 1,
}
impl From<Ussxten> for bool {
    #[inline(always)]
    fn from(variant: Ussxten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USSXTEN` reader - USSXT Enable."]
pub type UssxtenR = crate::BitReader<Ussxten>;
impl UssxtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ussxten {
        match self.bits {
            false => Ussxten::Ussxten0,
            true => Ussxten::Ussxten1,
        }
    }
    #[doc = "Disable USSXT Oscillator"]
    #[inline(always)]
    pub fn is_ussxten_0(&self) -> bool {
        *self == Ussxten::Ussxten0
    }
    #[doc = "Enable USSXT Oscillator"]
    #[inline(always)]
    pub fn is_ussxten_1(&self) -> bool {
        *self == Ussxten::Ussxten1
    }
}
#[doc = "Field `USSXTEN` writer - USSXT Enable."]
pub type UssxtenW<'a, REG> = crate::BitWriter<'a, REG, Ussxten>;
impl<'a, REG> UssxtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable USSXT Oscillator"]
    #[inline(always)]
    pub fn ussxten_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ussxten::Ussxten0)
    }
    #[doc = "Enable USSXT Oscillator"]
    #[inline(always)]
    pub fn ussxten_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ussxten::Ussxten1)
    }
}
#[doc = "Oscillator Status Bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscstate {
    #[doc = "0: Oscillator is either not enabled or in the middle of start-up transition."]
    Oscstate0 = 0,
    #[doc = "1: Oscillator has started but is not stable yet. Wait for sufficient time for stabilization."]
    Oscstate1 = 1,
}
impl From<Oscstate> for bool {
    #[inline(always)]
    fn from(variant: Oscstate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSTATE` reader - Oscillator Status Bit."]
pub type OscstateR = crate::BitReader<Oscstate>;
impl OscstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscstate {
        match self.bits {
            false => Oscstate::Oscstate0,
            true => Oscstate::Oscstate1,
        }
    }
    #[doc = "Oscillator is either not enabled or in the middle of start-up transition."]
    #[inline(always)]
    pub fn is_oscstate_0(&self) -> bool {
        *self == Oscstate::Oscstate0
    }
    #[doc = "Oscillator has started but is not stable yet. Wait for sufficient time for stabilization."]
    #[inline(always)]
    pub fn is_oscstate_1(&self) -> bool {
        *self == Oscstate::Oscstate1
    }
}
#[doc = "USSXT Buffered Output OFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xtoutoff {
    #[doc = "0: Enable USSXT buffered output"]
    Xtoutoff0 = 0,
    #[doc = "1: Disable USSXT buffered output. Default."]
    Xtoutoff1 = 1,
}
impl From<Xtoutoff> for bool {
    #[inline(always)]
    fn from(variant: Xtoutoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTOUTOFF` reader - USSXT Buffered Output OFF"]
pub type XtoutoffR = crate::BitReader<Xtoutoff>;
impl XtoutoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xtoutoff {
        match self.bits {
            false => Xtoutoff::Xtoutoff0,
            true => Xtoutoff::Xtoutoff1,
        }
    }
    #[doc = "Enable USSXT buffered output"]
    #[inline(always)]
    pub fn is_xtoutoff_0(&self) -> bool {
        *self == Xtoutoff::Xtoutoff0
    }
    #[doc = "Disable USSXT buffered output. Default."]
    #[inline(always)]
    pub fn is_xtoutoff_1(&self) -> bool {
        *self == Xtoutoff::Xtoutoff1
    }
}
#[doc = "Field `XTOUTOFF` writer - USSXT Buffered Output OFF"]
pub type XtoutoffW<'a, REG> = crate::BitWriter<'a, REG, Xtoutoff>;
impl<'a, REG> XtoutoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable USSXT buffered output"]
    #[inline(always)]
    pub fn xtoutoff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xtoutoff::Xtoutoff0)
    }
    #[doc = "Disable USSXT buffered output. Default."]
    #[inline(always)]
    pub fn xtoutoff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xtoutoff::Xtoutoff1)
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Osctype {
    #[doc = "0: Gating Counter Length: 4096. It is recommended to use this configuration for crystal resonators. Note: the counter counts the oscillator clock, so total time can be calculated as Time = 4096 x 1/Oscillator Clock Frequency."]
    Xtal = 0,
    #[doc = "1: Gating Counter Length: 512. It is recommended to use this configuration for ceramic resonators. Note: the counter counts the oscillator clock, so total time can be calculated as Time = 512x 1/Oscillator Clock Frequency."]
    Ceramic = 1,
}
impl From<Osctype> for bool {
    #[inline(always)]
    fn from(variant: Osctype) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCTYPE` reader - Reserved"]
pub type OsctypeR = crate::BitReader<Osctype>;
impl OsctypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Osctype {
        match self.bits {
            false => Osctype::Xtal,
            true => Osctype::Ceramic,
        }
    }
    #[doc = "Gating Counter Length: 4096. It is recommended to use this configuration for crystal resonators. Note: the counter counts the oscillator clock, so total time can be calculated as Time = 4096 x 1/Oscillator Clock Frequency."]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == Osctype::Xtal
    }
    #[doc = "Gating Counter Length: 512. It is recommended to use this configuration for ceramic resonators. Note: the counter counts the oscillator clock, so total time can be calculated as Time = 512x 1/Oscillator Clock Frequency."]
    #[inline(always)]
    pub fn is_ceramic(&self) -> bool {
        *self == Osctype::Ceramic
    }
}
#[doc = "Field `OSCTYPE` writer - Reserved"]
pub type OsctypeW<'a, REG> = crate::BitWriter<'a, REG, Osctype>;
impl<'a, REG> OsctypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gating Counter Length: 4096. It is recommended to use this configuration for crystal resonators. Note: the counter counts the oscillator clock, so total time can be calculated as Time = 4096 x 1/Oscillator Clock Frequency."]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(Osctype::Xtal)
    }
    #[doc = "Gating Counter Length: 512. It is recommended to use this configuration for ceramic resonators. Note: the counter counts the oscillator clock, so total time can be calculated as Time = 512x 1/Oscillator Clock Frequency."]
    #[inline(always)]
    pub fn ceramic(self) -> &'a mut crate::W<REG> {
        self.variant(Osctype::Ceramic)
    }
}
impl R {
    #[doc = "Bit 0 - USSXT Enable."]
    #[inline(always)]
    pub fn ussxten(&self) -> UssxtenR {
        UssxtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Oscillator Status Bit."]
    #[inline(always)]
    pub fn oscstate(&self) -> OscstateR {
        OscstateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - USSXT Buffered Output OFF"]
    #[inline(always)]
    pub fn xtoutoff(&self) -> XtoutoffR {
        XtoutoffR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn osctype(&self) -> OsctypeR {
        OsctypeR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USSXT Enable."]
    #[inline(always)]
    pub fn ussxten(&mut self) -> UssxtenW<'_, HspllussxtlctlSpec> {
        UssxtenW::new(self, 0)
    }
    #[doc = "Bit 8 - USSXT Buffered Output OFF"]
    #[inline(always)]
    pub fn xtoutoff(&mut self) -> XtoutoffW<'_, HspllussxtlctlSpec> {
        XtoutoffW::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn osctype(&mut self) -> OsctypeW<'_, HspllussxtlctlSpec> {
        OsctypeW::new(self, 9)
    }
}
#[doc = "USSXT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hspllussxtlctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspllussxtlctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HspllussxtlctlSpec;
impl crate::RegisterSpec for HspllussxtlctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hspllussxtlctl::R`](R) reader structure"]
impl crate::Readable for HspllussxtlctlSpec {}
#[doc = "`write(|w| ..)` method takes [`hspllussxtlctl::W`](W) writer structure"]
impl crate::Writable for HspllussxtlctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSPLLUSSXTLCTL to value 0"]
impl crate::Resettable for HspllussxtlctlSpec {}
