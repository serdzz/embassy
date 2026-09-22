#[doc = "Register `P2REN` reader"]
pub type R = crate::R<P2renSpec>;
#[doc = "Register `P2REN` writer"]
pub type W = crate::W<P2renSpec>;
#[doc = "Field `P2REN0` reader - P2REN0"]
pub type P2ren0R = crate::BitReader;
#[doc = "Field `P2REN0` writer - P2REN0"]
pub type P2ren0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2REN1` reader - P2REN1"]
pub type P2ren1R = crate::BitReader;
#[doc = "Field `P2REN1` writer - P2REN1"]
pub type P2ren1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2REN2` reader - P2REN2"]
pub type P2ren2R = crate::BitReader;
#[doc = "Field `P2REN2` writer - P2REN2"]
pub type P2ren2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2REN3` reader - P2REN3"]
pub type P2ren3R = crate::BitReader;
#[doc = "Field `P2REN3` writer - P2REN3"]
pub type P2ren3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2REN4` reader - P2REN4"]
pub type P2ren4R = crate::BitReader;
#[doc = "Field `P2REN4` writer - P2REN4"]
pub type P2ren4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2REN5` reader - P2REN5"]
pub type P2ren5R = crate::BitReader;
#[doc = "Field `P2REN5` writer - P2REN5"]
pub type P2ren5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2REN6` reader - P2REN6"]
pub type P2ren6R = crate::BitReader;
#[doc = "Field `P2REN6` writer - P2REN6"]
pub type P2ren6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2REN7` reader - P2REN7"]
pub type P2ren7R = crate::BitReader;
#[doc = "Field `P2REN7` writer - P2REN7"]
pub type P2ren7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P2REN0"]
    #[inline(always)]
    pub fn p2ren0(&self) -> P2ren0R {
        P2ren0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2REN1"]
    #[inline(always)]
    pub fn p2ren1(&self) -> P2ren1R {
        P2ren1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2REN2"]
    #[inline(always)]
    pub fn p2ren2(&self) -> P2ren2R {
        P2ren2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2REN3"]
    #[inline(always)]
    pub fn p2ren3(&self) -> P2ren3R {
        P2ren3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2REN4"]
    #[inline(always)]
    pub fn p2ren4(&self) -> P2ren4R {
        P2ren4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2REN5"]
    #[inline(always)]
    pub fn p2ren5(&self) -> P2ren5R {
        P2ren5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2REN6"]
    #[inline(always)]
    pub fn p2ren6(&self) -> P2ren6R {
        P2ren6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2REN7"]
    #[inline(always)]
    pub fn p2ren7(&self) -> P2ren7R {
        P2ren7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2REN0"]
    #[inline(always)]
    pub fn p2ren0(&mut self) -> P2ren0W<'_, P2renSpec> {
        P2ren0W::new(self, 0)
    }
    #[doc = "Bit 1 - P2REN1"]
    #[inline(always)]
    pub fn p2ren1(&mut self) -> P2ren1W<'_, P2renSpec> {
        P2ren1W::new(self, 1)
    }
    #[doc = "Bit 2 - P2REN2"]
    #[inline(always)]
    pub fn p2ren2(&mut self) -> P2ren2W<'_, P2renSpec> {
        P2ren2W::new(self, 2)
    }
    #[doc = "Bit 3 - P2REN3"]
    #[inline(always)]
    pub fn p2ren3(&mut self) -> P2ren3W<'_, P2renSpec> {
        P2ren3W::new(self, 3)
    }
    #[doc = "Bit 4 - P2REN4"]
    #[inline(always)]
    pub fn p2ren4(&mut self) -> P2ren4W<'_, P2renSpec> {
        P2ren4W::new(self, 4)
    }
    #[doc = "Bit 5 - P2REN5"]
    #[inline(always)]
    pub fn p2ren5(&mut self) -> P2ren5W<'_, P2renSpec> {
        P2ren5W::new(self, 5)
    }
    #[doc = "Bit 6 - P2REN6"]
    #[inline(always)]
    pub fn p2ren6(&mut self) -> P2ren6W<'_, P2renSpec> {
        P2ren6W::new(self, 6)
    }
    #[doc = "Bit 7 - P2REN7"]
    #[inline(always)]
    pub fn p2ren7(&mut self) -> P2ren7W<'_, P2renSpec> {
        P2ren7W::new(self, 7)
    }
}
#[doc = "Port 2 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2renSpec;
impl crate::RegisterSpec for P2renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2ren::R`](R) reader structure"]
impl crate::Readable for P2renSpec {}
#[doc = "`write(|w| ..)` method takes [`p2ren::W`](W) writer structure"]
impl crate::Writable for P2renSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2REN to value 0"]
impl crate::Resettable for P2renSpec {}
