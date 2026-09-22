#[doc = "Register `P1OUT` reader"]
pub type R = crate::R<P1outSpec>;
#[doc = "Register `P1OUT` writer"]
pub type W = crate::W<P1outSpec>;
#[doc = "Field `P1OUT0` reader - P1OUT0"]
pub type P1out0R = crate::BitReader;
#[doc = "Field `P1OUT0` writer - P1OUT0"]
pub type P1out0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1OUT1` reader - P1OUT1"]
pub type P1out1R = crate::BitReader;
#[doc = "Field `P1OUT1` writer - P1OUT1"]
pub type P1out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1OUT2` reader - P1OUT2"]
pub type P1out2R = crate::BitReader;
#[doc = "Field `P1OUT2` writer - P1OUT2"]
pub type P1out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1OUT3` reader - P1OUT3"]
pub type P1out3R = crate::BitReader;
#[doc = "Field `P1OUT3` writer - P1OUT3"]
pub type P1out3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1OUT4` reader - P1OUT4"]
pub type P1out4R = crate::BitReader;
#[doc = "Field `P1OUT4` writer - P1OUT4"]
pub type P1out4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1OUT5` reader - P1OUT5"]
pub type P1out5R = crate::BitReader;
#[doc = "Field `P1OUT5` writer - P1OUT5"]
pub type P1out5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1OUT6` reader - P1OUT6"]
pub type P1out6R = crate::BitReader;
#[doc = "Field `P1OUT6` writer - P1OUT6"]
pub type P1out6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1OUT7` reader - P1OUT7"]
pub type P1out7R = crate::BitReader;
#[doc = "Field `P1OUT7` writer - P1OUT7"]
pub type P1out7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P1OUT0"]
    #[inline(always)]
    pub fn p1out0(&self) -> P1out0R {
        P1out0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1OUT1"]
    #[inline(always)]
    pub fn p1out1(&self) -> P1out1R {
        P1out1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1OUT2"]
    #[inline(always)]
    pub fn p1out2(&self) -> P1out2R {
        P1out2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1OUT3"]
    #[inline(always)]
    pub fn p1out3(&self) -> P1out3R {
        P1out3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1OUT4"]
    #[inline(always)]
    pub fn p1out4(&self) -> P1out4R {
        P1out4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1OUT5"]
    #[inline(always)]
    pub fn p1out5(&self) -> P1out5R {
        P1out5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1OUT6"]
    #[inline(always)]
    pub fn p1out6(&self) -> P1out6R {
        P1out6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1OUT7"]
    #[inline(always)]
    pub fn p1out7(&self) -> P1out7R {
        P1out7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1OUT0"]
    #[inline(always)]
    pub fn p1out0(&mut self) -> P1out0W<'_, P1outSpec> {
        P1out0W::new(self, 0)
    }
    #[doc = "Bit 1 - P1OUT1"]
    #[inline(always)]
    pub fn p1out1(&mut self) -> P1out1W<'_, P1outSpec> {
        P1out1W::new(self, 1)
    }
    #[doc = "Bit 2 - P1OUT2"]
    #[inline(always)]
    pub fn p1out2(&mut self) -> P1out2W<'_, P1outSpec> {
        P1out2W::new(self, 2)
    }
    #[doc = "Bit 3 - P1OUT3"]
    #[inline(always)]
    pub fn p1out3(&mut self) -> P1out3W<'_, P1outSpec> {
        P1out3W::new(self, 3)
    }
    #[doc = "Bit 4 - P1OUT4"]
    #[inline(always)]
    pub fn p1out4(&mut self) -> P1out4W<'_, P1outSpec> {
        P1out4W::new(self, 4)
    }
    #[doc = "Bit 5 - P1OUT5"]
    #[inline(always)]
    pub fn p1out5(&mut self) -> P1out5W<'_, P1outSpec> {
        P1out5W::new(self, 5)
    }
    #[doc = "Bit 6 - P1OUT6"]
    #[inline(always)]
    pub fn p1out6(&mut self) -> P1out6W<'_, P1outSpec> {
        P1out6W::new(self, 6)
    }
    #[doc = "Bit 7 - P1OUT7"]
    #[inline(always)]
    pub fn p1out7(&mut self) -> P1out7W<'_, P1outSpec> {
        P1out7W::new(self, 7)
    }
}
#[doc = "Port 1 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p1out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1outSpec;
impl crate::RegisterSpec for P1outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1out::R`](R) reader structure"]
impl crate::Readable for P1outSpec {}
#[doc = "`write(|w| ..)` method takes [`p1out::W`](W) writer structure"]
impl crate::Writable for P1outSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P1OUT to value 0"]
impl crate::Resettable for P1outSpec {}
