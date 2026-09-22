#[doc = "Register `LCDPCTL0` reader"]
pub type R = crate::R<Lcdpctl0Spec>;
#[doc = "Register `LCDPCTL0` writer"]
pub type W = crate::W<Lcdpctl0Spec>;
#[doc = "Field `LCDS0` reader - LCD Segment 0 enable."]
pub type Lcds0R = crate::BitReader;
#[doc = "Field `LCDS0` writer - LCD Segment 0 enable."]
pub type Lcds0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS1` reader - LCD Segment 1 enable."]
pub type Lcds1R = crate::BitReader;
#[doc = "Field `LCDS1` writer - LCD Segment 1 enable."]
pub type Lcds1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS2` reader - LCD Segment 2 enable."]
pub type Lcds2R = crate::BitReader;
#[doc = "Field `LCDS2` writer - LCD Segment 2 enable."]
pub type Lcds2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS3` reader - LCD Segment 3 enable."]
pub type Lcds3R = crate::BitReader;
#[doc = "Field `LCDS3` writer - LCD Segment 3 enable."]
pub type Lcds3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS4` reader - LCD Segment 4 enable."]
pub type Lcds4R = crate::BitReader;
#[doc = "Field `LCDS4` writer - LCD Segment 4 enable."]
pub type Lcds4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS5` reader - LCD Segment 5 enable."]
pub type Lcds5R = crate::BitReader;
#[doc = "Field `LCDS5` writer - LCD Segment 5 enable."]
pub type Lcds5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS6` reader - LCD Segment 6 enable."]
pub type Lcds6R = crate::BitReader;
#[doc = "Field `LCDS6` writer - LCD Segment 6 enable."]
pub type Lcds6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS7` reader - LCD Segment 7 enable."]
pub type Lcds7R = crate::BitReader;
#[doc = "Field `LCDS7` writer - LCD Segment 7 enable."]
pub type Lcds7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS8` reader - LCD Segment 8 enable."]
pub type Lcds8R = crate::BitReader;
#[doc = "Field `LCDS8` writer - LCD Segment 8 enable."]
pub type Lcds8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS9` reader - LCD Segment 9 enable."]
pub type Lcds9R = crate::BitReader;
#[doc = "Field `LCDS9` writer - LCD Segment 9 enable."]
pub type Lcds9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS10` reader - LCD Segment 10 enable."]
pub type Lcds10R = crate::BitReader;
#[doc = "Field `LCDS10` writer - LCD Segment 10 enable."]
pub type Lcds10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS11` reader - LCD Segment 11 enable."]
pub type Lcds11R = crate::BitReader;
#[doc = "Field `LCDS11` writer - LCD Segment 11 enable."]
pub type Lcds11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS12` reader - LCD Segment 12 enable."]
pub type Lcds12R = crate::BitReader;
#[doc = "Field `LCDS12` writer - LCD Segment 12 enable."]
pub type Lcds12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS13` reader - LCD Segment 13 enable."]
pub type Lcds13R = crate::BitReader;
#[doc = "Field `LCDS13` writer - LCD Segment 13 enable."]
pub type Lcds13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS14` reader - LCD Segment 14 enable."]
pub type Lcds14R = crate::BitReader;
#[doc = "Field `LCDS14` writer - LCD Segment 14 enable."]
pub type Lcds14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS15` reader - LCD Segment 15 enable."]
pub type Lcds15R = crate::BitReader;
#[doc = "Field `LCDS15` writer - LCD Segment 15 enable."]
pub type Lcds15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCD Segment 0 enable."]
    #[inline(always)]
    pub fn lcds0(&self) -> Lcds0R {
        Lcds0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD Segment 1 enable."]
    #[inline(always)]
    pub fn lcds1(&self) -> Lcds1R {
        Lcds1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD Segment 2 enable."]
    #[inline(always)]
    pub fn lcds2(&self) -> Lcds2R {
        Lcds2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD Segment 3 enable."]
    #[inline(always)]
    pub fn lcds3(&self) -> Lcds3R {
        Lcds3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD Segment 4 enable."]
    #[inline(always)]
    pub fn lcds4(&self) -> Lcds4R {
        Lcds4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD Segment 5 enable."]
    #[inline(always)]
    pub fn lcds5(&self) -> Lcds5R {
        Lcds5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD Segment 6 enable."]
    #[inline(always)]
    pub fn lcds6(&self) -> Lcds6R {
        Lcds6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD Segment 7 enable."]
    #[inline(always)]
    pub fn lcds7(&self) -> Lcds7R {
        Lcds7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD Segment 8 enable."]
    #[inline(always)]
    pub fn lcds8(&self) -> Lcds8R {
        Lcds8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD Segment 9 enable."]
    #[inline(always)]
    pub fn lcds9(&self) -> Lcds9R {
        Lcds9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD Segment 10 enable."]
    #[inline(always)]
    pub fn lcds10(&self) -> Lcds10R {
        Lcds10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCD Segment 11 enable."]
    #[inline(always)]
    pub fn lcds11(&self) -> Lcds11R {
        Lcds11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LCD Segment 12 enable."]
    #[inline(always)]
    pub fn lcds12(&self) -> Lcds12R {
        Lcds12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCD Segment 13 enable."]
    #[inline(always)]
    pub fn lcds13(&self) -> Lcds13R {
        Lcds13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LCD Segment 14 enable."]
    #[inline(always)]
    pub fn lcds14(&self) -> Lcds14R {
        Lcds14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LCD Segment 15 enable."]
    #[inline(always)]
    pub fn lcds15(&self) -> Lcds15R {
        Lcds15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Segment 0 enable."]
    #[inline(always)]
    pub fn lcds0(&mut self) -> Lcds0W<'_, Lcdpctl0Spec> {
        Lcds0W::new(self, 0)
    }
    #[doc = "Bit 1 - LCD Segment 1 enable."]
    #[inline(always)]
    pub fn lcds1(&mut self) -> Lcds1W<'_, Lcdpctl0Spec> {
        Lcds1W::new(self, 1)
    }
    #[doc = "Bit 2 - LCD Segment 2 enable."]
    #[inline(always)]
    pub fn lcds2(&mut self) -> Lcds2W<'_, Lcdpctl0Spec> {
        Lcds2W::new(self, 2)
    }
    #[doc = "Bit 3 - LCD Segment 3 enable."]
    #[inline(always)]
    pub fn lcds3(&mut self) -> Lcds3W<'_, Lcdpctl0Spec> {
        Lcds3W::new(self, 3)
    }
    #[doc = "Bit 4 - LCD Segment 4 enable."]
    #[inline(always)]
    pub fn lcds4(&mut self) -> Lcds4W<'_, Lcdpctl0Spec> {
        Lcds4W::new(self, 4)
    }
    #[doc = "Bit 5 - LCD Segment 5 enable."]
    #[inline(always)]
    pub fn lcds5(&mut self) -> Lcds5W<'_, Lcdpctl0Spec> {
        Lcds5W::new(self, 5)
    }
    #[doc = "Bit 6 - LCD Segment 6 enable."]
    #[inline(always)]
    pub fn lcds6(&mut self) -> Lcds6W<'_, Lcdpctl0Spec> {
        Lcds6W::new(self, 6)
    }
    #[doc = "Bit 7 - LCD Segment 7 enable."]
    #[inline(always)]
    pub fn lcds7(&mut self) -> Lcds7W<'_, Lcdpctl0Spec> {
        Lcds7W::new(self, 7)
    }
    #[doc = "Bit 8 - LCD Segment 8 enable."]
    #[inline(always)]
    pub fn lcds8(&mut self) -> Lcds8W<'_, Lcdpctl0Spec> {
        Lcds8W::new(self, 8)
    }
    #[doc = "Bit 9 - LCD Segment 9 enable."]
    #[inline(always)]
    pub fn lcds9(&mut self) -> Lcds9W<'_, Lcdpctl0Spec> {
        Lcds9W::new(self, 9)
    }
    #[doc = "Bit 10 - LCD Segment 10 enable."]
    #[inline(always)]
    pub fn lcds10(&mut self) -> Lcds10W<'_, Lcdpctl0Spec> {
        Lcds10W::new(self, 10)
    }
    #[doc = "Bit 11 - LCD Segment 11 enable."]
    #[inline(always)]
    pub fn lcds11(&mut self) -> Lcds11W<'_, Lcdpctl0Spec> {
        Lcds11W::new(self, 11)
    }
    #[doc = "Bit 12 - LCD Segment 12 enable."]
    #[inline(always)]
    pub fn lcds12(&mut self) -> Lcds12W<'_, Lcdpctl0Spec> {
        Lcds12W::new(self, 12)
    }
    #[doc = "Bit 13 - LCD Segment 13 enable."]
    #[inline(always)]
    pub fn lcds13(&mut self) -> Lcds13W<'_, Lcdpctl0Spec> {
        Lcds13W::new(self, 13)
    }
    #[doc = "Bit 14 - LCD Segment 14 enable."]
    #[inline(always)]
    pub fn lcds14(&mut self) -> Lcds14W<'_, Lcdpctl0Spec> {
        Lcds14W::new(self, 14)
    }
    #[doc = "Bit 15 - LCD Segment 15 enable."]
    #[inline(always)]
    pub fn lcds15(&mut self) -> Lcds15W<'_, Lcdpctl0Spec> {
        Lcds15W::new(self, 15)
    }
}
#[doc = "LCD_E Port Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdpctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdpctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdpctl0Spec;
impl crate::RegisterSpec for Lcdpctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdpctl0::R`](R) reader structure"]
impl crate::Readable for Lcdpctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdpctl0::W`](W) writer structure"]
impl crate::Writable for Lcdpctl0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LCDPCTL0 to value 0"]
impl crate::Resettable for Lcdpctl0Spec {}
