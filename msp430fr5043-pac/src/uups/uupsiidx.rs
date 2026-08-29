#[doc = "Register `UUPSIIDX` reader"]
pub type R = crate::R<UupsiidxSpec>;
#[doc = "Register `UUPSIIDX` writer"]
pub type W = crate::W<UupsiidxSpec>;
#[doc = "UUPS Interrupt Vector Value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Iidx {
    #[doc = "0: No Interrupt pending"]
    Iidx0 = 0,
    #[doc = "1: Interrupt Source: PTMOUT; Interrupt Priority: Highest"]
    Iidx1 = 1,
    #[doc = "2: Interrupt Source: PREQIG"]
    Iidx2 = 2,
    #[doc = "3: Interrupt Source: STPBYDB"]
    Iidx3 = 3,
    #[doc = "4: Reserved; Interrupt Priority: Lowest"]
    Iidx4 = 4,
}
impl From<Iidx> for u16 {
    #[inline(always)]
    fn from(variant: Iidx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iidx {
    type Ux = u16;
}
impl crate::IsEnum for Iidx {}
#[doc = "Field `IIDX` reader - UUPS Interrupt Vector Value."]
pub type IidxR = crate::FieldReader<Iidx>;
impl IidxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Iidx> {
        match self.bits {
            0 => Some(Iidx::Iidx0),
            1 => Some(Iidx::Iidx1),
            2 => Some(Iidx::Iidx2),
            3 => Some(Iidx::Iidx3),
            4 => Some(Iidx::Iidx4),
            _ => None,
        }
    }
    #[doc = "No Interrupt pending"]
    #[inline(always)]
    pub fn is_iidx_0(&self) -> bool {
        *self == Iidx::Iidx0
    }
    #[doc = "Interrupt Source: PTMOUT; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_iidx_1(&self) -> bool {
        *self == Iidx::Iidx1
    }
    #[doc = "Interrupt Source: PREQIG"]
    #[inline(always)]
    pub fn is_iidx_2(&self) -> bool {
        *self == Iidx::Iidx2
    }
    #[doc = "Interrupt Source: STPBYDB"]
    #[inline(always)]
    pub fn is_iidx_3(&self) -> bool {
        *self == Iidx::Iidx3
    }
    #[doc = "Reserved; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_iidx_4(&self) -> bool {
        *self == Iidx::Iidx4
    }
}
impl R {
    #[doc = "Bits 1:15 - UUPS Interrupt Vector Value."]
    #[inline(always)]
    pub fn iidx(&self) -> IidxR {
        IidxR::new((self.bits >> 1) & 0x7fff)
    }
}
impl W {}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsiidx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsiidx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UupsiidxSpec;
impl crate::RegisterSpec for UupsiidxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uupsiidx::R`](R) reader structure"]
impl crate::Readable for UupsiidxSpec {}
#[doc = "`write(|w| ..)` method takes [`uupsiidx::W`](W) writer structure"]
impl crate::Writable for UupsiidxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UUPSIIDX to value 0"]
impl crate::Resettable for UupsiidxSpec {}
