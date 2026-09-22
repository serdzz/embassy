#[doc = "Register `P1DIR` reader"]
pub type R = crate::R<P1dirSpec>;
#[doc = "Register `P1DIR` writer"]
pub type W = crate::W<P1dirSpec>;
#[doc = "Field `P1DIR0` reader - P1DIR0"]
pub type P1dir0R = crate::BitReader;
#[doc = "Field `P1DIR0` writer - P1DIR0"]
pub type P1dir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1DIR1` reader - P1DIR1"]
pub type P1dir1R = crate::BitReader;
#[doc = "Field `P1DIR1` writer - P1DIR1"]
pub type P1dir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1DIR2` reader - P1DIR2"]
pub type P1dir2R = crate::BitReader;
#[doc = "Field `P1DIR2` writer - P1DIR2"]
pub type P1dir2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1DIR3` reader - P1DIR3"]
pub type P1dir3R = crate::BitReader;
#[doc = "Field `P1DIR3` writer - P1DIR3"]
pub type P1dir3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1DIR4` reader - P1DIR4"]
pub type P1dir4R = crate::BitReader;
#[doc = "Field `P1DIR4` writer - P1DIR4"]
pub type P1dir4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1DIR5` reader - P1DIR5"]
pub type P1dir5R = crate::BitReader;
#[doc = "Field `P1DIR5` writer - P1DIR5"]
pub type P1dir5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1DIR6` reader - P1DIR6"]
pub type P1dir6R = crate::BitReader;
#[doc = "Field `P1DIR6` writer - P1DIR6"]
pub type P1dir6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1DIR7` reader - P1DIR7"]
pub type P1dir7R = crate::BitReader;
#[doc = "Field `P1DIR7` writer - P1DIR7"]
pub type P1dir7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P1DIR0"]
    #[inline(always)]
    pub fn p1dir0(&self) -> P1dir0R {
        P1dir0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1DIR1"]
    #[inline(always)]
    pub fn p1dir1(&self) -> P1dir1R {
        P1dir1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1DIR2"]
    #[inline(always)]
    pub fn p1dir2(&self) -> P1dir2R {
        P1dir2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1DIR3"]
    #[inline(always)]
    pub fn p1dir3(&self) -> P1dir3R {
        P1dir3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1DIR4"]
    #[inline(always)]
    pub fn p1dir4(&self) -> P1dir4R {
        P1dir4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1DIR5"]
    #[inline(always)]
    pub fn p1dir5(&self) -> P1dir5R {
        P1dir5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1DIR6"]
    #[inline(always)]
    pub fn p1dir6(&self) -> P1dir6R {
        P1dir6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1DIR7"]
    #[inline(always)]
    pub fn p1dir7(&self) -> P1dir7R {
        P1dir7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1DIR0"]
    #[inline(always)]
    pub fn p1dir0(&mut self) -> P1dir0W<'_, P1dirSpec> {
        P1dir0W::new(self, 0)
    }
    #[doc = "Bit 1 - P1DIR1"]
    #[inline(always)]
    pub fn p1dir1(&mut self) -> P1dir1W<'_, P1dirSpec> {
        P1dir1W::new(self, 1)
    }
    #[doc = "Bit 2 - P1DIR2"]
    #[inline(always)]
    pub fn p1dir2(&mut self) -> P1dir2W<'_, P1dirSpec> {
        P1dir2W::new(self, 2)
    }
    #[doc = "Bit 3 - P1DIR3"]
    #[inline(always)]
    pub fn p1dir3(&mut self) -> P1dir3W<'_, P1dirSpec> {
        P1dir3W::new(self, 3)
    }
    #[doc = "Bit 4 - P1DIR4"]
    #[inline(always)]
    pub fn p1dir4(&mut self) -> P1dir4W<'_, P1dirSpec> {
        P1dir4W::new(self, 4)
    }
    #[doc = "Bit 5 - P1DIR5"]
    #[inline(always)]
    pub fn p1dir5(&mut self) -> P1dir5W<'_, P1dirSpec> {
        P1dir5W::new(self, 5)
    }
    #[doc = "Bit 6 - P1DIR6"]
    #[inline(always)]
    pub fn p1dir6(&mut self) -> P1dir6W<'_, P1dirSpec> {
        P1dir6W::new(self, 6)
    }
    #[doc = "Bit 7 - P1DIR7"]
    #[inline(always)]
    pub fn p1dir7(&mut self) -> P1dir7W<'_, P1dirSpec> {
        P1dir7W::new(self, 7)
    }
}
#[doc = "Port 1 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p1dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1dirSpec;
impl crate::RegisterSpec for P1dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1dir::R`](R) reader structure"]
impl crate::Readable for P1dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p1dir::W`](W) writer structure"]
impl crate::Writable for P1dirSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P1DIR to value 0"]
impl crate::Resettable for P1dirSpec {}
