#[doc = "Register `SDHSCTL2` reader"]
pub type R = crate::R<Sdhsctl2Spec>;
#[doc = "Register `SDHSCTL2` writer"]
pub type W = crate::W<Sdhsctl2Spec>;
#[doc = "Field `SMPSZ` reader - Total Sample Size."]
pub type SmpszR = crate::FieldReader<u16>;
#[doc = "Field `SMPSZ` writer - Total Sample Size."]
pub type SmpszW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Disable sampling size counting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smpctloff {
    #[doc = "0: Total sampling size is determined by SMPSZ bits. The SDHS automatically stops data conversion."]
    Smpctloff0 = 0,
    #[doc = "1: SMPSZ bits are ignored. Conversion does not stop until the trigger source selected by TRGSRC bits is deasserted."]
    Smpctloff1 = 1,
}
impl From<Smpctloff> for bool {
    #[inline(always)]
    fn from(variant: Smpctloff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPCTLOFF` reader - Disable sampling size counting."]
pub type SmpctloffR = crate::BitReader<Smpctloff>;
impl SmpctloffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smpctloff {
        match self.bits {
            false => Smpctloff::Smpctloff0,
            true => Smpctloff::Smpctloff1,
        }
    }
    #[doc = "Total sampling size is determined by SMPSZ bits. The SDHS automatically stops data conversion."]
    #[inline(always)]
    pub fn is_smpctloff_0(&self) -> bool {
        *self == Smpctloff::Smpctloff0
    }
    #[doc = "SMPSZ bits are ignored. Conversion does not stop until the trigger source selected by TRGSRC bits is deasserted."]
    #[inline(always)]
    pub fn is_smpctloff_1(&self) -> bool {
        *self == Smpctloff::Smpctloff1
    }
}
#[doc = "Field `SMPCTLOFF` writer - Disable sampling size counting."]
pub type SmpctloffW<'a, REG> = crate::BitWriter<'a, REG, Smpctloff>;
impl<'a, REG> SmpctloffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Total sampling size is determined by SMPSZ bits. The SDHS automatically stops data conversion."]
    #[inline(always)]
    pub fn smpctloff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Smpctloff::Smpctloff0)
    }
    #[doc = "SMPSZ bits are ignored. Conversion does not stop until the trigger source selected by TRGSRC bits is deasserted."]
    #[inline(always)]
    pub fn smpctloff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Smpctloff::Smpctloff1)
    }
}
#[doc = "Window Comparator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wincmpen {
    #[doc = "0: Window Comparator is disabled"]
    Wincmpen0 = 0,
    #[doc = "1: Window Comparator is enabled"]
    Wincmpen1 = 1,
}
impl From<Wincmpen> for bool {
    #[inline(always)]
    fn from(variant: Wincmpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINCMPEN` reader - Window Comparator Enable"]
pub type WincmpenR = crate::BitReader<Wincmpen>;
impl WincmpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wincmpen {
        match self.bits {
            false => Wincmpen::Wincmpen0,
            true => Wincmpen::Wincmpen1,
        }
    }
    #[doc = "Window Comparator is disabled"]
    #[inline(always)]
    pub fn is_wincmpen_0(&self) -> bool {
        *self == Wincmpen::Wincmpen0
    }
    #[doc = "Window Comparator is enabled"]
    #[inline(always)]
    pub fn is_wincmpen_1(&self) -> bool {
        *self == Wincmpen::Wincmpen1
    }
}
#[doc = "Field `WINCMPEN` writer - Window Comparator Enable"]
pub type WincmpenW<'a, REG> = crate::BitWriter<'a, REG, Wincmpen>;
impl<'a, REG> WincmpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Window Comparator is disabled"]
    #[inline(always)]
    pub fn wincmpen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wincmpen::Wincmpen0)
    }
    #[doc = "Window Comparator is enabled"]
    #[inline(always)]
    pub fn wincmpen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wincmpen::Wincmpen1)
    }
}
#[doc = "Data Transfer Controller (DTC) Off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtcoff {
    #[doc = "0: DTC enabled. The DTC automatically transfers the data from the SDHSDT register to the address specified in the DTCDA register."]
    Dtcoff0 = 0,
    #[doc = "1: DTC disabled. The data in the SDHSDT register must be read by CPU, otherwise the overflow interrupt flag (RIS.OVF) will eventually be asserted."]
    Dtcoff1 = 1,
}
impl From<Dtcoff> for bool {
    #[inline(always)]
    fn from(variant: Dtcoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTCOFF` reader - Data Transfer Controller (DTC) Off"]
pub type DtcoffR = crate::BitReader<Dtcoff>;
impl DtcoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtcoff {
        match self.bits {
            false => Dtcoff::Dtcoff0,
            true => Dtcoff::Dtcoff1,
        }
    }
    #[doc = "DTC enabled. The DTC automatically transfers the data from the SDHSDT register to the address specified in the DTCDA register."]
    #[inline(always)]
    pub fn is_dtcoff_0(&self) -> bool {
        *self == Dtcoff::Dtcoff0
    }
    #[doc = "DTC disabled. The data in the SDHSDT register must be read by CPU, otherwise the overflow interrupt flag (RIS.OVF) will eventually be asserted."]
    #[inline(always)]
    pub fn is_dtcoff_1(&self) -> bool {
        *self == Dtcoff::Dtcoff1
    }
}
#[doc = "Field `DTCOFF` writer - Data Transfer Controller (DTC) Off"]
pub type DtcoffW<'a, REG> = crate::BitWriter<'a, REG, Dtcoff>;
impl<'a, REG> DtcoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTC enabled. The DTC automatically transfers the data from the SDHSDT register to the address specified in the DTCDA register."]
    #[inline(always)]
    pub fn dtcoff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcoff::Dtcoff0)
    }
    #[doc = "DTC disabled. The data in the SDHSDT register must be read by CPU, otherwise the overflow interrupt flag (RIS.OVF) will eventually be asserted."]
    #[inline(always)]
    pub fn dtcoff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcoff::Dtcoff1)
    }
}
impl R {
    #[doc = "Bits 0:9 - Total Sample Size."]
    #[inline(always)]
    pub fn smpsz(&self) -> SmpszR {
        SmpszR::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 10 - Disable sampling size counting."]
    #[inline(always)]
    pub fn smpctloff(&self) -> SmpctloffR {
        SmpctloffR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - Window Comparator Enable"]
    #[inline(always)]
    pub fn wincmpen(&self) -> WincmpenR {
        WincmpenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Transfer Controller (DTC) Off"]
    #[inline(always)]
    pub fn dtcoff(&self) -> DtcoffR {
        DtcoffR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Total Sample Size."]
    #[inline(always)]
    pub fn smpsz(&mut self) -> SmpszW<'_, Sdhsctl2Spec> {
        SmpszW::new(self, 0)
    }
    #[doc = "Bit 10 - Disable sampling size counting."]
    #[inline(always)]
    pub fn smpctloff(&mut self) -> SmpctloffW<'_, Sdhsctl2Spec> {
        SmpctloffW::new(self, 10)
    }
    #[doc = "Bit 14 - Window Comparator Enable"]
    #[inline(always)]
    pub fn wincmpen(&mut self) -> WincmpenW<'_, Sdhsctl2Spec> {
        WincmpenW::new(self, 14)
    }
    #[doc = "Bit 15 - Data Transfer Controller (DTC) Off"]
    #[inline(always)]
    pub fn dtcoff(&mut self) -> DtcoffW<'_, Sdhsctl2Spec> {
        DtcoffW::new(self, 15)
    }
}
#[doc = "SDHS Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdhsctl2Spec;
impl crate::RegisterSpec for Sdhsctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsctl2::R`](R) reader structure"]
impl crate::Readable for Sdhsctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`sdhsctl2::W`](W) writer structure"]
impl crate::Writable for Sdhsctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSCTL2 to value 0"]
impl crate::Resettable for Sdhsctl2Spec {}
