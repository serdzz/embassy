#[doc = "Register `LCDCSSEL1` reader"]
pub type R = crate::R<Lcdcssel1Spec>;
#[doc = "Register `LCDCSSEL1` writer"]
pub type W = crate::W<Lcdcssel1Spec>;
#[doc = "Field `LCDCSS16` reader - Selects pin L16 as either common or segment line"]
pub type Lcdcss16R = crate::BitReader;
#[doc = "Field `LCDCSS16` writer - Selects pin L16 as either common or segment line"]
pub type Lcdcss16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS17` reader - Selects pin L17 as either common or segment line"]
pub type Lcdcss17R = crate::BitReader;
#[doc = "Field `LCDCSS17` writer - Selects pin L17 as either common or segment line"]
pub type Lcdcss17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS18` reader - Selects pin L18 as either common or segment line"]
pub type Lcdcss18R = crate::BitReader;
#[doc = "Field `LCDCSS18` writer - Selects pin L18 as either common or segment line"]
pub type Lcdcss18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS19` reader - Selects pin L19 as either common or segment line"]
pub type Lcdcss19R = crate::BitReader;
#[doc = "Field `LCDCSS19` writer - Selects pin L19 as either common or segment line"]
pub type Lcdcss19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS20` reader - Selects pin L20 as either common or segment line"]
pub type Lcdcss20R = crate::BitReader;
#[doc = "Field `LCDCSS20` writer - Selects pin L20 as either common or segment line"]
pub type Lcdcss20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS21` reader - Selects pin L21 as either common or segment line"]
pub type Lcdcss21R = crate::BitReader;
#[doc = "Field `LCDCSS21` writer - Selects pin L21 as either common or segment line"]
pub type Lcdcss21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS22` reader - Selects pin L22 as either common or segment line"]
pub type Lcdcss22R = crate::BitReader;
#[doc = "Field `LCDCSS22` writer - Selects pin L22 as either common or segment line"]
pub type Lcdcss22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS23` reader - Selects pin L23 as either common or segment line"]
pub type Lcdcss23R = crate::BitReader;
#[doc = "Field `LCDCSS23` writer - Selects pin L23 as either common or segment line"]
pub type Lcdcss23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS24` reader - Selects pin L24 as either common or segment line"]
pub type Lcdcss24R = crate::BitReader;
#[doc = "Field `LCDCSS24` writer - Selects pin L24 as either common or segment line"]
pub type Lcdcss24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS25` reader - Selects pin L25 as either common or segment line"]
pub type Lcdcss25R = crate::BitReader;
#[doc = "Field `LCDCSS25` writer - Selects pin L25 as either common or segment line"]
pub type Lcdcss25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS26` reader - Selects pin L26 as either common or segment line"]
pub type Lcdcss26R = crate::BitReader;
#[doc = "Field `LCDCSS26` writer - Selects pin L26 as either common or segment line"]
pub type Lcdcss26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS27` reader - Selects pin L27 as either common or segment line"]
pub type Lcdcss27R = crate::BitReader;
#[doc = "Field `LCDCSS27` writer - Selects pin L27 as either common or segment line"]
pub type Lcdcss27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS28` reader - Selects pin L28 as either common or segment line"]
pub type Lcdcss28R = crate::BitReader;
#[doc = "Field `LCDCSS28` writer - Selects pin L28 as either common or segment line"]
pub type Lcdcss28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS29` reader - Selects pin L29 as either common or segment line"]
pub type Lcdcss29R = crate::BitReader;
#[doc = "Field `LCDCSS29` writer - Selects pin L29 as either common or segment line"]
pub type Lcdcss29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS30` reader - Selects pin L30 as either common or segment line"]
pub type Lcdcss30R = crate::BitReader;
#[doc = "Field `LCDCSS30` writer - Selects pin L30 as either common or segment line"]
pub type Lcdcss30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS31` reader - Selects pin L31 as either common or segment line"]
pub type Lcdcss31R = crate::BitReader;
#[doc = "Field `LCDCSS31` writer - Selects pin L31 as either common or segment line"]
pub type Lcdcss31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Selects pin L16 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss16(&self) -> Lcdcss16R {
        Lcdcss16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects pin L17 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss17(&self) -> Lcdcss17R {
        Lcdcss17R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects pin L18 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss18(&self) -> Lcdcss18R {
        Lcdcss18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects pin L19 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss19(&self) -> Lcdcss19R {
        Lcdcss19R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects pin L20 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss20(&self) -> Lcdcss20R {
        Lcdcss20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects pin L21 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss21(&self) -> Lcdcss21R {
        Lcdcss21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects pin L22 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss22(&self) -> Lcdcss22R {
        Lcdcss22R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects pin L23 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss23(&self) -> Lcdcss23R {
        Lcdcss23R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects pin L24 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss24(&self) -> Lcdcss24R {
        Lcdcss24R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects pin L25 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss25(&self) -> Lcdcss25R {
        Lcdcss25R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects pin L26 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss26(&self) -> Lcdcss26R {
        Lcdcss26R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects pin L27 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss27(&self) -> Lcdcss27R {
        Lcdcss27R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects pin L28 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss28(&self) -> Lcdcss28R {
        Lcdcss28R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selects pin L29 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss29(&self) -> Lcdcss29R {
        Lcdcss29R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Selects pin L30 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss30(&self) -> Lcdcss30R {
        Lcdcss30R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Selects pin L31 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss31(&self) -> Lcdcss31R {
        Lcdcss31R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin L16 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss16(&mut self) -> Lcdcss16W<'_, Lcdcssel1Spec> {
        Lcdcss16W::new(self, 0)
    }
    #[doc = "Bit 1 - Selects pin L17 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss17(&mut self) -> Lcdcss17W<'_, Lcdcssel1Spec> {
        Lcdcss17W::new(self, 1)
    }
    #[doc = "Bit 2 - Selects pin L18 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss18(&mut self) -> Lcdcss18W<'_, Lcdcssel1Spec> {
        Lcdcss18W::new(self, 2)
    }
    #[doc = "Bit 3 - Selects pin L19 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss19(&mut self) -> Lcdcss19W<'_, Lcdcssel1Spec> {
        Lcdcss19W::new(self, 3)
    }
    #[doc = "Bit 4 - Selects pin L20 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss20(&mut self) -> Lcdcss20W<'_, Lcdcssel1Spec> {
        Lcdcss20W::new(self, 4)
    }
    #[doc = "Bit 5 - Selects pin L21 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss21(&mut self) -> Lcdcss21W<'_, Lcdcssel1Spec> {
        Lcdcss21W::new(self, 5)
    }
    #[doc = "Bit 6 - Selects pin L22 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss22(&mut self) -> Lcdcss22W<'_, Lcdcssel1Spec> {
        Lcdcss22W::new(self, 6)
    }
    #[doc = "Bit 7 - Selects pin L23 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss23(&mut self) -> Lcdcss23W<'_, Lcdcssel1Spec> {
        Lcdcss23W::new(self, 7)
    }
    #[doc = "Bit 8 - Selects pin L24 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss24(&mut self) -> Lcdcss24W<'_, Lcdcssel1Spec> {
        Lcdcss24W::new(self, 8)
    }
    #[doc = "Bit 9 - Selects pin L25 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss25(&mut self) -> Lcdcss25W<'_, Lcdcssel1Spec> {
        Lcdcss25W::new(self, 9)
    }
    #[doc = "Bit 10 - Selects pin L26 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss26(&mut self) -> Lcdcss26W<'_, Lcdcssel1Spec> {
        Lcdcss26W::new(self, 10)
    }
    #[doc = "Bit 11 - Selects pin L27 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss27(&mut self) -> Lcdcss27W<'_, Lcdcssel1Spec> {
        Lcdcss27W::new(self, 11)
    }
    #[doc = "Bit 12 - Selects pin L28 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss28(&mut self) -> Lcdcss28W<'_, Lcdcssel1Spec> {
        Lcdcss28W::new(self, 12)
    }
    #[doc = "Bit 13 - Selects pin L29 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss29(&mut self) -> Lcdcss29W<'_, Lcdcssel1Spec> {
        Lcdcss29W::new(self, 13)
    }
    #[doc = "Bit 14 - Selects pin L30 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss30(&mut self) -> Lcdcss30W<'_, Lcdcssel1Spec> {
        Lcdcss30W::new(self, 14)
    }
    #[doc = "Bit 15 - Selects pin L31 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss31(&mut self) -> Lcdcss31W<'_, Lcdcssel1Spec> {
        Lcdcss31W::new(self, 15)
    }
}
#[doc = "LCD_E COM/SEG select register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcssel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcssel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdcssel1Spec;
impl crate::RegisterSpec for Lcdcssel1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdcssel1::R`](R) reader structure"]
impl crate::Readable for Lcdcssel1Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdcssel1::W`](W) writer structure"]
impl crate::Writable for Lcdcssel1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LCDCSSEL1 to value 0"]
impl crate::Resettable for Lcdcssel1Spec {}
