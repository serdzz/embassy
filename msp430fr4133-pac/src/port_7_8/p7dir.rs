#[doc = "Register `P7DIR` reader"]
pub type R = crate::R<P7dirSpec>;
#[doc = "Register `P7DIR` writer"]
pub type W = crate::W<P7dirSpec>;
#[doc = "Field `P7DIR0` reader - P7DIR0"]
pub type P7dir0R = crate::BitReader;
#[doc = "Field `P7DIR0` writer - P7DIR0"]
pub type P7dir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7DIR1` reader - P7DIR1"]
pub type P7dir1R = crate::BitReader;
#[doc = "Field `P7DIR1` writer - P7DIR1"]
pub type P7dir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7DIR2` reader - P7DIR2"]
pub type P7dir2R = crate::BitReader;
#[doc = "Field `P7DIR2` writer - P7DIR2"]
pub type P7dir2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7DIR3` reader - P7DIR3"]
pub type P7dir3R = crate::BitReader;
#[doc = "Field `P7DIR3` writer - P7DIR3"]
pub type P7dir3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7DIR4` reader - P7DIR4"]
pub type P7dir4R = crate::BitReader;
#[doc = "Field `P7DIR4` writer - P7DIR4"]
pub type P7dir4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7DIR5` reader - P7DIR5"]
pub type P7dir5R = crate::BitReader;
#[doc = "Field `P7DIR5` writer - P7DIR5"]
pub type P7dir5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7DIR6` reader - P7DIR6"]
pub type P7dir6R = crate::BitReader;
#[doc = "Field `P7DIR6` writer - P7DIR6"]
pub type P7dir6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7DIR7` reader - P7DIR7"]
pub type P7dir7R = crate::BitReader;
#[doc = "Field `P7DIR7` writer - P7DIR7"]
pub type P7dir7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P7DIR0"]
    #[inline(always)]
    pub fn p7dir0(&self) -> P7dir0R {
        P7dir0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P7DIR1"]
    #[inline(always)]
    pub fn p7dir1(&self) -> P7dir1R {
        P7dir1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P7DIR2"]
    #[inline(always)]
    pub fn p7dir2(&self) -> P7dir2R {
        P7dir2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P7DIR3"]
    #[inline(always)]
    pub fn p7dir3(&self) -> P7dir3R {
        P7dir3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P7DIR4"]
    #[inline(always)]
    pub fn p7dir4(&self) -> P7dir4R {
        P7dir4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P7DIR5"]
    #[inline(always)]
    pub fn p7dir5(&self) -> P7dir5R {
        P7dir5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P7DIR6"]
    #[inline(always)]
    pub fn p7dir6(&self) -> P7dir6R {
        P7dir6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P7DIR7"]
    #[inline(always)]
    pub fn p7dir7(&self) -> P7dir7R {
        P7dir7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7DIR0"]
    #[inline(always)]
    pub fn p7dir0(&mut self) -> P7dir0W<'_, P7dirSpec> {
        P7dir0W::new(self, 0)
    }
    #[doc = "Bit 1 - P7DIR1"]
    #[inline(always)]
    pub fn p7dir1(&mut self) -> P7dir1W<'_, P7dirSpec> {
        P7dir1W::new(self, 1)
    }
    #[doc = "Bit 2 - P7DIR2"]
    #[inline(always)]
    pub fn p7dir2(&mut self) -> P7dir2W<'_, P7dirSpec> {
        P7dir2W::new(self, 2)
    }
    #[doc = "Bit 3 - P7DIR3"]
    #[inline(always)]
    pub fn p7dir3(&mut self) -> P7dir3W<'_, P7dirSpec> {
        P7dir3W::new(self, 3)
    }
    #[doc = "Bit 4 - P7DIR4"]
    #[inline(always)]
    pub fn p7dir4(&mut self) -> P7dir4W<'_, P7dirSpec> {
        P7dir4W::new(self, 4)
    }
    #[doc = "Bit 5 - P7DIR5"]
    #[inline(always)]
    pub fn p7dir5(&mut self) -> P7dir5W<'_, P7dirSpec> {
        P7dir5W::new(self, 5)
    }
    #[doc = "Bit 6 - P7DIR6"]
    #[inline(always)]
    pub fn p7dir6(&mut self) -> P7dir6W<'_, P7dirSpec> {
        P7dir6W::new(self, 6)
    }
    #[doc = "Bit 7 - P7DIR7"]
    #[inline(always)]
    pub fn p7dir7(&mut self) -> P7dir7W<'_, P7dirSpec> {
        P7dir7W::new(self, 7)
    }
}
#[doc = "Port 7 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p7dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7dirSpec;
impl crate::RegisterSpec for P7dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7dir::R`](R) reader structure"]
impl crate::Readable for P7dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p7dir::W`](W) writer structure"]
impl crate::Writable for P7dirSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P7DIR to value 0"]
impl crate::Resettable for P7dirSpec {}
