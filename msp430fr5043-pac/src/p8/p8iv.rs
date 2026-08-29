#[doc = "Register `P8IV` reader"]
pub type R = crate::R<P8ivSpec>;
#[doc = "Register `P8IV` writer"]
pub type W = crate::W<P8ivSpec>;
#[doc = "Port 8 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P8iv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Port 8.0 interrupt; Interrupt Flag: P8IFG0; Interrupt Priority: Highest"]
    P8ifg0 = 2,
    #[doc = "4: Interrupt Source: Port 8.1 interrupt; Interrupt Flag: P8IFG1"]
    P8ifg1 = 4,
    #[doc = "6: Interrupt Source: Port 8.2 interrupt; Interrupt Flag: P8IFG2"]
    P8ifg2 = 6,
    #[doc = "8: Interrupt Source: Port 8.3 interrupt; Interrupt Flag: P8IFG3"]
    P8ifg3 = 8,
    #[doc = "10: Interrupt Source: Port 8.4 interrupt; Interrupt Flag: P8IFG4"]
    P8ifg4 = 10,
    #[doc = "12: Interrupt Source: Port 8.5 interrupt; Interrupt Flag: P8IFG5"]
    P8ifg5 = 12,
    #[doc = "14: Interrupt Source: Port 8.6 interrupt; Interrupt Flag: P8IFG6"]
    P8ifg6 = 14,
    #[doc = "16: Interrupt Source: Port 8.7 interrupt; Interrupt Flag: P8IFG7; Interrupt Priority: Lowest"]
    P8ifg7 = 16,
}
impl From<P8iv> for u8 {
    #[inline(always)]
    fn from(variant: P8iv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P8iv {
    type Ux = u8;
}
impl crate::IsEnum for P8iv {}
#[doc = "Field `P8IV` reader - Port 8 interrupt vector value"]
pub type P8ivR = crate::FieldReader<P8iv>;
impl P8ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P8iv> {
        match self.bits {
            0 => Some(P8iv::None),
            2 => Some(P8iv::P8ifg0),
            4 => Some(P8iv::P8ifg1),
            6 => Some(P8iv::P8ifg2),
            8 => Some(P8iv::P8ifg3),
            10 => Some(P8iv::P8ifg4),
            12 => Some(P8iv::P8ifg5),
            14 => Some(P8iv::P8ifg6),
            16 => Some(P8iv::P8ifg7),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P8iv::None
    }
    #[doc = "Interrupt Source: Port 8.0 interrupt; Interrupt Flag: P8IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p8ifg0(&self) -> bool {
        *self == P8iv::P8ifg0
    }
    #[doc = "Interrupt Source: Port 8.1 interrupt; Interrupt Flag: P8IFG1"]
    #[inline(always)]
    pub fn is_p8ifg1(&self) -> bool {
        *self == P8iv::P8ifg1
    }
    #[doc = "Interrupt Source: Port 8.2 interrupt; Interrupt Flag: P8IFG2"]
    #[inline(always)]
    pub fn is_p8ifg2(&self) -> bool {
        *self == P8iv::P8ifg2
    }
    #[doc = "Interrupt Source: Port 8.3 interrupt; Interrupt Flag: P8IFG3"]
    #[inline(always)]
    pub fn is_p8ifg3(&self) -> bool {
        *self == P8iv::P8ifg3
    }
    #[doc = "Interrupt Source: Port 8.4 interrupt; Interrupt Flag: P8IFG4"]
    #[inline(always)]
    pub fn is_p8ifg4(&self) -> bool {
        *self == P8iv::P8ifg4
    }
    #[doc = "Interrupt Source: Port 8.5 interrupt; Interrupt Flag: P8IFG5"]
    #[inline(always)]
    pub fn is_p8ifg5(&self) -> bool {
        *self == P8iv::P8ifg5
    }
    #[doc = "Interrupt Source: Port 8.6 interrupt; Interrupt Flag: P8IFG6"]
    #[inline(always)]
    pub fn is_p8ifg6(&self) -> bool {
        *self == P8iv::P8ifg6
    }
    #[doc = "Interrupt Source: Port 8.7 interrupt; Interrupt Flag: P8IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p8ifg7(&self) -> bool {
        *self == P8iv::P8ifg7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 8 interrupt vector value"]
    #[inline(always)]
    pub fn p8iv(&self) -> P8ivR {
        P8ivR::new((self.bits & 0x1f) as u8)
    }
}
impl W {}
#[doc = "Port 8 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p8iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8ivSpec;
impl crate::RegisterSpec for P8ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p8iv::R`](R) reader structure"]
impl crate::Readable for P8ivSpec {}
#[doc = "`write(|w| ..)` method takes [`p8iv::W`](W) writer structure"]
impl crate::Writable for P8ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P8IV to value 0"]
impl crate::Resettable for P8ivSpec {}
