#[doc = "Register `P7REN` reader"]
pub type R = crate::R<P7renSpec>;
#[doc = "Register `P7REN` writer"]
pub type W = crate::W<P7renSpec>;
#[doc = "Field `P7REN0` reader - P7REN0"]
pub type P7ren0R = crate::BitReader;
#[doc = "Field `P7REN0` writer - P7REN0"]
pub type P7ren0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7REN1` reader - P7REN1"]
pub type P7ren1R = crate::BitReader;
#[doc = "Field `P7REN1` writer - P7REN1"]
pub type P7ren1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7REN2` reader - P7REN2"]
pub type P7ren2R = crate::BitReader;
#[doc = "Field `P7REN2` writer - P7REN2"]
pub type P7ren2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7REN3` reader - P7REN3"]
pub type P7ren3R = crate::BitReader;
#[doc = "Field `P7REN3` writer - P7REN3"]
pub type P7ren3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7REN4` reader - P7REN4"]
pub type P7ren4R = crate::BitReader;
#[doc = "Field `P7REN4` writer - P7REN4"]
pub type P7ren4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7REN5` reader - P7REN5"]
pub type P7ren5R = crate::BitReader;
#[doc = "Field `P7REN5` writer - P7REN5"]
pub type P7ren5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7REN6` reader - P7REN6"]
pub type P7ren6R = crate::BitReader;
#[doc = "Field `P7REN6` writer - P7REN6"]
pub type P7ren6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7REN7` reader - P7REN7"]
pub type P7ren7R = crate::BitReader;
#[doc = "Field `P7REN7` writer - P7REN7"]
pub type P7ren7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P7REN0"]
    #[inline(always)]
    pub fn p7ren0(&self) -> P7ren0R {
        P7ren0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P7REN1"]
    #[inline(always)]
    pub fn p7ren1(&self) -> P7ren1R {
        P7ren1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P7REN2"]
    #[inline(always)]
    pub fn p7ren2(&self) -> P7ren2R {
        P7ren2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P7REN3"]
    #[inline(always)]
    pub fn p7ren3(&self) -> P7ren3R {
        P7ren3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P7REN4"]
    #[inline(always)]
    pub fn p7ren4(&self) -> P7ren4R {
        P7ren4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P7REN5"]
    #[inline(always)]
    pub fn p7ren5(&self) -> P7ren5R {
        P7ren5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P7REN6"]
    #[inline(always)]
    pub fn p7ren6(&self) -> P7ren6R {
        P7ren6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P7REN7"]
    #[inline(always)]
    pub fn p7ren7(&self) -> P7ren7R {
        P7ren7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7REN0"]
    #[inline(always)]
    pub fn p7ren0(&mut self) -> P7ren0W<'_, P7renSpec> {
        P7ren0W::new(self, 0)
    }
    #[doc = "Bit 1 - P7REN1"]
    #[inline(always)]
    pub fn p7ren1(&mut self) -> P7ren1W<'_, P7renSpec> {
        P7ren1W::new(self, 1)
    }
    #[doc = "Bit 2 - P7REN2"]
    #[inline(always)]
    pub fn p7ren2(&mut self) -> P7ren2W<'_, P7renSpec> {
        P7ren2W::new(self, 2)
    }
    #[doc = "Bit 3 - P7REN3"]
    #[inline(always)]
    pub fn p7ren3(&mut self) -> P7ren3W<'_, P7renSpec> {
        P7ren3W::new(self, 3)
    }
    #[doc = "Bit 4 - P7REN4"]
    #[inline(always)]
    pub fn p7ren4(&mut self) -> P7ren4W<'_, P7renSpec> {
        P7ren4W::new(self, 4)
    }
    #[doc = "Bit 5 - P7REN5"]
    #[inline(always)]
    pub fn p7ren5(&mut self) -> P7ren5W<'_, P7renSpec> {
        P7ren5W::new(self, 5)
    }
    #[doc = "Bit 6 - P7REN6"]
    #[inline(always)]
    pub fn p7ren6(&mut self) -> P7ren6W<'_, P7renSpec> {
        P7ren6W::new(self, 6)
    }
    #[doc = "Bit 7 - P7REN7"]
    #[inline(always)]
    pub fn p7ren7(&mut self) -> P7ren7W<'_, P7renSpec> {
        P7ren7W::new(self, 7)
    }
}
#[doc = "Port 7 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p7ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7renSpec;
impl crate::RegisterSpec for P7renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7ren::R`](R) reader structure"]
impl crate::Readable for P7renSpec {}
#[doc = "`write(|w| ..)` method takes [`p7ren::W`](W) writer structure"]
impl crate::Writable for P7renSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P7REN to value 0"]
impl crate::Resettable for P7renSpec {}
