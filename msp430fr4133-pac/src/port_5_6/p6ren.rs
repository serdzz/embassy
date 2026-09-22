#[doc = "Register `P6REN` reader"]
pub type R = crate::R<P6renSpec>;
#[doc = "Register `P6REN` writer"]
pub type W = crate::W<P6renSpec>;
#[doc = "Field `P6REN0` reader - P6REN0"]
pub type P6ren0R = crate::BitReader;
#[doc = "Field `P6REN0` writer - P6REN0"]
pub type P6ren0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6REN1` reader - P6REN1"]
pub type P6ren1R = crate::BitReader;
#[doc = "Field `P6REN1` writer - P6REN1"]
pub type P6ren1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6REN2` reader - P6REN2"]
pub type P6ren2R = crate::BitReader;
#[doc = "Field `P6REN2` writer - P6REN2"]
pub type P6ren2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6REN3` reader - P6REN3"]
pub type P6ren3R = crate::BitReader;
#[doc = "Field `P6REN3` writer - P6REN3"]
pub type P6ren3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6REN4` reader - P6REN4"]
pub type P6ren4R = crate::BitReader;
#[doc = "Field `P6REN4` writer - P6REN4"]
pub type P6ren4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6REN5` reader - P6REN5"]
pub type P6ren5R = crate::BitReader;
#[doc = "Field `P6REN5` writer - P6REN5"]
pub type P6ren5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6REN6` reader - P6REN6"]
pub type P6ren6R = crate::BitReader;
#[doc = "Field `P6REN6` writer - P6REN6"]
pub type P6ren6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6REN7` reader - P6REN7"]
pub type P6ren7R = crate::BitReader;
#[doc = "Field `P6REN7` writer - P6REN7"]
pub type P6ren7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P6REN0"]
    #[inline(always)]
    pub fn p6ren0(&self) -> P6ren0R {
        P6ren0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P6REN1"]
    #[inline(always)]
    pub fn p6ren1(&self) -> P6ren1R {
        P6ren1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P6REN2"]
    #[inline(always)]
    pub fn p6ren2(&self) -> P6ren2R {
        P6ren2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P6REN3"]
    #[inline(always)]
    pub fn p6ren3(&self) -> P6ren3R {
        P6ren3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P6REN4"]
    #[inline(always)]
    pub fn p6ren4(&self) -> P6ren4R {
        P6ren4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P6REN5"]
    #[inline(always)]
    pub fn p6ren5(&self) -> P6ren5R {
        P6ren5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P6REN6"]
    #[inline(always)]
    pub fn p6ren6(&self) -> P6ren6R {
        P6ren6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P6REN7"]
    #[inline(always)]
    pub fn p6ren7(&self) -> P6ren7R {
        P6ren7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6REN0"]
    #[inline(always)]
    pub fn p6ren0(&mut self) -> P6ren0W<'_, P6renSpec> {
        P6ren0W::new(self, 0)
    }
    #[doc = "Bit 1 - P6REN1"]
    #[inline(always)]
    pub fn p6ren1(&mut self) -> P6ren1W<'_, P6renSpec> {
        P6ren1W::new(self, 1)
    }
    #[doc = "Bit 2 - P6REN2"]
    #[inline(always)]
    pub fn p6ren2(&mut self) -> P6ren2W<'_, P6renSpec> {
        P6ren2W::new(self, 2)
    }
    #[doc = "Bit 3 - P6REN3"]
    #[inline(always)]
    pub fn p6ren3(&mut self) -> P6ren3W<'_, P6renSpec> {
        P6ren3W::new(self, 3)
    }
    #[doc = "Bit 4 - P6REN4"]
    #[inline(always)]
    pub fn p6ren4(&mut self) -> P6ren4W<'_, P6renSpec> {
        P6ren4W::new(self, 4)
    }
    #[doc = "Bit 5 - P6REN5"]
    #[inline(always)]
    pub fn p6ren5(&mut self) -> P6ren5W<'_, P6renSpec> {
        P6ren5W::new(self, 5)
    }
    #[doc = "Bit 6 - P6REN6"]
    #[inline(always)]
    pub fn p6ren6(&mut self) -> P6ren6W<'_, P6renSpec> {
        P6ren6W::new(self, 6)
    }
    #[doc = "Bit 7 - P6REN7"]
    #[inline(always)]
    pub fn p6ren7(&mut self) -> P6ren7W<'_, P6renSpec> {
        P6ren7W::new(self, 7)
    }
}
#[doc = "Port 6 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p6ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6renSpec;
impl crate::RegisterSpec for P6renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6ren::R`](R) reader structure"]
impl crate::Readable for P6renSpec {}
#[doc = "`write(|w| ..)` method takes [`p6ren::W`](W) writer structure"]
impl crate::Writable for P6renSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P6REN to value 0"]
impl crate::Resettable for P6renSpec {}
