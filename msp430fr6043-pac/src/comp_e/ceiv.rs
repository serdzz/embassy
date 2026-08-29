#[doc = "Register `CEIV` reader"]
pub type R = crate::R<CeivSpec>;
#[doc = "Register `CEIV` writer"]
pub type W = crate::W<CeivSpec>;
#[doc = "Comparator interrupt vector word register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Ceiv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: CEOUT interrupt; Interrupt Flag: CEIFG; Interrupt Priority: Highest"]
    Ceifg = 2,
    #[doc = "4: Interrupt Source: CEOUT interrupt inverted polarity; Interrupt Flag: CEIIFG"]
    Ceiifg = 4,
    #[doc = "10: Interrupt Source: Comparator ready interrupt; Interrupt Flag: CERDYIFG; Interrupt Priority: Lowest"]
    Cerdyifg = 10,
}
impl From<Ceiv> for u16 {
    #[inline(always)]
    fn from(variant: Ceiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ceiv {
    type Ux = u16;
}
impl crate::IsEnum for Ceiv {}
#[doc = "Field `CEIV` reader - Comparator interrupt vector word register"]
pub type CeivR = crate::FieldReader<Ceiv>;
impl CeivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ceiv> {
        match self.bits {
            0 => Some(Ceiv::None),
            2 => Some(Ceiv::Ceifg),
            4 => Some(Ceiv::Ceiifg),
            10 => Some(Ceiv::Cerdyifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ceiv::None
    }
    #[doc = "Interrupt Source: CEOUT interrupt; Interrupt Flag: CEIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_ceifg(&self) -> bool {
        *self == Ceiv::Ceifg
    }
    #[doc = "Interrupt Source: CEOUT interrupt inverted polarity; Interrupt Flag: CEIIFG"]
    #[inline(always)]
    pub fn is_ceiifg(&self) -> bool {
        *self == Ceiv::Ceiifg
    }
    #[doc = "Interrupt Source: Comparator ready interrupt; Interrupt Flag: CERDYIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_cerdyifg(&self) -> bool {
        *self == Ceiv::Cerdyifg
    }
}
impl R {
    #[doc = "Bits 0:15 - Comparator interrupt vector word register"]
    #[inline(always)]
    pub fn ceiv(&self) -> CeivR {
        CeivR::new(self.bits)
    }
}
impl W {}
#[doc = "Comparator Interrupt Vector Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ceiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ceiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CeivSpec;
impl crate::RegisterSpec for CeivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ceiv::R`](R) reader structure"]
impl crate::Readable for CeivSpec {}
#[doc = "`write(|w| ..)` method takes [`ceiv::W`](W) writer structure"]
impl crate::Writable for CeivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CEIV to value 0"]
impl crate::Resettable for CeivSpec {}
