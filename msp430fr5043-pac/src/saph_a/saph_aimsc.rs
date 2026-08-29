#[doc = "Register `SAPH_AIMSC` reader"]
pub type R = crate::R<SaphAimscSpec>;
#[doc = "Register `SAPH_AIMSC` writer"]
pub type W = crate::W<SaphAimscSpec>;
#[doc = "Field `DATAERR` reader - This bit enables the DATAERR interrupt."]
pub type DataerrR = crate::BitReader;
#[doc = "Field `DATAERR` writer - This bit enables the DATAERR interrupt."]
pub type DataerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMFTO` reader - This bit enables the TIMEMARK F (timeout) interrupt."]
pub type TmftoR = crate::BitReader;
#[doc = "Field `TMFTO` writer - This bit enables the TIMEMARK F (timeout) interrupt."]
pub type TmftoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQDN` reader - This bit enables the SEQDN interrupt"]
pub type SeqdnR = crate::BitReader;
#[doc = "Field `SEQDN` writer - This bit enables the SEQDN interrupt"]
pub type SeqdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PNGDN` reader - This bit enables the PNGDN interrupt"]
pub type PngdnR = crate::BitReader;
#[doc = "Field `PNGDN` writer - This bit enables the PNGDN interrupt"]
pub type PngdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAV` reader - This bit indicates a DMA access violation"]
pub type DavR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit enables the DATAERR interrupt."]
    #[inline(always)]
    pub fn dataerr(&self) -> DataerrR {
        DataerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit enables the TIMEMARK F (timeout) interrupt."]
    #[inline(always)]
    pub fn tmfto(&self) -> TmftoR {
        TmftoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit enables the SEQDN interrupt"]
    #[inline(always)]
    pub fn seqdn(&self) -> SeqdnR {
        SeqdnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit enables the PNGDN interrupt"]
    #[inline(always)]
    pub fn pngdn(&self) -> PngdnR {
        PngdnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates a DMA access violation"]
    #[inline(always)]
    pub fn dav(&self) -> DavR {
        DavR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables the DATAERR interrupt."]
    #[inline(always)]
    pub fn dataerr(&mut self) -> DataerrW<'_, SaphAimscSpec> {
        DataerrW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit enables the TIMEMARK F (timeout) interrupt."]
    #[inline(always)]
    pub fn tmfto(&mut self) -> TmftoW<'_, SaphAimscSpec> {
        TmftoW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit enables the SEQDN interrupt"]
    #[inline(always)]
    pub fn seqdn(&mut self) -> SeqdnW<'_, SaphAimscSpec> {
        SeqdnW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit enables the PNGDN interrupt"]
    #[inline(always)]
    pub fn pngdn(&mut self) -> PngdnW<'_, SaphAimscSpec> {
        PngdnW::new(self, 3)
    }
}
#[doc = "Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aimsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aimsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAimscSpec;
impl crate::RegisterSpec for SaphAimscSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aimsc::R`](R) reader structure"]
impl crate::Readable for SaphAimscSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_aimsc::W`](W) writer structure"]
impl crate::Writable for SaphAimscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AIMSC to value 0"]
impl crate::Resettable for SaphAimscSpec {}
