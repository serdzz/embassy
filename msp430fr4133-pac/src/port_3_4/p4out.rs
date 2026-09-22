#[doc = "Register `P4OUT` reader"]
pub type R = crate::R<P4outSpec>;
#[doc = "Register `P4OUT` writer"]
pub type W = crate::W<P4outSpec>;
#[doc = "Field `P4OUT0` reader - P4OUT0"]
pub type P4out0R = crate::BitReader;
#[doc = "Field `P4OUT0` writer - P4OUT0"]
pub type P4out0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4OUT1` reader - P4OUT1"]
pub type P4out1R = crate::BitReader;
#[doc = "Field `P4OUT1` writer - P4OUT1"]
pub type P4out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4OUT2` reader - P4OUT2"]
pub type P4out2R = crate::BitReader;
#[doc = "Field `P4OUT2` writer - P4OUT2"]
pub type P4out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4OUT3` reader - P4OUT3"]
pub type P4out3R = crate::BitReader;
#[doc = "Field `P4OUT3` writer - P4OUT3"]
pub type P4out3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4OUT4` reader - P4OUT4"]
pub type P4out4R = crate::BitReader;
#[doc = "Field `P4OUT4` writer - P4OUT4"]
pub type P4out4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4OUT5` reader - P4OUT5"]
pub type P4out5R = crate::BitReader;
#[doc = "Field `P4OUT5` writer - P4OUT5"]
pub type P4out5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4OUT6` reader - P4OUT6"]
pub type P4out6R = crate::BitReader;
#[doc = "Field `P4OUT6` writer - P4OUT6"]
pub type P4out6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4OUT7` reader - P4OUT7"]
pub type P4out7R = crate::BitReader;
#[doc = "Field `P4OUT7` writer - P4OUT7"]
pub type P4out7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P4OUT0"]
    #[inline(always)]
    pub fn p4out0(&self) -> P4out0R {
        P4out0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P4OUT1"]
    #[inline(always)]
    pub fn p4out1(&self) -> P4out1R {
        P4out1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P4OUT2"]
    #[inline(always)]
    pub fn p4out2(&self) -> P4out2R {
        P4out2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P4OUT3"]
    #[inline(always)]
    pub fn p4out3(&self) -> P4out3R {
        P4out3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P4OUT4"]
    #[inline(always)]
    pub fn p4out4(&self) -> P4out4R {
        P4out4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P4OUT5"]
    #[inline(always)]
    pub fn p4out5(&self) -> P4out5R {
        P4out5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P4OUT6"]
    #[inline(always)]
    pub fn p4out6(&self) -> P4out6R {
        P4out6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P4OUT7"]
    #[inline(always)]
    pub fn p4out7(&self) -> P4out7R {
        P4out7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4OUT0"]
    #[inline(always)]
    pub fn p4out0(&mut self) -> P4out0W<'_, P4outSpec> {
        P4out0W::new(self, 0)
    }
    #[doc = "Bit 1 - P4OUT1"]
    #[inline(always)]
    pub fn p4out1(&mut self) -> P4out1W<'_, P4outSpec> {
        P4out1W::new(self, 1)
    }
    #[doc = "Bit 2 - P4OUT2"]
    #[inline(always)]
    pub fn p4out2(&mut self) -> P4out2W<'_, P4outSpec> {
        P4out2W::new(self, 2)
    }
    #[doc = "Bit 3 - P4OUT3"]
    #[inline(always)]
    pub fn p4out3(&mut self) -> P4out3W<'_, P4outSpec> {
        P4out3W::new(self, 3)
    }
    #[doc = "Bit 4 - P4OUT4"]
    #[inline(always)]
    pub fn p4out4(&mut self) -> P4out4W<'_, P4outSpec> {
        P4out4W::new(self, 4)
    }
    #[doc = "Bit 5 - P4OUT5"]
    #[inline(always)]
    pub fn p4out5(&mut self) -> P4out5W<'_, P4outSpec> {
        P4out5W::new(self, 5)
    }
    #[doc = "Bit 6 - P4OUT6"]
    #[inline(always)]
    pub fn p4out6(&mut self) -> P4out6W<'_, P4outSpec> {
        P4out6W::new(self, 6)
    }
    #[doc = "Bit 7 - P4OUT7"]
    #[inline(always)]
    pub fn p4out7(&mut self) -> P4out7W<'_, P4outSpec> {
        P4out7W::new(self, 7)
    }
}
#[doc = "Port 4 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p4out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4outSpec;
impl crate::RegisterSpec for P4outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4out::R`](R) reader structure"]
impl crate::Readable for P4outSpec {}
#[doc = "`write(|w| ..)` method takes [`p4out::W`](W) writer structure"]
impl crate::Writable for P4outSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P4OUT to value 0"]
impl crate::Resettable for P4outSpec {}
