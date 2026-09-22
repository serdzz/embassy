#[doc = "Register `LCDCSSEL2` reader"]
pub type R = crate::R<Lcdcssel2Spec>;
#[doc = "Register `LCDCSSEL2` writer"]
pub type W = crate::W<Lcdcssel2Spec>;
#[doc = "Field `LCDCSS32` reader - Selects pin L32 as either common or segment line"]
pub type Lcdcss32R = crate::BitReader;
#[doc = "Field `LCDCSS32` writer - Selects pin L32 as either common or segment line"]
pub type Lcdcss32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS33` reader - Selects pin L33 as either common or segment line"]
pub type Lcdcss33R = crate::BitReader;
#[doc = "Field `LCDCSS33` writer - Selects pin L33 as either common or segment line"]
pub type Lcdcss33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS34` reader - Selects pin L34 as either common or segment line"]
pub type Lcdcss34R = crate::BitReader;
#[doc = "Field `LCDCSS34` writer - Selects pin L34 as either common or segment line"]
pub type Lcdcss34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS35` reader - Selects pin L35 as either common or segment line"]
pub type Lcdcss35R = crate::BitReader;
#[doc = "Field `LCDCSS35` writer - Selects pin L35 as either common or segment line"]
pub type Lcdcss35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS36` reader - Selects pin L36 as either common or segment line"]
pub type Lcdcss36R = crate::BitReader;
#[doc = "Field `LCDCSS36` writer - Selects pin L36 as either common or segment line"]
pub type Lcdcss36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS37` reader - Selects pin L37 as either common or segment line"]
pub type Lcdcss37R = crate::BitReader;
#[doc = "Field `LCDCSS37` writer - Selects pin L37 as either common or segment line"]
pub type Lcdcss37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS38` reader - Selects pin L38 as either common or segment line"]
pub type Lcdcss38R = crate::BitReader;
#[doc = "Field `LCDCSS38` writer - Selects pin L38 as either common or segment line"]
pub type Lcdcss38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS39` reader - Selects pin L39 as either common or segment line"]
pub type Lcdcss39R = crate::BitReader;
#[doc = "Field `LCDCSS39` writer - Selects pin L39 as either common or segment line"]
pub type Lcdcss39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS40` reader - Selects pin L40 as either common or segment line"]
pub type Lcdcss40R = crate::BitReader;
#[doc = "Field `LCDCSS40` writer - Selects pin L40 as either common or segment line"]
pub type Lcdcss40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS41` reader - Selects pin L41 as either common or segment line"]
pub type Lcdcss41R = crate::BitReader;
#[doc = "Field `LCDCSS41` writer - Selects pin L41 as either common or segment line"]
pub type Lcdcss41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS42` reader - Selects pin L42 as either common or segment line"]
pub type Lcdcss42R = crate::BitReader;
#[doc = "Field `LCDCSS42` writer - Selects pin L42 as either common or segment line"]
pub type Lcdcss42W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS43` reader - Selects pin L43 as either common or segment line"]
pub type Lcdcss43R = crate::BitReader;
#[doc = "Field `LCDCSS43` writer - Selects pin L43 as either common or segment line"]
pub type Lcdcss43W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS44` reader - Selects pin L44 as either common or segment line"]
pub type Lcdcss44R = crate::BitReader;
#[doc = "Field `LCDCSS44` writer - Selects pin L44 as either common or segment line"]
pub type Lcdcss44W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS45` reader - Selects pin L45 as either common or segment line"]
pub type Lcdcss45R = crate::BitReader;
#[doc = "Field `LCDCSS45` writer - Selects pin L45 as either common or segment line"]
pub type Lcdcss45W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS46` reader - Selects pin L46 as either common or segment line"]
pub type Lcdcss46R = crate::BitReader;
#[doc = "Field `LCDCSS46` writer - Selects pin L46 as either common or segment line"]
pub type Lcdcss46W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCSS47` reader - Selects pin L47 as either common or segment line"]
pub type Lcdcss47R = crate::BitReader;
#[doc = "Field `LCDCSS47` writer - Selects pin L47 as either common or segment line"]
pub type Lcdcss47W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Selects pin L32 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss32(&self) -> Lcdcss32R {
        Lcdcss32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects pin L33 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss33(&self) -> Lcdcss33R {
        Lcdcss33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects pin L34 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss34(&self) -> Lcdcss34R {
        Lcdcss34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects pin L35 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss35(&self) -> Lcdcss35R {
        Lcdcss35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects pin L36 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss36(&self) -> Lcdcss36R {
        Lcdcss36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects pin L37 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss37(&self) -> Lcdcss37R {
        Lcdcss37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects pin L38 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss38(&self) -> Lcdcss38R {
        Lcdcss38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects pin L39 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss39(&self) -> Lcdcss39R {
        Lcdcss39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects pin L40 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss40(&self) -> Lcdcss40R {
        Lcdcss40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects pin L41 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss41(&self) -> Lcdcss41R {
        Lcdcss41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects pin L42 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss42(&self) -> Lcdcss42R {
        Lcdcss42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects pin L43 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss43(&self) -> Lcdcss43R {
        Lcdcss43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects pin L44 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss44(&self) -> Lcdcss44R {
        Lcdcss44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selects pin L45 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss45(&self) -> Lcdcss45R {
        Lcdcss45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Selects pin L46 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss46(&self) -> Lcdcss46R {
        Lcdcss46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Selects pin L47 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss47(&self) -> Lcdcss47R {
        Lcdcss47R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin L32 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss32(&mut self) -> Lcdcss32W<'_, Lcdcssel2Spec> {
        Lcdcss32W::new(self, 0)
    }
    #[doc = "Bit 1 - Selects pin L33 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss33(&mut self) -> Lcdcss33W<'_, Lcdcssel2Spec> {
        Lcdcss33W::new(self, 1)
    }
    #[doc = "Bit 2 - Selects pin L34 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss34(&mut self) -> Lcdcss34W<'_, Lcdcssel2Spec> {
        Lcdcss34W::new(self, 2)
    }
    #[doc = "Bit 3 - Selects pin L35 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss35(&mut self) -> Lcdcss35W<'_, Lcdcssel2Spec> {
        Lcdcss35W::new(self, 3)
    }
    #[doc = "Bit 4 - Selects pin L36 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss36(&mut self) -> Lcdcss36W<'_, Lcdcssel2Spec> {
        Lcdcss36W::new(self, 4)
    }
    #[doc = "Bit 5 - Selects pin L37 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss37(&mut self) -> Lcdcss37W<'_, Lcdcssel2Spec> {
        Lcdcss37W::new(self, 5)
    }
    #[doc = "Bit 6 - Selects pin L38 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss38(&mut self) -> Lcdcss38W<'_, Lcdcssel2Spec> {
        Lcdcss38W::new(self, 6)
    }
    #[doc = "Bit 7 - Selects pin L39 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss39(&mut self) -> Lcdcss39W<'_, Lcdcssel2Spec> {
        Lcdcss39W::new(self, 7)
    }
    #[doc = "Bit 8 - Selects pin L40 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss40(&mut self) -> Lcdcss40W<'_, Lcdcssel2Spec> {
        Lcdcss40W::new(self, 8)
    }
    #[doc = "Bit 9 - Selects pin L41 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss41(&mut self) -> Lcdcss41W<'_, Lcdcssel2Spec> {
        Lcdcss41W::new(self, 9)
    }
    #[doc = "Bit 10 - Selects pin L42 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss42(&mut self) -> Lcdcss42W<'_, Lcdcssel2Spec> {
        Lcdcss42W::new(self, 10)
    }
    #[doc = "Bit 11 - Selects pin L43 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss43(&mut self) -> Lcdcss43W<'_, Lcdcssel2Spec> {
        Lcdcss43W::new(self, 11)
    }
    #[doc = "Bit 12 - Selects pin L44 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss44(&mut self) -> Lcdcss44W<'_, Lcdcssel2Spec> {
        Lcdcss44W::new(self, 12)
    }
    #[doc = "Bit 13 - Selects pin L45 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss45(&mut self) -> Lcdcss45W<'_, Lcdcssel2Spec> {
        Lcdcss45W::new(self, 13)
    }
    #[doc = "Bit 14 - Selects pin L46 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss46(&mut self) -> Lcdcss46W<'_, Lcdcssel2Spec> {
        Lcdcss46W::new(self, 14)
    }
    #[doc = "Bit 15 - Selects pin L47 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss47(&mut self) -> Lcdcss47W<'_, Lcdcssel2Spec> {
        Lcdcss47W::new(self, 15)
    }
}
#[doc = "LCD_E COM/SEG select register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcssel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcssel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdcssel2Spec;
impl crate::RegisterSpec for Lcdcssel2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdcssel2::R`](R) reader structure"]
impl crate::Readable for Lcdcssel2Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdcssel2::W`](W) writer structure"]
impl crate::Writable for Lcdcssel2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LCDCSSEL2 to value 0"]
impl crate::Resettable for Lcdcssel2Spec {}
