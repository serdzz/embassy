#[doc = "Register `SDHSIIDX` reader"]
pub type R = crate::R<SdhsiidxSpec>;
#[doc = "Register `SDHSIIDX` writer"]
pub type W = crate::W<SdhsiidxSpec>;
#[doc = "SDHS Interrupt Vector Value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Iidx {
    #[doc = "0: No Interrupt pending."]
    Iidx0 = 0,
    #[doc = "1: Interrupt Source: RIS.OVF; Interrupt Priority: Highest"]
    Iidx1 = 1,
    #[doc = "2: Interrupt Source: RIS.ACQDONE"]
    Iidx2 = 2,
    #[doc = "3: Interrupt Source: RIS.SSTRG"]
    Iidx3 = 3,
    #[doc = "4: Interrupt Source: RIS.DTRDY"]
    Iidx4 = 4,
    #[doc = "5: Interrupt Source: RIS.WINHI"]
    Iidx5 = 5,
    #[doc = "6: Interrupt Source: RIS.WINLO"]
    Iidx6 = 6,
    #[doc = "7: Reserved; Interrupt"]
    Iidx7 = 7,
    #[doc = "8: Reserved; Interrupt Priority: Lowest"]
    Iidx8 = 8,
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
#[doc = "Field `IIDX` reader - SDHS Interrupt Vector Value."]
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
            5 => Some(Iidx::Iidx5),
            6 => Some(Iidx::Iidx6),
            7 => Some(Iidx::Iidx7),
            8 => Some(Iidx::Iidx8),
            _ => None,
        }
    }
    #[doc = "No Interrupt pending."]
    #[inline(always)]
    pub fn is_iidx_0(&self) -> bool {
        *self == Iidx::Iidx0
    }
    #[doc = "Interrupt Source: RIS.OVF; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_iidx_1(&self) -> bool {
        *self == Iidx::Iidx1
    }
    #[doc = "Interrupt Source: RIS.ACQDONE"]
    #[inline(always)]
    pub fn is_iidx_2(&self) -> bool {
        *self == Iidx::Iidx2
    }
    #[doc = "Interrupt Source: RIS.SSTRG"]
    #[inline(always)]
    pub fn is_iidx_3(&self) -> bool {
        *self == Iidx::Iidx3
    }
    #[doc = "Interrupt Source: RIS.DTRDY"]
    #[inline(always)]
    pub fn is_iidx_4(&self) -> bool {
        *self == Iidx::Iidx4
    }
    #[doc = "Interrupt Source: RIS.WINHI"]
    #[inline(always)]
    pub fn is_iidx_5(&self) -> bool {
        *self == Iidx::Iidx5
    }
    #[doc = "Interrupt Source: RIS.WINLO"]
    #[inline(always)]
    pub fn is_iidx_6(&self) -> bool {
        *self == Iidx::Iidx6
    }
    #[doc = "Reserved; Interrupt"]
    #[inline(always)]
    pub fn is_iidx_7(&self) -> bool {
        *self == Iidx::Iidx7
    }
    #[doc = "Reserved; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_iidx_8(&self) -> bool {
        *self == Iidx::Iidx8
    }
}
impl R {
    #[doc = "Bits 1:15 - SDHS Interrupt Vector Value."]
    #[inline(always)]
    pub fn iidx(&self) -> IidxR {
        IidxR::new((self.bits >> 1) & 0x7fff)
    }
}
impl W {}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsiidx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsiidx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhsiidxSpec;
impl crate::RegisterSpec for SdhsiidxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsiidx::R`](R) reader structure"]
impl crate::Readable for SdhsiidxSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhsiidx::W`](W) writer structure"]
impl crate::Writable for SdhsiidxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSIIDX to value 0"]
impl crate::Resettable for SdhsiidxSpec {}
