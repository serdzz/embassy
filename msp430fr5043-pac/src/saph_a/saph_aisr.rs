#[doc = "Register `SAPH_AISR` reader"]
pub type R = crate::R<SaphAisrSpec>;
#[doc = "Register `SAPH_AISR` writer"]
pub type W = crate::W<SaphAisrSpec>;
#[doc = "Field `DATAERR` reader - Writing one this bit generates a DATAERR interrupt by software."]
pub type DataerrR = crate::BitReader;
#[doc = "Field `DATAERR` writer - Writing one this bit generates a DATAERR interrupt by software."]
pub type DataerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMFTO` reader - Writing one this bit to generate a TIMEMARK F (timeout) interrupt by software."]
pub type TmftoR = crate::BitReader;
#[doc = "Field `TMFTO` writer - Writing one this bit to generate a TIMEMARK F (timeout) interrupt by software."]
pub type TmftoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQDN` reader - Writing one this bit to generate a SEQDN interrupt by software."]
pub type SeqdnR = crate::BitReader;
#[doc = "Field `SEQDN` writer - Writing one this bit to generate a SEQDN interrupt by software."]
pub type SeqdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PNGDN` reader - Writing one this bit to generate a PNGDN interrupt by software."]
pub type PngdnR = crate::BitReader;
#[doc = "Field `PNGDN` writer - Writing one this bit to generate a PNGDN interrupt by software."]
pub type PngdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAV` reader - Writing one this bit to set RIS.DAV by software."]
pub type DavR = crate::BitReader;
#[doc = "Field `DAV` writer - Writing one this bit to set RIS.DAV by software."]
pub type DavW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing one this bit generates a DATAERR interrupt by software."]
    #[inline(always)]
    pub fn dataerr(&self) -> DataerrR {
        DataerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Writing one this bit to generate a TIMEMARK F (timeout) interrupt by software."]
    #[inline(always)]
    pub fn tmfto(&self) -> TmftoR {
        TmftoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Writing one this bit to generate a SEQDN interrupt by software."]
    #[inline(always)]
    pub fn seqdn(&self) -> SeqdnR {
        SeqdnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Writing one this bit to generate a PNGDN interrupt by software."]
    #[inline(always)]
    pub fn pngdn(&self) -> PngdnR {
        PngdnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 14 - Writing one this bit to set RIS.DAV by software."]
    #[inline(always)]
    pub fn dav(&self) -> DavR {
        DavR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing one this bit generates a DATAERR interrupt by software."]
    #[inline(always)]
    pub fn dataerr(&mut self) -> DataerrW<'_, SaphAisrSpec> {
        DataerrW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing one this bit to generate a TIMEMARK F (timeout) interrupt by software."]
    #[inline(always)]
    pub fn tmfto(&mut self) -> TmftoW<'_, SaphAisrSpec> {
        TmftoW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing one this bit to generate a SEQDN interrupt by software."]
    #[inline(always)]
    pub fn seqdn(&mut self) -> SeqdnW<'_, SaphAisrSpec> {
        SeqdnW::new(self, 2)
    }
    #[doc = "Bit 3 - Writing one this bit to generate a PNGDN interrupt by software."]
    #[inline(always)]
    pub fn pngdn(&mut self) -> PngdnW<'_, SaphAisrSpec> {
        PngdnW::new(self, 3)
    }
    #[doc = "Bit 14 - Writing one this bit to set RIS.DAV by software."]
    #[inline(always)]
    pub fn dav(&mut self) -> DavW<'_, SaphAisrSpec> {
        DavW::new(self, 14)
    }
}
#[doc = "Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAisrSpec;
impl crate::RegisterSpec for SaphAisrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aisr::R`](R) reader structure"]
impl crate::Readable for SaphAisrSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_aisr::W`](W) writer structure"]
impl crate::Writable for SaphAisrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AISR to value 0"]
impl crate::Resettable for SaphAisrSpec {}
