#[doc = "Register `SDHSRIS` reader"]
pub type R = crate::R<SdhsrisSpec>;
#[doc = "Register `SDHSRIS` writer"]
pub type W = crate::W<SdhsrisSpec>;
#[doc = "SDHS Data Overflow Raw Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovf {
    #[doc = "0: No OVF event"]
    Ovf0 = 0,
    #[doc = "1: When DTC is enabled (CTL2.DTCOFF = 0), DTC has dropped at least one sample. This indicates that the system clock needs to be increased. When DTC is disabled (CTL2.DTCOFF = 1), At least one new sample has been overwritten to SDHSDT register before the previous value is read."]
    Ovf1 = 1,
}
impl From<Ovf> for bool {
    #[inline(always)]
    fn from(variant: Ovf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVF` reader - SDHS Data Overflow Raw Interrupt Status bit."]
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
    #[doc = "No OVF event"]
    #[inline(always)]
    pub fn is_ovf_0(&self) -> bool {
        *self == Ovf::Ovf0
    }
    #[doc = "When DTC is enabled (CTL2.DTCOFF = 0), DTC has dropped at least one sample. This indicates that the system clock needs to be increased. When DTC is disabled (CTL2.DTCOFF = 1), At least one new sample has been overwritten to SDHSDT register before the previous value is read."]
    #[inline(always)]
    pub fn is_ovf_1(&self) -> bool {
        *self == Ovf::Ovf1
    }
}
#[doc = "Acquisition Done Raw Interrupt Status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acqdone {
    #[doc = "0: No ACQDONE event"]
    Acqdone0 = 0,
    #[doc = "1: Data conversion has been finished (either complete or incomplete)."]
    Acqdone1 = 1,
}
impl From<Acqdone> for bool {
    #[inline(always)]
    fn from(variant: Acqdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACQDONE` reader - Acquisition Done Raw Interrupt Status bit"]
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
    #[doc = "No ACQDONE event"]
    #[inline(always)]
    pub fn is_acqdone_0(&self) -> bool {
        *self == Acqdone::Acqdone0
    }
    #[doc = "Data conversion has been finished (either complete or incomplete)."]
    #[inline(always)]
    pub fn is_acqdone_1(&self) -> bool {
        *self == Acqdone::Acqdone1
    }
}
#[doc = "SDHS Start Conversion Trigger Raw Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sstrg {
    #[doc = "0: No SSTRG event"]
    Sstrg0 = 0,
    #[doc = "1: Converson Start signal has been asserted"]
    Sstrg1 = 1,
}
impl From<Sstrg> for bool {
    #[inline(always)]
    fn from(variant: Sstrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSTRG` reader - SDHS Start Conversion Trigger Raw Interrupt Status bit."]
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
    #[doc = "No SSTRG event"]
    #[inline(always)]
    pub fn is_sstrg_0(&self) -> bool {
        *self == Sstrg::Sstrg0
    }
    #[doc = "Converson Start signal has been asserted"]
    #[inline(always)]
    pub fn is_sstrg_1(&self) -> bool {
        *self == Sstrg::Sstrg1
    }
}
#[doc = "SDHS Data Ready Raw Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtrdy {
    #[doc = "0: No DTRDY event"]
    Dtrdy0 = 0,
    #[doc = "1: The data buffer has become empty."]
    Dtrdy1 = 1,
}
impl From<Dtrdy> for bool {
    #[inline(always)]
    fn from(variant: Dtrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTRDY` reader - SDHS Data Ready Raw Interrupt Status bit."]
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
    #[doc = "No DTRDY event"]
    #[inline(always)]
    pub fn is_dtrdy_0(&self) -> bool {
        *self == Dtrdy::Dtrdy0
    }
    #[doc = "The data buffer has become empty."]
    #[inline(always)]
    pub fn is_dtrdy_1(&self) -> bool {
        *self == Dtrdy::Dtrdy1
    }
}
#[doc = "SDHS Window High Raw Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Winhi {
    #[doc = "0: No WINHI event"]
    Winhi0 = 0,
    #[doc = "1: The output data value is higher than the value in the WINHITH register"]
    Winhi1 = 1,
}
impl From<Winhi> for bool {
    #[inline(always)]
    fn from(variant: Winhi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINHI` reader - SDHS Window High Raw Interrupt Status bit."]
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
    #[doc = "No WINHI event"]
    #[inline(always)]
    pub fn is_winhi_0(&self) -> bool {
        *self == Winhi::Winhi0
    }
    #[doc = "The output data value is higher than the value in the WINHITH register"]
    #[inline(always)]
    pub fn is_winhi_1(&self) -> bool {
        *self == Winhi::Winhi1
    }
}
#[doc = "SDHS Window Low Raw Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Winlo {
    #[doc = "0: No new data is lower than the value in the WINLOTH register"]
    Winlo0 = 0,
    #[doc = "1: New data is low than the value in the WINLOTH register"]
    Winlo1 = 1,
}
impl From<Winlo> for bool {
    #[inline(always)]
    fn from(variant: Winlo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINLO` reader - SDHS Window Low Raw Interrupt Status bit."]
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
    #[doc = "No new data is lower than the value in the WINLOTH register"]
    #[inline(always)]
    pub fn is_winlo_0(&self) -> bool {
        *self == Winlo::Winlo0
    }
    #[doc = "New data is low than the value in the WINLOTH register"]
    #[inline(always)]
    pub fn is_winlo_1(&self) -> bool {
        *self == Winlo::Winlo1
    }
}
#[doc = "Incomplete Stop Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Istop {
    #[doc = "0: No ISTOP event"]
    Istop0 = 0,
    #[doc = "1: Conversion has been interrupted and stopped before completing the number of samples defined in CTL2.SAMPSZ."]
    Istop1 = 1,
}
impl From<Istop> for bool {
    #[inline(always)]
    fn from(variant: Istop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISTOP` reader - Incomplete Stop Status bit."]
pub type IstopR = crate::BitReader<Istop>;
impl IstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Istop {
        match self.bits {
            false => Istop::Istop0,
            true => Istop::Istop1,
        }
    }
    #[doc = "No ISTOP event"]
    #[inline(always)]
    pub fn is_istop_0(&self) -> bool {
        *self == Istop::Istop0
    }
    #[doc = "Conversion has been interrupted and stopped before completing the number of samples defined in CTL2.SAMPSZ."]
    #[inline(always)]
    pub fn is_istop_1(&self) -> bool {
        *self == Istop::Istop1
    }
}
impl R {
    #[doc = "Bit 0 - SDHS Data Overflow Raw Interrupt Status bit."]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Acquisition Done Raw Interrupt Status bit"]
    #[inline(always)]
    pub fn acqdone(&self) -> AcqdoneR {
        AcqdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDHS Start Conversion Trigger Raw Interrupt Status bit."]
    #[inline(always)]
    pub fn sstrg(&self) -> SstrgR {
        SstrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDHS Data Ready Raw Interrupt Status bit."]
    #[inline(always)]
    pub fn dtrdy(&self) -> DtrdyR {
        DtrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SDHS Window High Raw Interrupt Status bit."]
    #[inline(always)]
    pub fn winhi(&self) -> WinhiR {
        WinhiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SDHS Window Low Raw Interrupt Status bit."]
    #[inline(always)]
    pub fn winlo(&self) -> WinloR {
        WinloR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 15 - Incomplete Stop Status bit."]
    #[inline(always)]
    pub fn istop(&self) -> IstopR {
        IstopR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsris::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsris::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhsrisSpec;
impl crate::RegisterSpec for SdhsrisSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsris::R`](R) reader structure"]
impl crate::Readable for SdhsrisSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhsris::W`](W) writer structure"]
impl crate::Writable for SdhsrisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSRIS to value 0"]
impl crate::Resettable for SdhsrisSpec {}
