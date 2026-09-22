#[doc = "Register `P2OUT` reader"]
pub type R = crate::R<P2outSpec>;
#[doc = "Register `P2OUT` writer"]
pub type W = crate::W<P2outSpec>;
#[doc = "Field `P2OUT0` reader - P2OUT0"]
pub type P2out0R = crate::BitReader;
#[doc = "Field `P2OUT0` writer - P2OUT0"]
pub type P2out0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2OUT1` reader - P2OUT1"]
pub type P2out1R = crate::BitReader;
#[doc = "Field `P2OUT1` writer - P2OUT1"]
pub type P2out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2OUT2` reader - P2OUT2"]
pub type P2out2R = crate::BitReader;
#[doc = "Field `P2OUT2` writer - P2OUT2"]
pub type P2out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2OUT3` reader - P2OUT3"]
pub type P2out3R = crate::BitReader;
#[doc = "Field `P2OUT3` writer - P2OUT3"]
pub type P2out3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2OUT4` reader - P2OUT4"]
pub type P2out4R = crate::BitReader;
#[doc = "Field `P2OUT4` writer - P2OUT4"]
pub type P2out4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2OUT5` reader - P2OUT5"]
pub type P2out5R = crate::BitReader;
#[doc = "Field `P2OUT5` writer - P2OUT5"]
pub type P2out5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2OUT6` reader - P2OUT6"]
pub type P2out6R = crate::BitReader;
#[doc = "Field `P2OUT6` writer - P2OUT6"]
pub type P2out6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2OUT7` reader - P2OUT7"]
pub type P2out7R = crate::BitReader;
#[doc = "Field `P2OUT7` writer - P2OUT7"]
pub type P2out7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P2OUT0"]
    #[inline(always)]
    pub fn p2out0(&self) -> P2out0R {
        P2out0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2OUT1"]
    #[inline(always)]
    pub fn p2out1(&self) -> P2out1R {
        P2out1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2OUT2"]
    #[inline(always)]
    pub fn p2out2(&self) -> P2out2R {
        P2out2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2OUT3"]
    #[inline(always)]
    pub fn p2out3(&self) -> P2out3R {
        P2out3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2OUT4"]
    #[inline(always)]
    pub fn p2out4(&self) -> P2out4R {
        P2out4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2OUT5"]
    #[inline(always)]
    pub fn p2out5(&self) -> P2out5R {
        P2out5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2OUT6"]
    #[inline(always)]
    pub fn p2out6(&self) -> P2out6R {
        P2out6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2OUT7"]
    #[inline(always)]
    pub fn p2out7(&self) -> P2out7R {
        P2out7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2OUT0"]
    #[inline(always)]
    pub fn p2out0(&mut self) -> P2out0W<'_, P2outSpec> {
        P2out0W::new(self, 0)
    }
    #[doc = "Bit 1 - P2OUT1"]
    #[inline(always)]
    pub fn p2out1(&mut self) -> P2out1W<'_, P2outSpec> {
        P2out1W::new(self, 1)
    }
    #[doc = "Bit 2 - P2OUT2"]
    #[inline(always)]
    pub fn p2out2(&mut self) -> P2out2W<'_, P2outSpec> {
        P2out2W::new(self, 2)
    }
    #[doc = "Bit 3 - P2OUT3"]
    #[inline(always)]
    pub fn p2out3(&mut self) -> P2out3W<'_, P2outSpec> {
        P2out3W::new(self, 3)
    }
    #[doc = "Bit 4 - P2OUT4"]
    #[inline(always)]
    pub fn p2out4(&mut self) -> P2out4W<'_, P2outSpec> {
        P2out4W::new(self, 4)
    }
    #[doc = "Bit 5 - P2OUT5"]
    #[inline(always)]
    pub fn p2out5(&mut self) -> P2out5W<'_, P2outSpec> {
        P2out5W::new(self, 5)
    }
    #[doc = "Bit 6 - P2OUT6"]
    #[inline(always)]
    pub fn p2out6(&mut self) -> P2out6W<'_, P2outSpec> {
        P2out6W::new(self, 6)
    }
    #[doc = "Bit 7 - P2OUT7"]
    #[inline(always)]
    pub fn p2out7(&mut self) -> P2out7W<'_, P2outSpec> {
        P2out7W::new(self, 7)
    }
}
#[doc = "Port 2 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p2out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2outSpec;
impl crate::RegisterSpec for P2outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2out::R`](R) reader structure"]
impl crate::Readable for P2outSpec {}
#[doc = "`write(|w| ..)` method takes [`p2out::W`](W) writer structure"]
impl crate::Writable for P2outSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2OUT to value 0"]
impl crate::Resettable for P2outSpec {}
