#[doc = "Register `P5REN` reader"]
pub type R = crate::R<P5renSpec>;
#[doc = "Register `P5REN` writer"]
pub type W = crate::W<P5renSpec>;
#[doc = "Field `P5REN0` reader - P5REN0"]
pub type P5ren0R = crate::BitReader;
#[doc = "Field `P5REN0` writer - P5REN0"]
pub type P5ren0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5REN1` reader - P5REN1"]
pub type P5ren1R = crate::BitReader;
#[doc = "Field `P5REN1` writer - P5REN1"]
pub type P5ren1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5REN2` reader - P5REN2"]
pub type P5ren2R = crate::BitReader;
#[doc = "Field `P5REN2` writer - P5REN2"]
pub type P5ren2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5REN3` reader - P5REN3"]
pub type P5ren3R = crate::BitReader;
#[doc = "Field `P5REN3` writer - P5REN3"]
pub type P5ren3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5REN4` reader - P5REN4"]
pub type P5ren4R = crate::BitReader;
#[doc = "Field `P5REN4` writer - P5REN4"]
pub type P5ren4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5REN5` reader - P5REN5"]
pub type P5ren5R = crate::BitReader;
#[doc = "Field `P5REN5` writer - P5REN5"]
pub type P5ren5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5REN6` reader - P5REN6"]
pub type P5ren6R = crate::BitReader;
#[doc = "Field `P5REN6` writer - P5REN6"]
pub type P5ren6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5REN7` reader - P5REN7"]
pub type P5ren7R = crate::BitReader;
#[doc = "Field `P5REN7` writer - P5REN7"]
pub type P5ren7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P5REN0"]
    #[inline(always)]
    pub fn p5ren0(&self) -> P5ren0R {
        P5ren0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P5REN1"]
    #[inline(always)]
    pub fn p5ren1(&self) -> P5ren1R {
        P5ren1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P5REN2"]
    #[inline(always)]
    pub fn p5ren2(&self) -> P5ren2R {
        P5ren2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P5REN3"]
    #[inline(always)]
    pub fn p5ren3(&self) -> P5ren3R {
        P5ren3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P5REN4"]
    #[inline(always)]
    pub fn p5ren4(&self) -> P5ren4R {
        P5ren4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P5REN5"]
    #[inline(always)]
    pub fn p5ren5(&self) -> P5ren5R {
        P5ren5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P5REN6"]
    #[inline(always)]
    pub fn p5ren6(&self) -> P5ren6R {
        P5ren6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P5REN7"]
    #[inline(always)]
    pub fn p5ren7(&self) -> P5ren7R {
        P5ren7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5REN0"]
    #[inline(always)]
    pub fn p5ren0(&mut self) -> P5ren0W<'_, P5renSpec> {
        P5ren0W::new(self, 0)
    }
    #[doc = "Bit 1 - P5REN1"]
    #[inline(always)]
    pub fn p5ren1(&mut self) -> P5ren1W<'_, P5renSpec> {
        P5ren1W::new(self, 1)
    }
    #[doc = "Bit 2 - P5REN2"]
    #[inline(always)]
    pub fn p5ren2(&mut self) -> P5ren2W<'_, P5renSpec> {
        P5ren2W::new(self, 2)
    }
    #[doc = "Bit 3 - P5REN3"]
    #[inline(always)]
    pub fn p5ren3(&mut self) -> P5ren3W<'_, P5renSpec> {
        P5ren3W::new(self, 3)
    }
    #[doc = "Bit 4 - P5REN4"]
    #[inline(always)]
    pub fn p5ren4(&mut self) -> P5ren4W<'_, P5renSpec> {
        P5ren4W::new(self, 4)
    }
    #[doc = "Bit 5 - P5REN5"]
    #[inline(always)]
    pub fn p5ren5(&mut self) -> P5ren5W<'_, P5renSpec> {
        P5ren5W::new(self, 5)
    }
    #[doc = "Bit 6 - P5REN6"]
    #[inline(always)]
    pub fn p5ren6(&mut self) -> P5ren6W<'_, P5renSpec> {
        P5ren6W::new(self, 6)
    }
    #[doc = "Bit 7 - P5REN7"]
    #[inline(always)]
    pub fn p5ren7(&mut self) -> P5ren7W<'_, P5renSpec> {
        P5ren7W::new(self, 7)
    }
}
#[doc = "Port 5 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p5ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5renSpec;
impl crate::RegisterSpec for P5renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5ren::R`](R) reader structure"]
impl crate::Readable for P5renSpec {}
#[doc = "`write(|w| ..)` method takes [`p5ren::W`](W) writer structure"]
impl crate::Writable for P5renSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P5REN to value 0"]
impl crate::Resettable for P5renSpec {}
