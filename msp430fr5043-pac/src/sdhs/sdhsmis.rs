#[doc = "Register `SDHSMIS` reader"]
pub type R = crate::R<SdhsmisSpec>;
#[doc = "Register `SDHSMIS` writer"]
pub type W = crate::W<SdhsmisSpec>;
#[doc = "SDHS Data Overflow Masked Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovf {
    #[doc = "0: No interrupt pending"]
    Ovf0 = 0,
    #[doc = "1: Interrupt pending"]
    Ovf1 = 1,
}
impl From<Ovf> for bool {
    #[inline(always)]
    fn from(variant: Ovf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVF` reader - SDHS Data Overflow Masked Interrupt Status bit."]
pub type OvfR = crate::BitReader<Ovf>;
impl OvfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovf {
        match self.bits {
            false => Ovf::Ovf0,
            true => Ovf::Ovf1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ovf_0(&self) -> bool {
        *self == Ovf::Ovf0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ovf_1(&self) -> bool {
        *self == Ovf::Ovf1
    }
}
#[doc = "Acquisition Done Masked Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acqdone {
    #[doc = "0: No interrupt pending"]
    Acqdone0 = 0,
    #[doc = "1: Interrupt pending"]
    Acqdone1 = 1,
}
impl From<Acqdone> for bool {
    #[inline(always)]
    fn from(variant: Acqdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACQDONE` reader - Acquisition Done Masked Interrupt Status bit."]
pub type AcqdoneR = crate::BitReader<Acqdone>;
impl AcqdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acqdone {
        match self.bits {
            false => Acqdone::Acqdone0,
            true => Acqdone::Acqdone1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_acqdone_0(&self) -> bool {
        *self == Acqdone::Acqdone0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_acqdone_1(&self) -> bool {
        *self == Acqdone::Acqdone1
    }
}
#[doc = "SDHS Start Conversion Trigger Masked Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sstrg {
    #[doc = "0: No interrupt pending"]
    Sstrg0 = 0,
    #[doc = "1: Interrupt pending"]
    Sstrg1 = 1,
}
impl From<Sstrg> for bool {
    #[inline(always)]
    fn from(variant: Sstrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSTRG` reader - SDHS Start Conversion Trigger Masked Interrupt Status bit."]
pub type SstrgR = crate::BitReader<Sstrg>;
impl SstrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sstrg {
        match self.bits {
            false => Sstrg::Sstrg0,
            true => Sstrg::Sstrg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_sstrg_0(&self) -> bool {
        *self == Sstrg::Sstrg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_sstrg_1(&self) -> bool {
        *self == Sstrg::Sstrg1
    }
}
#[doc = "SDHS Data Ready Masked Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtrdy {
    #[doc = "0: No interrupt pending"]
    Dtrdy0 = 0,
    #[doc = "1: Interrupt pending"]
    Dtrdy1 = 1,
}
impl From<Dtrdy> for bool {
    #[inline(always)]
    fn from(variant: Dtrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTRDY` reader - SDHS Data Ready Masked Interrupt Status bit."]
pub type DtrdyR = crate::BitReader<Dtrdy>;
impl DtrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtrdy {
        match self.bits {
            false => Dtrdy::Dtrdy0,
            true => Dtrdy::Dtrdy1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_dtrdy_0(&self) -> bool {
        *self == Dtrdy::Dtrdy0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_dtrdy_1(&self) -> bool {
        *self == Dtrdy::Dtrdy1
    }
}
#[doc = "SDHS Window High Masked Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Winhi {
    #[doc = "0: No interrupt pending"]
    Winhi0 = 0,
    #[doc = "1: Interrupt pending"]
    Winhi1 = 1,
}
impl From<Winhi> for bool {
    #[inline(always)]
    fn from(variant: Winhi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINHI` reader - SDHS Window High Masked Interrupt Status bit."]
pub type WinhiR = crate::BitReader<Winhi>;
impl WinhiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Winhi {
        match self.bits {
            false => Winhi::Winhi0,
            true => Winhi::Winhi1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_winhi_0(&self) -> bool {
        *self == Winhi::Winhi0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_winhi_1(&self) -> bool {
        *self == Winhi::Winhi1
    }
}
#[doc = "SDHS Window Low Masked Interrupt Status and Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Winlo {
    #[doc = "0: No interrupt pending"]
    Winlo0 = 0,
    #[doc = "1: Interrupt pending"]
    Winlo1 = 1,
}
impl From<Winlo> for bool {
    #[inline(always)]
    fn from(variant: Winlo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINLO` reader - SDHS Window Low Masked Interrupt Status and Clear bit."]
pub type WinloR = crate::BitReader<Winlo>;
impl WinloR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Winlo {
        match self.bits {
            false => Winlo::Winlo0,
            true => Winlo::Winlo1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_winlo_0(&self) -> bool {
        *self == Winlo::Winlo0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_winlo_1(&self) -> bool {
        *self == Winlo::Winlo1
    }
}
impl R {
    #[doc = "Bit 0 - SDHS Data Overflow Masked Interrupt Status bit."]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Acquisition Done Masked Interrupt Status bit."]
    #[inline(always)]
    pub fn acqdone(&self) -> AcqdoneR {
        AcqdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDHS Start Conversion Trigger Masked Interrupt Status bit."]
    #[inline(always)]
    pub fn sstrg(&self) -> SstrgR {
        SstrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDHS Data Ready Masked Interrupt Status bit."]
    #[inline(always)]
    pub fn dtrdy(&self) -> DtrdyR {
        DtrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SDHS Window High Masked Interrupt Status bit."]
    #[inline(always)]
    pub fn winhi(&self) -> WinhiR {
        WinhiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SDHS Window Low Masked Interrupt Status and Clear bit."]
    #[inline(always)]
    pub fn winlo(&self) -> WinloR {
        WinloR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {}
#[doc = "Masked Interrupt Status and Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsmis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsmis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhsmisSpec;
impl crate::RegisterSpec for SdhsmisSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsmis::R`](R) reader structure"]
impl crate::Readable for SdhsmisSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhsmis::W`](W) writer structure"]
impl crate::Writable for SdhsmisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSMIS to value 0"]
impl crate::Resettable for SdhsmisSpec {}
