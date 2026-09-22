#[doc = "Register `P8OUT` reader"]
pub type R = crate::R<P8outSpec>;
#[doc = "Register `P8OUT` writer"]
pub type W = crate::W<P8outSpec>;
#[doc = "Field `P8OUT0` reader - P8OUT0"]
pub type P8out0R = crate::BitReader;
#[doc = "Field `P8OUT0` writer - P8OUT0"]
pub type P8out0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8OUT1` reader - P8OUT1"]
pub type P8out1R = crate::BitReader;
#[doc = "Field `P8OUT1` writer - P8OUT1"]
pub type P8out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8OUT2` reader - P8OUT2"]
pub type P8out2R = crate::BitReader;
#[doc = "Field `P8OUT2` writer - P8OUT2"]
pub type P8out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8OUT3` reader - P8OUT3"]
pub type P8out3R = crate::BitReader;
#[doc = "Field `P8OUT3` writer - P8OUT3"]
pub type P8out3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8OUT4` reader - P8OUT4"]
pub type P8out4R = crate::BitReader;
#[doc = "Field `P8OUT4` writer - P8OUT4"]
pub type P8out4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8OUT5` reader - P8OUT5"]
pub type P8out5R = crate::BitReader;
#[doc = "Field `P8OUT5` writer - P8OUT5"]
pub type P8out5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8OUT6` reader - P8OUT6"]
pub type P8out6R = crate::BitReader;
#[doc = "Field `P8OUT6` writer - P8OUT6"]
pub type P8out6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8OUT7` reader - P8OUT7"]
pub type P8out7R = crate::BitReader;
#[doc = "Field `P8OUT7` writer - P8OUT7"]
pub type P8out7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P8OUT0"]
    #[inline(always)]
    pub fn p8out0(&self) -> P8out0R {
        P8out0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P8OUT1"]
    #[inline(always)]
    pub fn p8out1(&self) -> P8out1R {
        P8out1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P8OUT2"]
    #[inline(always)]
    pub fn p8out2(&self) -> P8out2R {
        P8out2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P8OUT3"]
    #[inline(always)]
    pub fn p8out3(&self) -> P8out3R {
        P8out3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P8OUT4"]
    #[inline(always)]
    pub fn p8out4(&self) -> P8out4R {
        P8out4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P8OUT5"]
    #[inline(always)]
    pub fn p8out5(&self) -> P8out5R {
        P8out5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P8OUT6"]
    #[inline(always)]
    pub fn p8out6(&self) -> P8out6R {
        P8out6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P8OUT7"]
    #[inline(always)]
    pub fn p8out7(&self) -> P8out7R {
        P8out7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8OUT0"]
    #[inline(always)]
    pub fn p8out0(&mut self) -> P8out0W<'_, P8outSpec> {
        P8out0W::new(self, 0)
    }
    #[doc = "Bit 1 - P8OUT1"]
    #[inline(always)]
    pub fn p8out1(&mut self) -> P8out1W<'_, P8outSpec> {
        P8out1W::new(self, 1)
    }
    #[doc = "Bit 2 - P8OUT2"]
    #[inline(always)]
    pub fn p8out2(&mut self) -> P8out2W<'_, P8outSpec> {
        P8out2W::new(self, 2)
    }
    #[doc = "Bit 3 - P8OUT3"]
    #[inline(always)]
    pub fn p8out3(&mut self) -> P8out3W<'_, P8outSpec> {
        P8out3W::new(self, 3)
    }
    #[doc = "Bit 4 - P8OUT4"]
    #[inline(always)]
    pub fn p8out4(&mut self) -> P8out4W<'_, P8outSpec> {
        P8out4W::new(self, 4)
    }
    #[doc = "Bit 5 - P8OUT5"]
    #[inline(always)]
    pub fn p8out5(&mut self) -> P8out5W<'_, P8outSpec> {
        P8out5W::new(self, 5)
    }
    #[doc = "Bit 6 - P8OUT6"]
    #[inline(always)]
    pub fn p8out6(&mut self) -> P8out6W<'_, P8outSpec> {
        P8out6W::new(self, 6)
    }
    #[doc = "Bit 7 - P8OUT7"]
    #[inline(always)]
    pub fn p8out7(&mut self) -> P8out7W<'_, P8outSpec> {
        P8out7W::new(self, 7)
    }
}
#[doc = "Port 8 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p8out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8outSpec;
impl crate::RegisterSpec for P8outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8out::R`](R) reader structure"]
impl crate::Readable for P8outSpec {}
#[doc = "`write(|w| ..)` method takes [`p8out::W`](W) writer structure"]
impl crate::Writable for P8outSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P8OUT to value 0"]
impl crate::Resettable for P8outSpec {}
