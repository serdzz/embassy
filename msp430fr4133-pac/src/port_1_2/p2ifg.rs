#[doc = "Register `P2IFG` reader"]
pub type R = crate::R<P2ifgSpec>;
#[doc = "Register `P2IFG` writer"]
pub type W = crate::W<P2ifgSpec>;
#[doc = "Field `P2IFG0` reader - P2IFG0"]
pub type P2ifg0R = crate::BitReader;
#[doc = "Field `P2IFG0` writer - P2IFG0"]
pub type P2ifg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IFG1` reader - P2IFG1"]
pub type P2ifg1R = crate::BitReader;
#[doc = "Field `P2IFG1` writer - P2IFG1"]
pub type P2ifg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IFG2` reader - P2IFG2"]
pub type P2ifg2R = crate::BitReader;
#[doc = "Field `P2IFG2` writer - P2IFG2"]
pub type P2ifg2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IFG3` reader - P2IFG3"]
pub type P2ifg3R = crate::BitReader;
#[doc = "Field `P2IFG3` writer - P2IFG3"]
pub type P2ifg3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IFG4` reader - P2IFG4"]
pub type P2ifg4R = crate::BitReader;
#[doc = "Field `P2IFG4` writer - P2IFG4"]
pub type P2ifg4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IFG5` reader - P2IFG5"]
pub type P2ifg5R = crate::BitReader;
#[doc = "Field `P2IFG5` writer - P2IFG5"]
pub type P2ifg5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IFG6` reader - P2IFG6"]
pub type P2ifg6R = crate::BitReader;
#[doc = "Field `P2IFG6` writer - P2IFG6"]
pub type P2ifg6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IFG7` reader - P2IFG7"]
pub type P2ifg7R = crate::BitReader;
#[doc = "Field `P2IFG7` writer - P2IFG7"]
pub type P2ifg7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P2IFG0"]
    #[inline(always)]
    pub fn p2ifg0(&self) -> P2ifg0R {
        P2ifg0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2IFG1"]
    #[inline(always)]
    pub fn p2ifg1(&self) -> P2ifg1R {
        P2ifg1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2IFG2"]
    #[inline(always)]
    pub fn p2ifg2(&self) -> P2ifg2R {
        P2ifg2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2IFG3"]
    #[inline(always)]
    pub fn p2ifg3(&self) -> P2ifg3R {
        P2ifg3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2IFG4"]
    #[inline(always)]
    pub fn p2ifg4(&self) -> P2ifg4R {
        P2ifg4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2IFG5"]
    #[inline(always)]
    pub fn p2ifg5(&self) -> P2ifg5R {
        P2ifg5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2IFG6"]
    #[inline(always)]
    pub fn p2ifg6(&self) -> P2ifg6R {
        P2ifg6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2IFG7"]
    #[inline(always)]
    pub fn p2ifg7(&self) -> P2ifg7R {
        P2ifg7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2IFG0"]
    #[inline(always)]
    pub fn p2ifg0(&mut self) -> P2ifg0W<'_, P2ifgSpec> {
        P2ifg0W::new(self, 0)
    }
    #[doc = "Bit 1 - P2IFG1"]
    #[inline(always)]
    pub fn p2ifg1(&mut self) -> P2ifg1W<'_, P2ifgSpec> {
        P2ifg1W::new(self, 1)
    }
    #[doc = "Bit 2 - P2IFG2"]
    #[inline(always)]
    pub fn p2ifg2(&mut self) -> P2ifg2W<'_, P2ifgSpec> {
        P2ifg2W::new(self, 2)
    }
    #[doc = "Bit 3 - P2IFG3"]
    #[inline(always)]
    pub fn p2ifg3(&mut self) -> P2ifg3W<'_, P2ifgSpec> {
        P2ifg3W::new(self, 3)
    }
    #[doc = "Bit 4 - P2IFG4"]
    #[inline(always)]
    pub fn p2ifg4(&mut self) -> P2ifg4W<'_, P2ifgSpec> {
        P2ifg4W::new(self, 4)
    }
    #[doc = "Bit 5 - P2IFG5"]
    #[inline(always)]
    pub fn p2ifg5(&mut self) -> P2ifg5W<'_, P2ifgSpec> {
        P2ifg5W::new(self, 5)
    }
    #[doc = "Bit 6 - P2IFG6"]
    #[inline(always)]
    pub fn p2ifg6(&mut self) -> P2ifg6W<'_, P2ifgSpec> {
        P2ifg6W::new(self, 6)
    }
    #[doc = "Bit 7 - P2IFG7"]
    #[inline(always)]
    pub fn p2ifg7(&mut self) -> P2ifg7W<'_, P2ifgSpec> {
        P2ifg7W::new(self, 7)
    }
}
#[doc = "Port 2 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2ifgSpec;
impl crate::RegisterSpec for P2ifgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2ifg::R`](R) reader structure"]
impl crate::Readable for P2ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`p2ifg::W`](W) writer structure"]
impl crate::Writable for P2ifgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2IFG to value 0"]
impl crate::Resettable for P2ifgSpec {}
