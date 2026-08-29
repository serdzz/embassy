#[doc = "Register `SDHSIMSC` reader"]
pub type R = crate::R<SdhsimscSpec>;
#[doc = "Register `SDHSIMSC` writer"]
pub type W = crate::W<SdhsimscSpec>;
#[doc = "SDHS Data Overflow Interrupt Mask bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovf {
    #[doc = "0: Interrupt is disabled"]
    Ovf0 = 0,
    #[doc = "1: Interrupt is enabled"]
    Ovf1 = 1,
}
impl From<Ovf> for bool {
    #[inline(always)]
    fn from(variant: Ovf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVF` reader - SDHS Data Overflow Interrupt Mask bit."]
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
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_ovf_0(&self) -> bool {
        *self == Ovf::Ovf0
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_ovf_1(&self) -> bool {
        *self == Ovf::Ovf1
    }
}
#[doc = "Field `OVF` writer - SDHS Data Overflow Interrupt Mask bit."]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG, Ovf>;
impl<'a, REG> OvfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn ovf_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovf::Ovf0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn ovf_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovf::Ovf1)
    }
}
#[doc = "Acquisition Done Interrupt Mask bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acqdone {
    #[doc = "0: Interrupt is disabled"]
    Acqdone0 = 0,
    #[doc = "1: Interrupt is enabled"]
    Acqdone1 = 1,
}
impl From<Acqdone> for bool {
    #[inline(always)]
    fn from(variant: Acqdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACQDONE` reader - Acquisition Done Interrupt Mask bit."]
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
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_acqdone_0(&self) -> bool {
        *self == Acqdone::Acqdone0
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_acqdone_1(&self) -> bool {
        *self == Acqdone::Acqdone1
    }
}
#[doc = "Field `ACQDONE` writer - Acquisition Done Interrupt Mask bit."]
pub type AcqdoneW<'a, REG> = crate::BitWriter<'a, REG, Acqdone>;
impl<'a, REG> AcqdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn acqdone_0(self) -> &'a mut crate::W<REG> {
        self.variant(Acqdone::Acqdone0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn acqdone_1(self) -> &'a mut crate::W<REG> {
        self.variant(Acqdone::Acqdone1)
    }
}
#[doc = "SDHS Start Conversion Trigger Interrupt Mask bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sstrg {
    #[doc = "0: Interrupt is disabled"]
    Sstrg0 = 0,
    #[doc = "1: Interrupt is enabled"]
    Sstrg1 = 1,
}
impl From<Sstrg> for bool {
    #[inline(always)]
    fn from(variant: Sstrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSTRG` reader - SDHS Start Conversion Trigger Interrupt Mask bit."]
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
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_sstrg_0(&self) -> bool {
        *self == Sstrg::Sstrg0
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_sstrg_1(&self) -> bool {
        *self == Sstrg::Sstrg1
    }
}
#[doc = "Field `SSTRG` writer - SDHS Start Conversion Trigger Interrupt Mask bit."]
pub type SstrgW<'a, REG> = crate::BitWriter<'a, REG, Sstrg>;
impl<'a, REG> SstrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn sstrg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sstrg::Sstrg0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn sstrg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sstrg::Sstrg1)
    }
}
#[doc = "SDHS Data Ready Interrupt Mask bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtrdy {
    #[doc = "0: Interrupt is disabled"]
    Dtrdy0 = 0,
    #[doc = "1: Interrupt is enabled"]
    Dtrdy1 = 1,
}
impl From<Dtrdy> for bool {
    #[inline(always)]
    fn from(variant: Dtrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTRDY` reader - SDHS Data Ready Interrupt Mask bit."]
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
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_dtrdy_0(&self) -> bool {
        *self == Dtrdy::Dtrdy0
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_dtrdy_1(&self) -> bool {
        *self == Dtrdy::Dtrdy1
    }
}
#[doc = "Field `DTRDY` writer - SDHS Data Ready Interrupt Mask bit."]
pub type DtrdyW<'a, REG> = crate::BitWriter<'a, REG, Dtrdy>;
impl<'a, REG> DtrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn dtrdy_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtrdy::Dtrdy0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn dtrdy_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtrdy::Dtrdy1)
    }
}
#[doc = "SDHS Window High Interrupt Mask bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Winhi {
    #[doc = "0: Interrupt is disabled"]
    Winhi0 = 0,
    #[doc = "1: Interrupt is enabled"]
    Winhi1 = 1,
}
impl From<Winhi> for bool {
    #[inline(always)]
    fn from(variant: Winhi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINHI` reader - SDHS Window High Interrupt Mask bit."]
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
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_winhi_0(&self) -> bool {
        *self == Winhi::Winhi0
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_winhi_1(&self) -> bool {
        *self == Winhi::Winhi1
    }
}
#[doc = "Field `WINHI` writer - SDHS Window High Interrupt Mask bit."]
pub type WinhiW<'a, REG> = crate::BitWriter<'a, REG, Winhi>;
impl<'a, REG> WinhiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn winhi_0(self) -> &'a mut crate::W<REG> {
        self.variant(Winhi::Winhi0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn winhi_1(self) -> &'a mut crate::W<REG> {
        self.variant(Winhi::Winhi1)
    }
}
#[doc = "SDHS Window Low Interrupt Mask bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Winlo {
    #[doc = "0: Interrupt is disabled"]
    Winlo0 = 0,
    #[doc = "1: Interrupt is enabled"]
    Winlo1 = 1,
}
impl From<Winlo> for bool {
    #[inline(always)]
    fn from(variant: Winlo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINLO` reader - SDHS Window Low Interrupt Mask bit."]
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
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_winlo_0(&self) -> bool {
        *self == Winlo::Winlo0
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_winlo_1(&self) -> bool {
        *self == Winlo::Winlo1
    }
}
#[doc = "Field `WINLO` writer - SDHS Window Low Interrupt Mask bit."]
pub type WinloW<'a, REG> = crate::BitWriter<'a, REG, Winlo>;
impl<'a, REG> WinloW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn winlo_0(self) -> &'a mut crate::W<REG> {
        self.variant(Winlo::Winlo0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn winlo_1(self) -> &'a mut crate::W<REG> {
        self.variant(Winlo::Winlo1)
    }
}
impl R {
    #[doc = "Bit 0 - SDHS Data Overflow Interrupt Mask bit."]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Acquisition Done Interrupt Mask bit."]
    #[inline(always)]
    pub fn acqdone(&self) -> AcqdoneR {
        AcqdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDHS Start Conversion Trigger Interrupt Mask bit."]
    #[inline(always)]
    pub fn sstrg(&self) -> SstrgR {
        SstrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDHS Data Ready Interrupt Mask bit."]
    #[inline(always)]
    pub fn dtrdy(&self) -> DtrdyR {
        DtrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SDHS Window High Interrupt Mask bit."]
    #[inline(always)]
    pub fn winhi(&self) -> WinhiR {
        WinhiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SDHS Window Low Interrupt Mask bit."]
    #[inline(always)]
    pub fn winlo(&self) -> WinloR {
        WinloR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDHS Data Overflow Interrupt Mask bit."]
    #[inline(always)]
    pub fn ovf(&mut self) -> OvfW<'_, SdhsimscSpec> {
        OvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Acquisition Done Interrupt Mask bit."]
    #[inline(always)]
    pub fn acqdone(&mut self) -> AcqdoneW<'_, SdhsimscSpec> {
        AcqdoneW::new(self, 1)
    }
    #[doc = "Bit 2 - SDHS Start Conversion Trigger Interrupt Mask bit."]
    #[inline(always)]
    pub fn sstrg(&mut self) -> SstrgW<'_, SdhsimscSpec> {
        SstrgW::new(self, 2)
    }
    #[doc = "Bit 3 - SDHS Data Ready Interrupt Mask bit."]
    #[inline(always)]
    pub fn dtrdy(&mut self) -> DtrdyW<'_, SdhsimscSpec> {
        DtrdyW::new(self, 3)
    }
    #[doc = "Bit 4 - SDHS Window High Interrupt Mask bit."]
    #[inline(always)]
    pub fn winhi(&mut self) -> WinhiW<'_, SdhsimscSpec> {
        WinhiW::new(self, 4)
    }
    #[doc = "Bit 5 - SDHS Window Low Interrupt Mask bit."]
    #[inline(always)]
    pub fn winlo(&mut self) -> WinloW<'_, SdhsimscSpec> {
        WinloW::new(self, 5)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsimsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsimsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhsimscSpec;
impl crate::RegisterSpec for SdhsimscSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsimsc::R`](R) reader structure"]
impl crate::Readable for SdhsimscSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhsimsc::W`](W) writer structure"]
impl crate::Writable for SdhsimscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSIMSC to value 0"]
impl crate::Resettable for SdhsimscSpec {}
