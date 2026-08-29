#[doc = "Register `P5IV` reader"]
pub type R = crate::R<P5ivSpec>;
#[doc = "Register `P5IV` writer"]
pub type W = crate::W<P5ivSpec>;
#[doc = "Port 5 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P5iv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Port 5.0 interrupt; Interrupt Flag: P5IFG0; Interrupt Priority: Highest"]
    P5ifg0 = 2,
    #[doc = "4: Interrupt Source: Port 5.1 interrupt; Interrupt Flag: P5IFG1"]
    P5ifg1 = 4,
    #[doc = "6: Interrupt Source: Port 5.2 interrupt; Interrupt Flag: P5IFG2"]
    P5ifg2 = 6,
    #[doc = "8: Interrupt Source: Port 5.3 interrupt; Interrupt Flag: P5IFG3"]
    P5ifg3 = 8,
    #[doc = "10: Interrupt Source: Port 5.4 interrupt; Interrupt Flag: P5IFG4"]
    P5ifg4 = 10,
    #[doc = "12: Interrupt Source: Port 5.5 interrupt; Interrupt Flag: P5IFG5"]
    P5ifg5 = 12,
    #[doc = "14: Interrupt Source: Port 5.6 interrupt; Interrupt Flag: P5IFG6"]
    P5ifg6 = 14,
    #[doc = "16: Interrupt Source: Port 5.7 interrupt; Interrupt Flag: P5IFG7; Interrupt Priority: Lowest"]
    P5ifg7 = 16,
}
impl From<P5iv> for u8 {
    #[inline(always)]
    fn from(variant: P5iv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P5iv {
    type Ux = u8;
}
impl crate::IsEnum for P5iv {}
#[doc = "Field `P5IV` reader - Port 5 interrupt vector value"]
pub type P5ivR = crate::FieldReader<P5iv>;
impl P5ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P5iv> {
        match self.bits {
            0 => Some(P5iv::None),
            2 => Some(P5iv::P5ifg0),
            4 => Some(P5iv::P5ifg1),
            6 => Some(P5iv::P5ifg2),
            8 => Some(P5iv::P5ifg3),
            10 => Some(P5iv::P5ifg4),
            12 => Some(P5iv::P5ifg5),
            14 => Some(P5iv::P5ifg6),
            16 => Some(P5iv::P5ifg7),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P5iv::None
    }
    #[doc = "Interrupt Source: Port 5.0 interrupt; Interrupt Flag: P5IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p5ifg0(&self) -> bool {
        *self == P5iv::P5ifg0
    }
    #[doc = "Interrupt Source: Port 5.1 interrupt; Interrupt Flag: P5IFG1"]
    #[inline(always)]
    pub fn is_p5ifg1(&self) -> bool {
        *self == P5iv::P5ifg1
    }
    #[doc = "Interrupt Source: Port 5.2 interrupt; Interrupt Flag: P5IFG2"]
    #[inline(always)]
    pub fn is_p5ifg2(&self) -> bool {
        *self == P5iv::P5ifg2
    }
    #[doc = "Interrupt Source: Port 5.3 interrupt; Interrupt Flag: P5IFG3"]
    #[inline(always)]
    pub fn is_p5ifg3(&self) -> bool {
        *self == P5iv::P5ifg3
    }
    #[doc = "Interrupt Source: Port 5.4 interrupt; Interrupt Flag: P5IFG4"]
    #[inline(always)]
    pub fn is_p5ifg4(&self) -> bool {
        *self == P5iv::P5ifg4
    }
    #[doc = "Interrupt Source: Port 5.5 interrupt; Interrupt Flag: P5IFG5"]
    #[inline(always)]
    pub fn is_p5ifg5(&self) -> bool {
        *self == P5iv::P5ifg5
    }
    #[doc = "Interrupt Source: Port 5.6 interrupt; Interrupt Flag: P5IFG6"]
    #[inline(always)]
    pub fn is_p5ifg6(&self) -> bool {
        *self == P5iv::P5ifg6
    }
    #[doc = "Interrupt Source: Port 5.7 interrupt; Interrupt Flag: P5IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p5ifg7(&self) -> bool {
        *self == P5iv::P5ifg7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 5 interrupt vector value"]
    #[inline(always)]
    pub fn p5iv(&self) -> P5ivR {
        P5ivR::new((self.bits & 0x1f) as u8)
    }
}
impl W {}
#[doc = "Port 5 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p5iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5ivSpec;
impl crate::RegisterSpec for P5ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p5iv::R`](R) reader structure"]
impl crate::Readable for P5ivSpec {}
#[doc = "`write(|w| ..)` method takes [`p5iv::W`](W) writer structure"]
impl crate::Writable for P5ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P5IV to value 0"]
impl crate::Resettable for P5ivSpec {}
