#[doc = "Register `SAPH_AIIDX` reader"]
pub type R = crate::R<SaphAiidxSpec>;
#[doc = "Register `SAPH_AIIDX` writer"]
pub type W = crate::W<SaphAiidxSpec>;
#[doc = "This register provides the highest priority enabled interrupt index.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iidx {
    #[doc = "0: no interrupts pending"]
    None = 0,
    #[doc = "1: This interrupt indicates that either WINHI interrupt or WINLO interrupt has occurred in SDHS."]
    Dataerr = 1,
    #[doc = "2: This interrupt is valid when ASQ is activcve (auto mode). The interrupt indicates that the time counter in ASQ has reached to TIMEMARK_F (timeout)."]
    Tmfto = 2,
    #[doc = "3: This interrupt is valid when ASQ is activcve (auto mode). The interrupt occurs when ASQ completes all of the measurements programmed in ASCTL0.PNGCNT. For example, when ASCTL0.PNGCNT = 3, total four measurements are performed. The interrupt indicates that all of the four measurements have been completed."]
    Seqdn = 3,
    #[doc = "4: This interrupt is valid when ASQ is active (auto mode). The interrupt occurs when ASQ completes one measurement sequence. For example, when ASCTL0.PNGCNT = 3, total four measurements are performed. The interrupt indicates that one measurement has been completed."]
    Pngdn = 4,
}
impl From<Iidx> for u8 {
    #[inline(always)]
    fn from(variant: Iidx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iidx {
    type Ux = u8;
}
impl crate::IsEnum for Iidx {}
#[doc = "Field `IIDX` reader - This register provides the highest priority enabled interrupt index."]
pub type IidxR = crate::FieldReader<Iidx>;
impl IidxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Iidx> {
        match self.bits {
            0 => Some(Iidx::None),
            1 => Some(Iidx::Dataerr),
            2 => Some(Iidx::Tmfto),
            3 => Some(Iidx::Seqdn),
            4 => Some(Iidx::Pngdn),
            _ => None,
        }
    }
    #[doc = "no interrupts pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Iidx::None
    }
    #[doc = "This interrupt indicates that either WINHI interrupt or WINLO interrupt has occurred in SDHS."]
    #[inline(always)]
    pub fn is_dataerr(&self) -> bool {
        *self == Iidx::Dataerr
    }
    #[doc = "This interrupt is valid when ASQ is activcve (auto mode). The interrupt indicates that the time counter in ASQ has reached to TIMEMARK_F (timeout)."]
    #[inline(always)]
    pub fn is_tmfto(&self) -> bool {
        *self == Iidx::Tmfto
    }
    #[doc = "This interrupt is valid when ASQ is activcve (auto mode). The interrupt occurs when ASQ completes all of the measurements programmed in ASCTL0.PNGCNT. For example, when ASCTL0.PNGCNT = 3, total four measurements are performed. The interrupt indicates that all of the four measurements have been completed."]
    #[inline(always)]
    pub fn is_seqdn(&self) -> bool {
        *self == Iidx::Seqdn
    }
    #[doc = "This interrupt is valid when ASQ is active (auto mode). The interrupt occurs when ASQ completes one measurement sequence. For example, when ASCTL0.PNGCNT = 3, total four measurements are performed. The interrupt indicates that one measurement has been completed."]
    #[inline(always)]
    pub fn is_pngdn(&self) -> bool {
        *self == Iidx::Pngdn
    }
}
impl R {
    #[doc = "Bits 1:3 - This register provides the highest priority enabled interrupt index."]
    #[inline(always)]
    pub fn iidx(&self) -> IidxR {
        IidxR::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {}
#[doc = "Interrupt Index\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aiidx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aiidx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAiidxSpec;
impl crate::RegisterSpec for SaphAiidxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aiidx::R`](R) reader structure"]
impl crate::Readable for SaphAiidxSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_aiidx::W`](W) writer structure"]
impl crate::Writable for SaphAiidxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AIIDX to value 0"]
impl crate::Resettable for SaphAiidxSpec {}
