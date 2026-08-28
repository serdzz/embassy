#[doc = "Register `CACTL2` reader"]
pub type R = crate::R<Cactl2Spec>;
#[doc = "Register `CACTL2` writer"]
pub type W = crate::W<Cactl2Spec>;
#[doc = "Field `CAOUT` reader - Comp. A Output"]
pub type CaoutR = crate::BitReader;
#[doc = "Field `CAOUT` writer - Comp. A Output"]
pub type CaoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAF` reader - Comp. A Enable Output Filter"]
pub type CafR = crate::BitReader;
#[doc = "Field `CAF` writer - Comp. A Enable Output Filter"]
pub type CafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2CA0` reader - Comp. A +Terminal Multiplexer"]
pub type P2ca0R = crate::BitReader;
#[doc = "Field `P2CA0` writer - Comp. A +Terminal Multiplexer"]
pub type P2ca0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2CA1` reader - Comp. A -Terminal Multiplexer"]
pub type P2ca1R = crate::BitReader;
#[doc = "Field `P2CA1` writer - Comp. A -Terminal Multiplexer"]
pub type P2ca1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2CA2` reader - Comp. A -Terminal Multiplexer"]
pub type P2ca2R = crate::BitReader;
#[doc = "Field `P2CA2` writer - Comp. A -Terminal Multiplexer"]
pub type P2ca2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2CA3` reader - Comp. A -Terminal Multiplexer"]
pub type P2ca3R = crate::BitReader;
#[doc = "Field `P2CA3` writer - Comp. A -Terminal Multiplexer"]
pub type P2ca3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2CA4` reader - Comp. A +Terminal Multiplexer"]
pub type P2ca4R = crate::BitReader;
#[doc = "Field `P2CA4` writer - Comp. A +Terminal Multiplexer"]
pub type P2ca4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CASHORT` reader - Comp. A Short + and - Terminals"]
pub type CashortR = crate::BitReader;
#[doc = "Field `CASHORT` writer - Comp. A Short + and - Terminals"]
pub type CashortW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comp. A Output"]
    #[inline(always)]
    pub fn caout(&self) -> CaoutR {
        CaoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comp. A Enable Output Filter"]
    #[inline(always)]
    pub fn caf(&self) -> CafR {
        CafR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca0(&self) -> P2ca0R {
        P2ca0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca1(&self) -> P2ca1R {
        P2ca1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca2(&self) -> P2ca2R {
        P2ca2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca3(&self) -> P2ca3R {
        P2ca3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca4(&self) -> P2ca4R {
        P2ca4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comp. A Short + and - Terminals"]
    #[inline(always)]
    pub fn cashort(&self) -> CashortR {
        CashortR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. A Output"]
    #[inline(always)]
    pub fn caout(&mut self) -> CaoutW<'_, Cactl2Spec> {
        CaoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Comp. A Enable Output Filter"]
    #[inline(always)]
    pub fn caf(&mut self) -> CafW<'_, Cactl2Spec> {
        CafW::new(self, 1)
    }
    #[doc = "Bit 2 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca0(&mut self) -> P2ca0W<'_, Cactl2Spec> {
        P2ca0W::new(self, 2)
    }
    #[doc = "Bit 3 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca1(&mut self) -> P2ca1W<'_, Cactl2Spec> {
        P2ca1W::new(self, 3)
    }
    #[doc = "Bit 4 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca2(&mut self) -> P2ca2W<'_, Cactl2Spec> {
        P2ca2W::new(self, 4)
    }
    #[doc = "Bit 5 - Comp. A -Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca3(&mut self) -> P2ca3W<'_, Cactl2Spec> {
        P2ca3W::new(self, 5)
    }
    #[doc = "Bit 6 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca4(&mut self) -> P2ca4W<'_, Cactl2Spec> {
        P2ca4W::new(self, 6)
    }
    #[doc = "Bit 7 - Comp. A Short + and - Terminals"]
    #[inline(always)]
    pub fn cashort(&mut self) -> CashortW<'_, Cactl2Spec> {
        CashortW::new(self, 7)
    }
}
#[doc = "Comparator A Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cactl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cactl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cactl2Spec;
impl crate::RegisterSpec for Cactl2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cactl2::R`](R) reader structure"]
impl crate::Readable for Cactl2Spec {}
#[doc = "`write(|w| ..)` method takes [`cactl2::W`](W) writer structure"]
impl crate::Writable for Cactl2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CACTL2 to value 0"]
impl crate::Resettable for Cactl2Spec {}
