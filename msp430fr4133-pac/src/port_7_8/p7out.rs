#[doc = "Register `P7OUT` reader"]
pub type R = crate::R<P7outSpec>;
#[doc = "Register `P7OUT` writer"]
pub type W = crate::W<P7outSpec>;
#[doc = "Field `P7OUT0` reader - P7OUT0"]
pub type P7out0R = crate::BitReader;
#[doc = "Field `P7OUT0` writer - P7OUT0"]
pub type P7out0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7OUT1` reader - P7OUT1"]
pub type P7out1R = crate::BitReader;
#[doc = "Field `P7OUT1` writer - P7OUT1"]
pub type P7out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7OUT2` reader - P7OUT2"]
pub type P7out2R = crate::BitReader;
#[doc = "Field `P7OUT2` writer - P7OUT2"]
pub type P7out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7OUT3` reader - P7OUT3"]
pub type P7out3R = crate::BitReader;
#[doc = "Field `P7OUT3` writer - P7OUT3"]
pub type P7out3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7OUT4` reader - P7OUT4"]
pub type P7out4R = crate::BitReader;
#[doc = "Field `P7OUT4` writer - P7OUT4"]
pub type P7out4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7OUT5` reader - P7OUT5"]
pub type P7out5R = crate::BitReader;
#[doc = "Field `P7OUT5` writer - P7OUT5"]
pub type P7out5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7OUT6` reader - P7OUT6"]
pub type P7out6R = crate::BitReader;
#[doc = "Field `P7OUT6` writer - P7OUT6"]
pub type P7out6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7OUT7` reader - P7OUT7"]
pub type P7out7R = crate::BitReader;
#[doc = "Field `P7OUT7` writer - P7OUT7"]
pub type P7out7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P7OUT0"]
    #[inline(always)]
    pub fn p7out0(&self) -> P7out0R {
        P7out0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P7OUT1"]
    #[inline(always)]
    pub fn p7out1(&self) -> P7out1R {
        P7out1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P7OUT2"]
    #[inline(always)]
    pub fn p7out2(&self) -> P7out2R {
        P7out2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P7OUT3"]
    #[inline(always)]
    pub fn p7out3(&self) -> P7out3R {
        P7out3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P7OUT4"]
    #[inline(always)]
    pub fn p7out4(&self) -> P7out4R {
        P7out4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P7OUT5"]
    #[inline(always)]
    pub fn p7out5(&self) -> P7out5R {
        P7out5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P7OUT6"]
    #[inline(always)]
    pub fn p7out6(&self) -> P7out6R {
        P7out6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P7OUT7"]
    #[inline(always)]
    pub fn p7out7(&self) -> P7out7R {
        P7out7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7OUT0"]
    #[inline(always)]
    pub fn p7out0(&mut self) -> P7out0W<'_, P7outSpec> {
        P7out0W::new(self, 0)
    }
    #[doc = "Bit 1 - P7OUT1"]
    #[inline(always)]
    pub fn p7out1(&mut self) -> P7out1W<'_, P7outSpec> {
        P7out1W::new(self, 1)
    }
    #[doc = "Bit 2 - P7OUT2"]
    #[inline(always)]
    pub fn p7out2(&mut self) -> P7out2W<'_, P7outSpec> {
        P7out2W::new(self, 2)
    }
    #[doc = "Bit 3 - P7OUT3"]
    #[inline(always)]
    pub fn p7out3(&mut self) -> P7out3W<'_, P7outSpec> {
        P7out3W::new(self, 3)
    }
    #[doc = "Bit 4 - P7OUT4"]
    #[inline(always)]
    pub fn p7out4(&mut self) -> P7out4W<'_, P7outSpec> {
        P7out4W::new(self, 4)
    }
    #[doc = "Bit 5 - P7OUT5"]
    #[inline(always)]
    pub fn p7out5(&mut self) -> P7out5W<'_, P7outSpec> {
        P7out5W::new(self, 5)
    }
    #[doc = "Bit 6 - P7OUT6"]
    #[inline(always)]
    pub fn p7out6(&mut self) -> P7out6W<'_, P7outSpec> {
        P7out6W::new(self, 6)
    }
    #[doc = "Bit 7 - P7OUT7"]
    #[inline(always)]
    pub fn p7out7(&mut self) -> P7out7W<'_, P7outSpec> {
        P7out7W::new(self, 7)
    }
}
#[doc = "Port 7 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p7out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7outSpec;
impl crate::RegisterSpec for P7outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7out::R`](R) reader structure"]
impl crate::Readable for P7outSpec {}
#[doc = "`write(|w| ..)` method takes [`p7out::W`](W) writer structure"]
impl crate::Writable for P7outSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P7OUT to value 0"]
impl crate::Resettable for P7outSpec {}
