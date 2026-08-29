#[doc = "Register `HSPLLIIDX` reader"]
pub type R = crate::R<HsplliidxSpec>;
#[doc = "Register `HSPLLIIDX` writer"]
pub type W = crate::W<HsplliidxSpec>;
#[doc = "HSPLL Interrupt Vector Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Iidx {
    #[doc = "0: No Interrupt pending"]
    Iidx0 = 0,
    #[doc = "1: Interrupt Source: PLLUNLOCK; Interrupt Priority: Highest"]
    Iidx1 = 1,
    #[doc = "2: Reserved; Interrupt Priority: Lowest"]
    Iidx2 = 2,
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
#[doc = "Field `IIDX` reader - HSPLL Interrupt Vector Value"]
pub type IidxR = crate::FieldReader<Iidx>;
impl IidxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Iidx> {
        match self.bits {
            0 => Some(Iidx::Iidx0),
            1 => Some(Iidx::Iidx1),
            2 => Some(Iidx::Iidx2),
            _ => None,
        }
    }
    #[doc = "No Interrupt pending"]
    #[inline(always)]
    pub fn is_iidx_0(&self) -> bool {
        *self == Iidx::Iidx0
    }
    #[doc = "Interrupt Source: PLLUNLOCK; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_iidx_1(&self) -> bool {
        *self == Iidx::Iidx1
    }
    #[doc = "Reserved; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_iidx_2(&self) -> bool {
        *self == Iidx::Iidx2
    }
}
impl R {
    #[doc = "Bits 1:15 - HSPLL Interrupt Vector Value"]
    #[inline(always)]
    pub fn iidx(&self) -> IidxR {
        IidxR::new((self.bits >> 1) & 0x7fff)
    }
}
impl W {}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsplliidx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsplliidx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsplliidxSpec;
impl crate::RegisterSpec for HsplliidxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hsplliidx::R`](R) reader structure"]
impl crate::Readable for HsplliidxSpec {}
#[doc = "`write(|w| ..)` method takes [`hsplliidx::W`](W) writer structure"]
impl crate::Writable for HsplliidxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSPLLIIDX to value 0"]
impl crate::Resettable for HsplliidxSpec {}
