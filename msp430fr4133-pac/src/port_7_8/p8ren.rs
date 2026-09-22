#[doc = "Register `P8REN` reader"]
pub type R = crate::R<P8renSpec>;
#[doc = "Register `P8REN` writer"]
pub type W = crate::W<P8renSpec>;
#[doc = "Field `P8REN0` reader - P8REN0"]
pub type P8ren0R = crate::BitReader;
#[doc = "Field `P8REN0` writer - P8REN0"]
pub type P8ren0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8REN1` reader - P8REN1"]
pub type P8ren1R = crate::BitReader;
#[doc = "Field `P8REN1` writer - P8REN1"]
pub type P8ren1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8REN2` reader - P8REN2"]
pub type P8ren2R = crate::BitReader;
#[doc = "Field `P8REN2` writer - P8REN2"]
pub type P8ren2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8REN3` reader - P8REN3"]
pub type P8ren3R = crate::BitReader;
#[doc = "Field `P8REN3` writer - P8REN3"]
pub type P8ren3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8REN4` reader - P8REN4"]
pub type P8ren4R = crate::BitReader;
#[doc = "Field `P8REN4` writer - P8REN4"]
pub type P8ren4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8REN5` reader - P8REN5"]
pub type P8ren5R = crate::BitReader;
#[doc = "Field `P8REN5` writer - P8REN5"]
pub type P8ren5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8REN6` reader - P8REN6"]
pub type P8ren6R = crate::BitReader;
#[doc = "Field `P8REN6` writer - P8REN6"]
pub type P8ren6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8REN7` reader - P8REN7"]
pub type P8ren7R = crate::BitReader;
#[doc = "Field `P8REN7` writer - P8REN7"]
pub type P8ren7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P8REN0"]
    #[inline(always)]
    pub fn p8ren0(&self) -> P8ren0R {
        P8ren0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P8REN1"]
    #[inline(always)]
    pub fn p8ren1(&self) -> P8ren1R {
        P8ren1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P8REN2"]
    #[inline(always)]
    pub fn p8ren2(&self) -> P8ren2R {
        P8ren2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P8REN3"]
    #[inline(always)]
    pub fn p8ren3(&self) -> P8ren3R {
        P8ren3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P8REN4"]
    #[inline(always)]
    pub fn p8ren4(&self) -> P8ren4R {
        P8ren4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P8REN5"]
    #[inline(always)]
    pub fn p8ren5(&self) -> P8ren5R {
        P8ren5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P8REN6"]
    #[inline(always)]
    pub fn p8ren6(&self) -> P8ren6R {
        P8ren6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P8REN7"]
    #[inline(always)]
    pub fn p8ren7(&self) -> P8ren7R {
        P8ren7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8REN0"]
    #[inline(always)]
    pub fn p8ren0(&mut self) -> P8ren0W<'_, P8renSpec> {
        P8ren0W::new(self, 0)
    }
    #[doc = "Bit 1 - P8REN1"]
    #[inline(always)]
    pub fn p8ren1(&mut self) -> P8ren1W<'_, P8renSpec> {
        P8ren1W::new(self, 1)
    }
    #[doc = "Bit 2 - P8REN2"]
    #[inline(always)]
    pub fn p8ren2(&mut self) -> P8ren2W<'_, P8renSpec> {
        P8ren2W::new(self, 2)
    }
    #[doc = "Bit 3 - P8REN3"]
    #[inline(always)]
    pub fn p8ren3(&mut self) -> P8ren3W<'_, P8renSpec> {
        P8ren3W::new(self, 3)
    }
    #[doc = "Bit 4 - P8REN4"]
    #[inline(always)]
    pub fn p8ren4(&mut self) -> P8ren4W<'_, P8renSpec> {
        P8ren4W::new(self, 4)
    }
    #[doc = "Bit 5 - P8REN5"]
    #[inline(always)]
    pub fn p8ren5(&mut self) -> P8ren5W<'_, P8renSpec> {
        P8ren5W::new(self, 5)
    }
    #[doc = "Bit 6 - P8REN6"]
    #[inline(always)]
    pub fn p8ren6(&mut self) -> P8ren6W<'_, P8renSpec> {
        P8ren6W::new(self, 6)
    }
    #[doc = "Bit 7 - P8REN7"]
    #[inline(always)]
    pub fn p8ren7(&mut self) -> P8ren7W<'_, P8renSpec> {
        P8ren7W::new(self, 7)
    }
}
#[doc = "Port 8 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p8ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8renSpec;
impl crate::RegisterSpec for P8renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8ren::R`](R) reader structure"]
impl crate::Readable for P8renSpec {}
#[doc = "`write(|w| ..)` method takes [`p8ren::W`](W) writer structure"]
impl crate::Writable for P8renSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P8REN to value 0"]
impl crate::Resettable for P8renSpec {}
