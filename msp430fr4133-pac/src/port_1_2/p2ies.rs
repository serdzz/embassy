#[doc = "Register `P2IES` reader"]
pub type R = crate::R<P2iesSpec>;
#[doc = "Register `P2IES` writer"]
pub type W = crate::W<P2iesSpec>;
#[doc = "Field `P2IES0` reader - P2IES0"]
pub type P2ies0R = crate::BitReader;
#[doc = "Field `P2IES0` writer - P2IES0"]
pub type P2ies0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IES1` reader - P2IES1"]
pub type P2ies1R = crate::BitReader;
#[doc = "Field `P2IES1` writer - P2IES1"]
pub type P2ies1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IES2` reader - P2IES2"]
pub type P2ies2R = crate::BitReader;
#[doc = "Field `P2IES2` writer - P2IES2"]
pub type P2ies2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IES3` reader - P2IES3"]
pub type P2ies3R = crate::BitReader;
#[doc = "Field `P2IES3` writer - P2IES3"]
pub type P2ies3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IES4` reader - P2IES4"]
pub type P2ies4R = crate::BitReader;
#[doc = "Field `P2IES4` writer - P2IES4"]
pub type P2ies4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IES5` reader - P2IES5"]
pub type P2ies5R = crate::BitReader;
#[doc = "Field `P2IES5` writer - P2IES5"]
pub type P2ies5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IES6` reader - P2IES6"]
pub type P2ies6R = crate::BitReader;
#[doc = "Field `P2IES6` writer - P2IES6"]
pub type P2ies6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IES7` reader - P2IES7"]
pub type P2ies7R = crate::BitReader;
#[doc = "Field `P2IES7` writer - P2IES7"]
pub type P2ies7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P2IES0"]
    #[inline(always)]
    pub fn p2ies0(&self) -> P2ies0R {
        P2ies0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2IES1"]
    #[inline(always)]
    pub fn p2ies1(&self) -> P2ies1R {
        P2ies1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2IES2"]
    #[inline(always)]
    pub fn p2ies2(&self) -> P2ies2R {
        P2ies2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2IES3"]
    #[inline(always)]
    pub fn p2ies3(&self) -> P2ies3R {
        P2ies3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2IES4"]
    #[inline(always)]
    pub fn p2ies4(&self) -> P2ies4R {
        P2ies4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2IES5"]
    #[inline(always)]
    pub fn p2ies5(&self) -> P2ies5R {
        P2ies5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2IES6"]
    #[inline(always)]
    pub fn p2ies6(&self) -> P2ies6R {
        P2ies6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2IES7"]
    #[inline(always)]
    pub fn p2ies7(&self) -> P2ies7R {
        P2ies7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2IES0"]
    #[inline(always)]
    pub fn p2ies0(&mut self) -> P2ies0W<'_, P2iesSpec> {
        P2ies0W::new(self, 0)
    }
    #[doc = "Bit 1 - P2IES1"]
    #[inline(always)]
    pub fn p2ies1(&mut self) -> P2ies1W<'_, P2iesSpec> {
        P2ies1W::new(self, 1)
    }
    #[doc = "Bit 2 - P2IES2"]
    #[inline(always)]
    pub fn p2ies2(&mut self) -> P2ies2W<'_, P2iesSpec> {
        P2ies2W::new(self, 2)
    }
    #[doc = "Bit 3 - P2IES3"]
    #[inline(always)]
    pub fn p2ies3(&mut self) -> P2ies3W<'_, P2iesSpec> {
        P2ies3W::new(self, 3)
    }
    #[doc = "Bit 4 - P2IES4"]
    #[inline(always)]
    pub fn p2ies4(&mut self) -> P2ies4W<'_, P2iesSpec> {
        P2ies4W::new(self, 4)
    }
    #[doc = "Bit 5 - P2IES5"]
    #[inline(always)]
    pub fn p2ies5(&mut self) -> P2ies5W<'_, P2iesSpec> {
        P2ies5W::new(self, 5)
    }
    #[doc = "Bit 6 - P2IES6"]
    #[inline(always)]
    pub fn p2ies6(&mut self) -> P2ies6W<'_, P2iesSpec> {
        P2ies6W::new(self, 6)
    }
    #[doc = "Bit 7 - P2IES7"]
    #[inline(always)]
    pub fn p2ies7(&mut self) -> P2ies7W<'_, P2iesSpec> {
        P2ies7W::new(self, 7)
    }
}
#[doc = "Port 2 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2iesSpec;
impl crate::RegisterSpec for P2iesSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2ies::R`](R) reader structure"]
impl crate::Readable for P2iesSpec {}
#[doc = "`write(|w| ..)` method takes [`p2ies::W`](W) writer structure"]
impl crate::Writable for P2iesSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2IES to value 0"]
impl crate::Resettable for P2iesSpec {}
