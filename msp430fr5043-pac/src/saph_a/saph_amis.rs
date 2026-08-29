#[doc = "Register `SAPH_AMIS` reader"]
pub type R = crate::R<SaphAmisSpec>;
#[doc = "Register `SAPH_AMIS` writer"]
pub type W = crate::W<SaphAmisSpec>;
#[doc = "Field `DATAERR` reader - This interrupt indicates that either WINHI interrupt or WINLO interrupt has occurred in SDHS."]
pub type DataerrR = crate::BitReader;
#[doc = "Field `TMFTO` reader - This bit indicates a TIMEMARK F (timeout) event has happened."]
pub type TmftoR = crate::BitReader;
#[doc = "Field `SEQDN` reader - This interrupt is valid when ASQ is activcve (auto mode). The interrupt occurs when ASQ completes all of the measurements programmed in ASCTL0.PNGCNT. For example, when ASCTL0.PNGCNT = 3, total four measurements are performed. The interrupt indicates that all of the four measurements have been completed."]
pub type SeqdnR = crate::BitReader;
#[doc = "Field `PNGDN` reader - This interrupt is valid when ASQ is active (auto mode). The interrupt occurs when ASQ completes one measurement sequence. For example, when ASCTL0.PNGCNT = 3, total four measurements are performed. The interrupt indicates that one measurement has been completed."]
pub type PngdnR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This interrupt indicates that either WINHI interrupt or WINLO interrupt has occurred in SDHS."]
    #[inline(always)]
    pub fn dataerr(&self) -> DataerrR {
        DataerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit indicates a TIMEMARK F (timeout) event has happened."]
    #[inline(always)]
    pub fn tmfto(&self) -> TmftoR {
        TmftoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This interrupt is valid when ASQ is activcve (auto mode). The interrupt occurs when ASQ completes all of the measurements programmed in ASCTL0.PNGCNT. For example, when ASCTL0.PNGCNT = 3, total four measurements are performed. The interrupt indicates that all of the four measurements have been completed."]
    #[inline(always)]
    pub fn seqdn(&self) -> SeqdnR {
        SeqdnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This interrupt is valid when ASQ is active (auto mode). The interrupt occurs when ASQ completes one measurement sequence. For example, when ASCTL0.PNGCNT = 3, total four measurements are performed. The interrupt indicates that one measurement has been completed."]
    #[inline(always)]
    pub fn pngdn(&self) -> PngdnR {
        PngdnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {}
#[doc = "Masked Interrupt Satus\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_amis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_amis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAmisSpec;
impl crate::RegisterSpec for SaphAmisSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_amis::R`](R) reader structure"]
impl crate::Readable for SaphAmisSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_amis::W`](W) writer structure"]
impl crate::Writable for SaphAmisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AMIS to value 0"]
impl crate::Resettable for SaphAmisSpec {}
