#[doc = "Register `UCA0CTL1` reader"]
pub type R = crate::R<Uca0ctl1Spec>;
#[doc = "Register `UCA0CTL1` writer"]
pub type W = crate::W<Uca0ctl1Spec>;
#[doc = "Field `UCSWRST` reader - USCI Software Reset"]
pub type UcswrstR = crate::BitReader;
#[doc = "Field `UCSWRST` writer - USCI Software Reset"]
pub type UcswrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXBRK` reader - Send next Data as Break"]
pub type UctxbrkR = crate::BitReader;
#[doc = "Field `UCTXBRK` writer - Send next Data as Break"]
pub type UctxbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXADDR` reader - Send next Data as Address"]
pub type UctxaddrR = crate::BitReader;
#[doc = "Field `UCTXADDR` writer - Send next Data as Address"]
pub type UctxaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCDORM` reader - Dormant (Sleep) Mode"]
pub type UcdormR = crate::BitReader;
#[doc = "Field `UCDORM` writer - Dormant (Sleep) Mode"]
pub type UcdormW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBRKIE` reader - Break interrupt enable"]
pub type UcbrkieR = crate::BitReader;
#[doc = "Field `UCBRKIE` writer - Break interrupt enable"]
pub type UcbrkieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCRXEIE` reader - RX Error interrupt enable"]
pub type UcrxeieR = crate::BitReader;
#[doc = "Field `UCRXEIE` writer - RX Error interrupt enable"]
pub type UcrxeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USCI 0 Clock Source Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucssel {
    #[doc = "0: USCI 0 Clock Source: 0"]
    Ucssel0 = 0,
    #[doc = "1: USCI 0 Clock Source: 1"]
    Ucssel1 = 1,
    #[doc = "2: USCI 0 Clock Source: 2"]
    Ucssel2 = 2,
    #[doc = "3: USCI 0 Clock Source: 3"]
    Ucssel3 = 3,
}
impl From<Ucssel> for u8 {
    #[inline(always)]
    fn from(variant: Ucssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucssel {
    type Ux = u8;
}
impl crate::IsEnum for Ucssel {}
#[doc = "Field `UCSSEL` reader - USCI 0 Clock Source Select 1"]
pub type UcsselR = crate::FieldReader<Ucssel>;
impl UcsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucssel {
        match self.bits {
            0 => Ucssel::Ucssel0,
            1 => Ucssel::Ucssel1,
            2 => Ucssel::Ucssel2,
            3 => Ucssel::Ucssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "USCI 0 Clock Source: 0"]
    #[inline(always)]
    pub fn is_ucssel_0(&self) -> bool {
        *self == Ucssel::Ucssel0
    }
    #[doc = "USCI 0 Clock Source: 1"]
    #[inline(always)]
    pub fn is_ucssel_1(&self) -> bool {
        *self == Ucssel::Ucssel1
    }
    #[doc = "USCI 0 Clock Source: 2"]
    #[inline(always)]
    pub fn is_ucssel_2(&self) -> bool {
        *self == Ucssel::Ucssel2
    }
    #[doc = "USCI 0 Clock Source: 3"]
    #[inline(always)]
    pub fn is_ucssel_3(&self) -> bool {
        *self == Ucssel::Ucssel3
    }
}
#[doc = "Field `UCSSEL` writer - USCI 0 Clock Source Select 1"]
pub type UcsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucssel, crate::Safe>;
impl<'a, REG> UcsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USCI 0 Clock Source: 0"]
    #[inline(always)]
    pub fn ucssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Ucssel0)
    }
    #[doc = "USCI 0 Clock Source: 1"]
    #[inline(always)]
    pub fn ucssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Ucssel1)
    }
    #[doc = "USCI 0 Clock Source: 2"]
    #[inline(always)]
    pub fn ucssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Ucssel2)
    }
    #[doc = "USCI 0 Clock Source: 3"]
    #[inline(always)]
    pub fn ucssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Ucssel3)
    }
}
impl R {
    #[doc = "Bit 0 - USCI Software Reset"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UcswrstR {
        UcswrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Send next Data as Break"]
    #[inline(always)]
    pub fn uctxbrk(&self) -> UctxbrkR {
        UctxbrkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Send next Data as Address"]
    #[inline(always)]
    pub fn uctxaddr(&self) -> UctxaddrR {
        UctxaddrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dormant (Sleep) Mode"]
    #[inline(always)]
    pub fn ucdorm(&self) -> UcdormR {
        UcdormR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break interrupt enable"]
    #[inline(always)]
    pub fn ucbrkie(&self) -> UcbrkieR {
        UcbrkieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX Error interrupt enable"]
    #[inline(always)]
    pub fn ucrxeie(&self) -> UcrxeieR {
        UcrxeieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - USCI 0 Clock Source Select 1"]
    #[inline(always)]
    pub fn ucssel(&self) -> UcsselR {
        UcsselR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Software Reset"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UcswrstW<'_, Uca0ctl1Spec> {
        UcswrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Send next Data as Break"]
    #[inline(always)]
    pub fn uctxbrk(&mut self) -> UctxbrkW<'_, Uca0ctl1Spec> {
        UctxbrkW::new(self, 1)
    }
    #[doc = "Bit 2 - Send next Data as Address"]
    #[inline(always)]
    pub fn uctxaddr(&mut self) -> UctxaddrW<'_, Uca0ctl1Spec> {
        UctxaddrW::new(self, 2)
    }
    #[doc = "Bit 3 - Dormant (Sleep) Mode"]
    #[inline(always)]
    pub fn ucdorm(&mut self) -> UcdormW<'_, Uca0ctl1Spec> {
        UcdormW::new(self, 3)
    }
    #[doc = "Bit 4 - Break interrupt enable"]
    #[inline(always)]
    pub fn ucbrkie(&mut self) -> UcbrkieW<'_, Uca0ctl1Spec> {
        UcbrkieW::new(self, 4)
    }
    #[doc = "Bit 5 - RX Error interrupt enable"]
    #[inline(always)]
    pub fn ucrxeie(&mut self) -> UcrxeieW<'_, Uca0ctl1Spec> {
        UcrxeieW::new(self, 5)
    }
    #[doc = "Bits 6:7 - USCI 0 Clock Source Select 1"]
    #[inline(always)]
    pub fn ucssel(&mut self) -> UcsselW<'_, Uca0ctl1Spec> {
        UcsselW::new(self, 6)
    }
}
#[doc = "USCI A0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0ctl1Spec;
impl crate::RegisterSpec for Uca0ctl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0ctl1::R`](R) reader structure"]
impl crate::Readable for Uca0ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`uca0ctl1::W`](W) writer structure"]
impl crate::Writable for Uca0ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0CTL1 to value 0"]
impl crate::Resettable for Uca0ctl1Spec {}
