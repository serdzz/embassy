#[doc = "Register `TA1IV` reader"]
pub type R = crate::R<Ta1ivSpec>;
#[doc = "Register `TA1IV` writer"]
pub type W = crate::W<Ta1ivSpec>;
#[doc = "TimerA interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Taiv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Capture/compare 1; Interrupt Flag: TAxCCR1 CCIFG; Interrupt Priority: Highest"]
    Taccr1 = 2,
    #[doc = "4: Interrupt Source: Capture/compare 2; Interrupt Flag: TAxCCR2 CCIFG"]
    Taccr2 = 4,
    #[doc = "6: Interrupt Source: Capture/compare 3; Interrupt Flag: TAxCCR3 CCIFG"]
    Taccr3 = 6,
    #[doc = "8: Interrupt Source: Capture/compare 4; Interrupt Flag: TAxCCR4 CCIFG"]
    Taccr4 = 8,
    #[doc = "10: Interrupt Source: Capture/compare 5; Interrupt Flag: TAxCCR5 CCIFG"]
    Taccr5 = 10,
    #[doc = "12: Interrupt Source: Capture/compare 6; Interrupt Flag: TAxCCR6 CCIFG"]
    Taccr6 = 12,
    #[doc = "14: Interrupt Source: Timer overflow; Interrupt Flag: TAxCTL TAIFG; Interrupt Priority: Lowest"]
    Taifg = 14,
}
impl From<Taiv> for u16 {
    #[inline(always)]
    fn from(variant: Taiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Taiv {
    type Ux = u16;
}
impl crate::IsEnum for Taiv {}
#[doc = "Field `TAIV` reader - TimerA interrupt vector value"]
pub type TaivR = crate::FieldReader<Taiv>;
impl TaivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Taiv> {
        match self.bits {
            0 => Some(Taiv::None),
            2 => Some(Taiv::Taccr1),
            4 => Some(Taiv::Taccr2),
            6 => Some(Taiv::Taccr3),
            8 => Some(Taiv::Taccr4),
            10 => Some(Taiv::Taccr5),
            12 => Some(Taiv::Taccr6),
            14 => Some(Taiv::Taifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Taiv::None
    }
    #[doc = "Interrupt Source: Capture/compare 1; Interrupt Flag: TAxCCR1 CCIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_taccr1(&self) -> bool {
        *self == Taiv::Taccr1
    }
    #[doc = "Interrupt Source: Capture/compare 2; Interrupt Flag: TAxCCR2 CCIFG"]
    #[inline(always)]
    pub fn is_taccr2(&self) -> bool {
        *self == Taiv::Taccr2
    }
    #[doc = "Interrupt Source: Capture/compare 3; Interrupt Flag: TAxCCR3 CCIFG"]
    #[inline(always)]
    pub fn is_taccr3(&self) -> bool {
        *self == Taiv::Taccr3
    }
    #[doc = "Interrupt Source: Capture/compare 4; Interrupt Flag: TAxCCR4 CCIFG"]
    #[inline(always)]
    pub fn is_taccr4(&self) -> bool {
        *self == Taiv::Taccr4
    }
    #[doc = "Interrupt Source: Capture/compare 5; Interrupt Flag: TAxCCR5 CCIFG"]
    #[inline(always)]
    pub fn is_taccr5(&self) -> bool {
        *self == Taiv::Taccr5
    }
    #[doc = "Interrupt Source: Capture/compare 6; Interrupt Flag: TAxCCR6 CCIFG"]
    #[inline(always)]
    pub fn is_taccr6(&self) -> bool {
        *self == Taiv::Taccr6
    }
    #[doc = "Interrupt Source: Timer overflow; Interrupt Flag: TAxCTL TAIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_taifg(&self) -> bool {
        *self == Taiv::Taifg
    }
}
impl R {
    #[doc = "Bits 0:15 - TimerA interrupt vector value"]
    #[inline(always)]
    pub fn taiv(&self) -> TaivR {
        TaivR::new(self.bits)
    }
}
impl W {}
#[doc = "TimerAx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta1iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta1iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta1ivSpec;
impl crate::RegisterSpec for Ta1ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta1iv::R`](R) reader structure"]
impl crate::Readable for Ta1ivSpec {}
#[doc = "`write(|w| ..)` method takes [`ta1iv::W`](W) writer structure"]
impl crate::Writable for Ta1ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA1IV to value 0"]
impl crate::Resettable for Ta1ivSpec {}
