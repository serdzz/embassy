#[doc = "Register `P3DIR` reader"]
pub type R = crate::R<P3dirSpec>;
#[doc = "Register `P3DIR` writer"]
pub type W = crate::W<P3dirSpec>;
#[doc = "Field `P3DIR0` reader - P3DIR0"]
pub type P3dir0R = crate::BitReader;
#[doc = "Field `P3DIR0` writer - P3DIR0"]
pub type P3dir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3DIR1` reader - P3DIR1"]
pub type P3dir1R = crate::BitReader;
#[doc = "Field `P3DIR1` writer - P3DIR1"]
pub type P3dir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3DIR2` reader - P3DIR2"]
pub type P3dir2R = crate::BitReader;
#[doc = "Field `P3DIR2` writer - P3DIR2"]
pub type P3dir2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3DIR3` reader - P3DIR3"]
pub type P3dir3R = crate::BitReader;
#[doc = "Field `P3DIR3` writer - P3DIR3"]
pub type P3dir3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3DIR4` reader - P3DIR4"]
pub type P3dir4R = crate::BitReader;
#[doc = "Field `P3DIR4` writer - P3DIR4"]
pub type P3dir4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3DIR5` reader - P3DIR5"]
pub type P3dir5R = crate::BitReader;
#[doc = "Field `P3DIR5` writer - P3DIR5"]
pub type P3dir5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3DIR6` reader - P3DIR6"]
pub type P3dir6R = crate::BitReader;
#[doc = "Field `P3DIR6` writer - P3DIR6"]
pub type P3dir6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3DIR7` reader - P3DIR7"]
pub type P3dir7R = crate::BitReader;
#[doc = "Field `P3DIR7` writer - P3DIR7"]
pub type P3dir7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P3DIR0"]
    #[inline(always)]
    pub fn p3dir0(&self) -> P3dir0R {
        P3dir0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P3DIR1"]
    #[inline(always)]
    pub fn p3dir1(&self) -> P3dir1R {
        P3dir1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P3DIR2"]
    #[inline(always)]
    pub fn p3dir2(&self) -> P3dir2R {
        P3dir2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3DIR3"]
    #[inline(always)]
    pub fn p3dir3(&self) -> P3dir3R {
        P3dir3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P3DIR4"]
    #[inline(always)]
    pub fn p3dir4(&self) -> P3dir4R {
        P3dir4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P3DIR5"]
    #[inline(always)]
    pub fn p3dir5(&self) -> P3dir5R {
        P3dir5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P3DIR6"]
    #[inline(always)]
    pub fn p3dir6(&self) -> P3dir6R {
        P3dir6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P3DIR7"]
    #[inline(always)]
    pub fn p3dir7(&self) -> P3dir7R {
        P3dir7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3DIR0"]
    #[inline(always)]
    pub fn p3dir0(&mut self) -> P3dir0W<'_, P3dirSpec> {
        P3dir0W::new(self, 0)
    }
    #[doc = "Bit 1 - P3DIR1"]
    #[inline(always)]
    pub fn p3dir1(&mut self) -> P3dir1W<'_, P3dirSpec> {
        P3dir1W::new(self, 1)
    }
    #[doc = "Bit 2 - P3DIR2"]
    #[inline(always)]
    pub fn p3dir2(&mut self) -> P3dir2W<'_, P3dirSpec> {
        P3dir2W::new(self, 2)
    }
    #[doc = "Bit 3 - P3DIR3"]
    #[inline(always)]
    pub fn p3dir3(&mut self) -> P3dir3W<'_, P3dirSpec> {
        P3dir3W::new(self, 3)
    }
    #[doc = "Bit 4 - P3DIR4"]
    #[inline(always)]
    pub fn p3dir4(&mut self) -> P3dir4W<'_, P3dirSpec> {
        P3dir4W::new(self, 4)
    }
    #[doc = "Bit 5 - P3DIR5"]
    #[inline(always)]
    pub fn p3dir5(&mut self) -> P3dir5W<'_, P3dirSpec> {
        P3dir5W::new(self, 5)
    }
    #[doc = "Bit 6 - P3DIR6"]
    #[inline(always)]
    pub fn p3dir6(&mut self) -> P3dir6W<'_, P3dirSpec> {
        P3dir6W::new(self, 6)
    }
    #[doc = "Bit 7 - P3DIR7"]
    #[inline(always)]
    pub fn p3dir7(&mut self) -> P3dir7W<'_, P3dirSpec> {
        P3dir7W::new(self, 7)
    }
}
#[doc = "Port 3 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p3dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3dirSpec;
impl crate::RegisterSpec for P3dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3dir::R`](R) reader structure"]
impl crate::Readable for P3dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p3dir::W`](W) writer structure"]
impl crate::Writable for P3dirSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P3DIR to value 0"]
impl crate::Resettable for P3dirSpec {}
