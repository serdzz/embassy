#[doc = "Register `UUPSIMSC` reader"]
pub type R = crate::R<UupsimscSpec>;
#[doc = "Register `UUPSIMSC` writer"]
pub type W = crate::W<UupsimscSpec>;
#[doc = "UUPS Power Up Time Out Interrupt Mask bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ptmout {
    #[doc = "0: UUPS Power Up Time Out Interrupt is disabled."]
    Ptmout0 = 0,
    #[doc = "1: UUPS Power Up Time Out Interrupt is enabled."]
    Ptmout1 = 1,
}
impl From<Ptmout> for bool {
    #[inline(always)]
    fn from(variant: Ptmout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTMOUT` reader - UUPS Power Up Time Out Interrupt Mask bit."]
pub type PtmoutR = crate::BitReader<Ptmout>;
impl PtmoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ptmout {
        match self.bits {
            false => Ptmout::Ptmout0,
            true => Ptmout::Ptmout1,
        }
    }
    #[doc = "UUPS Power Up Time Out Interrupt is disabled."]
    #[inline(always)]
    pub fn is_ptmout_0(&self) -> bool {
        *self == Ptmout::Ptmout0
    }
    #[doc = "UUPS Power Up Time Out Interrupt is enabled."]
    #[inline(always)]
    pub fn is_ptmout_1(&self) -> bool {
        *self == Ptmout::Ptmout1
    }
}
#[doc = "Field `PTMOUT` writer - UUPS Power Up Time Out Interrupt Mask bit."]
pub type PtmoutW<'a, REG> = crate::BitWriter<'a, REG, Ptmout>;
impl<'a, REG> PtmoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UUPS Power Up Time Out Interrupt is disabled."]
    #[inline(always)]
    pub fn ptmout_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ptmout::Ptmout0)
    }
    #[doc = "UUPS Power Up Time Out Interrupt is enabled."]
    #[inline(always)]
    pub fn ptmout_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ptmout::Ptmout1)
    }
}
#[doc = "Power Request Ignore Interrupt Mask bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preqig {
    #[doc = "0: Power Request Ignore Interrupt is disabled."]
    Preqig0 = 0,
    #[doc = "1: Power Request Ignore Interrupt is enabled."]
    Preqig1 = 1,
}
impl From<Preqig> for bool {
    #[inline(always)]
    fn from(variant: Preqig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREQIG` reader - Power Request Ignore Interrupt Mask bit."]
pub type PreqigR = crate::BitReader<Preqig>;
impl PreqigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Preqig {
        match self.bits {
            false => Preqig::Preqig0,
            true => Preqig::Preqig1,
        }
    }
    #[doc = "Power Request Ignore Interrupt is disabled."]
    #[inline(always)]
    pub fn is_preqig_0(&self) -> bool {
        *self == Preqig::Preqig0
    }
    #[doc = "Power Request Ignore Interrupt is enabled."]
    #[inline(always)]
    pub fn is_preqig_1(&self) -> bool {
        *self == Preqig::Preqig1
    }
}
#[doc = "Field `PREQIG` writer - Power Request Ignore Interrupt Mask bit."]
pub type PreqigW<'a, REG> = crate::BitWriter<'a, REG, Preqig>;
impl<'a, REG> PreqigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power Request Ignore Interrupt is disabled."]
    #[inline(always)]
    pub fn preqig_0(self) -> &'a mut crate::W<REG> {
        self.variant(Preqig::Preqig0)
    }
    #[doc = "Power Request Ignore Interrupt is enabled."]
    #[inline(always)]
    pub fn preqig_1(self) -> &'a mut crate::W<REG> {
        self.variant(Preqig::Preqig1)
    }
}
#[doc = "USS has been interrupted by debug mode Interrupt Mask bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stpbydb {
    #[doc = "0: STPBYDB Interrupt is disabled"]
    Stpbydb0 = 0,
    #[doc = "1: STPBYDB Interrupt is enabled"]
    Stpbydb1 = 1,
}
impl From<Stpbydb> for bool {
    #[inline(always)]
    fn from(variant: Stpbydb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPBYDB` reader - USS has been interrupted by debug mode Interrupt Mask bit."]
pub type StpbydbR = crate::BitReader<Stpbydb>;
impl StpbydbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stpbydb {
        match self.bits {
            false => Stpbydb::Stpbydb0,
            true => Stpbydb::Stpbydb1,
        }
    }
    #[doc = "STPBYDB Interrupt is disabled"]
    #[inline(always)]
    pub fn is_stpbydb_0(&self) -> bool {
        *self == Stpbydb::Stpbydb0
    }
    #[doc = "STPBYDB Interrupt is enabled"]
    #[inline(always)]
    pub fn is_stpbydb_1(&self) -> bool {
        *self == Stpbydb::Stpbydb1
    }
}
#[doc = "Field `STPBYDB` writer - USS has been interrupted by debug mode Interrupt Mask bit."]
pub type StpbydbW<'a, REG> = crate::BitWriter<'a, REG, Stpbydb>;
impl<'a, REG> StpbydbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STPBYDB Interrupt is disabled"]
    #[inline(always)]
    pub fn stpbydb_0(self) -> &'a mut crate::W<REG> {
        self.variant(Stpbydb::Stpbydb0)
    }
    #[doc = "STPBYDB Interrupt is enabled"]
    #[inline(always)]
    pub fn stpbydb_1(self) -> &'a mut crate::W<REG> {
        self.variant(Stpbydb::Stpbydb1)
    }
}
impl R {
    #[doc = "Bit 0 - UUPS Power Up Time Out Interrupt Mask bit."]
    #[inline(always)]
    pub fn ptmout(&self) -> PtmoutR {
        PtmoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Request Ignore Interrupt Mask bit."]
    #[inline(always)]
    pub fn preqig(&self) -> PreqigR {
        PreqigR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USS has been interrupted by debug mode Interrupt Mask bit."]
    #[inline(always)]
    pub fn stpbydb(&self) -> StpbydbR {
        StpbydbR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UUPS Power Up Time Out Interrupt Mask bit."]
    #[inline(always)]
    pub fn ptmout(&mut self) -> PtmoutW<'_, UupsimscSpec> {
        PtmoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Power Request Ignore Interrupt Mask bit."]
    #[inline(always)]
    pub fn preqig(&mut self) -> PreqigW<'_, UupsimscSpec> {
        PreqigW::new(self, 1)
    }
    #[doc = "Bit 2 - USS has been interrupted by debug mode Interrupt Mask bit."]
    #[inline(always)]
    pub fn stpbydb(&mut self) -> StpbydbW<'_, UupsimscSpec> {
        StpbydbW::new(self, 2)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsimsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsimsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UupsimscSpec;
impl crate::RegisterSpec for UupsimscSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uupsimsc::R`](R) reader structure"]
impl crate::Readable for UupsimscSpec {}
#[doc = "`write(|w| ..)` method takes [`uupsimsc::W`](W) writer structure"]
impl crate::Writable for UupsimscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UUPSIMSC to value 0"]
impl crate::Resettable for UupsimscSpec {}
