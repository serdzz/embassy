#[doc = "Register `P2DIR` reader"]
pub type R = crate::R<P2dirSpec>;
#[doc = "Register `P2DIR` writer"]
pub type W = crate::W<P2dirSpec>;
#[doc = "Field `P2DIR0` reader - P2DIR0"]
pub type P2dir0R = crate::BitReader;
#[doc = "Field `P2DIR0` writer - P2DIR0"]
pub type P2dir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2DIR1` reader - P2DIR1"]
pub type P2dir1R = crate::BitReader;
#[doc = "Field `P2DIR1` writer - P2DIR1"]
pub type P2dir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2DIR2` reader - P2DIR2"]
pub type P2dir2R = crate::BitReader;
#[doc = "Field `P2DIR2` writer - P2DIR2"]
pub type P2dir2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2DIR3` reader - P2DIR3"]
pub type P2dir3R = crate::BitReader;
#[doc = "Field `P2DIR3` writer - P2DIR3"]
pub type P2dir3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2DIR4` reader - P2DIR4"]
pub type P2dir4R = crate::BitReader;
#[doc = "Field `P2DIR4` writer - P2DIR4"]
pub type P2dir4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2DIR5` reader - P2DIR5"]
pub type P2dir5R = crate::BitReader;
#[doc = "Field `P2DIR5` writer - P2DIR5"]
pub type P2dir5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2DIR6` reader - P2DIR6"]
pub type P2dir6R = crate::BitReader;
#[doc = "Field `P2DIR6` writer - P2DIR6"]
pub type P2dir6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2DIR7` reader - P2DIR7"]
pub type P2dir7R = crate::BitReader;
#[doc = "Field `P2DIR7` writer - P2DIR7"]
pub type P2dir7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P2DIR0"]
    #[inline(always)]
    pub fn p2dir0(&self) -> P2dir0R {
        P2dir0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2DIR1"]
    #[inline(always)]
    pub fn p2dir1(&self) -> P2dir1R {
        P2dir1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2DIR2"]
    #[inline(always)]
    pub fn p2dir2(&self) -> P2dir2R {
        P2dir2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2DIR3"]
    #[inline(always)]
    pub fn p2dir3(&self) -> P2dir3R {
        P2dir3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2DIR4"]
    #[inline(always)]
    pub fn p2dir4(&self) -> P2dir4R {
        P2dir4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2DIR5"]
    #[inline(always)]
    pub fn p2dir5(&self) -> P2dir5R {
        P2dir5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2DIR6"]
    #[inline(always)]
    pub fn p2dir6(&self) -> P2dir6R {
        P2dir6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2DIR7"]
    #[inline(always)]
    pub fn p2dir7(&self) -> P2dir7R {
        P2dir7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2DIR0"]
    #[inline(always)]
    pub fn p2dir0(&mut self) -> P2dir0W<'_, P2dirSpec> {
        P2dir0W::new(self, 0)
    }
    #[doc = "Bit 1 - P2DIR1"]
    #[inline(always)]
    pub fn p2dir1(&mut self) -> P2dir1W<'_, P2dirSpec> {
        P2dir1W::new(self, 1)
    }
    #[doc = "Bit 2 - P2DIR2"]
    #[inline(always)]
    pub fn p2dir2(&mut self) -> P2dir2W<'_, P2dirSpec> {
        P2dir2W::new(self, 2)
    }
    #[doc = "Bit 3 - P2DIR3"]
    #[inline(always)]
    pub fn p2dir3(&mut self) -> P2dir3W<'_, P2dirSpec> {
        P2dir3W::new(self, 3)
    }
    #[doc = "Bit 4 - P2DIR4"]
    #[inline(always)]
    pub fn p2dir4(&mut self) -> P2dir4W<'_, P2dirSpec> {
        P2dir4W::new(self, 4)
    }
    #[doc = "Bit 5 - P2DIR5"]
    #[inline(always)]
    pub fn p2dir5(&mut self) -> P2dir5W<'_, P2dirSpec> {
        P2dir5W::new(self, 5)
    }
    #[doc = "Bit 6 - P2DIR6"]
    #[inline(always)]
    pub fn p2dir6(&mut self) -> P2dir6W<'_, P2dirSpec> {
        P2dir6W::new(self, 6)
    }
    #[doc = "Bit 7 - P2DIR7"]
    #[inline(always)]
    pub fn p2dir7(&mut self) -> P2dir7W<'_, P2dirSpec> {
        P2dir7W::new(self, 7)
    }
}
#[doc = "Port 2 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p2dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2dirSpec;
impl crate::RegisterSpec for P2dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2dir::R`](R) reader structure"]
impl crate::Readable for P2dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p2dir::W`](W) writer structure"]
impl crate::Writable for P2dirSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2DIR to value 0"]
impl crate::Resettable for P2dirSpec {}
