#[doc = "Register `P6DIR` reader"]
pub type R = crate::R<P6dirSpec>;
#[doc = "Register `P6DIR` writer"]
pub type W = crate::W<P6dirSpec>;
#[doc = "Field `P6DIR0` reader - P6DIR0"]
pub type P6dir0R = crate::BitReader;
#[doc = "Field `P6DIR0` writer - P6DIR0"]
pub type P6dir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6DIR1` reader - P6DIR1"]
pub type P6dir1R = crate::BitReader;
#[doc = "Field `P6DIR1` writer - P6DIR1"]
pub type P6dir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6DIR2` reader - P6DIR2"]
pub type P6dir2R = crate::BitReader;
#[doc = "Field `P6DIR2` writer - P6DIR2"]
pub type P6dir2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6DIR3` reader - P6DIR3"]
pub type P6dir3R = crate::BitReader;
#[doc = "Field `P6DIR3` writer - P6DIR3"]
pub type P6dir3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6DIR4` reader - P6DIR4"]
pub type P6dir4R = crate::BitReader;
#[doc = "Field `P6DIR4` writer - P6DIR4"]
pub type P6dir4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6DIR5` reader - P6DIR5"]
pub type P6dir5R = crate::BitReader;
#[doc = "Field `P6DIR5` writer - P6DIR5"]
pub type P6dir5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6DIR6` reader - P6DIR6"]
pub type P6dir6R = crate::BitReader;
#[doc = "Field `P6DIR6` writer - P6DIR6"]
pub type P6dir6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6DIR7` reader - P6DIR7"]
pub type P6dir7R = crate::BitReader;
#[doc = "Field `P6DIR7` writer - P6DIR7"]
pub type P6dir7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P6DIR0"]
    #[inline(always)]
    pub fn p6dir0(&self) -> P6dir0R {
        P6dir0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P6DIR1"]
    #[inline(always)]
    pub fn p6dir1(&self) -> P6dir1R {
        P6dir1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P6DIR2"]
    #[inline(always)]
    pub fn p6dir2(&self) -> P6dir2R {
        P6dir2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P6DIR3"]
    #[inline(always)]
    pub fn p6dir3(&self) -> P6dir3R {
        P6dir3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P6DIR4"]
    #[inline(always)]
    pub fn p6dir4(&self) -> P6dir4R {
        P6dir4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P6DIR5"]
    #[inline(always)]
    pub fn p6dir5(&self) -> P6dir5R {
        P6dir5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P6DIR6"]
    #[inline(always)]
    pub fn p6dir6(&self) -> P6dir6R {
        P6dir6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P6DIR7"]
    #[inline(always)]
    pub fn p6dir7(&self) -> P6dir7R {
        P6dir7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6DIR0"]
    #[inline(always)]
    pub fn p6dir0(&mut self) -> P6dir0W<'_, P6dirSpec> {
        P6dir0W::new(self, 0)
    }
    #[doc = "Bit 1 - P6DIR1"]
    #[inline(always)]
    pub fn p6dir1(&mut self) -> P6dir1W<'_, P6dirSpec> {
        P6dir1W::new(self, 1)
    }
    #[doc = "Bit 2 - P6DIR2"]
    #[inline(always)]
    pub fn p6dir2(&mut self) -> P6dir2W<'_, P6dirSpec> {
        P6dir2W::new(self, 2)
    }
    #[doc = "Bit 3 - P6DIR3"]
    #[inline(always)]
    pub fn p6dir3(&mut self) -> P6dir3W<'_, P6dirSpec> {
        P6dir3W::new(self, 3)
    }
    #[doc = "Bit 4 - P6DIR4"]
    #[inline(always)]
    pub fn p6dir4(&mut self) -> P6dir4W<'_, P6dirSpec> {
        P6dir4W::new(self, 4)
    }
    #[doc = "Bit 5 - P6DIR5"]
    #[inline(always)]
    pub fn p6dir5(&mut self) -> P6dir5W<'_, P6dirSpec> {
        P6dir5W::new(self, 5)
    }
    #[doc = "Bit 6 - P6DIR6"]
    #[inline(always)]
    pub fn p6dir6(&mut self) -> P6dir6W<'_, P6dirSpec> {
        P6dir6W::new(self, 6)
    }
    #[doc = "Bit 7 - P6DIR7"]
    #[inline(always)]
    pub fn p6dir7(&mut self) -> P6dir7W<'_, P6dirSpec> {
        P6dir7W::new(self, 7)
    }
}
#[doc = "Port 6 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p6dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6dirSpec;
impl crate::RegisterSpec for P6dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6dir::R`](R) reader structure"]
impl crate::Readable for P6dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p6dir::W`](W) writer structure"]
impl crate::Writable for P6dirSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P6DIR to value 0"]
impl crate::Resettable for P6dirSpec {}
