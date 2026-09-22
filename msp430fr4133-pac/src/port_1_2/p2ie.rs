#[doc = "Register `P2IE` reader"]
pub type R = crate::R<P2ieSpec>;
#[doc = "Register `P2IE` writer"]
pub type W = crate::W<P2ieSpec>;
#[doc = "Field `P2IE0` reader - P2IE0"]
pub type P2ie0R = crate::BitReader;
#[doc = "Field `P2IE0` writer - P2IE0"]
pub type P2ie0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IE1` reader - P2IE1"]
pub type P2ie1R = crate::BitReader;
#[doc = "Field `P2IE1` writer - P2IE1"]
pub type P2ie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IE2` reader - P2IE2"]
pub type P2ie2R = crate::BitReader;
#[doc = "Field `P2IE2` writer - P2IE2"]
pub type P2ie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IE3` reader - P2IE3"]
pub type P2ie3R = crate::BitReader;
#[doc = "Field `P2IE3` writer - P2IE3"]
pub type P2ie3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IE4` reader - P2IE4"]
pub type P2ie4R = crate::BitReader;
#[doc = "Field `P2IE4` writer - P2IE4"]
pub type P2ie4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IE5` reader - P2IE5"]
pub type P2ie5R = crate::BitReader;
#[doc = "Field `P2IE5` writer - P2IE5"]
pub type P2ie5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IE6` reader - P2IE6"]
pub type P2ie6R = crate::BitReader;
#[doc = "Field `P2IE6` writer - P2IE6"]
pub type P2ie6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IE7` reader - P2IE7"]
pub type P2ie7R = crate::BitReader;
#[doc = "Field `P2IE7` writer - P2IE7"]
pub type P2ie7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P2IE0"]
    #[inline(always)]
    pub fn p2ie0(&self) -> P2ie0R {
        P2ie0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2IE1"]
    #[inline(always)]
    pub fn p2ie1(&self) -> P2ie1R {
        P2ie1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2IE2"]
    #[inline(always)]
    pub fn p2ie2(&self) -> P2ie2R {
        P2ie2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2IE3"]
    #[inline(always)]
    pub fn p2ie3(&self) -> P2ie3R {
        P2ie3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2IE4"]
    #[inline(always)]
    pub fn p2ie4(&self) -> P2ie4R {
        P2ie4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2IE5"]
    #[inline(always)]
    pub fn p2ie5(&self) -> P2ie5R {
        P2ie5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2IE6"]
    #[inline(always)]
    pub fn p2ie6(&self) -> P2ie6R {
        P2ie6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2IE7"]
    #[inline(always)]
    pub fn p2ie7(&self) -> P2ie7R {
        P2ie7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2IE0"]
    #[inline(always)]
    pub fn p2ie0(&mut self) -> P2ie0W<'_, P2ieSpec> {
        P2ie0W::new(self, 0)
    }
    #[doc = "Bit 1 - P2IE1"]
    #[inline(always)]
    pub fn p2ie1(&mut self) -> P2ie1W<'_, P2ieSpec> {
        P2ie1W::new(self, 1)
    }
    #[doc = "Bit 2 - P2IE2"]
    #[inline(always)]
    pub fn p2ie2(&mut self) -> P2ie2W<'_, P2ieSpec> {
        P2ie2W::new(self, 2)
    }
    #[doc = "Bit 3 - P2IE3"]
    #[inline(always)]
    pub fn p2ie3(&mut self) -> P2ie3W<'_, P2ieSpec> {
        P2ie3W::new(self, 3)
    }
    #[doc = "Bit 4 - P2IE4"]
    #[inline(always)]
    pub fn p2ie4(&mut self) -> P2ie4W<'_, P2ieSpec> {
        P2ie4W::new(self, 4)
    }
    #[doc = "Bit 5 - P2IE5"]
    #[inline(always)]
    pub fn p2ie5(&mut self) -> P2ie5W<'_, P2ieSpec> {
        P2ie5W::new(self, 5)
    }
    #[doc = "Bit 6 - P2IE6"]
    #[inline(always)]
    pub fn p2ie6(&mut self) -> P2ie6W<'_, P2ieSpec> {
        P2ie6W::new(self, 6)
    }
    #[doc = "Bit 7 - P2IE7"]
    #[inline(always)]
    pub fn p2ie7(&mut self) -> P2ie7W<'_, P2ieSpec> {
        P2ie7W::new(self, 7)
    }
}
#[doc = "Port 2 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2ieSpec;
impl crate::RegisterSpec for P2ieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2ie::R`](R) reader structure"]
impl crate::Readable for P2ieSpec {}
#[doc = "`write(|w| ..)` method takes [`p2ie::W`](W) writer structure"]
impl crate::Writable for P2ieSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2IE to value 0"]
impl crate::Resettable for P2ieSpec {}
