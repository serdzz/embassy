#[doc = "Register `P5DIR` reader"]
pub type R = crate::R<P5dirSpec>;
#[doc = "Register `P5DIR` writer"]
pub type W = crate::W<P5dirSpec>;
#[doc = "Field `P5DIR0` reader - P5DIR0"]
pub type P5dir0R = crate::BitReader;
#[doc = "Field `P5DIR0` writer - P5DIR0"]
pub type P5dir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5DIR1` reader - P5DIR1"]
pub type P5dir1R = crate::BitReader;
#[doc = "Field `P5DIR1` writer - P5DIR1"]
pub type P5dir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5DIR2` reader - P5DIR2"]
pub type P5dir2R = crate::BitReader;
#[doc = "Field `P5DIR2` writer - P5DIR2"]
pub type P5dir2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5DIR3` reader - P5DIR3"]
pub type P5dir3R = crate::BitReader;
#[doc = "Field `P5DIR3` writer - P5DIR3"]
pub type P5dir3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5DIR4` reader - P5DIR4"]
pub type P5dir4R = crate::BitReader;
#[doc = "Field `P5DIR4` writer - P5DIR4"]
pub type P5dir4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5DIR5` reader - P5DIR5"]
pub type P5dir5R = crate::BitReader;
#[doc = "Field `P5DIR5` writer - P5DIR5"]
pub type P5dir5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5DIR6` reader - P5DIR6"]
pub type P5dir6R = crate::BitReader;
#[doc = "Field `P5DIR6` writer - P5DIR6"]
pub type P5dir6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5DIR7` reader - P5DIR7"]
pub type P5dir7R = crate::BitReader;
#[doc = "Field `P5DIR7` writer - P5DIR7"]
pub type P5dir7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P5DIR0"]
    #[inline(always)]
    pub fn p5dir0(&self) -> P5dir0R {
        P5dir0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P5DIR1"]
    #[inline(always)]
    pub fn p5dir1(&self) -> P5dir1R {
        P5dir1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P5DIR2"]
    #[inline(always)]
    pub fn p5dir2(&self) -> P5dir2R {
        P5dir2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P5DIR3"]
    #[inline(always)]
    pub fn p5dir3(&self) -> P5dir3R {
        P5dir3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P5DIR4"]
    #[inline(always)]
    pub fn p5dir4(&self) -> P5dir4R {
        P5dir4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P5DIR5"]
    #[inline(always)]
    pub fn p5dir5(&self) -> P5dir5R {
        P5dir5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P5DIR6"]
    #[inline(always)]
    pub fn p5dir6(&self) -> P5dir6R {
        P5dir6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P5DIR7"]
    #[inline(always)]
    pub fn p5dir7(&self) -> P5dir7R {
        P5dir7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5DIR0"]
    #[inline(always)]
    pub fn p5dir0(&mut self) -> P5dir0W<'_, P5dirSpec> {
        P5dir0W::new(self, 0)
    }
    #[doc = "Bit 1 - P5DIR1"]
    #[inline(always)]
    pub fn p5dir1(&mut self) -> P5dir1W<'_, P5dirSpec> {
        P5dir1W::new(self, 1)
    }
    #[doc = "Bit 2 - P5DIR2"]
    #[inline(always)]
    pub fn p5dir2(&mut self) -> P5dir2W<'_, P5dirSpec> {
        P5dir2W::new(self, 2)
    }
    #[doc = "Bit 3 - P5DIR3"]
    #[inline(always)]
    pub fn p5dir3(&mut self) -> P5dir3W<'_, P5dirSpec> {
        P5dir3W::new(self, 3)
    }
    #[doc = "Bit 4 - P5DIR4"]
    #[inline(always)]
    pub fn p5dir4(&mut self) -> P5dir4W<'_, P5dirSpec> {
        P5dir4W::new(self, 4)
    }
    #[doc = "Bit 5 - P5DIR5"]
    #[inline(always)]
    pub fn p5dir5(&mut self) -> P5dir5W<'_, P5dirSpec> {
        P5dir5W::new(self, 5)
    }
    #[doc = "Bit 6 - P5DIR6"]
    #[inline(always)]
    pub fn p5dir6(&mut self) -> P5dir6W<'_, P5dirSpec> {
        P5dir6W::new(self, 6)
    }
    #[doc = "Bit 7 - P5DIR7"]
    #[inline(always)]
    pub fn p5dir7(&mut self) -> P5dir7W<'_, P5dirSpec> {
        P5dir7W::new(self, 7)
    }
}
#[doc = "Port 5 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p5dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5dirSpec;
impl crate::RegisterSpec for P5dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5dir::R`](R) reader structure"]
impl crate::Readable for P5dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p5dir::W`](W) writer structure"]
impl crate::Writable for P5dirSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P5DIR to value 0"]
impl crate::Resettable for P5dirSpec {}
