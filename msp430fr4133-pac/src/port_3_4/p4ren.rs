#[doc = "Register `P4REN` reader"]
pub type R = crate::R<P4renSpec>;
#[doc = "Register `P4REN` writer"]
pub type W = crate::W<P4renSpec>;
#[doc = "Field `P4REN0` reader - P4REN0"]
pub type P4ren0R = crate::BitReader;
#[doc = "Field `P4REN0` writer - P4REN0"]
pub type P4ren0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4REN1` reader - P4REN1"]
pub type P4ren1R = crate::BitReader;
#[doc = "Field `P4REN1` writer - P4REN1"]
pub type P4ren1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4REN2` reader - P4REN2"]
pub type P4ren2R = crate::BitReader;
#[doc = "Field `P4REN2` writer - P4REN2"]
pub type P4ren2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4REN3` reader - P4REN3"]
pub type P4ren3R = crate::BitReader;
#[doc = "Field `P4REN3` writer - P4REN3"]
pub type P4ren3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4REN4` reader - P4REN4"]
pub type P4ren4R = crate::BitReader;
#[doc = "Field `P4REN4` writer - P4REN4"]
pub type P4ren4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4REN5` reader - P4REN5"]
pub type P4ren5R = crate::BitReader;
#[doc = "Field `P4REN5` writer - P4REN5"]
pub type P4ren5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4REN6` reader - P4REN6"]
pub type P4ren6R = crate::BitReader;
#[doc = "Field `P4REN6` writer - P4REN6"]
pub type P4ren6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4REN7` reader - P4REN7"]
pub type P4ren7R = crate::BitReader;
#[doc = "Field `P4REN7` writer - P4REN7"]
pub type P4ren7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P4REN0"]
    #[inline(always)]
    pub fn p4ren0(&self) -> P4ren0R {
        P4ren0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P4REN1"]
    #[inline(always)]
    pub fn p4ren1(&self) -> P4ren1R {
        P4ren1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P4REN2"]
    #[inline(always)]
    pub fn p4ren2(&self) -> P4ren2R {
        P4ren2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P4REN3"]
    #[inline(always)]
    pub fn p4ren3(&self) -> P4ren3R {
        P4ren3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P4REN4"]
    #[inline(always)]
    pub fn p4ren4(&self) -> P4ren4R {
        P4ren4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P4REN5"]
    #[inline(always)]
    pub fn p4ren5(&self) -> P4ren5R {
        P4ren5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P4REN6"]
    #[inline(always)]
    pub fn p4ren6(&self) -> P4ren6R {
        P4ren6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P4REN7"]
    #[inline(always)]
    pub fn p4ren7(&self) -> P4ren7R {
        P4ren7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4REN0"]
    #[inline(always)]
    pub fn p4ren0(&mut self) -> P4ren0W<'_, P4renSpec> {
        P4ren0W::new(self, 0)
    }
    #[doc = "Bit 1 - P4REN1"]
    #[inline(always)]
    pub fn p4ren1(&mut self) -> P4ren1W<'_, P4renSpec> {
        P4ren1W::new(self, 1)
    }
    #[doc = "Bit 2 - P4REN2"]
    #[inline(always)]
    pub fn p4ren2(&mut self) -> P4ren2W<'_, P4renSpec> {
        P4ren2W::new(self, 2)
    }
    #[doc = "Bit 3 - P4REN3"]
    #[inline(always)]
    pub fn p4ren3(&mut self) -> P4ren3W<'_, P4renSpec> {
        P4ren3W::new(self, 3)
    }
    #[doc = "Bit 4 - P4REN4"]
    #[inline(always)]
    pub fn p4ren4(&mut self) -> P4ren4W<'_, P4renSpec> {
        P4ren4W::new(self, 4)
    }
    #[doc = "Bit 5 - P4REN5"]
    #[inline(always)]
    pub fn p4ren5(&mut self) -> P4ren5W<'_, P4renSpec> {
        P4ren5W::new(self, 5)
    }
    #[doc = "Bit 6 - P4REN6"]
    #[inline(always)]
    pub fn p4ren6(&mut self) -> P4ren6W<'_, P4renSpec> {
        P4ren6W::new(self, 6)
    }
    #[doc = "Bit 7 - P4REN7"]
    #[inline(always)]
    pub fn p4ren7(&mut self) -> P4ren7W<'_, P4renSpec> {
        P4ren7W::new(self, 7)
    }
}
#[doc = "Port 4 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p4ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4renSpec;
impl crate::RegisterSpec for P4renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4ren::R`](R) reader structure"]
impl crate::Readable for P4renSpec {}
#[doc = "`write(|w| ..)` method takes [`p4ren::W`](W) writer structure"]
impl crate::Writable for P4renSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P4REN to value 0"]
impl crate::Resettable for P4renSpec {}
