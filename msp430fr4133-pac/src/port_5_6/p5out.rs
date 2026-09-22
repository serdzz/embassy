#[doc = "Register `P5OUT` reader"]
pub type R = crate::R<P5outSpec>;
#[doc = "Register `P5OUT` writer"]
pub type W = crate::W<P5outSpec>;
#[doc = "Field `P5OUT0` reader - P5OUT0"]
pub type P5out0R = crate::BitReader;
#[doc = "Field `P5OUT0` writer - P5OUT0"]
pub type P5out0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5OUT1` reader - P5OUT1"]
pub type P5out1R = crate::BitReader;
#[doc = "Field `P5OUT1` writer - P5OUT1"]
pub type P5out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5OUT2` reader - P5OUT2"]
pub type P5out2R = crate::BitReader;
#[doc = "Field `P5OUT2` writer - P5OUT2"]
pub type P5out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5OUT3` reader - P5OUT3"]
pub type P5out3R = crate::BitReader;
#[doc = "Field `P5OUT3` writer - P5OUT3"]
pub type P5out3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5OUT4` reader - P5OUT4"]
pub type P5out4R = crate::BitReader;
#[doc = "Field `P5OUT4` writer - P5OUT4"]
pub type P5out4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5OUT5` reader - P5OUT5"]
pub type P5out5R = crate::BitReader;
#[doc = "Field `P5OUT5` writer - P5OUT5"]
pub type P5out5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5OUT6` reader - P5OUT6"]
pub type P5out6R = crate::BitReader;
#[doc = "Field `P5OUT6` writer - P5OUT6"]
pub type P5out6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5OUT7` reader - P5OUT7"]
pub type P5out7R = crate::BitReader;
#[doc = "Field `P5OUT7` writer - P5OUT7"]
pub type P5out7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P5OUT0"]
    #[inline(always)]
    pub fn p5out0(&self) -> P5out0R {
        P5out0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P5OUT1"]
    #[inline(always)]
    pub fn p5out1(&self) -> P5out1R {
        P5out1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P5OUT2"]
    #[inline(always)]
    pub fn p5out2(&self) -> P5out2R {
        P5out2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P5OUT3"]
    #[inline(always)]
    pub fn p5out3(&self) -> P5out3R {
        P5out3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P5OUT4"]
    #[inline(always)]
    pub fn p5out4(&self) -> P5out4R {
        P5out4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P5OUT5"]
    #[inline(always)]
    pub fn p5out5(&self) -> P5out5R {
        P5out5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P5OUT6"]
    #[inline(always)]
    pub fn p5out6(&self) -> P5out6R {
        P5out6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P5OUT7"]
    #[inline(always)]
    pub fn p5out7(&self) -> P5out7R {
        P5out7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5OUT0"]
    #[inline(always)]
    pub fn p5out0(&mut self) -> P5out0W<'_, P5outSpec> {
        P5out0W::new(self, 0)
    }
    #[doc = "Bit 1 - P5OUT1"]
    #[inline(always)]
    pub fn p5out1(&mut self) -> P5out1W<'_, P5outSpec> {
        P5out1W::new(self, 1)
    }
    #[doc = "Bit 2 - P5OUT2"]
    #[inline(always)]
    pub fn p5out2(&mut self) -> P5out2W<'_, P5outSpec> {
        P5out2W::new(self, 2)
    }
    #[doc = "Bit 3 - P5OUT3"]
    #[inline(always)]
    pub fn p5out3(&mut self) -> P5out3W<'_, P5outSpec> {
        P5out3W::new(self, 3)
    }
    #[doc = "Bit 4 - P5OUT4"]
    #[inline(always)]
    pub fn p5out4(&mut self) -> P5out4W<'_, P5outSpec> {
        P5out4W::new(self, 4)
    }
    #[doc = "Bit 5 - P5OUT5"]
    #[inline(always)]
    pub fn p5out5(&mut self) -> P5out5W<'_, P5outSpec> {
        P5out5W::new(self, 5)
    }
    #[doc = "Bit 6 - P5OUT6"]
    #[inline(always)]
    pub fn p5out6(&mut self) -> P5out6W<'_, P5outSpec> {
        P5out6W::new(self, 6)
    }
    #[doc = "Bit 7 - P5OUT7"]
    #[inline(always)]
    pub fn p5out7(&mut self) -> P5out7W<'_, P5outSpec> {
        P5out7W::new(self, 7)
    }
}
#[doc = "Port 5 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p5out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5outSpec;
impl crate::RegisterSpec for P5outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5out::R`](R) reader structure"]
impl crate::Readable for P5outSpec {}
#[doc = "`write(|w| ..)` method takes [`p5out::W`](W) writer structure"]
impl crate::Writable for P5outSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P5OUT to value 0"]
impl crate::Resettable for P5outSpec {}
