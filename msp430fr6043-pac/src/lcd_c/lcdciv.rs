#[doc = "Register `LCDCIV` reader"]
pub type R = crate::R<LcdcivSpec>;
#[doc = "Register `LCDCIV` writer"]
pub type W = crate::W<LcdcivSpec>;
#[doc = "LCD_C interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Lcdciv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: No capacitor connected; Interrupt Flag: LCDNOCAPIFG; Interrupt Priority: Highest"]
    Lcdnocapifg = 2,
    #[doc = "4: Interrupt Source: Blink, segments off; Interrupt Flag: LCDBLKOFFIFG"]
    Lcdblkoffifg = 4,
    #[doc = "6: Interrupt Source: Blink, segments on; Interrupt Flag: LCDBLKONIFG"]
    Lcdblkonifg = 6,
    #[doc = "8: Interrupt Source: Frame interrupt; Interrupt Flag: LCDFRMIFG; Interrupt Priority: Lowest"]
    Lcdfrmifg = 8,
}
impl From<Lcdciv> for u16 {
    #[inline(always)]
    fn from(variant: Lcdciv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcdciv {
    type Ux = u16;
}
impl crate::IsEnum for Lcdciv {}
#[doc = "Field `LCDCIV` reader - LCD_C interrupt vector value"]
pub type LcdcivR = crate::FieldReader<Lcdciv>;
impl LcdcivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lcdciv> {
        match self.bits {
            0 => Some(Lcdciv::None),
            2 => Some(Lcdciv::Lcdnocapifg),
            4 => Some(Lcdciv::Lcdblkoffifg),
            6 => Some(Lcdciv::Lcdblkonifg),
            8 => Some(Lcdciv::Lcdfrmifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Lcdciv::None
    }
    #[doc = "Interrupt Source: No capacitor connected; Interrupt Flag: LCDNOCAPIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_lcdnocapifg(&self) -> bool {
        *self == Lcdciv::Lcdnocapifg
    }
    #[doc = "Interrupt Source: Blink, segments off; Interrupt Flag: LCDBLKOFFIFG"]
    #[inline(always)]
    pub fn is_lcdblkoffifg(&self) -> bool {
        *self == Lcdciv::Lcdblkoffifg
    }
    #[doc = "Interrupt Source: Blink, segments on; Interrupt Flag: LCDBLKONIFG"]
    #[inline(always)]
    pub fn is_lcdblkonifg(&self) -> bool {
        *self == Lcdciv::Lcdblkonifg
    }
    #[doc = "Interrupt Source: Frame interrupt; Interrupt Flag: LCDFRMIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_lcdfrmifg(&self) -> bool {
        *self == Lcdciv::Lcdfrmifg
    }
}
impl R {
    #[doc = "Bits 0:15 - LCD_C interrupt vector value"]
    #[inline(always)]
    pub fn lcdciv(&self) -> LcdcivR {
        LcdcivR::new(self.bits)
    }
}
impl W {}
#[doc = "LCD_C interrupt vector\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdciv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdciv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcivSpec;
impl crate::RegisterSpec for LcdcivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdciv::R`](R) reader structure"]
impl crate::Readable for LcdcivSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdciv::W`](W) writer structure"]
impl crate::Writable for LcdcivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDCIV to value 0"]
impl crate::Resettable for LcdcivSpec {}
