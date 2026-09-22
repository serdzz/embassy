#[doc = "Register `LCDCSSEL0` reader"]
pub type R = crate::R<Lcdcssel0Spec>;
#[doc = "Register `LCDCSSEL0` writer"]
pub type W = crate::W<Lcdcssel0Spec>;
#[doc = "Field `LCDCSS0` reader - Selects pin L0 as either common or segment line"]
pub type Lcdcss0R = crate::BitReader;
#[doc = "Field `LCDCSS0` writer - Selects pin L0 as either common or segment line"]
pub type Lcdcss0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS1` reader - Selects pin L1 as either common or segment line"]
pub type Lcdcss1R = crate::BitReader;
#[doc = "Field `LCDCSS1` writer - Selects pin L1 as either common or segment line"]
pub type Lcdcss1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS2` reader - Selects pin L2 as either common or segment line"]
pub type Lcdcss2R = crate::BitReader;
#[doc = "Field `LCDCSS2` writer - Selects pin L2 as either common or segment line"]
pub type Lcdcss2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS3` reader - Selects pin L3 as either common or segment line"]
pub type Lcdcss3R = crate::BitReader;
#[doc = "Field `LCDCSS3` writer - Selects pin L3 as either common or segment line"]
pub type Lcdcss3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS4` reader - Selects pin L4 as either common or segment line"]
pub type Lcdcss4R = crate::BitReader;
#[doc = "Field `LCDCSS4` writer - Selects pin L4 as either common or segment line"]
pub type Lcdcss4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS5` reader - Selects pin L5 as either common or segment line"]
pub type Lcdcss5R = crate::BitReader;
#[doc = "Field `LCDCSS5` writer - Selects pin L5 as either common or segment line"]
pub type Lcdcss5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS6` reader - Selects pin L6 as either common or segment line"]
pub type Lcdcss6R = crate::BitReader;
#[doc = "Field `LCDCSS6` writer - Selects pin L6 as either common or segment line"]
pub type Lcdcss6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS7` reader - Selects pin L7 as either common or segment line"]
pub type Lcdcss7R = crate::BitReader;
#[doc = "Field `LCDCSS7` writer - Selects pin L7 as either common or segment line"]
pub type Lcdcss7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS8` reader - Selects pin L8 as either common or segment line"]
pub type Lcdcss8R = crate::BitReader;
#[doc = "Field `LCDCSS8` writer - Selects pin L8 as either common or segment line"]
pub type Lcdcss8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS9` reader - Selects pin L9 as either common or segment line"]
pub type Lcdcss9R = crate::BitReader;
#[doc = "Field `LCDCSS9` writer - Selects pin L9 as either common or segment line"]
pub type Lcdcss9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS10` reader - Selects pin L10 as either common or segment line"]
pub type Lcdcss10R = crate::BitReader;
#[doc = "Field `LCDCSS10` writer - Selects pin L10 as either common or segment line"]
pub type Lcdcss10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS11` reader - Selects pin L11 as either common or segment line"]
pub type Lcdcss11R = crate::BitReader;
#[doc = "Field `LCDCSS11` writer - Selects pin L11 as either common or segment line"]
pub type Lcdcss11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS12` reader - Selects pin L12 as either common or segment line"]
pub type Lcdcss12R = crate::BitReader;
#[doc = "Field `LCDCSS12` writer - Selects pin L12 as either common or segment line"]
pub type Lcdcss12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS13` reader - Selects pin L13 as either common or segment line"]
pub type Lcdcss13R = crate::BitReader;
#[doc = "Field `LCDCSS13` writer - Selects pin L13 as either common or segment line"]
pub type Lcdcss13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS14` reader - Selects pin L14 as either common or segment line"]
pub type Lcdcss14R = crate::BitReader;
#[doc = "Field `LCDCSS14` writer - Selects pin L14 as either common or segment line"]
pub type Lcdcss14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS15` reader - Selects pin L15 as either common or segment line"]
pub type Lcdcss15R = crate::BitReader;
#[doc = "Field `LCDCSS15` writer - Selects pin L15 as either common or segment line"]
pub type Lcdcss15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Selects pin L0 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss0(&self) -> Lcdcss0R {
        Lcdcss0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects pin L1 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss1(&self) -> Lcdcss1R {
        Lcdcss1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects pin L2 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss2(&self) -> Lcdcss2R {
        Lcdcss2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects pin L3 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss3(&self) -> Lcdcss3R {
        Lcdcss3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects pin L4 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss4(&self) -> Lcdcss4R {
        Lcdcss4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects pin L5 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss5(&self) -> Lcdcss5R {
        Lcdcss5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects pin L6 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss6(&self) -> Lcdcss6R {
        Lcdcss6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects pin L7 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss7(&self) -> Lcdcss7R {
        Lcdcss7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects pin L8 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss8(&self) -> Lcdcss8R {
        Lcdcss8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects pin L9 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss9(&self) -> Lcdcss9R {
        Lcdcss9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects pin L10 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss10(&self) -> Lcdcss10R {
        Lcdcss10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects pin L11 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss11(&self) -> Lcdcss11R {
        Lcdcss11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects pin L12 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss12(&self) -> Lcdcss12R {
        Lcdcss12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selects pin L13 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss13(&self) -> Lcdcss13R {
        Lcdcss13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Selects pin L14 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss14(&self) -> Lcdcss14R {
        Lcdcss14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Selects pin L15 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss15(&self) -> Lcdcss15R {
        Lcdcss15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin L0 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss0(&mut self) -> Lcdcss0W<'_, Lcdcssel0Spec> {
        Lcdcss0W::new(self, 0)
    }
    #[doc = "Bit 1 - Selects pin L1 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss1(&mut self) -> Lcdcss1W<'_, Lcdcssel0Spec> {
        Lcdcss1W::new(self, 1)
    }
    #[doc = "Bit 2 - Selects pin L2 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss2(&mut self) -> Lcdcss2W<'_, Lcdcssel0Spec> {
        Lcdcss2W::new(self, 2)
    }
    #[doc = "Bit 3 - Selects pin L3 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss3(&mut self) -> Lcdcss3W<'_, Lcdcssel0Spec> {
        Lcdcss3W::new(self, 3)
    }
    #[doc = "Bit 4 - Selects pin L4 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss4(&mut self) -> Lcdcss4W<'_, Lcdcssel0Spec> {
        Lcdcss4W::new(self, 4)
    }
    #[doc = "Bit 5 - Selects pin L5 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss5(&mut self) -> Lcdcss5W<'_, Lcdcssel0Spec> {
        Lcdcss5W::new(self, 5)
    }
    #[doc = "Bit 6 - Selects pin L6 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss6(&mut self) -> Lcdcss6W<'_, Lcdcssel0Spec> {
        Lcdcss6W::new(self, 6)
    }
    #[doc = "Bit 7 - Selects pin L7 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss7(&mut self) -> Lcdcss7W<'_, Lcdcssel0Spec> {
        Lcdcss7W::new(self, 7)
    }
    #[doc = "Bit 8 - Selects pin L8 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss8(&mut self) -> Lcdcss8W<'_, Lcdcssel0Spec> {
        Lcdcss8W::new(self, 8)
    }
    #[doc = "Bit 9 - Selects pin L9 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss9(&mut self) -> Lcdcss9W<'_, Lcdcssel0Spec> {
        Lcdcss9W::new(self, 9)
    }
    #[doc = "Bit 10 - Selects pin L10 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss10(&mut self) -> Lcdcss10W<'_, Lcdcssel0Spec> {
        Lcdcss10W::new(self, 10)
    }
    #[doc = "Bit 11 - Selects pin L11 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss11(&mut self) -> Lcdcss11W<'_, Lcdcssel0Spec> {
        Lcdcss11W::new(self, 11)
    }
    #[doc = "Bit 12 - Selects pin L12 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss12(&mut self) -> Lcdcss12W<'_, Lcdcssel0Spec> {
        Lcdcss12W::new(self, 12)
    }
    #[doc = "Bit 13 - Selects pin L13 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss13(&mut self) -> Lcdcss13W<'_, Lcdcssel0Spec> {
        Lcdcss13W::new(self, 13)
    }
    #[doc = "Bit 14 - Selects pin L14 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss14(&mut self) -> Lcdcss14W<'_, Lcdcssel0Spec> {
        Lcdcss14W::new(self, 14)
    }
    #[doc = "Bit 15 - Selects pin L15 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss15(&mut self) -> Lcdcss15W<'_, Lcdcssel0Spec> {
        Lcdcss15W::new(self, 15)
    }
}
#[doc = "LCD_E COM/SEG select register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcssel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcssel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdcssel0Spec;
impl crate::RegisterSpec for Lcdcssel0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdcssel0::R`](R) reader structure"]
impl crate::Readable for Lcdcssel0Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdcssel0::W`](W) writer structure"]
impl crate::Writable for Lcdcssel0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LCDCSSEL0 to value 0"]
impl crate::Resettable for Lcdcssel0Spec {}
