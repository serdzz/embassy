#[doc = "Register `P9IV` reader"]
pub type R = crate::R<P9ivSpec>;
#[doc = "Register `P9IV` writer"]
pub type W = crate::W<P9ivSpec>;
#[doc = "Port 9 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P9iv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Port 9.0 interrupt; Interrupt Flag: P9IFG0; Interrupt Priority: Highest"]
    P9ifg0 = 2,
    #[doc = "4: Interrupt Source: Port 9.1 interrupt; Interrupt Flag: P9IFG1"]
    P9ifg1 = 4,
    #[doc = "6: Interrupt Source: Port 9.2 interrupt; Interrupt Flag: P9IFG2"]
    P9ifg2 = 6,
    #[doc = "8: Interrupt Source: Port 9.3 interrupt; Interrupt Flag: P9IFG3"]
    P9ifg3 = 8,
    #[doc = "10: Interrupt Source: Port 9.4 interrupt; Interrupt Flag: P9IFG4"]
    P9ifg4 = 10,
    #[doc = "12: Interrupt Source: Port 9.5 interrupt; Interrupt Flag: P9IFG5"]
    P9ifg5 = 12,
    #[doc = "14: Interrupt Source: Port 9.6 interrupt; Interrupt Flag: P9IFG6"]
    P9ifg6 = 14,
    #[doc = "16: Interrupt Source: Port 9.7 interrupt; Interrupt Flag: P9IFG7; Interrupt Priority: Lowest"]
    P9ifg7 = 16,
}
impl From<P9iv> for u8 {
    #[inline(always)]
    fn from(variant: P9iv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P9iv {
    type Ux = u8;
}
impl crate::IsEnum for P9iv {}
#[doc = "Field `P9IV` reader - Port 9 interrupt vector value"]
pub type P9ivR = crate::FieldReader<P9iv>;
impl P9ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P9iv> {
        match self.bits {
            0 => Some(P9iv::None),
            2 => Some(P9iv::P9ifg0),
            4 => Some(P9iv::P9ifg1),
            6 => Some(P9iv::P9ifg2),
            8 => Some(P9iv::P9ifg3),
            10 => Some(P9iv::P9ifg4),
            12 => Some(P9iv::P9ifg5),
            14 => Some(P9iv::P9ifg6),
            16 => Some(P9iv::P9ifg7),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P9iv::None
    }
    #[doc = "Interrupt Source: Port 9.0 interrupt; Interrupt Flag: P9IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p9ifg0(&self) -> bool {
        *self == P9iv::P9ifg0
    }
    #[doc = "Interrupt Source: Port 9.1 interrupt; Interrupt Flag: P9IFG1"]
    #[inline(always)]
    pub fn is_p9ifg1(&self) -> bool {
        *self == P9iv::P9ifg1
    }
    #[doc = "Interrupt Source: Port 9.2 interrupt; Interrupt Flag: P9IFG2"]
    #[inline(always)]
    pub fn is_p9ifg2(&self) -> bool {
        *self == P9iv::P9ifg2
    }
    #[doc = "Interrupt Source: Port 9.3 interrupt; Interrupt Flag: P9IFG3"]
    #[inline(always)]
    pub fn is_p9ifg3(&self) -> bool {
        *self == P9iv::P9ifg3
    }
    #[doc = "Interrupt Source: Port 9.4 interrupt; Interrupt Flag: P9IFG4"]
    #[inline(always)]
    pub fn is_p9ifg4(&self) -> bool {
        *self == P9iv::P9ifg4
    }
    #[doc = "Interrupt Source: Port 9.5 interrupt; Interrupt Flag: P9IFG5"]
    #[inline(always)]
    pub fn is_p9ifg5(&self) -> bool {
        *self == P9iv::P9ifg5
    }
    #[doc = "Interrupt Source: Port 9.6 interrupt; Interrupt Flag: P9IFG6"]
    #[inline(always)]
    pub fn is_p9ifg6(&self) -> bool {
        *self == P9iv::P9ifg6
    }
    #[doc = "Interrupt Source: Port 9.7 interrupt; Interrupt Flag: P9IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p9ifg7(&self) -> bool {
        *self == P9iv::P9ifg7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 9 interrupt vector value"]
    #[inline(always)]
    pub fn p9iv(&self) -> P9ivR {
        P9ivR::new((self.bits & 0x1f) as u8)
    }
}
impl W {}
#[doc = "Port 9 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p9iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P9ivSpec;
impl crate::RegisterSpec for P9ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p9iv::R`](R) reader structure"]
impl crate::Readable for P9ivSpec {}
#[doc = "`write(|w| ..)` method takes [`p9iv::W`](W) writer structure"]
impl crate::Writable for P9ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P9IV to value 0"]
impl crate::Resettable for P9ivSpec {}
