#[doc = "Register `SDHSICR` reader"]
pub type R = crate::R<SdhsicrSpec>;
#[doc = "Register `SDHSICR` writer"]
pub type W = crate::W<SdhsicrSpec>;
#[doc = "Field `OVF` reader - SDHS Data Overflow Interrupt Clear bit."]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - SDHS Data Overflow Interrupt Clear bit."]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACQDONE` reader - Acquisition Done Interrupt Clear bit."]
pub type AcqdoneR = crate::BitReader;
#[doc = "Field `ACQDONE` writer - Acquisition Done Interrupt Clear bit."]
pub type AcqdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTRG` reader - SDHS Start Conversion Trigger Interrupt Clear bit."]
pub type SstrgR = crate::BitReader;
#[doc = "Field `SSTRG` writer - SDHS Start Conversion Trigger Interrupt Clear bit."]
pub type SstrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTRDY` reader - SDHS Data Ready Interrupt Clear bit."]
pub type DtrdyR = crate::BitReader;
#[doc = "Field `DTRDY` writer - SDHS Data Ready Interrupt Clear bit."]
pub type DtrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINHI` reader - SDHS Window High Interrupt Clear bit."]
pub type WinhiR = crate::BitReader;
#[doc = "Field `WINHI` writer - SDHS Window High Interrupt Clear bit."]
pub type WinhiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINLO` reader - SDHS Window Low Interrupt Clear bit."]
pub type WinloR = crate::BitReader;
#[doc = "Field `WINLO` writer - SDHS Window Low Interrupt Clear bit."]
pub type WinloW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISTOP` reader - Incomplete Stop Interrupt Clear bit."]
pub type IstopR = crate::BitReader;
#[doc = "Field `ISTOP` writer - Incomplete Stop Interrupt Clear bit."]
pub type IstopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SDHS Data Overflow Interrupt Clear bit."]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Acquisition Done Interrupt Clear bit."]
    #[inline(always)]
    pub fn acqdone(&self) -> AcqdoneR {
        AcqdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDHS Start Conversion Trigger Interrupt Clear bit."]
    #[inline(always)]
    pub fn sstrg(&self) -> SstrgR {
        SstrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDHS Data Ready Interrupt Clear bit."]
    #[inline(always)]
    pub fn dtrdy(&self) -> DtrdyR {
        DtrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SDHS Window High Interrupt Clear bit."]
    #[inline(always)]
    pub fn winhi(&self) -> WinhiR {
        WinhiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SDHS Window Low Interrupt Clear bit."]
    #[inline(always)]
    pub fn winlo(&self) -> WinloR {
        WinloR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 15 - Incomplete Stop Interrupt Clear bit."]
    #[inline(always)]
    pub fn istop(&self) -> IstopR {
        IstopR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDHS Data Overflow Interrupt Clear bit."]
    #[inline(always)]
    pub fn ovf(&mut self) -> OvfW<'_, SdhsicrSpec> {
        OvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Acquisition Done Interrupt Clear bit."]
    #[inline(always)]
    pub fn acqdone(&mut self) -> AcqdoneW<'_, SdhsicrSpec> {
        AcqdoneW::new(self, 1)
    }
    #[doc = "Bit 2 - SDHS Start Conversion Trigger Interrupt Clear bit."]
    #[inline(always)]
    pub fn sstrg(&mut self) -> SstrgW<'_, SdhsicrSpec> {
        SstrgW::new(self, 2)
    }
    #[doc = "Bit 3 - SDHS Data Ready Interrupt Clear bit."]
    #[inline(always)]
    pub fn dtrdy(&mut self) -> DtrdyW<'_, SdhsicrSpec> {
        DtrdyW::new(self, 3)
    }
    #[doc = "Bit 4 - SDHS Window High Interrupt Clear bit."]
    #[inline(always)]
    pub fn winhi(&mut self) -> WinhiW<'_, SdhsicrSpec> {
        WinhiW::new(self, 4)
    }
    #[doc = "Bit 5 - SDHS Window Low Interrupt Clear bit."]
    #[inline(always)]
    pub fn winlo(&mut self) -> WinloW<'_, SdhsicrSpec> {
        WinloW::new(self, 5)
    }
    #[doc = "Bit 15 - Incomplete Stop Interrupt Clear bit."]
    #[inline(always)]
    pub fn istop(&mut self) -> IstopW<'_, SdhsicrSpec> {
        IstopW::new(self, 15)
    }
}
#[doc = "Interrupt Clear Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhsicrSpec;
impl crate::RegisterSpec for SdhsicrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsicr::R`](R) reader structure"]
impl crate::Readable for SdhsicrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhsicr::W`](W) writer structure"]
impl crate::Writable for SdhsicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSICR to value 0"]
impl crate::Resettable for SdhsicrSpec {}
