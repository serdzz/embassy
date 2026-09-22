#[doc = "Register `P8SEL0` reader"]
pub type R = crate::R<P8sel0Spec>;
#[doc = "Register `P8SEL0` writer"]
pub type W = crate::W<P8sel0Spec>;
#[doc = "Field `P8SEL0_0` reader - P8SEL0_0"]
pub type P8sel0_0R = crate::BitReader;
#[doc = "Field `P8SEL0_0` writer - P8SEL0_0"]
pub type P8sel0_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8SEL0_1` reader - P8SEL0_1"]
pub type P8sel0_1R = crate::BitReader;
#[doc = "Field `P8SEL0_1` writer - P8SEL0_1"]
pub type P8sel0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8SEL0_2` reader - P8SEL0_2"]
pub type P8sel0_2R = crate::BitReader;
#[doc = "Field `P8SEL0_2` writer - P8SEL0_2"]
pub type P8sel0_2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8SEL0_3` reader - P8SEL0_3"]
pub type P8sel0_3R = crate::BitReader;
#[doc = "Field `P8SEL0_3` writer - P8SEL0_3"]
pub type P8sel0_3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8SEL0_4` reader - P8SEL0_4"]
pub type P8sel0_4R = crate::BitReader;
#[doc = "Field `P8SEL0_4` writer - P8SEL0_4"]
pub type P8sel0_4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8SEL0_5` reader - P8SEL0_5"]
pub type P8sel0_5R = crate::BitReader;
#[doc = "Field `P8SEL0_5` writer - P8SEL0_5"]
pub type P8sel0_5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8SEL0_6` reader - P8SEL0_6"]
pub type P8sel0_6R = crate::BitReader;
#[doc = "Field `P8SEL0_6` writer - P8SEL0_6"]
pub type P8sel0_6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8SEL0_7` reader - P8SEL0_7"]
pub type P8sel0_7R = crate::BitReader;
#[doc = "Field `P8SEL0_7` writer - P8SEL0_7"]
pub type P8sel0_7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P8SEL0_0"]
    #[inline(always)]
    pub fn p8sel0_0(&self) -> P8sel0_0R {
        P8sel0_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P8SEL0_1"]
    #[inline(always)]
    pub fn p8sel0_1(&self) -> P8sel0_1R {
        P8sel0_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P8SEL0_2"]
    #[inline(always)]
    pub fn p8sel0_2(&self) -> P8sel0_2R {
        P8sel0_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P8SEL0_3"]
    #[inline(always)]
    pub fn p8sel0_3(&self) -> P8sel0_3R {
        P8sel0_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P8SEL0_4"]
    #[inline(always)]
    pub fn p8sel0_4(&self) -> P8sel0_4R {
        P8sel0_4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P8SEL0_5"]
    #[inline(always)]
    pub fn p8sel0_5(&self) -> P8sel0_5R {
        P8sel0_5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P8SEL0_6"]
    #[inline(always)]
    pub fn p8sel0_6(&self) -> P8sel0_6R {
        P8sel0_6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P8SEL0_7"]
    #[inline(always)]
    pub fn p8sel0_7(&self) -> P8sel0_7R {
        P8sel0_7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8SEL0_0"]
    #[inline(always)]
    pub fn p8sel0_0(&mut self) -> P8sel0_0W<'_, P8sel0Spec> {
        P8sel0_0W::new(self, 0)
    }
    #[doc = "Bit 1 - P8SEL0_1"]
    #[inline(always)]
    pub fn p8sel0_1(&mut self) -> P8sel0_1W<'_, P8sel0Spec> {
        P8sel0_1W::new(self, 1)
    }
    #[doc = "Bit 2 - P8SEL0_2"]
    #[inline(always)]
    pub fn p8sel0_2(&mut self) -> P8sel0_2W<'_, P8sel0Spec> {
        P8sel0_2W::new(self, 2)
    }
    #[doc = "Bit 3 - P8SEL0_3"]
    #[inline(always)]
    pub fn p8sel0_3(&mut self) -> P8sel0_3W<'_, P8sel0Spec> {
        P8sel0_3W::new(self, 3)
    }
    #[doc = "Bit 4 - P8SEL0_4"]
    #[inline(always)]
    pub fn p8sel0_4(&mut self) -> P8sel0_4W<'_, P8sel0Spec> {
        P8sel0_4W::new(self, 4)
    }
    #[doc = "Bit 5 - P8SEL0_5"]
    #[inline(always)]
    pub fn p8sel0_5(&mut self) -> P8sel0_5W<'_, P8sel0Spec> {
        P8sel0_5W::new(self, 5)
    }
    #[doc = "Bit 6 - P8SEL0_6"]
    #[inline(always)]
    pub fn p8sel0_6(&mut self) -> P8sel0_6W<'_, P8sel0Spec> {
        P8sel0_6W::new(self, 6)
    }
    #[doc = "Bit 7 - P8SEL0_7"]
    #[inline(always)]
    pub fn p8sel0_7(&mut self) -> P8sel0_7W<'_, P8sel0Spec> {
        P8sel0_7W::new(self, 7)
    }
}
#[doc = "Port 8 Selection 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p8sel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8sel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8sel0Spec;
impl crate::RegisterSpec for P8sel0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8sel0::R`](R) reader structure"]
impl crate::Readable for P8sel0Spec {}
#[doc = "`write(|w| ..)` method takes [`p8sel0::W`](W) writer structure"]
impl crate::Writable for P8sel0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P8SEL0 to value 0"]
impl crate::Resettable for P8sel0Spec {}
