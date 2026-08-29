#[doc = "Register `SAPH_AASCTL0` reader"]
pub type R = crate::R<SaphAasctl0Spec>;
#[doc = "Register `SAPH_AASCTL0` writer"]
pub type W = crate::W<SaphAasctl0Spec>;
#[doc = "Field `PNGCNT` reader - The total number of measurements to be performed. 0 = 1 measurement will be performed (Min) 1 = 2 measurements will be performed 2 = 3 measurements will be performed 3 = 4 measurements will be performed (Max) Note: This bit field is static, does not reflect the currently reamining measurement numbers."]
pub type PngcntR = crate::FieldReader;
#[doc = "Field `PNGCNT` writer - The total number of measurements to be performed. 0 = 1 measurement will be performed (Min) 1 = 2 measurements will be performed 2 = 3 measurements will be performed 3 = 4 measurements will be performed (Max) Note: This bit field is static, does not reflect the currently reamining measurement numbers."]
pub type PngcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "This bit selects the channel to start with when ASQ contols the measuremnet sequences.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asqchsel {
    #[doc = "0: CH0 is selected to start with. If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =1, then the channel selection would be CH0 - CH1 - CH0 - CH1. If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =0, then the channel selection would be CH0 - CH0 - CH0 - CH0."]
    Asqchsel0 = 0,
    #[doc = "1: CH1 is selected to start with. If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =0, then the channel selection would be CH1 - CH1 - CH1 - CH1. If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =1, then the channel selection would be CH1 - CH0 - CH1 - CH0."]
    Asqchsel1 = 1,
}
impl From<Asqchsel> for bool {
    #[inline(always)]
    fn from(variant: Asqchsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASQCHSEL` reader - This bit selects the channel to start with when ASQ contols the measuremnet sequences."]
pub type AsqchselR = crate::BitReader<Asqchsel>;
impl AsqchselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asqchsel {
        match self.bits {
            false => Asqchsel::Asqchsel0,
            true => Asqchsel::Asqchsel1,
        }
    }
    #[doc = "CH0 is selected to start with. If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =1, then the channel selection would be CH0 - CH1 - CH0 - CH1. If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =0, then the channel selection would be CH0 - CH0 - CH0 - CH0."]
    #[inline(always)]
    pub fn is_asqchsel_0(&self) -> bool {
        *self == Asqchsel::Asqchsel0
    }
    #[doc = "CH1 is selected to start with. If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =0, then the channel selection would be CH1 - CH1 - CH1 - CH1. If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =1, then the channel selection would be CH1 - CH0 - CH1 - CH0."]
    #[inline(always)]
    pub fn is_asqchsel_1(&self) -> bool {
        *self == Asqchsel::Asqchsel1
    }
}
#[doc = "Field `ASQCHSEL` writer - This bit selects the channel to start with when ASQ contols the measuremnet sequences."]
pub type AsqchselW<'a, REG> = crate::BitWriter<'a, REG, Asqchsel>;
impl<'a, REG> AsqchselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CH0 is selected to start with. If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =1, then the channel selection would be CH0 - CH1 - CH0 - CH1. If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =0, then the channel selection would be CH0 - CH0 - CH0 - CH0."]
    #[inline(always)]
    pub fn asqchsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Asqchsel::Asqchsel0)
    }
    #[doc = "CH1 is selected to start with. If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =0, then the channel selection would be CH1 - CH1 - CH1 - CH1. If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =1, then the channel selection would be CH1 - CH0 - CH1 - CH0."]
    #[inline(always)]
    pub fn asqchsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Asqchsel::Asqchsel1)
    }
}
#[doc = "Field `ASQSTOP` reader - Stop the ASQ. Writing '1' to this bit stops the measurement sequence controlled by the ASQ. This bit is self cleared."]
pub type AsqstopR = crate::BitReader;
#[doc = "Field `ASQSTOP` writer - Stop the ASQ. Writing '1' to this bit stops the measurement sequence controlled by the ASQ. This bit is self cleared."]
pub type AsqstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ASQ Trigger Enable. This bit can be used to indicate that the configuration of the ASQ is complete. The ASQ can only be triggered when this bit is set to '1' regardless of its trigger source. It is recommended to keep this bit zero while updating ASQ registerds.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asqten {
    #[doc = "0: ASQ trigger is disabled."]
    Asqen0 = 0,
    #[doc = "1: ASQ trigger is enabled."]
    Asqen1 = 1,
}
impl From<Asqten> for bool {
    #[inline(always)]
    fn from(variant: Asqten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASQTEN` reader - ASQ Trigger Enable. This bit can be used to indicate that the configuration of the ASQ is complete. The ASQ can only be triggered when this bit is set to '1' regardless of its trigger source. It is recommended to keep this bit zero while updating ASQ registerds."]
pub type AsqtenR = crate::BitReader<Asqten>;
impl AsqtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asqten {
        match self.bits {
            false => Asqten::Asqen0,
            true => Asqten::Asqen1,
        }
    }
    #[doc = "ASQ trigger is disabled."]
    #[inline(always)]
    pub fn is_asqen_0(&self) -> bool {
        *self == Asqten::Asqen0
    }
    #[doc = "ASQ trigger is enabled."]
    #[inline(always)]
    pub fn is_asqen_1(&self) -> bool {
        *self == Asqten::Asqen1
    }
}
#[doc = "Field `ASQTEN` writer - ASQ Trigger Enable. This bit can be used to indicate that the configuration of the ASQ is complete. The ASQ can only be triggered when this bit is set to '1' regardless of its trigger source. It is recommended to keep this bit zero while updating ASQ registerds."]
pub type AsqtenW<'a, REG> = crate::BitWriter<'a, REG, Asqten>;
impl<'a, REG> AsqtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ASQ trigger is disabled."]
    #[inline(always)]
    pub fn asqen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Asqten::Asqen0)
    }
    #[doc = "ASQ trigger is enabled."]
    #[inline(always)]
    pub fn asqen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Asqten::Asqen1)
    }
}
#[doc = "ASQ trigger select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigsel {
    #[doc = "0: Writing '1' to ASQTRIG.ASQTRIG"]
    Swtrig = 0,
    #[doc = "1: The PSQ is selected to start the ASQ."]
    Psq = 1,
    #[doc = "2: Ext. Signal (See device specific datasheet)"]
    Timer = 2,
    #[doc = "3: Ext. Signal (See device specific datasheet)"]
    Trigsel3 = 3,
}
impl From<Trigsel> for u8 {
    #[inline(always)]
    fn from(variant: Trigsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigsel {
    type Ux = u8;
}
impl crate::IsEnum for Trigsel {}
#[doc = "Field `TRIGSEL` reader - ASQ trigger select."]
pub type TrigselR = crate::FieldReader<Trigsel>;
impl TrigselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigsel {
        match self.bits {
            0 => Trigsel::Swtrig,
            1 => Trigsel::Psq,
            2 => Trigsel::Timer,
            3 => Trigsel::Trigsel3,
            _ => unreachable!(),
        }
    }
    #[doc = "Writing '1' to ASQTRIG.ASQTRIG"]
    #[inline(always)]
    pub fn is_swtrig(&self) -> bool {
        *self == Trigsel::Swtrig
    }
    #[doc = "The PSQ is selected to start the ASQ."]
    #[inline(always)]
    pub fn is_psq(&self) -> bool {
        *self == Trigsel::Psq
    }
    #[doc = "Ext. Signal (See device specific datasheet)"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == Trigsel::Timer
    }
    #[doc = "Ext. Signal (See device specific datasheet)"]
    #[inline(always)]
    pub fn is_trigsel_3(&self) -> bool {
        *self == Trigsel::Trigsel3
    }
}
#[doc = "Field `TRIGSEL` writer - ASQ trigger select."]
pub type TrigselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trigsel, crate::Safe>;
impl<'a, REG> TrigselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing '1' to ASQTRIG.ASQTRIG"]
    #[inline(always)]
    pub fn swtrig(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Swtrig)
    }
    #[doc = "The PSQ is selected to start the ASQ."]
    #[inline(always)]
    pub fn psq(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Psq)
    }
    #[doc = "Ext. Signal (See device specific datasheet)"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Timer)
    }
    #[doc = "Ext. Signal (See device specific datasheet)"]
    #[inline(always)]
    pub fn trigsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Trigsel3)
    }
}
#[doc = "Stop ASQ when the DATAERR interrupt occurrs when the mesurements controlled by ASQ (auto mode). Note that it is not possible to resume the measurement from where it was stopped. When ASQ is triggered again, the measurement seqeunce starts from the beginning.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erabrt {
    #[doc = "0: Continue the measurements until completion regardless of the DATAERR interrupt."]
    Erabrt0 = 0,
    #[doc = "1: Stop the ASQ upon the DATAERR interrupt."]
    Erabrt1 = 1,
}
impl From<Erabrt> for bool {
    #[inline(always)]
    fn from(variant: Erabrt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERABRT` reader - Stop ASQ when the DATAERR interrupt occurrs when the mesurements controlled by ASQ (auto mode). Note that it is not possible to resume the measurement from where it was stopped. When ASQ is triggered again, the measurement seqeunce starts from the beginning."]
pub type ErabrtR = crate::BitReader<Erabrt>;
impl ErabrtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erabrt {
        match self.bits {
            false => Erabrt::Erabrt0,
            true => Erabrt::Erabrt1,
        }
    }
    #[doc = "Continue the measurements until completion regardless of the DATAERR interrupt."]
    #[inline(always)]
    pub fn is_erabrt_0(&self) -> bool {
        *self == Erabrt::Erabrt0
    }
    #[doc = "Stop the ASQ upon the DATAERR interrupt."]
    #[inline(always)]
    pub fn is_erabrt_1(&self) -> bool {
        *self == Erabrt::Erabrt1
    }
}
#[doc = "Field `ERABRT` writer - Stop ASQ when the DATAERR interrupt occurrs when the mesurements controlled by ASQ (auto mode). Note that it is not possible to resume the measurement from where it was stopped. When ASQ is triggered again, the measurement seqeunce starts from the beginning."]
pub type ErabrtW<'a, REG> = crate::BitWriter<'a, REG, Erabrt>;
impl<'a, REG> ErabrtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue the measurements until completion regardless of the DATAERR interrupt."]
    #[inline(always)]
    pub fn erabrt_0(self) -> &'a mut crate::W<REG> {
        self.variant(Erabrt::Erabrt0)
    }
    #[doc = "Stop the ASQ upon the DATAERR interrupt."]
    #[inline(always)]
    pub fn erabrt_1(self) -> &'a mut crate::W<REG> {
        self.variant(Erabrt::Erabrt1)
    }
}
impl R {
    #[doc = "Bits 0:1 - The total number of measurements to be performed. 0 = 1 measurement will be performed (Min) 1 = 2 measurements will be performed 2 = 3 measurements will be performed 3 = 4 measurements will be performed (Max) Note: This bit field is static, does not reflect the currently reamining measurement numbers."]
    #[inline(always)]
    pub fn pngcnt(&self) -> PngcntR {
        PngcntR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - This bit selects the channel to start with when ASQ contols the measuremnet sequences."]
    #[inline(always)]
    pub fn asqchsel(&self) -> AsqchselR {
        AsqchselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Stop the ASQ. Writing '1' to this bit stops the measurement sequence controlled by the ASQ. This bit is self cleared."]
    #[inline(always)]
    pub fn asqstop(&self) -> AsqstopR {
        AsqstopR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - ASQ Trigger Enable. This bit can be used to indicate that the configuration of the ASQ is complete. The ASQ can only be triggered when this bit is set to '1' regardless of its trigger source. It is recommended to keep this bit zero while updating ASQ registerds."]
    #[inline(always)]
    pub fn asqten(&self) -> AsqtenR {
        AsqtenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - ASQ trigger select."]
    #[inline(always)]
    pub fn trigsel(&self) -> TrigselR {
        TrigselR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 13 - Stop ASQ when the DATAERR interrupt occurrs when the mesurements controlled by ASQ (auto mode). Note that it is not possible to resume the measurement from where it was stopped. When ASQ is triggered again, the measurement seqeunce starts from the beginning."]
    #[inline(always)]
    pub fn erabrt(&self) -> ErabrtR {
        ErabrtR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - The total number of measurements to be performed. 0 = 1 measurement will be performed (Min) 1 = 2 measurements will be performed 2 = 3 measurements will be performed 3 = 4 measurements will be performed (Max) Note: This bit field is static, does not reflect the currently reamining measurement numbers."]
    #[inline(always)]
    pub fn pngcnt(&mut self) -> PngcntW<'_, SaphAasctl0Spec> {
        PngcntW::new(self, 0)
    }
    #[doc = "Bit 4 - This bit selects the channel to start with when ASQ contols the measuremnet sequences."]
    #[inline(always)]
    pub fn asqchsel(&mut self) -> AsqchselW<'_, SaphAasctl0Spec> {
        AsqchselW::new(self, 4)
    }
    #[doc = "Bit 7 - Stop the ASQ. Writing '1' to this bit stops the measurement sequence controlled by the ASQ. This bit is self cleared."]
    #[inline(always)]
    pub fn asqstop(&mut self) -> AsqstopW<'_, SaphAasctl0Spec> {
        AsqstopW::new(self, 7)
    }
    #[doc = "Bit 9 - ASQ Trigger Enable. This bit can be used to indicate that the configuration of the ASQ is complete. The ASQ can only be triggered when this bit is set to '1' regardless of its trigger source. It is recommended to keep this bit zero while updating ASQ registerds."]
    #[inline(always)]
    pub fn asqten(&mut self) -> AsqtenW<'_, SaphAasctl0Spec> {
        AsqtenW::new(self, 9)
    }
    #[doc = "Bits 10:11 - ASQ trigger select."]
    #[inline(always)]
    pub fn trigsel(&mut self) -> TrigselW<'_, SaphAasctl0Spec> {
        TrigselW::new(self, 10)
    }
    #[doc = "Bit 13 - Stop ASQ when the DATAERR interrupt occurrs when the mesurements controlled by ASQ (auto mode). Note that it is not possible to resume the measurement from where it was stopped. When ASQ is triggered again, the measurement seqeunce starts from the beginning."]
    #[inline(always)]
    pub fn erabrt(&mut self) -> ErabrtW<'_, SaphAasctl0Spec> {
        ErabrtW::new(self, 13)
    }
}
#[doc = "A-SEQ control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aasctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aasctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAasctl0Spec;
impl crate::RegisterSpec for SaphAasctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aasctl0::R`](R) reader structure"]
impl crate::Readable for SaphAasctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`saph_aasctl0::W`](W) writer structure"]
impl crate::Writable for SaphAasctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AASCTL0 to value 0"]
impl crate::Resettable for SaphAasctl0Spec {}
