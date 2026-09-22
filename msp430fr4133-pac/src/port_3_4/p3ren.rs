#[doc = "Register `P3REN` reader"]
pub type R = crate::R<P3renSpec>;
#[doc = "Register `P3REN` writer"]
pub type W = crate::W<P3renSpec>;
#[doc = "Field `P3REN0` reader - P3REN0"]
pub type P3ren0R = crate::BitReader;
#[doc = "Field `P3REN0` writer - P3REN0"]
pub type P3ren0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3REN1` reader - P3REN1"]
pub type P3ren1R = crate::BitReader;
#[doc = "Field `P3REN1` writer - P3REN1"]
pub type P3ren1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3REN2` reader - P3REN2"]
pub type P3ren2R = crate::BitReader;
#[doc = "Field `P3REN2` writer - P3REN2"]
pub type P3ren2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3REN3` reader - P3REN3"]
pub type P3ren3R = crate::BitReader;
#[doc = "Field `P3REN3` writer - P3REN3"]
pub type P3ren3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3REN4` reader - P3REN4"]
pub type P3ren4R = crate::BitReader;
#[doc = "Field `P3REN4` writer - P3REN4"]
pub type P3ren4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3REN5` reader - P3REN5"]
pub type P3ren5R = crate::BitReader;
#[doc = "Field `P3REN5` writer - P3REN5"]
pub type P3ren5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3REN6` reader - P3REN6"]
pub type P3ren6R = crate::BitReader;
#[doc = "Field `P3REN6` writer - P3REN6"]
pub type P3ren6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3REN7` reader - P3REN7"]
pub type P3ren7R = crate::BitReader;
#[doc = "Field `P3REN7` writer - P3REN7"]
pub type P3ren7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P3REN0"]
    #[inline(always)]
    pub fn p3ren0(&self) -> P3ren0R {
        P3ren0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P3REN1"]
    #[inline(always)]
    pub fn p3ren1(&self) -> P3ren1R {
        P3ren1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P3REN2"]
    #[inline(always)]
    pub fn p3ren2(&self) -> P3ren2R {
        P3ren2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3REN3"]
    #[inline(always)]
    pub fn p3ren3(&self) -> P3ren3R {
        P3ren3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P3REN4"]
    #[inline(always)]
    pub fn p3ren4(&self) -> P3ren4R {
        P3ren4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P3REN5"]
    #[inline(always)]
    pub fn p3ren5(&self) -> P3ren5R {
        P3ren5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P3REN6"]
    #[inline(always)]
    pub fn p3ren6(&self) -> P3ren6R {
        P3ren6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P3REN7"]
    #[inline(always)]
    pub fn p3ren7(&self) -> P3ren7R {
        P3ren7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3REN0"]
    #[inline(always)]
    pub fn p3ren0(&mut self) -> P3ren0W<'_, P3renSpec> {
        P3ren0W::new(self, 0)
    }
    #[doc = "Bit 1 - P3REN1"]
    #[inline(always)]
    pub fn p3ren1(&mut self) -> P3ren1W<'_, P3renSpec> {
        P3ren1W::new(self, 1)
    }
    #[doc = "Bit 2 - P3REN2"]
    #[inline(always)]
    pub fn p3ren2(&mut self) -> P3ren2W<'_, P3renSpec> {
        P3ren2W::new(self, 2)
    }
    #[doc = "Bit 3 - P3REN3"]
    #[inline(always)]
    pub fn p3ren3(&mut self) -> P3ren3W<'_, P3renSpec> {
        P3ren3W::new(self, 3)
    }
    #[doc = "Bit 4 - P3REN4"]
    #[inline(always)]
    pub fn p3ren4(&mut self) -> P3ren4W<'_, P3renSpec> {
        P3ren4W::new(self, 4)
    }
    #[doc = "Bit 5 - P3REN5"]
    #[inline(always)]
    pub fn p3ren5(&mut self) -> P3ren5W<'_, P3renSpec> {
        P3ren5W::new(self, 5)
    }
    #[doc = "Bit 6 - P3REN6"]
    #[inline(always)]
    pub fn p3ren6(&mut self) -> P3ren6W<'_, P3renSpec> {
        P3ren6W::new(self, 6)
    }
    #[doc = "Bit 7 - P3REN7"]
    #[inline(always)]
    pub fn p3ren7(&mut self) -> P3ren7W<'_, P3renSpec> {
        P3ren7W::new(self, 7)
    }
}
#[doc = "Port 3 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p3ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3renSpec;
impl crate::RegisterSpec for P3renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3ren::R`](R) reader structure"]
impl crate::Readable for P3renSpec {}
#[doc = "`write(|w| ..)` method takes [`p3ren::W`](W) writer structure"]
impl crate::Writable for P3renSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P3REN to value 0"]
impl crate::Resettable for P3renSpec {}
