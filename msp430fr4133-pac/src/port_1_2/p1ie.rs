#[doc = "Register `P1IE` reader"]
pub type R = crate::R<P1ieSpec>;
#[doc = "Register `P1IE` writer"]
pub type W = crate::W<P1ieSpec>;
#[doc = "Field `P1IE0` reader - P1IE0"]
pub type P1ie0R = crate::BitReader;
#[doc = "Field `P1IE0` writer - P1IE0"]
pub type P1ie0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IE1` reader - P1IE1"]
pub type P1ie1R = crate::BitReader;
#[doc = "Field `P1IE1` writer - P1IE1"]
pub type P1ie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IE2` reader - P1IE2"]
pub type P1ie2R = crate::BitReader;
#[doc = "Field `P1IE2` writer - P1IE2"]
pub type P1ie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IE3` reader - P1IE3"]
pub type P1ie3R = crate::BitReader;
#[doc = "Field `P1IE3` writer - P1IE3"]
pub type P1ie3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IE4` reader - P1IE4"]
pub type P1ie4R = crate::BitReader;
#[doc = "Field `P1IE4` writer - P1IE4"]
pub type P1ie4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IE5` reader - P1IE5"]
pub type P1ie5R = crate::BitReader;
#[doc = "Field `P1IE5` writer - P1IE5"]
pub type P1ie5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IE6` reader - P1IE6"]
pub type P1ie6R = crate::BitReader;
#[doc = "Field `P1IE6` writer - P1IE6"]
pub type P1ie6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IE7` reader - P1IE7"]
pub type P1ie7R = crate::BitReader;
#[doc = "Field `P1IE7` writer - P1IE7"]
pub type P1ie7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P1IE0"]
    #[inline(always)]
    pub fn p1ie0(&self) -> P1ie0R {
        P1ie0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1IE1"]
    #[inline(always)]
    pub fn p1ie1(&self) -> P1ie1R {
        P1ie1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1IE2"]
    #[inline(always)]
    pub fn p1ie2(&self) -> P1ie2R {
        P1ie2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1IE3"]
    #[inline(always)]
    pub fn p1ie3(&self) -> P1ie3R {
        P1ie3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1IE4"]
    #[inline(always)]
    pub fn p1ie4(&self) -> P1ie4R {
        P1ie4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1IE5"]
    #[inline(always)]
    pub fn p1ie5(&self) -> P1ie5R {
        P1ie5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1IE6"]
    #[inline(always)]
    pub fn p1ie6(&self) -> P1ie6R {
        P1ie6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1IE7"]
    #[inline(always)]
    pub fn p1ie7(&self) -> P1ie7R {
        P1ie7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1IE0"]
    #[inline(always)]
    pub fn p1ie0(&mut self) -> P1ie0W<'_, P1ieSpec> {
        P1ie0W::new(self, 0)
    }
    #[doc = "Bit 1 - P1IE1"]
    #[inline(always)]
    pub fn p1ie1(&mut self) -> P1ie1W<'_, P1ieSpec> {
        P1ie1W::new(self, 1)
    }
    #[doc = "Bit 2 - P1IE2"]
    #[inline(always)]
    pub fn p1ie2(&mut self) -> P1ie2W<'_, P1ieSpec> {
        P1ie2W::new(self, 2)
    }
    #[doc = "Bit 3 - P1IE3"]
    #[inline(always)]
    pub fn p1ie3(&mut self) -> P1ie3W<'_, P1ieSpec> {
        P1ie3W::new(self, 3)
    }
    #[doc = "Bit 4 - P1IE4"]
    #[inline(always)]
    pub fn p1ie4(&mut self) -> P1ie4W<'_, P1ieSpec> {
        P1ie4W::new(self, 4)
    }
    #[doc = "Bit 5 - P1IE5"]
    #[inline(always)]
    pub fn p1ie5(&mut self) -> P1ie5W<'_, P1ieSpec> {
        P1ie5W::new(self, 5)
    }
    #[doc = "Bit 6 - P1IE6"]
    #[inline(always)]
    pub fn p1ie6(&mut self) -> P1ie6W<'_, P1ieSpec> {
        P1ie6W::new(self, 6)
    }
    #[doc = "Bit 7 - P1IE7"]
    #[inline(always)]
    pub fn p1ie7(&mut self) -> P1ie7W<'_, P1ieSpec> {
        P1ie7W::new(self, 7)
    }
}
#[doc = "Port 1 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1ieSpec;
impl crate::RegisterSpec for P1ieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1ie::R`](R) reader structure"]
impl crate::Readable for P1ieSpec {}
#[doc = "`write(|w| ..)` method takes [`p1ie::W`](W) writer structure"]
impl crate::Writable for P1ieSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P1IE to value 0"]
impl crate::Resettable for P1ieSpec {}
