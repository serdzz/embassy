#[doc = "Register `SAPH_ATBCTL` reader"]
pub type R = crate::R<SaphAtbctlSpec>;
#[doc = "Register `SAPH_ATBCTL` writer"]
pub type W = crate::W<SaphAtbctlSpec>;
#[doc = "Field `TCLR` reader - The ASQ time counter clear. Writing '1' to this bit clears the the counter value. The counter must be stopped prior to be cleared. This bit is self cleared. TSTOP, TSTART, and TCLR bits are offerred for only debugging purpose. It is not recommend to use this bit while ASQ is active. Note: This bit is write only. Reading always returns with zero."]
pub type TclrR = crate::BitReader;
#[doc = "Field `TCLR` writer - The ASQ time counter clear. Writing '1' to this bit clears the the counter value. The counter must be stopped prior to be cleared. This bit is self cleared. TSTOP, TSTART, and TCLR bits are offerred for only debugging purpose. It is not recommend to use this bit while ASQ is active. Note: This bit is write only. Reading always returns with zero."]
pub type TclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTART` reader - The ASQ time counter start. Writing '1' to this bit starts the counter. This bit is self cleared. TSTOP, TSTART, and TCLR bits are offerred for only debugging purpose. It is not recommend to use this bit while ASQ is active. Note: This bit is write only. Reading always returns with zero."]
pub type TstartR = crate::BitReader;
#[doc = "Field `TSTART` writer - The ASQ time counter start. Writing '1' to this bit starts the counter. This bit is self cleared. TSTOP, TSTART, and TCLR bits are offerred for only debugging purpose. It is not recommend to use this bit while ASQ is active. Note: This bit is write only. Reading always returns with zero."]
pub type TstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTOP` reader - The ASQ time counter stop. Writing '1' to this bit stops the counter. This bit is self cleared. TSTOP, TSTART, and TCLR bits are offerred for only debugging purpose. It is not recommend to use this bit while ASQ is active. Note: This bit is write only. Reading always returns with zero."]
pub type TstopR = crate::BitReader;
#[doc = "Field `TSTOP` writer - The ASQ time counter stop. Writing '1' to this bit stops the counter. This bit is self cleared. TSTOP, TSTART, and TCLR bits are offerred for only debugging purpose. It is not recommend to use this bit while ASQ is active. Note: This bit is write only. Reading always returns with zero."]
pub type TstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSSV` reader - ASQ pre-scaler shift. The value written to the PSSV bits shifts the start point of the ASQ's pre-scaler. Note that the value only affects the first cycle of the pre-scaler. 0 = No shift 1 = The pre-scaler starts 1 clock later 2 = The pre-scaler starts 2 clocks later ... 15 = The pre-scaler starts 15 clocks later"]
pub type PssvR = crate::FieldReader;
#[doc = "Field `PSSV` writer - ASQ pre-scaler shift. The value written to the PSSV bits shifts the start point of the ASQ's pre-scaler. Note that the value only affects the first cycle of the pre-scaler. 0 = No shift 1 = The pre-scaler starts 1 clock later 2 = The pre-scaler starts 2 clocks later ... 15 = The pre-scaler starts 15 clocks later"]
pub type PssvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - The ASQ time counter clear. Writing '1' to this bit clears the the counter value. The counter must be stopped prior to be cleared. This bit is self cleared. TSTOP, TSTART, and TCLR bits are offerred for only debugging purpose. It is not recommend to use this bit while ASQ is active. Note: This bit is write only. Reading always returns with zero."]
    #[inline(always)]
    pub fn tclr(&self) -> TclrR {
        TclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The ASQ time counter start. Writing '1' to this bit starts the counter. This bit is self cleared. TSTOP, TSTART, and TCLR bits are offerred for only debugging purpose. It is not recommend to use this bit while ASQ is active. Note: This bit is write only. Reading always returns with zero."]
    #[inline(always)]
    pub fn tstart(&self) -> TstartR {
        TstartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The ASQ time counter stop. Writing '1' to this bit stops the counter. This bit is self cleared. TSTOP, TSTART, and TCLR bits are offerred for only debugging purpose. It is not recommend to use this bit while ASQ is active. Note: This bit is write only. Reading always returns with zero."]
    #[inline(always)]
    pub fn tstop(&self) -> TstopR {
        TstopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - ASQ pre-scaler shift. The value written to the PSSV bits shifts the start point of the ASQ's pre-scaler. Note that the value only affects the first cycle of the pre-scaler. 0 = No shift 1 = The pre-scaler starts 1 clock later 2 = The pre-scaler starts 2 clocks later ... 15 = The pre-scaler starts 15 clocks later"]
    #[inline(always)]
    pub fn pssv(&self) -> PssvR {
        PssvR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - The ASQ time counter clear. Writing '1' to this bit clears the the counter value. The counter must be stopped prior to be cleared. This bit is self cleared. TSTOP, TSTART, and TCLR bits are offerred for only debugging purpose. It is not recommend to use this bit while ASQ is active. Note: This bit is write only. Reading always returns with zero."]
    #[inline(always)]
    pub fn tclr(&mut self) -> TclrW<'_, SaphAtbctlSpec> {
        TclrW::new(self, 0)
    }
    #[doc = "Bit 1 - The ASQ time counter start. Writing '1' to this bit starts the counter. This bit is self cleared. TSTOP, TSTART, and TCLR bits are offerred for only debugging purpose. It is not recommend to use this bit while ASQ is active. Note: This bit is write only. Reading always returns with zero."]
    #[inline(always)]
    pub fn tstart(&mut self) -> TstartW<'_, SaphAtbctlSpec> {
        TstartW::new(self, 1)
    }
    #[doc = "Bit 2 - The ASQ time counter stop. Writing '1' to this bit stops the counter. This bit is self cleared. TSTOP, TSTART, and TCLR bits are offerred for only debugging purpose. It is not recommend to use this bit while ASQ is active. Note: This bit is write only. Reading always returns with zero."]
    #[inline(always)]
    pub fn tstop(&mut self) -> TstopW<'_, SaphAtbctlSpec> {
        TstopW::new(self, 2)
    }
    #[doc = "Bits 4:7 - ASQ pre-scaler shift. The value written to the PSSV bits shifts the start point of the ASQ's pre-scaler. Note that the value only affects the first cycle of the pre-scaler. 0 = No shift 1 = The pre-scaler starts 1 clock later 2 = The pre-scaler starts 2 clocks later ... 15 = The pre-scaler starts 15 clocks later"]
    #[inline(always)]
    pub fn pssv(&mut self) -> PssvW<'_, SaphAtbctlSpec> {
        PssvW::new(self, 4)
    }
}
#[doc = "Time Base Control\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_atbctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_atbctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAtbctlSpec;
impl crate::RegisterSpec for SaphAtbctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_atbctl::R`](R) reader structure"]
impl crate::Readable for SaphAtbctlSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_atbctl::W`](W) writer structure"]
impl crate::Writable for SaphAtbctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_ATBCTL to value 0"]
impl crate::Resettable for SaphAtbctlSpec {}
