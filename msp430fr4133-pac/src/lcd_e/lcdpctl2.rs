#[doc = "Register `LCDPCTL2` reader"]
pub type R = crate::R<Lcdpctl2Spec>;
#[doc = "Register `LCDPCTL2` writer"]
pub type W = crate::W<Lcdpctl2Spec>;
#[doc = "Field `LCDS32` reader - LCD Segment 32 enable."]
pub type Lcds32R = crate::BitReader;
#[doc = "Field `LCDS32` writer - LCD Segment 32 enable."]
pub type Lcds32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS33` reader - LCD Segment 33 enable."]
pub type Lcds33R = crate::BitReader;
#[doc = "Field `LCDS33` writer - LCD Segment 33 enable."]
pub type Lcds33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS34` reader - LCD Segment 34 enable."]
pub type Lcds34R = crate::BitReader;
#[doc = "Field `LCDS34` writer - LCD Segment 34 enable."]
pub type Lcds34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS35` reader - LCD Segment 35 enable."]
pub type Lcds35R = crate::BitReader;
#[doc = "Field `LCDS35` writer - LCD Segment 35 enable."]
pub type Lcds35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS36` reader - LCD Segment 36 enable."]
pub type Lcds36R = crate::BitReader;
#[doc = "Field `LCDS36` writer - LCD Segment 36 enable."]
pub type Lcds36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS37` reader - LCD Segment 37 enable."]
pub type Lcds37R = crate::BitReader;
#[doc = "Field `LCDS37` writer - LCD Segment 37 enable."]
pub type Lcds37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS38` reader - LCD Segment 38 enable."]
pub type Lcds38R = crate::BitReader;
#[doc = "Field `LCDS38` writer - LCD Segment 38 enable."]
pub type Lcds38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS39` reader - LCD Segment 39 enable."]
pub type Lcds39R = crate::BitReader;
#[doc = "Field `LCDS39` writer - LCD Segment 39 enable."]
pub type Lcds39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS40` reader - LCD Segment 40 enable."]
pub type Lcds40R = crate::BitReader;
#[doc = "Field `LCDS40` writer - LCD Segment 40 enable."]
pub type Lcds40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS41` reader - LCD Segment 41 enable."]
pub type Lcds41R = crate::BitReader;
#[doc = "Field `LCDS41` writer - LCD Segment 41 enable."]
pub type Lcds41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS42` reader - LCD Segment 42 enable."]
pub type Lcds42R = crate::BitReader;
#[doc = "Field `LCDS42` writer - LCD Segment 42 enable."]
pub type Lcds42W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS43` reader - LCD Segment 43 enable."]
pub type Lcds43R = crate::BitReader;
#[doc = "Field `LCDS43` writer - LCD Segment 43 enable."]
pub type Lcds43W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS44` reader - LCD Segment 44 enable."]
pub type Lcds44R = crate::BitReader;
#[doc = "Field `LCDS44` writer - LCD Segment 44 enable."]
pub type Lcds44W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS45` reader - LCD Segment 45 enable."]
pub type Lcds45R = crate::BitReader;
#[doc = "Field `LCDS45` writer - LCD Segment 45 enable."]
pub type Lcds45W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS46` reader - LCD Segment 46 enable."]
pub type Lcds46R = crate::BitReader;
#[doc = "Field `LCDS46` writer - LCD Segment 46 enable."]
pub type Lcds46W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS47` reader - LCD Segment 47 enable."]
pub type Lcds47R = crate::BitReader;
#[doc = "Field `LCDS47` writer - LCD Segment 47 enable."]
pub type Lcds47W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCD Segment 32 enable."]
    #[inline(always)]
    pub fn lcds32(&self) -> Lcds32R {
        Lcds32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD Segment 33 enable."]
    #[inline(always)]
    pub fn lcds33(&self) -> Lcds33R {
        Lcds33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD Segment 34 enable."]
    #[inline(always)]
    pub fn lcds34(&self) -> Lcds34R {
        Lcds34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD Segment 35 enable."]
    #[inline(always)]
    pub fn lcds35(&self) -> Lcds35R {
        Lcds35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD Segment 36 enable."]
    #[inline(always)]
    pub fn lcds36(&self) -> Lcds36R {
        Lcds36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD Segment 37 enable."]
    #[inline(always)]
    pub fn lcds37(&self) -> Lcds37R {
        Lcds37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD Segment 38 enable."]
    #[inline(always)]
    pub fn lcds38(&self) -> Lcds38R {
        Lcds38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD Segment 39 enable."]
    #[inline(always)]
    pub fn lcds39(&self) -> Lcds39R {
        Lcds39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD Segment 40 enable."]
    #[inline(always)]
    pub fn lcds40(&self) -> Lcds40R {
        Lcds40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD Segment 41 enable."]
    #[inline(always)]
    pub fn lcds41(&self) -> Lcds41R {
        Lcds41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD Segment 42 enable."]
    #[inline(always)]
    pub fn lcds42(&self) -> Lcds42R {
        Lcds42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCD Segment 43 enable."]
    #[inline(always)]
    pub fn lcds43(&self) -> Lcds43R {
        Lcds43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LCD Segment 44 enable."]
    #[inline(always)]
    pub fn lcds44(&self) -> Lcds44R {
        Lcds44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCD Segment 45 enable."]
    #[inline(always)]
    pub fn lcds45(&self) -> Lcds45R {
        Lcds45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LCD Segment 46 enable."]
    #[inline(always)]
    pub fn lcds46(&self) -> Lcds46R {
        Lcds46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LCD Segment 47 enable."]
    #[inline(always)]
    pub fn lcds47(&self) -> Lcds47R {
        Lcds47R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Segment 32 enable."]
    #[inline(always)]
    pub fn lcds32(&mut self) -> Lcds32W<'_, Lcdpctl2Spec> {
        Lcds32W::new(self, 0)
    }
    #[doc = "Bit 1 - LCD Segment 33 enable."]
    #[inline(always)]
    pub fn lcds33(&mut self) -> Lcds33W<'_, Lcdpctl2Spec> {
        Lcds33W::new(self, 1)
    }
    #[doc = "Bit 2 - LCD Segment 34 enable."]
    #[inline(always)]
    pub fn lcds34(&mut self) -> Lcds34W<'_, Lcdpctl2Spec> {
        Lcds34W::new(self, 2)
    }
    #[doc = "Bit 3 - LCD Segment 35 enable."]
    #[inline(always)]
    pub fn lcds35(&mut self) -> Lcds35W<'_, Lcdpctl2Spec> {
        Lcds35W::new(self, 3)
    }
    #[doc = "Bit 4 - LCD Segment 36 enable."]
    #[inline(always)]
    pub fn lcds36(&mut self) -> Lcds36W<'_, Lcdpctl2Spec> {
        Lcds36W::new(self, 4)
    }
    #[doc = "Bit 5 - LCD Segment 37 enable."]
    #[inline(always)]
    pub fn lcds37(&mut self) -> Lcds37W<'_, Lcdpctl2Spec> {
        Lcds37W::new(self, 5)
    }
    #[doc = "Bit 6 - LCD Segment 38 enable."]
    #[inline(always)]
    pub fn lcds38(&mut self) -> Lcds38W<'_, Lcdpctl2Spec> {
        Lcds38W::new(self, 6)
    }
    #[doc = "Bit 7 - LCD Segment 39 enable."]
    #[inline(always)]
    pub fn lcds39(&mut self) -> Lcds39W<'_, Lcdpctl2Spec> {
        Lcds39W::new(self, 7)
    }
    #[doc = "Bit 8 - LCD Segment 40 enable."]
    #[inline(always)]
    pub fn lcds40(&mut self) -> Lcds40W<'_, Lcdpctl2Spec> {
        Lcds40W::new(self, 8)
    }
    #[doc = "Bit 9 - LCD Segment 41 enable."]
    #[inline(always)]
    pub fn lcds41(&mut self) -> Lcds41W<'_, Lcdpctl2Spec> {
        Lcds41W::new(self, 9)
    }
    #[doc = "Bit 10 - LCD Segment 42 enable."]
    #[inline(always)]
    pub fn lcds42(&mut self) -> Lcds42W<'_, Lcdpctl2Spec> {
        Lcds42W::new(self, 10)
    }
    #[doc = "Bit 11 - LCD Segment 43 enable."]
    #[inline(always)]
    pub fn lcds43(&mut self) -> Lcds43W<'_, Lcdpctl2Spec> {
        Lcds43W::new(self, 11)
    }
    #[doc = "Bit 12 - LCD Segment 44 enable."]
    #[inline(always)]
    pub fn lcds44(&mut self) -> Lcds44W<'_, Lcdpctl2Spec> {
        Lcds44W::new(self, 12)
    }
    #[doc = "Bit 13 - LCD Segment 45 enable."]
    #[inline(always)]
    pub fn lcds45(&mut self) -> Lcds45W<'_, Lcdpctl2Spec> {
        Lcds45W::new(self, 13)
    }
    #[doc = "Bit 14 - LCD Segment 46 enable."]
    #[inline(always)]
    pub fn lcds46(&mut self) -> Lcds46W<'_, Lcdpctl2Spec> {
        Lcds46W::new(self, 14)
    }
    #[doc = "Bit 15 - LCD Segment 47 enable."]
    #[inline(always)]
    pub fn lcds47(&mut self) -> Lcds47W<'_, Lcdpctl2Spec> {
        Lcds47W::new(self, 15)
    }
}
#[doc = "LCD_E Port Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdpctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdpctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdpctl2Spec;
impl crate::RegisterSpec for Lcdpctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdpctl2::R`](R) reader structure"]
impl crate::Readable for Lcdpctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdpctl2::W`](W) writer structure"]
impl crate::Writable for Lcdpctl2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LCDPCTL2 to value 0"]
impl crate::Resettable for Lcdpctl2Spec {}
