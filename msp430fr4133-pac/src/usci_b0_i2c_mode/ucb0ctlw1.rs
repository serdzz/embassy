#[doc = "Register `UCB0CTLW1` reader"]
pub type R = crate::R<Ucb0ctlw1Spec>;
#[doc = "Register `UCB0CTLW1` writer"]
pub type W = crate::W<Ucb0ctlw1Spec>;
#[doc = "USCI Deglitch time Bit: 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucglit {
    #[doc = "0: USCI Deglitch time: 0"]
    Ucglit0 = 0,
    #[doc = "1: USCI Deglitch time: 1"]
    Ucglit1 = 1,
    #[doc = "2: USCI Deglitch time: 2"]
    Ucglit2 = 2,
    #[doc = "3: USCI Deglitch time: 3"]
    Ucglit3 = 3,
}
impl From<Ucglit> for u8 {
    #[inline(always)]
    fn from(variant: Ucglit) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucglit {
    type Ux = u8;
}
impl crate::IsEnum for Ucglit {}
#[doc = "Field `UCGLIT` reader - USCI Deglitch time Bit: 1"]
pub type UcglitR = crate::FieldReader<Ucglit>;
impl UcglitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucglit {
        match self.bits {
            0 => Ucglit::Ucglit0,
            1 => Ucglit::Ucglit1,
            2 => Ucglit::Ucglit2,
            3 => Ucglit::Ucglit3,
            _ => unreachable!(),
        }
    }
    #[doc = "USCI Deglitch time: 0"]
    #[inline(always)]
    pub fn is_ucglit_0(&self) -> bool {
        *self == Ucglit::Ucglit0
    }
    #[doc = "USCI Deglitch time: 1"]
    #[inline(always)]
    pub fn is_ucglit_1(&self) -> bool {
        *self == Ucglit::Ucglit1
    }
    #[doc = "USCI Deglitch time: 2"]
    #[inline(always)]
    pub fn is_ucglit_2(&self) -> bool {
        *self == Ucglit::Ucglit2
    }
    #[doc = "USCI Deglitch time: 3"]
    #[inline(always)]
    pub fn is_ucglit_3(&self) -> bool {
        *self == Ucglit::Ucglit3
    }
}
#[doc = "Field `UCGLIT` writer - USCI Deglitch time Bit: 1"]
pub type UcglitW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucglit, crate::Safe>;
impl<'a, REG> UcglitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USCI Deglitch time: 0"]
    #[inline(always)]
    pub fn ucglit_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucglit::Ucglit0)
    }
    #[doc = "USCI Deglitch time: 1"]
    #[inline(always)]
    pub fn ucglit_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucglit::Ucglit1)
    }
    #[doc = "USCI Deglitch time: 2"]
    #[inline(always)]
    pub fn ucglit_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucglit::Ucglit2)
    }
    #[doc = "USCI Deglitch time: 3"]
    #[inline(always)]
    pub fn ucglit_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucglit::Ucglit3)
    }
}
#[doc = "USCI Automatic Stop condition generation Bit: 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucastp {
    #[doc = "0: USCI Automatic Stop condition generation: 0"]
    Ucastp0 = 0,
    #[doc = "1: USCI Automatic Stop condition generation: 1"]
    Ucastp1 = 1,
    #[doc = "2: USCI Automatic Stop condition generation: 2"]
    Ucastp2 = 2,
    #[doc = "3: USCI Automatic Stop condition generation: 3"]
    Ucastp3 = 3,
}
impl From<Ucastp> for u8 {
    #[inline(always)]
    fn from(variant: Ucastp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucastp {
    type Ux = u8;
}
impl crate::IsEnum for Ucastp {}
#[doc = "Field `UCASTP` reader - USCI Automatic Stop condition generation Bit: 1"]
pub type UcastpR = crate::FieldReader<Ucastp>;
impl UcastpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucastp {
        match self.bits {
            0 => Ucastp::Ucastp0,
            1 => Ucastp::Ucastp1,
            2 => Ucastp::Ucastp2,
            3 => Ucastp::Ucastp3,
            _ => unreachable!(),
        }
    }
    #[doc = "USCI Automatic Stop condition generation: 0"]
    #[inline(always)]
    pub fn is_ucastp_0(&self) -> bool {
        *self == Ucastp::Ucastp0
    }
    #[doc = "USCI Automatic Stop condition generation: 1"]
    #[inline(always)]
    pub fn is_ucastp_1(&self) -> bool {
        *self == Ucastp::Ucastp1
    }
    #[doc = "USCI Automatic Stop condition generation: 2"]
    #[inline(always)]
    pub fn is_ucastp_2(&self) -> bool {
        *self == Ucastp::Ucastp2
    }
    #[doc = "USCI Automatic Stop condition generation: 3"]
    #[inline(always)]
    pub fn is_ucastp_3(&self) -> bool {
        *self == Ucastp::Ucastp3
    }
}
#[doc = "Field `UCASTP` writer - USCI Automatic Stop condition generation Bit: 1"]
pub type UcastpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucastp, crate::Safe>;
impl<'a, REG> UcastpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USCI Automatic Stop condition generation: 0"]
    #[inline(always)]
    pub fn ucastp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucastp::Ucastp0)
    }
    #[doc = "USCI Automatic Stop condition generation: 1"]
    #[inline(always)]
    pub fn ucastp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucastp::Ucastp1)
    }
    #[doc = "USCI Automatic Stop condition generation: 2"]
    #[inline(always)]
    pub fn ucastp_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucastp::Ucastp2)
    }
    #[doc = "USCI Automatic Stop condition generation: 3"]
    #[inline(always)]
    pub fn ucastp_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucastp::Ucastp3)
    }
}
#[doc = "Field `UCSWACK` reader - USCI Software controlled ACK"]
pub type UcswackR = crate::BitReader;
#[doc = "Field `UCSWACK` writer - USCI Software controlled ACK"]
pub type UcswackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTPNACK` reader - USCI Acknowledge Stop last byte"]
pub type UcstpnackR = crate::BitReader;
#[doc = "Field `UCSTPNACK` writer - USCI Acknowledge Stop last byte"]
pub type UcstpnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USCI Clock low timeout Bit: 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucclto {
    #[doc = "0: USCI Clock low timeout: 0"]
    Ucclto0 = 0,
    #[doc = "1: USCI Clock low timeout: 1"]
    Ucclto1 = 1,
    #[doc = "2: USCI Clock low timeout: 2"]
    Ucclto2 = 2,
    #[doc = "3: USCI Clock low timeout: 3"]
    Ucclto3 = 3,
}
impl From<Ucclto> for u8 {
    #[inline(always)]
    fn from(variant: Ucclto) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucclto {
    type Ux = u8;
}
impl crate::IsEnum for Ucclto {}
#[doc = "Field `UCCLTO` reader - USCI Clock low timeout Bit: 1"]
pub type UccltoR = crate::FieldReader<Ucclto>;
impl UccltoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucclto {
        match self.bits {
            0 => Ucclto::Ucclto0,
            1 => Ucclto::Ucclto1,
            2 => Ucclto::Ucclto2,
            3 => Ucclto::Ucclto3,
            _ => unreachable!(),
        }
    }
    #[doc = "USCI Clock low timeout: 0"]
    #[inline(always)]
    pub fn is_ucclto_0(&self) -> bool {
        *self == Ucclto::Ucclto0
    }
    #[doc = "USCI Clock low timeout: 1"]
    #[inline(always)]
    pub fn is_ucclto_1(&self) -> bool {
        *self == Ucclto::Ucclto1
    }
    #[doc = "USCI Clock low timeout: 2"]
    #[inline(always)]
    pub fn is_ucclto_2(&self) -> bool {
        *self == Ucclto::Ucclto2
    }
    #[doc = "USCI Clock low timeout: 3"]
    #[inline(always)]
    pub fn is_ucclto_3(&self) -> bool {
        *self == Ucclto::Ucclto3
    }
}
#[doc = "Field `UCCLTO` writer - USCI Clock low timeout Bit: 1"]
pub type UccltoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucclto, crate::Safe>;
impl<'a, REG> UccltoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USCI Clock low timeout: 0"]
    #[inline(always)]
    pub fn ucclto_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucclto::Ucclto0)
    }
    #[doc = "USCI Clock low timeout: 1"]
    #[inline(always)]
    pub fn ucclto_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucclto::Ucclto1)
    }
    #[doc = "USCI Clock low timeout: 2"]
    #[inline(always)]
    pub fn ucclto_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucclto::Ucclto2)
    }
    #[doc = "USCI Clock low timeout: 3"]
    #[inline(always)]
    pub fn ucclto_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucclto::Ucclto3)
    }
}
#[doc = "Field `UCETXINT` reader - USCI Early UCTXIFG0"]
pub type UcetxintR = crate::BitReader;
#[doc = "Field `UCETXINT` writer - USCI Early UCTXIFG0"]
pub type UcetxintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - USCI Deglitch time Bit: 1"]
    #[inline(always)]
    pub fn ucglit(&self) -> UcglitR {
        UcglitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - USCI Automatic Stop condition generation Bit: 1"]
    #[inline(always)]
    pub fn ucastp(&self) -> UcastpR {
        UcastpR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - USCI Software controlled ACK"]
    #[inline(always)]
    pub fn ucswack(&self) -> UcswackR {
        UcswackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USCI Acknowledge Stop last byte"]
    #[inline(always)]
    pub fn ucstpnack(&self) -> UcstpnackR {
        UcstpnackR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - USCI Clock low timeout Bit: 1"]
    #[inline(always)]
    pub fn ucclto(&self) -> UccltoR {
        UccltoR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - USCI Early UCTXIFG0"]
    #[inline(always)]
    pub fn ucetxint(&self) -> UcetxintR {
        UcetxintR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USCI Deglitch time Bit: 1"]
    #[inline(always)]
    pub fn ucglit(&mut self) -> UcglitW<'_, Ucb0ctlw1Spec> {
        UcglitW::new(self, 0)
    }
    #[doc = "Bits 2:3 - USCI Automatic Stop condition generation Bit: 1"]
    #[inline(always)]
    pub fn ucastp(&mut self) -> UcastpW<'_, Ucb0ctlw1Spec> {
        UcastpW::new(self, 2)
    }
    #[doc = "Bit 4 - USCI Software controlled ACK"]
    #[inline(always)]
    pub fn ucswack(&mut self) -> UcswackW<'_, Ucb0ctlw1Spec> {
        UcswackW::new(self, 4)
    }
    #[doc = "Bit 5 - USCI Acknowledge Stop last byte"]
    #[inline(always)]
    pub fn ucstpnack(&mut self) -> UcstpnackW<'_, Ucb0ctlw1Spec> {
        UcstpnackW::new(self, 5)
    }
    #[doc = "Bits 6:7 - USCI Clock low timeout Bit: 1"]
    #[inline(always)]
    pub fn ucclto(&mut self) -> UccltoW<'_, Ucb0ctlw1Spec> {
        UccltoW::new(self, 6)
    }
    #[doc = "Bit 8 - USCI Early UCTXIFG0"]
    #[inline(always)]
    pub fn ucetxint(&mut self) -> UcetxintW<'_, Ucb0ctlw1Spec> {
        UcetxintW::new(self, 8)
    }
}
#[doc = "USCI B0 Control Word Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctlw1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctlw1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0ctlw1Spec;
impl crate::RegisterSpec for Ucb0ctlw1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0ctlw1::R`](R) reader structure"]
impl crate::Readable for Ucb0ctlw1Spec {}
#[doc = "`write(|w| ..)` method takes [`ucb0ctlw1::W`](W) writer structure"]
impl crate::Writable for Ucb0ctlw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0CTLW1 to value 0"]
impl crate::Resettable for Ucb0ctlw1Spec {}
