#[doc = "Register `CSCTL5` reader"]
pub type R = crate::R<Csctl5Spec>;
#[doc = "Register `CSCTL5` writer"]
pub type W = crate::W<Csctl5Spec>;
#[doc = "LFXT oscillator fault flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfxtoffg {
    #[doc = "0: No fault condition occurred after the last reset"]
    Lfxtoffg0 = 0,
    #[doc = "1: LFXT fault; an LFXT fault occurred after the last reset"]
    Lfxtoffg1 = 1,
}
impl From<Lfxtoffg> for bool {
    #[inline(always)]
    fn from(variant: Lfxtoffg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXTOFFG` reader - LFXT oscillator fault flag"]
pub type LfxtoffgR = crate::BitReader<Lfxtoffg>;
impl LfxtoffgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfxtoffg {
        match self.bits {
            false => Lfxtoffg::Lfxtoffg0,
            true => Lfxtoffg::Lfxtoffg1,
        }
    }
    #[doc = "No fault condition occurred after the last reset"]
    #[inline(always)]
    pub fn is_lfxtoffg_0(&self) -> bool {
        *self == Lfxtoffg::Lfxtoffg0
    }
    #[doc = "LFXT fault; an LFXT fault occurred after the last reset"]
    #[inline(always)]
    pub fn is_lfxtoffg_1(&self) -> bool {
        *self == Lfxtoffg::Lfxtoffg1
    }
}
#[doc = "Field `LFXTOFFG` writer - LFXT oscillator fault flag"]
pub type LfxtoffgW<'a, REG> = crate::BitWriter<'a, REG, Lfxtoffg>;
impl<'a, REG> LfxtoffgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No fault condition occurred after the last reset"]
    #[inline(always)]
    pub fn lfxtoffg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtoffg::Lfxtoffg0)
    }
    #[doc = "LFXT fault; an LFXT fault occurred after the last reset"]
    #[inline(always)]
    pub fn lfxtoffg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtoffg::Lfxtoffg1)
    }
}
#[doc = "HFXT oscillator fault flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfxtoffg {
    #[doc = "0: No fault condition occurred after the last reset"]
    Hfxtoffg0 = 0,
    #[doc = "1: HFXT fault; an HFXT fault occurred after the last reset"]
    Hfxtoffg1 = 1,
}
impl From<Hfxtoffg> for bool {
    #[inline(always)]
    fn from(variant: Hfxtoffg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFXTOFFG` reader - HFXT oscillator fault flag"]
pub type HfxtoffgR = crate::BitReader<Hfxtoffg>;
impl HfxtoffgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfxtoffg {
        match self.bits {
            false => Hfxtoffg::Hfxtoffg0,
            true => Hfxtoffg::Hfxtoffg1,
        }
    }
    #[doc = "No fault condition occurred after the last reset"]
    #[inline(always)]
    pub fn is_hfxtoffg_0(&self) -> bool {
        *self == Hfxtoffg::Hfxtoffg0
    }
    #[doc = "HFXT fault; an HFXT fault occurred after the last reset"]
    #[inline(always)]
    pub fn is_hfxtoffg_1(&self) -> bool {
        *self == Hfxtoffg::Hfxtoffg1
    }
}
#[doc = "Field `HFXTOFFG` writer - HFXT oscillator fault flag"]
pub type HfxtoffgW<'a, REG> = crate::BitWriter<'a, REG, Hfxtoffg>;
impl<'a, REG> HfxtoffgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No fault condition occurred after the last reset"]
    #[inline(always)]
    pub fn hfxtoffg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtoffg::Hfxtoffg0)
    }
    #[doc = "HFXT fault; an HFXT fault occurred after the last reset"]
    #[inline(always)]
    pub fn hfxtoffg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtoffg::Hfxtoffg1)
    }
}
#[doc = "Enable start counter for LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enstfcnt1 {
    #[doc = "0: Startup fault counter disabled. Counter is cleared."]
    Disable = 0,
    #[doc = "1: Startup fault counter enabled"]
    Enable = 1,
}
impl From<Enstfcnt1> for bool {
    #[inline(always)]
    fn from(variant: Enstfcnt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENSTFCNT1` reader - Enable start counter for LFXT"]
pub type Enstfcnt1R = crate::BitReader<Enstfcnt1>;
impl Enstfcnt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enstfcnt1 {
        match self.bits {
            false => Enstfcnt1::Disable,
            true => Enstfcnt1::Enable,
        }
    }
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enstfcnt1::Disable
    }
    #[doc = "Startup fault counter enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enstfcnt1::Enable
    }
}
#[doc = "Field `ENSTFCNT1` writer - Enable start counter for LFXT"]
pub type Enstfcnt1W<'a, REG> = crate::BitWriter<'a, REG, Enstfcnt1>;
impl<'a, REG> Enstfcnt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enstfcnt1::Disable)
    }
    #[doc = "Startup fault counter enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enstfcnt1::Enable)
    }
}
#[doc = "Enable start counter for HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enstfcnt2 {
    #[doc = "0: Startup fault counter disabled. Counter is cleared."]
    Disable = 0,
    #[doc = "1: Startup fault counter enabled"]
    Enable = 1,
}
impl From<Enstfcnt2> for bool {
    #[inline(always)]
    fn from(variant: Enstfcnt2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENSTFCNT2` reader - Enable start counter for HFXT"]
pub type Enstfcnt2R = crate::BitReader<Enstfcnt2>;
impl Enstfcnt2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enstfcnt2 {
        match self.bits {
            false => Enstfcnt2::Disable,
            true => Enstfcnt2::Enable,
        }
    }
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enstfcnt2::Disable
    }
    #[doc = "Startup fault counter enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enstfcnt2::Enable
    }
}
#[doc = "Field `ENSTFCNT2` writer - Enable start counter for HFXT"]
pub type Enstfcnt2W<'a, REG> = crate::BitWriter<'a, REG, Enstfcnt2>;
impl<'a, REG> Enstfcnt2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enstfcnt2::Disable)
    }
    #[doc = "Startup fault counter enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enstfcnt2::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - LFXT oscillator fault flag"]
    #[inline(always)]
    pub fn lfxtoffg(&self) -> LfxtoffgR {
        LfxtoffgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXT oscillator fault flag"]
    #[inline(always)]
    pub fn hfxtoffg(&self) -> HfxtoffgR {
        HfxtoffgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable start counter for LFXT"]
    #[inline(always)]
    pub fn enstfcnt1(&self) -> Enstfcnt1R {
        Enstfcnt1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable start counter for HFXT"]
    #[inline(always)]
    pub fn enstfcnt2(&self) -> Enstfcnt2R {
        Enstfcnt2R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LFXT oscillator fault flag"]
    #[inline(always)]
    pub fn lfxtoffg(&mut self) -> LfxtoffgW<'_, Csctl5Spec> {
        LfxtoffgW::new(self, 0)
    }
    #[doc = "Bit 1 - HFXT oscillator fault flag"]
    #[inline(always)]
    pub fn hfxtoffg(&mut self) -> HfxtoffgW<'_, Csctl5Spec> {
        HfxtoffgW::new(self, 1)
    }
    #[doc = "Bit 6 - Enable start counter for LFXT"]
    #[inline(always)]
    pub fn enstfcnt1(&mut self) -> Enstfcnt1W<'_, Csctl5Spec> {
        Enstfcnt1W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable start counter for HFXT"]
    #[inline(always)]
    pub fn enstfcnt2(&mut self) -> Enstfcnt2W<'_, Csctl5Spec> {
        Enstfcnt2W::new(self, 7)
    }
}
#[doc = "Clock System Control 5\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl5Spec;
impl crate::RegisterSpec for Csctl5Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl5::R`](R) reader structure"]
impl crate::Readable for Csctl5Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl5::W`](W) writer structure"]
impl crate::Writable for Csctl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL5 to value 0"]
impl crate::Resettable for Csctl5Spec {}
