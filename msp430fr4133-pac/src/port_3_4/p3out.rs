#[doc = "Register `P3OUT` reader"]
pub type R = crate::R<P3outSpec>;
#[doc = "Register `P3OUT` writer"]
pub type W = crate::W<P3outSpec>;
#[doc = "Field `P3OUT0` reader - P3OUT0"]
pub type P3out0R = crate::BitReader;
#[doc = "Field `P3OUT0` writer - P3OUT0"]
pub type P3out0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3OUT1` reader - P3OUT1"]
pub type P3out1R = crate::BitReader;
#[doc = "Field `P3OUT1` writer - P3OUT1"]
pub type P3out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3OUT2` reader - P3OUT2"]
pub type P3out2R = crate::BitReader;
#[doc = "Field `P3OUT2` writer - P3OUT2"]
pub type P3out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3OUT3` reader - P3OUT3"]
pub type P3out3R = crate::BitReader;
#[doc = "Field `P3OUT3` writer - P3OUT3"]
pub type P3out3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3OUT4` reader - P3OUT4"]
pub type P3out4R = crate::BitReader;
#[doc = "Field `P3OUT4` writer - P3OUT4"]
pub type P3out4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3OUT5` reader - P3OUT5"]
pub type P3out5R = crate::BitReader;
#[doc = "Field `P3OUT5` writer - P3OUT5"]
pub type P3out5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3OUT6` reader - P3OUT6"]
pub type P3out6R = crate::BitReader;
#[doc = "Field `P3OUT6` writer - P3OUT6"]
pub type P3out6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3OUT7` reader - P3OUT7"]
pub type P3out7R = crate::BitReader;
#[doc = "Field `P3OUT7` writer - P3OUT7"]
pub type P3out7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P3OUT0"]
    #[inline(always)]
    pub fn p3out0(&self) -> P3out0R {
        P3out0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P3OUT1"]
    #[inline(always)]
    pub fn p3out1(&self) -> P3out1R {
        P3out1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P3OUT2"]
    #[inline(always)]
    pub fn p3out2(&self) -> P3out2R {
        P3out2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3OUT3"]
    #[inline(always)]
    pub fn p3out3(&self) -> P3out3R {
        P3out3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P3OUT4"]
    #[inline(always)]
    pub fn p3out4(&self) -> P3out4R {
        P3out4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P3OUT5"]
    #[inline(always)]
    pub fn p3out5(&self) -> P3out5R {
        P3out5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P3OUT6"]
    #[inline(always)]
    pub fn p3out6(&self) -> P3out6R {
        P3out6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P3OUT7"]
    #[inline(always)]
    pub fn p3out7(&self) -> P3out7R {
        P3out7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3OUT0"]
    #[inline(always)]
    pub fn p3out0(&mut self) -> P3out0W<'_, P3outSpec> {
        P3out0W::new(self, 0)
    }
    #[doc = "Bit 1 - P3OUT1"]
    #[inline(always)]
    pub fn p3out1(&mut self) -> P3out1W<'_, P3outSpec> {
        P3out1W::new(self, 1)
    }
    #[doc = "Bit 2 - P3OUT2"]
    #[inline(always)]
    pub fn p3out2(&mut self) -> P3out2W<'_, P3outSpec> {
        P3out2W::new(self, 2)
    }
    #[doc = "Bit 3 - P3OUT3"]
    #[inline(always)]
    pub fn p3out3(&mut self) -> P3out3W<'_, P3outSpec> {
        P3out3W::new(self, 3)
    }
    #[doc = "Bit 4 - P3OUT4"]
    #[inline(always)]
    pub fn p3out4(&mut self) -> P3out4W<'_, P3outSpec> {
        P3out4W::new(self, 4)
    }
    #[doc = "Bit 5 - P3OUT5"]
    #[inline(always)]
    pub fn p3out5(&mut self) -> P3out5W<'_, P3outSpec> {
        P3out5W::new(self, 5)
    }
    #[doc = "Bit 6 - P3OUT6"]
    #[inline(always)]
    pub fn p3out6(&mut self) -> P3out6W<'_, P3outSpec> {
        P3out6W::new(self, 6)
    }
    #[doc = "Bit 7 - P3OUT7"]
    #[inline(always)]
    pub fn p3out7(&mut self) -> P3out7W<'_, P3outSpec> {
        P3out7W::new(self, 7)
    }
}
#[doc = "Port 3 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p3out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3outSpec;
impl crate::RegisterSpec for P3outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3out::R`](R) reader structure"]
impl crate::Readable for P3outSpec {}
#[doc = "`write(|w| ..)` method takes [`p3out::W`](W) writer structure"]
impl crate::Writable for P3outSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P3OUT to value 0"]
impl crate::Resettable for P3outSpec {}
