#[doc = "Register `LCDPCTL1` reader"]
pub type R = crate::R<Lcdpctl1Spec>;
#[doc = "Register `LCDPCTL1` writer"]
pub type W = crate::W<Lcdpctl1Spec>;
#[doc = "Field `LCDS16` reader - LCD Segment 16 enable."]
pub type Lcds16R = crate::BitReader;
#[doc = "Field `LCDS16` writer - LCD Segment 16 enable."]
pub type Lcds16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS17` reader - LCD Segment 17 enable."]
pub type Lcds17R = crate::BitReader;
#[doc = "Field `LCDS17` writer - LCD Segment 17 enable."]
pub type Lcds17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS18` reader - LCD Segment 18 enable."]
pub type Lcds18R = crate::BitReader;
#[doc = "Field `LCDS18` writer - LCD Segment 18 enable."]
pub type Lcds18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS19` reader - LCD Segment 19 enable."]
pub type Lcds19R = crate::BitReader;
#[doc = "Field `LCDS19` writer - LCD Segment 19 enable."]
pub type Lcds19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS20` reader - LCD Segment 20 enable."]
pub type Lcds20R = crate::BitReader;
#[doc = "Field `LCDS20` writer - LCD Segment 20 enable."]
pub type Lcds20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS21` reader - LCD Segment 21 enable."]
pub type Lcds21R = crate::BitReader;
#[doc = "Field `LCDS21` writer - LCD Segment 21 enable."]
pub type Lcds21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS22` reader - LCD Segment 22 enable."]
pub type Lcds22R = crate::BitReader;
#[doc = "Field `LCDS22` writer - LCD Segment 22 enable."]
pub type Lcds22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS23` reader - LCD Segment 23 enable."]
pub type Lcds23R = crate::BitReader;
#[doc = "Field `LCDS23` writer - LCD Segment 23 enable."]
pub type Lcds23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS24` reader - LCD Segment 24 enable."]
pub type Lcds24R = crate::BitReader;
#[doc = "Field `LCDS24` writer - LCD Segment 24 enable."]
pub type Lcds24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS25` reader - LCD Segment 25 enable."]
pub type Lcds25R = crate::BitReader;
#[doc = "Field `LCDS25` writer - LCD Segment 25 enable."]
pub type Lcds25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS26` reader - LCD Segment 26 enable."]
pub type Lcds26R = crate::BitReader;
#[doc = "Field `LCDS26` writer - LCD Segment 26 enable."]
pub type Lcds26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS27` reader - LCD Segment 27 enable."]
pub type Lcds27R = crate::BitReader;
#[doc = "Field `LCDS27` writer - LCD Segment 27 enable."]
pub type Lcds27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS28` reader - LCD Segment 28 enable."]
pub type Lcds28R = crate::BitReader;
#[doc = "Field `LCDS28` writer - LCD Segment 28 enable."]
pub type Lcds28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS29` reader - LCD Segment 29 enable."]
pub type Lcds29R = crate::BitReader;
#[doc = "Field `LCDS29` writer - LCD Segment 29 enable."]
pub type Lcds29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS30` reader - LCD Segment 30 enable."]
pub type Lcds30R = crate::BitReader;
#[doc = "Field `LCDS30` writer - LCD Segment 30 enable."]
pub type Lcds30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS31` reader - LCD Segment 31 enable."]
pub type Lcds31R = crate::BitReader;
#[doc = "Field `LCDS31` writer - LCD Segment 31 enable."]
pub type Lcds31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCD Segment 16 enable."]
    #[inline(always)]
    pub fn lcds16(&self) -> Lcds16R {
        Lcds16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD Segment 17 enable."]
    #[inline(always)]
    pub fn lcds17(&self) -> Lcds17R {
        Lcds17R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD Segment 18 enable."]
    #[inline(always)]
    pub fn lcds18(&self) -> Lcds18R {
        Lcds18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD Segment 19 enable."]
    #[inline(always)]
    pub fn lcds19(&self) -> Lcds19R {
        Lcds19R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD Segment 20 enable."]
    #[inline(always)]
    pub fn lcds20(&self) -> Lcds20R {
        Lcds20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD Segment 21 enable."]
    #[inline(always)]
    pub fn lcds21(&self) -> Lcds21R {
        Lcds21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD Segment 22 enable."]
    #[inline(always)]
    pub fn lcds22(&self) -> Lcds22R {
        Lcds22R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD Segment 23 enable."]
    #[inline(always)]
    pub fn lcds23(&self) -> Lcds23R {
        Lcds23R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD Segment 24 enable."]
    #[inline(always)]
    pub fn lcds24(&self) -> Lcds24R {
        Lcds24R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD Segment 25 enable."]
    #[inline(always)]
    pub fn lcds25(&self) -> Lcds25R {
        Lcds25R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD Segment 26 enable."]
    #[inline(always)]
    pub fn lcds26(&self) -> Lcds26R {
        Lcds26R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCD Segment 27 enable."]
    #[inline(always)]
    pub fn lcds27(&self) -> Lcds27R {
        Lcds27R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LCD Segment 28 enable."]
    #[inline(always)]
    pub fn lcds28(&self) -> Lcds28R {
        Lcds28R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCD Segment 29 enable."]
    #[inline(always)]
    pub fn lcds29(&self) -> Lcds29R {
        Lcds29R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LCD Segment 30 enable."]
    #[inline(always)]
    pub fn lcds30(&self) -> Lcds30R {
        Lcds30R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LCD Segment 31 enable."]
    #[inline(always)]
    pub fn lcds31(&self) -> Lcds31R {
        Lcds31R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Segment 16 enable."]
    #[inline(always)]
    pub fn lcds16(&mut self) -> Lcds16W<'_, Lcdpctl1Spec> {
        Lcds16W::new(self, 0)
    }
    #[doc = "Bit 1 - LCD Segment 17 enable."]
    #[inline(always)]
    pub fn lcds17(&mut self) -> Lcds17W<'_, Lcdpctl1Spec> {
        Lcds17W::new(self, 1)
    }
    #[doc = "Bit 2 - LCD Segment 18 enable."]
    #[inline(always)]
    pub fn lcds18(&mut self) -> Lcds18W<'_, Lcdpctl1Spec> {
        Lcds18W::new(self, 2)
    }
    #[doc = "Bit 3 - LCD Segment 19 enable."]
    #[inline(always)]
    pub fn lcds19(&mut self) -> Lcds19W<'_, Lcdpctl1Spec> {
        Lcds19W::new(self, 3)
    }
    #[doc = "Bit 4 - LCD Segment 20 enable."]
    #[inline(always)]
    pub fn lcds20(&mut self) -> Lcds20W<'_, Lcdpctl1Spec> {
        Lcds20W::new(self, 4)
    }
    #[doc = "Bit 5 - LCD Segment 21 enable."]
    #[inline(always)]
    pub fn lcds21(&mut self) -> Lcds21W<'_, Lcdpctl1Spec> {
        Lcds21W::new(self, 5)
    }
    #[doc = "Bit 6 - LCD Segment 22 enable."]
    #[inline(always)]
    pub fn lcds22(&mut self) -> Lcds22W<'_, Lcdpctl1Spec> {
        Lcds22W::new(self, 6)
    }
    #[doc = "Bit 7 - LCD Segment 23 enable."]
    #[inline(always)]
    pub fn lcds23(&mut self) -> Lcds23W<'_, Lcdpctl1Spec> {
        Lcds23W::new(self, 7)
    }
    #[doc = "Bit 8 - LCD Segment 24 enable."]
    #[inline(always)]
    pub fn lcds24(&mut self) -> Lcds24W<'_, Lcdpctl1Spec> {
        Lcds24W::new(self, 8)
    }
    #[doc = "Bit 9 - LCD Segment 25 enable."]
    #[inline(always)]
    pub fn lcds25(&mut self) -> Lcds25W<'_, Lcdpctl1Spec> {
        Lcds25W::new(self, 9)
    }
    #[doc = "Bit 10 - LCD Segment 26 enable."]
    #[inline(always)]
    pub fn lcds26(&mut self) -> Lcds26W<'_, Lcdpctl1Spec> {
        Lcds26W::new(self, 10)
    }
    #[doc = "Bit 11 - LCD Segment 27 enable."]
    #[inline(always)]
    pub fn lcds27(&mut self) -> Lcds27W<'_, Lcdpctl1Spec> {
        Lcds27W::new(self, 11)
    }
    #[doc = "Bit 12 - LCD Segment 28 enable."]
    #[inline(always)]
    pub fn lcds28(&mut self) -> Lcds28W<'_, Lcdpctl1Spec> {
        Lcds28W::new(self, 12)
    }
    #[doc = "Bit 13 - LCD Segment 29 enable."]
    #[inline(always)]
    pub fn lcds29(&mut self) -> Lcds29W<'_, Lcdpctl1Spec> {
        Lcds29W::new(self, 13)
    }
    #[doc = "Bit 14 - LCD Segment 30 enable."]
    #[inline(always)]
    pub fn lcds30(&mut self) -> Lcds30W<'_, Lcdpctl1Spec> {
        Lcds30W::new(self, 14)
    }
    #[doc = "Bit 15 - LCD Segment 31 enable."]
    #[inline(always)]
    pub fn lcds31(&mut self) -> Lcds31W<'_, Lcdpctl1Spec> {
        Lcds31W::new(self, 15)
    }
}
#[doc = "LCD_E Port Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdpctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdpctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdpctl1Spec;
impl crate::RegisterSpec for Lcdpctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdpctl1::R`](R) reader structure"]
impl crate::Readable for Lcdpctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdpctl1::W`](W) writer structure"]
impl crate::Writable for Lcdpctl1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LCDPCTL1 to value 0"]
impl crate::Resettable for Lcdpctl1Spec {}
