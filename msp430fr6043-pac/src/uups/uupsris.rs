#[doc = "Register `UUPSRIS` reader"]
pub type R = crate::R<UupsrisSpec>;
#[doc = "Register `UUPSRIS` writer"]
pub type W = crate::W<UupsrisSpec>;
#[doc = "UUPS Power Up Timer Out Raw Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ptmout {
    #[doc = "0: Time out during power up has not occurred"]
    Ptmout0 = 0,
    #[doc = "1: Time out during power up has occurred"]
    Ptmout1 = 1,
}
impl From<Ptmout> for bool {
    #[inline(always)]
    fn from(variant: Ptmout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTMOUT` reader - UUPS Power Up Timer Out Raw Interrupt Status bit."]
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
    #[doc = "Time out during power up has not occurred"]
    #[inline(always)]
    pub fn is_ptmout_0(&self) -> bool {
        *self == Ptmout::Ptmout0
    }
    #[doc = "Time out during power up has occurred"]
    #[inline(always)]
    pub fn is_ptmout_1(&self) -> bool {
        *self == Ptmout::Ptmout1
    }
}
#[doc = "UUPS Power Request Ignore Masked Interrupt Status and Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preqig {
    #[doc = "0: No USS_PWRREQ signal has been ignored"]
    Preqig0 = 0,
    #[doc = "1: USS_PWRREQ signal has been ignored"]
    Preqig1 = 1,
}
impl From<Preqig> for bool {
    #[inline(always)]
    fn from(variant: Preqig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREQIG` reader - UUPS Power Request Ignore Masked Interrupt Status and Clear bit."]
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
    #[doc = "No USS_PWRREQ signal has been ignored"]
    #[inline(always)]
    pub fn is_preqig_0(&self) -> bool {
        *self == Preqig::Preqig0
    }
    #[doc = "USS_PWRREQ signal has been ignored"]
    #[inline(always)]
    pub fn is_preqig_1(&self) -> bool {
        *self == Preqig::Preqig1
    }
}
#[doc = "USS has been interrupted by debug mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stpbydb {
    #[doc = "0: USS has not been interrupted by debug halt mode."]
    Stpbydb0 = 0,
    #[doc = "1: USS has been interrupted by debug halt mode."]
    Stpbydb1 = 1,
}
impl From<Stpbydb> for bool {
    #[inline(always)]
    fn from(variant: Stpbydb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPBYDB` reader - USS has been interrupted by debug mode."]
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
    #[doc = "USS has not been interrupted by debug halt mode."]
    #[inline(always)]
    pub fn is_stpbydb_0(&self) -> bool {
        *self == Stpbydb::Stpbydb0
    }
    #[doc = "USS has been interrupted by debug halt mode."]
    #[inline(always)]
    pub fn is_stpbydb_1(&self) -> bool {
        *self == Stpbydb::Stpbydb1
    }
}
impl R {
    #[doc = "Bit 0 - UUPS Power Up Timer Out Raw Interrupt Status bit."]
    #[inline(always)]
    pub fn ptmout(&self) -> PtmoutR {
        PtmoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UUPS Power Request Ignore Masked Interrupt Status and Clear bit."]
    #[inline(always)]
    pub fn preqig(&self) -> PreqigR {
        PreqigR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USS has been interrupted by debug mode."]
    #[inline(always)]
    pub fn stpbydb(&self) -> StpbydbR {
        StpbydbR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsris::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsris::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UupsrisSpec;
impl crate::RegisterSpec for UupsrisSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uupsris::R`](R) reader structure"]
impl crate::Readable for UupsrisSpec {}
#[doc = "`write(|w| ..)` method takes [`uupsris::W`](W) writer structure"]
impl crate::Writable for UupsrisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UUPSRIS to value 0"]
impl crate::Resettable for UupsrisSpec {}
