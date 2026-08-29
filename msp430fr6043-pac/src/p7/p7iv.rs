#[doc = "Register `P7IV` reader"]
pub type R = crate::R<P7ivSpec>;
#[doc = "Register `P7IV` writer"]
pub type W = crate::W<P7ivSpec>;
#[doc = "Port 7 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P7iv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Port 7.0 interrupt; Interrupt Flag: P7IFG0; Interrupt Priority: Highest"]
    P7ifg0 = 2,
    #[doc = "4: Interrupt Source: Port 7.1 interrupt; Interrupt Flag: P7IFG1"]
    P7ifg1 = 4,
    #[doc = "6: Interrupt Source: Port 7.2 interrupt; Interrupt Flag: P7IFG2"]
    P7ifg2 = 6,
    #[doc = "8: Interrupt Source: Port 7.3 interrupt; Interrupt Flag: P7IFG3"]
    P7ifg3 = 8,
    #[doc = "10: Interrupt Source: Port 7.4 interrupt; Interrupt Flag: P7IFG4"]
    P7ifg4 = 10,
    #[doc = "12: Interrupt Source: Port 7.5 interrupt; Interrupt Flag: P7IFG5"]
    P7ifg5 = 12,
    #[doc = "14: Interrupt Source: Port 7.6 interrupt; Interrupt Flag: P7IFG6"]
    P7ifg6 = 14,
    #[doc = "16: Interrupt Source: Port 7.7 interrupt; Interrupt Flag: P7IFG7; Interrupt Priority: Lowest"]
    P7ifg7 = 16,
}
impl From<P7iv> for u8 {
    #[inline(always)]
    fn from(variant: P7iv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P7iv {
    type Ux = u8;
}
impl crate::IsEnum for P7iv {}
#[doc = "Field `P7IV` reader - Port 7 interrupt vector value"]
pub type P7ivR = crate::FieldReader<P7iv>;
impl P7ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P7iv> {
        match self.bits {
            0 => Some(P7iv::None),
            2 => Some(P7iv::P7ifg0),
            4 => Some(P7iv::P7ifg1),
            6 => Some(P7iv::P7ifg2),
            8 => Some(P7iv::P7ifg3),
            10 => Some(P7iv::P7ifg4),
            12 => Some(P7iv::P7ifg5),
            14 => Some(P7iv::P7ifg6),
            16 => Some(P7iv::P7ifg7),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P7iv::None
    }
    #[doc = "Interrupt Source: Port 7.0 interrupt; Interrupt Flag: P7IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p7ifg0(&self) -> bool {
        *self == P7iv::P7ifg0
    }
    #[doc = "Interrupt Source: Port 7.1 interrupt; Interrupt Flag: P7IFG1"]
    #[inline(always)]
    pub fn is_p7ifg1(&self) -> bool {
        *self == P7iv::P7ifg1
    }
    #[doc = "Interrupt Source: Port 7.2 interrupt; Interrupt Flag: P7IFG2"]
    #[inline(always)]
    pub fn is_p7ifg2(&self) -> bool {
        *self == P7iv::P7ifg2
    }
    #[doc = "Interrupt Source: Port 7.3 interrupt; Interrupt Flag: P7IFG3"]
    #[inline(always)]
    pub fn is_p7ifg3(&self) -> bool {
        *self == P7iv::P7ifg3
    }
    #[doc = "Interrupt Source: Port 7.4 interrupt; Interrupt Flag: P7IFG4"]
    #[inline(always)]
    pub fn is_p7ifg4(&self) -> bool {
        *self == P7iv::P7ifg4
    }
    #[doc = "Interrupt Source: Port 7.5 interrupt; Interrupt Flag: P7IFG5"]
    #[inline(always)]
    pub fn is_p7ifg5(&self) -> bool {
        *self == P7iv::P7ifg5
    }
    #[doc = "Interrupt Source: Port 7.6 interrupt; Interrupt Flag: P7IFG6"]
    #[inline(always)]
    pub fn is_p7ifg6(&self) -> bool {
        *self == P7iv::P7ifg6
    }
    #[doc = "Interrupt Source: Port 7.7 interrupt; Interrupt Flag: P7IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p7ifg7(&self) -> bool {
        *self == P7iv::P7ifg7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 7 interrupt vector value"]
    #[inline(always)]
    pub fn p7iv(&self) -> P7ivR {
        P7ivR::new((self.bits & 0x1f) as u8)
    }
}
impl W {}
#[doc = "Port 7 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p7iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7ivSpec;
impl crate::RegisterSpec for P7ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p7iv::R`](R) reader structure"]
impl crate::Readable for P7ivSpec {}
#[doc = "`write(|w| ..)` method takes [`p7iv::W`](W) writer structure"]
impl crate::Writable for P7ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P7IV to value 0"]
impl crate::Resettable for P7ivSpec {}
