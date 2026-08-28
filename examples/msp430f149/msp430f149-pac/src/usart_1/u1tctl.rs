#[doc = "Register `U1TCTL` reader"]
pub type R = crate::R<U1tctlSpec>;
#[doc = "Register `U1TCTL` writer"]
pub type W = crate::W<U1tctlSpec>;
#[doc = "Field `TXEPT` reader - TX Buffer empty"]
pub type TxeptR = crate::BitReader;
#[doc = "Field `TXEPT` writer - TX Buffer empty"]
pub type TxeptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STC` reader - SPI: STC enable 0:on / 1:off"]
pub type StcR = crate::BitReader;
#[doc = "Field `STC` writer - SPI: STC enable 0:on / 1:off"]
pub type StcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXWAKE` reader - TX Wake up mode"]
pub type TxwakeR = crate::BitReader;
#[doc = "Field `TXWAKE` writer - TX Wake up mode"]
pub type TxwakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URXSE` reader - Receive Start edge select"]
pub type UrxseR = crate::BitReader;
#[doc = "Field `URXSE` writer - Receive Start edge select"]
pub type UrxseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSEL0` reader - Clock Source Select 0"]
pub type Ssel0R = crate::BitReader;
#[doc = "Field `SSEL0` writer - Clock Source Select 0"]
pub type Ssel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSEL1` reader - Clock Source Select 1"]
pub type Ssel1R = crate::BitReader;
#[doc = "Field `SSEL1` writer - Clock Source Select 1"]
pub type Ssel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKPL` reader - Clock Polarity"]
pub type CkplR = crate::BitReader;
#[doc = "Field `CKPL` writer - Clock Polarity"]
pub type CkplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKPH` reader - SPI: Clock Phase"]
pub type CkphR = crate::BitReader;
#[doc = "Field `CKPH` writer - SPI: Clock Phase"]
pub type CkphW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX Buffer empty"]
    #[inline(always)]
    pub fn txept(&self) -> TxeptR {
        TxeptR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI: STC enable 0:on / 1:off"]
    #[inline(always)]
    pub fn stc(&self) -> StcR {
        StcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX Wake up mode"]
    #[inline(always)]
    pub fn txwake(&self) -> TxwakeR {
        TxwakeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive Start edge select"]
    #[inline(always)]
    pub fn urxse(&self) -> UrxseR {
        UrxseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Source Select 0"]
    #[inline(always)]
    pub fn ssel0(&self) -> Ssel0R {
        Ssel0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Source Select 1"]
    #[inline(always)]
    pub fn ssel1(&self) -> Ssel1R {
        Ssel1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock Polarity"]
    #[inline(always)]
    pub fn ckpl(&self) -> CkplR {
        CkplR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI: Clock Phase"]
    #[inline(always)]
    pub fn ckph(&self) -> CkphR {
        CkphR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Buffer empty"]
    #[inline(always)]
    pub fn txept(&mut self) -> TxeptW<'_, U1tctlSpec> {
        TxeptW::new(self, 0)
    }
    #[doc = "Bit 1 - SPI: STC enable 0:on / 1:off"]
    #[inline(always)]
    pub fn stc(&mut self) -> StcW<'_, U1tctlSpec> {
        StcW::new(self, 1)
    }
    #[doc = "Bit 2 - TX Wake up mode"]
    #[inline(always)]
    pub fn txwake(&mut self) -> TxwakeW<'_, U1tctlSpec> {
        TxwakeW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive Start edge select"]
    #[inline(always)]
    pub fn urxse(&mut self) -> UrxseW<'_, U1tctlSpec> {
        UrxseW::new(self, 3)
    }
    #[doc = "Bit 4 - Clock Source Select 0"]
    #[inline(always)]
    pub fn ssel0(&mut self) -> Ssel0W<'_, U1tctlSpec> {
        Ssel0W::new(self, 4)
    }
    #[doc = "Bit 5 - Clock Source Select 1"]
    #[inline(always)]
    pub fn ssel1(&mut self) -> Ssel1W<'_, U1tctlSpec> {
        Ssel1W::new(self, 5)
    }
    #[doc = "Bit 6 - Clock Polarity"]
    #[inline(always)]
    pub fn ckpl(&mut self) -> CkplW<'_, U1tctlSpec> {
        CkplW::new(self, 6)
    }
    #[doc = "Bit 7 - SPI: Clock Phase"]
    #[inline(always)]
    pub fn ckph(&mut self) -> CkphW<'_, U1tctlSpec> {
        CkphW::new(self, 7)
    }
}
#[doc = "USART 1 Transmit Control\n\nYou can [`read`](crate::Reg::read) this register and get [`u1tctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u1tctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U1tctlSpec;
impl crate::RegisterSpec for U1tctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`u1tctl::R`](R) reader structure"]
impl crate::Readable for U1tctlSpec {}
#[doc = "`write(|w| ..)` method takes [`u1tctl::W`](W) writer structure"]
impl crate::Writable for U1tctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets U1TCTL to value 0"]
impl crate::Resettable for U1tctlSpec {}
