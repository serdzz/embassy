#[doc = "Register `U0RCTL` reader"]
pub type R = crate::R<U0rctlSpec>;
#[doc = "Register `U0RCTL` writer"]
pub type W = crate::W<U0rctlSpec>;
#[doc = "Field `RXERR` reader - RX Error Error"]
pub type RxerrR = crate::BitReader;
#[doc = "Field `RXERR` writer - RX Error Error"]
pub type RxerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXWAKE` reader - RX Wake up detect"]
pub type RxwakeR = crate::BitReader;
#[doc = "Field `RXWAKE` writer - RX Wake up detect"]
pub type RxwakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URXWIE` reader - RX Wake up interrupt enable"]
pub type UrxwieR = crate::BitReader;
#[doc = "Field `URXWIE` writer - RX Wake up interrupt enable"]
pub type UrxwieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URXEIE` reader - RX Error interrupt enable"]
pub type UrxeieR = crate::BitReader;
#[doc = "Field `URXEIE` writer - RX Error interrupt enable"]
pub type UrxeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK` reader - Break detected"]
pub type BrkR = crate::BitReader;
#[doc = "Field `BRK` writer - Break detected"]
pub type BrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE` reader - Overrun Error"]
pub type OeR = crate::BitReader;
#[doc = "Field `OE` writer - Overrun Error"]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` reader - Parity Error"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - Parity Error"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FE` reader - Frame Error"]
pub type FeR = crate::BitReader;
#[doc = "Field `FE` writer - Frame Error"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX Error Error"]
    #[inline(always)]
    pub fn rxerr(&self) -> RxerrR {
        RxerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX Wake up detect"]
    #[inline(always)]
    pub fn rxwake(&self) -> RxwakeR {
        RxwakeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Wake up interrupt enable"]
    #[inline(always)]
    pub fn urxwie(&self) -> UrxwieR {
        UrxwieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX Error interrupt enable"]
    #[inline(always)]
    pub fn urxeie(&self) -> UrxeieR {
        UrxeieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break detected"]
    #[inline(always)]
    pub fn brk(&self) -> BrkR {
        BrkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Parity Error"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame Error"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX Error Error"]
    #[inline(always)]
    pub fn rxerr(&mut self) -> RxerrW<'_, U0rctlSpec> {
        RxerrW::new(self, 0)
    }
    #[doc = "Bit 1 - RX Wake up detect"]
    #[inline(always)]
    pub fn rxwake(&mut self) -> RxwakeW<'_, U0rctlSpec> {
        RxwakeW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Wake up interrupt enable"]
    #[inline(always)]
    pub fn urxwie(&mut self) -> UrxwieW<'_, U0rctlSpec> {
        UrxwieW::new(self, 2)
    }
    #[doc = "Bit 3 - RX Error interrupt enable"]
    #[inline(always)]
    pub fn urxeie(&mut self) -> UrxeieW<'_, U0rctlSpec> {
        UrxeieW::new(self, 3)
    }
    #[doc = "Bit 4 - Break detected"]
    #[inline(always)]
    pub fn brk(&mut self) -> BrkW<'_, U0rctlSpec> {
        BrkW::new(self, 4)
    }
    #[doc = "Bit 5 - Overrun Error"]
    #[inline(always)]
    pub fn oe(&mut self) -> OeW<'_, U0rctlSpec> {
        OeW::new(self, 5)
    }
    #[doc = "Bit 6 - Parity Error"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, U0rctlSpec> {
        PeW::new(self, 6)
    }
    #[doc = "Bit 7 - Frame Error"]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<'_, U0rctlSpec> {
        FeW::new(self, 7)
    }
}
#[doc = "USART 0 Receive Control\n\nYou can [`read`](crate::Reg::read) this register and get [`u0rctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u0rctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U0rctlSpec;
impl crate::RegisterSpec for U0rctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`u0rctl::R`](R) reader structure"]
impl crate::Readable for U0rctlSpec {}
#[doc = "`write(|w| ..)` method takes [`u0rctl::W`](W) writer structure"]
impl crate::Writable for U0rctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets U0RCTL to value 0"]
impl crate::Resettable for U0rctlSpec {}
