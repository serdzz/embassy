#[doc = "Register `P8DIR` reader"]
pub type R = crate::R<P8dirSpec>;
#[doc = "Register `P8DIR` writer"]
pub type W = crate::W<P8dirSpec>;
#[doc = "Field `P8DIR0` reader - P8DIR0"]
pub type P8dir0R = crate::BitReader;
#[doc = "Field `P8DIR0` writer - P8DIR0"]
pub type P8dir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DIR1` reader - P8DIR1"]
pub type P8dir1R = crate::BitReader;
#[doc = "Field `P8DIR1` writer - P8DIR1"]
pub type P8dir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DIR2` reader - P8DIR2"]
pub type P8dir2R = crate::BitReader;
#[doc = "Field `P8DIR2` writer - P8DIR2"]
pub type P8dir2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DIR3` reader - P8DIR3"]
pub type P8dir3R = crate::BitReader;
#[doc = "Field `P8DIR3` writer - P8DIR3"]
pub type P8dir3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DIR4` reader - P8DIR4"]
pub type P8dir4R = crate::BitReader;
#[doc = "Field `P8DIR4` writer - P8DIR4"]
pub type P8dir4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DIR5` reader - P8DIR5"]
pub type P8dir5R = crate::BitReader;
#[doc = "Field `P8DIR5` writer - P8DIR5"]
pub type P8dir5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DIR6` reader - P8DIR6"]
pub type P8dir6R = crate::BitReader;
#[doc = "Field `P8DIR6` writer - P8DIR6"]
pub type P8dir6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8DIR7` reader - P8DIR7"]
pub type P8dir7R = crate::BitReader;
#[doc = "Field `P8DIR7` writer - P8DIR7"]
pub type P8dir7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P8DIR0"]
    #[inline(always)]
    pub fn p8dir0(&self) -> P8dir0R {
        P8dir0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P8DIR1"]
    #[inline(always)]
    pub fn p8dir1(&self) -> P8dir1R {
        P8dir1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P8DIR2"]
    #[inline(always)]
    pub fn p8dir2(&self) -> P8dir2R {
        P8dir2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P8DIR3"]
    #[inline(always)]
    pub fn p8dir3(&self) -> P8dir3R {
        P8dir3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P8DIR4"]
    #[inline(always)]
    pub fn p8dir4(&self) -> P8dir4R {
        P8dir4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P8DIR5"]
    #[inline(always)]
    pub fn p8dir5(&self) -> P8dir5R {
        P8dir5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P8DIR6"]
    #[inline(always)]
    pub fn p8dir6(&self) -> P8dir6R {
        P8dir6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P8DIR7"]
    #[inline(always)]
    pub fn p8dir7(&self) -> P8dir7R {
        P8dir7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8DIR0"]
    #[inline(always)]
    pub fn p8dir0(&mut self) -> P8dir0W<'_, P8dirSpec> {
        P8dir0W::new(self, 0)
    }
    #[doc = "Bit 1 - P8DIR1"]
    #[inline(always)]
    pub fn p8dir1(&mut self) -> P8dir1W<'_, P8dirSpec> {
        P8dir1W::new(self, 1)
    }
    #[doc = "Bit 2 - P8DIR2"]
    #[inline(always)]
    pub fn p8dir2(&mut self) -> P8dir2W<'_, P8dirSpec> {
        P8dir2W::new(self, 2)
    }
    #[doc = "Bit 3 - P8DIR3"]
    #[inline(always)]
    pub fn p8dir3(&mut self) -> P8dir3W<'_, P8dirSpec> {
        P8dir3W::new(self, 3)
    }
    #[doc = "Bit 4 - P8DIR4"]
    #[inline(always)]
    pub fn p8dir4(&mut self) -> P8dir4W<'_, P8dirSpec> {
        P8dir4W::new(self, 4)
    }
    #[doc = "Bit 5 - P8DIR5"]
    #[inline(always)]
    pub fn p8dir5(&mut self) -> P8dir5W<'_, P8dirSpec> {
        P8dir5W::new(self, 5)
    }
    #[doc = "Bit 6 - P8DIR6"]
    #[inline(always)]
    pub fn p8dir6(&mut self) -> P8dir6W<'_, P8dirSpec> {
        P8dir6W::new(self, 6)
    }
    #[doc = "Bit 7 - P8DIR7"]
    #[inline(always)]
    pub fn p8dir7(&mut self) -> P8dir7W<'_, P8dirSpec> {
        P8dir7W::new(self, 7)
    }
}
#[doc = "Port 8 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p8dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8dirSpec;
impl crate::RegisterSpec for P8dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8dir::R`](R) reader structure"]
impl crate::Readable for P8dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p8dir::W`](W) writer structure"]
impl crate::Writable for P8dirSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P8DIR to value 0"]
impl crate::Resettable for P8dirSpec {}
