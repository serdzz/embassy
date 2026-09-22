#[doc = "Register `P4DIR` reader"]
pub type R = crate::R<P4dirSpec>;
#[doc = "Register `P4DIR` writer"]
pub type W = crate::W<P4dirSpec>;
#[doc = "Field `P4DIR0` reader - P4DIR0"]
pub type P4dir0R = crate::BitReader;
#[doc = "Field `P4DIR0` writer - P4DIR0"]
pub type P4dir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DIR1` reader - P4DIR1"]
pub type P4dir1R = crate::BitReader;
#[doc = "Field `P4DIR1` writer - P4DIR1"]
pub type P4dir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DIR2` reader - P4DIR2"]
pub type P4dir2R = crate::BitReader;
#[doc = "Field `P4DIR2` writer - P4DIR2"]
pub type P4dir2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DIR3` reader - P4DIR3"]
pub type P4dir3R = crate::BitReader;
#[doc = "Field `P4DIR3` writer - P4DIR3"]
pub type P4dir3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DIR4` reader - P4DIR4"]
pub type P4dir4R = crate::BitReader;
#[doc = "Field `P4DIR4` writer - P4DIR4"]
pub type P4dir4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DIR5` reader - P4DIR5"]
pub type P4dir5R = crate::BitReader;
#[doc = "Field `P4DIR5` writer - P4DIR5"]
pub type P4dir5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DIR6` reader - P4DIR6"]
pub type P4dir6R = crate::BitReader;
#[doc = "Field `P4DIR6` writer - P4DIR6"]
pub type P4dir6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4DIR7` reader - P4DIR7"]
pub type P4dir7R = crate::BitReader;
#[doc = "Field `P4DIR7` writer - P4DIR7"]
pub type P4dir7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P4DIR0"]
    #[inline(always)]
    pub fn p4dir0(&self) -> P4dir0R {
        P4dir0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P4DIR1"]
    #[inline(always)]
    pub fn p4dir1(&self) -> P4dir1R {
        P4dir1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P4DIR2"]
    #[inline(always)]
    pub fn p4dir2(&self) -> P4dir2R {
        P4dir2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P4DIR3"]
    #[inline(always)]
    pub fn p4dir3(&self) -> P4dir3R {
        P4dir3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P4DIR4"]
    #[inline(always)]
    pub fn p4dir4(&self) -> P4dir4R {
        P4dir4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P4DIR5"]
    #[inline(always)]
    pub fn p4dir5(&self) -> P4dir5R {
        P4dir5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P4DIR6"]
    #[inline(always)]
    pub fn p4dir6(&self) -> P4dir6R {
        P4dir6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P4DIR7"]
    #[inline(always)]
    pub fn p4dir7(&self) -> P4dir7R {
        P4dir7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4DIR0"]
    #[inline(always)]
    pub fn p4dir0(&mut self) -> P4dir0W<'_, P4dirSpec> {
        P4dir0W::new(self, 0)
    }
    #[doc = "Bit 1 - P4DIR1"]
    #[inline(always)]
    pub fn p4dir1(&mut self) -> P4dir1W<'_, P4dirSpec> {
        P4dir1W::new(self, 1)
    }
    #[doc = "Bit 2 - P4DIR2"]
    #[inline(always)]
    pub fn p4dir2(&mut self) -> P4dir2W<'_, P4dirSpec> {
        P4dir2W::new(self, 2)
    }
    #[doc = "Bit 3 - P4DIR3"]
    #[inline(always)]
    pub fn p4dir3(&mut self) -> P4dir3W<'_, P4dirSpec> {
        P4dir3W::new(self, 3)
    }
    #[doc = "Bit 4 - P4DIR4"]
    #[inline(always)]
    pub fn p4dir4(&mut self) -> P4dir4W<'_, P4dirSpec> {
        P4dir4W::new(self, 4)
    }
    #[doc = "Bit 5 - P4DIR5"]
    #[inline(always)]
    pub fn p4dir5(&mut self) -> P4dir5W<'_, P4dirSpec> {
        P4dir5W::new(self, 5)
    }
    #[doc = "Bit 6 - P4DIR6"]
    #[inline(always)]
    pub fn p4dir6(&mut self) -> P4dir6W<'_, P4dirSpec> {
        P4dir6W::new(self, 6)
    }
    #[doc = "Bit 7 - P4DIR7"]
    #[inline(always)]
    pub fn p4dir7(&mut self) -> P4dir7W<'_, P4dirSpec> {
        P4dir7W::new(self, 7)
    }
}
#[doc = "Port 4 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p4dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4dirSpec;
impl crate::RegisterSpec for P4dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4dir::R`](R) reader structure"]
impl crate::Readable for P4dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p4dir::W`](W) writer structure"]
impl crate::Writable for P4dirSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P4DIR to value 0"]
impl crate::Resettable for P4dirSpec {}
