#[doc = "Register `P6OUT` reader"]
pub type R = crate::R<P6outSpec>;
#[doc = "Register `P6OUT` writer"]
pub type W = crate::W<P6outSpec>;
#[doc = "Field `P6OUT0` reader - P6OUT0"]
pub type P6out0R = crate::BitReader;
#[doc = "Field `P6OUT0` writer - P6OUT0"]
pub type P6out0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6OUT1` reader - P6OUT1"]
pub type P6out1R = crate::BitReader;
#[doc = "Field `P6OUT1` writer - P6OUT1"]
pub type P6out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6OUT2` reader - P6OUT2"]
pub type P6out2R = crate::BitReader;
#[doc = "Field `P6OUT2` writer - P6OUT2"]
pub type P6out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6OUT3` reader - P6OUT3"]
pub type P6out3R = crate::BitReader;
#[doc = "Field `P6OUT3` writer - P6OUT3"]
pub type P6out3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6OUT4` reader - P6OUT4"]
pub type P6out4R = crate::BitReader;
#[doc = "Field `P6OUT4` writer - P6OUT4"]
pub type P6out4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6OUT5` reader - P6OUT5"]
pub type P6out5R = crate::BitReader;
#[doc = "Field `P6OUT5` writer - P6OUT5"]
pub type P6out5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6OUT6` reader - P6OUT6"]
pub type P6out6R = crate::BitReader;
#[doc = "Field `P6OUT6` writer - P6OUT6"]
pub type P6out6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6OUT7` reader - P6OUT7"]
pub type P6out7R = crate::BitReader;
#[doc = "Field `P6OUT7` writer - P6OUT7"]
pub type P6out7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P6OUT0"]
    #[inline(always)]
    pub fn p6out0(&self) -> P6out0R {
        P6out0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P6OUT1"]
    #[inline(always)]
    pub fn p6out1(&self) -> P6out1R {
        P6out1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P6OUT2"]
    #[inline(always)]
    pub fn p6out2(&self) -> P6out2R {
        P6out2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P6OUT3"]
    #[inline(always)]
    pub fn p6out3(&self) -> P6out3R {
        P6out3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P6OUT4"]
    #[inline(always)]
    pub fn p6out4(&self) -> P6out4R {
        P6out4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P6OUT5"]
    #[inline(always)]
    pub fn p6out5(&self) -> P6out5R {
        P6out5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P6OUT6"]
    #[inline(always)]
    pub fn p6out6(&self) -> P6out6R {
        P6out6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P6OUT7"]
    #[inline(always)]
    pub fn p6out7(&self) -> P6out7R {
        P6out7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6OUT0"]
    #[inline(always)]
    pub fn p6out0(&mut self) -> P6out0W<'_, P6outSpec> {
        P6out0W::new(self, 0)
    }
    #[doc = "Bit 1 - P6OUT1"]
    #[inline(always)]
    pub fn p6out1(&mut self) -> P6out1W<'_, P6outSpec> {
        P6out1W::new(self, 1)
    }
    #[doc = "Bit 2 - P6OUT2"]
    #[inline(always)]
    pub fn p6out2(&mut self) -> P6out2W<'_, P6outSpec> {
        P6out2W::new(self, 2)
    }
    #[doc = "Bit 3 - P6OUT3"]
    #[inline(always)]
    pub fn p6out3(&mut self) -> P6out3W<'_, P6outSpec> {
        P6out3W::new(self, 3)
    }
    #[doc = "Bit 4 - P6OUT4"]
    #[inline(always)]
    pub fn p6out4(&mut self) -> P6out4W<'_, P6outSpec> {
        P6out4W::new(self, 4)
    }
    #[doc = "Bit 5 - P6OUT5"]
    #[inline(always)]
    pub fn p6out5(&mut self) -> P6out5W<'_, P6outSpec> {
        P6out5W::new(self, 5)
    }
    #[doc = "Bit 6 - P6OUT6"]
    #[inline(always)]
    pub fn p6out6(&mut self) -> P6out6W<'_, P6outSpec> {
        P6out6W::new(self, 6)
    }
    #[doc = "Bit 7 - P6OUT7"]
    #[inline(always)]
    pub fn p6out7(&mut self) -> P6out7W<'_, P6outSpec> {
        P6out7W::new(self, 7)
    }
}
#[doc = "Port 6 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p6out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6outSpec;
impl crate::RegisterSpec for P6outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6out::R`](R) reader structure"]
impl crate::Readable for P6outSpec {}
#[doc = "`write(|w| ..)` method takes [`p6out::W`](W) writer structure"]
impl crate::Writable for P6outSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P6OUT to value 0"]
impl crate::Resettable for P6outSpec {}
