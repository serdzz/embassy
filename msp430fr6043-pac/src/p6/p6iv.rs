#[doc = "Register `P6IV` reader"]
pub type R = crate::R<P6ivSpec>;
#[doc = "Register `P6IV` writer"]
pub type W = crate::W<P6ivSpec>;
#[doc = "Port 6 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P6iv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Port 6.0 interrupt; Interrupt Flag: P6IFG0; Interrupt Priority: Highest"]
    P6ifg0 = 2,
    #[doc = "4: Interrupt Source: Port 6.1 interrupt; Interrupt Flag: P6IFG1"]
    P6ifg1 = 4,
    #[doc = "6: Interrupt Source: Port 6.2 interrupt; Interrupt Flag: P6IFG2"]
    P6ifg2 = 6,
    #[doc = "8: Interrupt Source: Port 6.3 interrupt; Interrupt Flag: P6IFG3"]
    P6ifg3 = 8,
    #[doc = "10: Interrupt Source: Port 6.4 interrupt; Interrupt Flag: P6IFG4"]
    P6ifg4 = 10,
    #[doc = "12: Interrupt Source: Port 6.5 interrupt; Interrupt Flag: P6IFG5"]
    P6ifg5 = 12,
    #[doc = "14: Interrupt Source: Port 6.6 interrupt; Interrupt Flag: P6IFG6"]
    P6ifg6 = 14,
    #[doc = "16: Interrupt Source: Port 6.7 interrupt; Interrupt Flag: P6IFG7; Interrupt Priority: Lowest"]
    P6ifg7 = 16,
}
impl From<P6iv> for u8 {
    #[inline(always)]
    fn from(variant: P6iv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P6iv {
    type Ux = u8;
}
impl crate::IsEnum for P6iv {}
#[doc = "Field `P6IV` reader - Port 6 interrupt vector value"]
pub type P6ivR = crate::FieldReader<P6iv>;
impl P6ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P6iv> {
        match self.bits {
            0 => Some(P6iv::None),
            2 => Some(P6iv::P6ifg0),
            4 => Some(P6iv::P6ifg1),
            6 => Some(P6iv::P6ifg2),
            8 => Some(P6iv::P6ifg3),
            10 => Some(P6iv::P6ifg4),
            12 => Some(P6iv::P6ifg5),
            14 => Some(P6iv::P6ifg6),
            16 => Some(P6iv::P6ifg7),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P6iv::None
    }
    #[doc = "Interrupt Source: Port 6.0 interrupt; Interrupt Flag: P6IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p6ifg0(&self) -> bool {
        *self == P6iv::P6ifg0
    }
    #[doc = "Interrupt Source: Port 6.1 interrupt; Interrupt Flag: P6IFG1"]
    #[inline(always)]
    pub fn is_p6ifg1(&self) -> bool {
        *self == P6iv::P6ifg1
    }
    #[doc = "Interrupt Source: Port 6.2 interrupt; Interrupt Flag: P6IFG2"]
    #[inline(always)]
    pub fn is_p6ifg2(&self) -> bool {
        *self == P6iv::P6ifg2
    }
    #[doc = "Interrupt Source: Port 6.3 interrupt; Interrupt Flag: P6IFG3"]
    #[inline(always)]
    pub fn is_p6ifg3(&self) -> bool {
        *self == P6iv::P6ifg3
    }
    #[doc = "Interrupt Source: Port 6.4 interrupt; Interrupt Flag: P6IFG4"]
    #[inline(always)]
    pub fn is_p6ifg4(&self) -> bool {
        *self == P6iv::P6ifg4
    }
    #[doc = "Interrupt Source: Port 6.5 interrupt; Interrupt Flag: P6IFG5"]
    #[inline(always)]
    pub fn is_p6ifg5(&self) -> bool {
        *self == P6iv::P6ifg5
    }
    #[doc = "Interrupt Source: Port 6.6 interrupt; Interrupt Flag: P6IFG6"]
    #[inline(always)]
    pub fn is_p6ifg6(&self) -> bool {
        *self == P6iv::P6ifg6
    }
    #[doc = "Interrupt Source: Port 6.7 interrupt; Interrupt Flag: P6IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p6ifg7(&self) -> bool {
        *self == P6iv::P6ifg7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 6 interrupt vector value"]
    #[inline(always)]
    pub fn p6iv(&self) -> P6ivR {
        P6ivR::new((self.bits & 0x1f) as u8)
    }
}
impl W {}
#[doc = "Port 6 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p6iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6ivSpec;
impl crate::RegisterSpec for P6ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p6iv::R`](R) reader structure"]
impl crate::Readable for P6ivSpec {}
#[doc = "`write(|w| ..)` method takes [`p6iv::W`](W) writer structure"]
impl crate::Writable for P6ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P6IV to value 0"]
impl crate::Resettable for P6ivSpec {}
