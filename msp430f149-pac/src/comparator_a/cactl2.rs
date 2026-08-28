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
#[doc = "Field `P2CA0` reader - Comp. A Connect External Signal to CA0 : 1"]
pub type P2ca0R = crate::BitReader;
#[doc = "Field `P2CA0` writer - Comp. A Connect External Signal to CA0 : 1"]
pub type P2ca0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2CA1` reader - Comp. A Connect External Signal to CA1 : 1"]
pub type P2ca1R = crate::BitReader;
#[doc = "Field `P2CA1` writer - Comp. A Connect External Signal to CA1 : 1"]
pub type P2ca1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACTL24` reader - CACTL24"]
pub type Cactl24R = crate::BitReader;
#[doc = "Field `CACTL24` writer - CACTL24"]
pub type Cactl24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACTL25` reader - CACTL25"]
pub type Cactl25R = crate::BitReader;
#[doc = "Field `CACTL25` writer - CACTL25"]
pub type Cactl25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACTL26` reader - CACTL26"]
pub type Cactl26R = crate::BitReader;
#[doc = "Field `CACTL26` writer - CACTL26"]
pub type Cactl26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACTL27` reader - CACTL27"]
pub type Cactl27R = crate::BitReader;
#[doc = "Field `CACTL27` writer - CACTL27"]
pub type Cactl27W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - Comp. A Connect External Signal to CA0 : 1"]
    #[inline(always)]
    pub fn p2ca0(&self) -> P2ca0R {
        P2ca0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comp. A Connect External Signal to CA1 : 1"]
    #[inline(always)]
    pub fn p2ca1(&self) -> P2ca1R {
        P2ca1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CACTL24"]
    #[inline(always)]
    pub fn cactl24(&self) -> Cactl24R {
        Cactl24R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CACTL25"]
    #[inline(always)]
    pub fn cactl25(&self) -> Cactl25R {
        Cactl25R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CACTL26"]
    #[inline(always)]
    pub fn cactl26(&self) -> Cactl26R {
        Cactl26R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CACTL27"]
    #[inline(always)]
    pub fn cactl27(&self) -> Cactl27R {
        Cactl27R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 2 - Comp. A Connect External Signal to CA0 : 1"]
    #[inline(always)]
    pub fn p2ca0(&mut self) -> P2ca0W<'_, Cactl2Spec> {
        P2ca0W::new(self, 2)
    }
    #[doc = "Bit 3 - Comp. A Connect External Signal to CA1 : 1"]
    #[inline(always)]
    pub fn p2ca1(&mut self) -> P2ca1W<'_, Cactl2Spec> {
        P2ca1W::new(self, 3)
    }
    #[doc = "Bit 4 - CACTL24"]
    #[inline(always)]
    pub fn cactl24(&mut self) -> Cactl24W<'_, Cactl2Spec> {
        Cactl24W::new(self, 4)
    }
    #[doc = "Bit 5 - CACTL25"]
    #[inline(always)]
    pub fn cactl25(&mut self) -> Cactl25W<'_, Cactl2Spec> {
        Cactl25W::new(self, 5)
    }
    #[doc = "Bit 6 - CACTL26"]
    #[inline(always)]
    pub fn cactl26(&mut self) -> Cactl26W<'_, Cactl2Spec> {
        Cactl26W::new(self, 6)
    }
    #[doc = "Bit 7 - CACTL27"]
    #[inline(always)]
    pub fn cactl27(&mut self) -> Cactl27W<'_, Cactl2Spec> {
        Cactl27W::new(self, 7)
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
