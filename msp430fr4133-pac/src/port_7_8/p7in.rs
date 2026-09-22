#[doc = "Register `P7IN` reader"]
pub type R = crate::R<P7inSpec>;
#[doc = "Register `P7IN` writer"]
pub type W = crate::W<P7inSpec>;
#[doc = "Field `P7IN0` reader - P7IN0"]
pub type P7in0R = crate::BitReader;
#[doc = "Field `P7IN0` writer - P7IN0"]
pub type P7in0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7IN1` reader - P7IN1"]
pub type P7in1R = crate::BitReader;
#[doc = "Field `P7IN1` writer - P7IN1"]
pub type P7in1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7IN2` reader - P7IN2"]
pub type P7in2R = crate::BitReader;
#[doc = "Field `P7IN2` writer - P7IN2"]
pub type P7in2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7IN3` reader - P7IN3"]
pub type P7in3R = crate::BitReader;
#[doc = "Field `P7IN3` writer - P7IN3"]
pub type P7in3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7IN4` reader - P7IN4"]
pub type P7in4R = crate::BitReader;
#[doc = "Field `P7IN4` writer - P7IN4"]
pub type P7in4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7IN5` reader - P7IN5"]
pub type P7in5R = crate::BitReader;
#[doc = "Field `P7IN5` writer - P7IN5"]
pub type P7in5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7IN6` reader - P7IN6"]
pub type P7in6R = crate::BitReader;
#[doc = "Field `P7IN6` writer - P7IN6"]
pub type P7in6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7IN7` reader - P7IN7"]
pub type P7in7R = crate::BitReader;
#[doc = "Field `P7IN7` writer - P7IN7"]
pub type P7in7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P7IN0"]
    #[inline(always)]
    pub fn p7in0(&self) -> P7in0R {
        P7in0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P7IN1"]
    #[inline(always)]
    pub fn p7in1(&self) -> P7in1R {
        P7in1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P7IN2"]
    #[inline(always)]
    pub fn p7in2(&self) -> P7in2R {
        P7in2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P7IN3"]
    #[inline(always)]
    pub fn p7in3(&self) -> P7in3R {
        P7in3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P7IN4"]
    #[inline(always)]
    pub fn p7in4(&self) -> P7in4R {
        P7in4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P7IN5"]
    #[inline(always)]
    pub fn p7in5(&self) -> P7in5R {
        P7in5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P7IN6"]
    #[inline(always)]
    pub fn p7in6(&self) -> P7in6R {
        P7in6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P7IN7"]
    #[inline(always)]
    pub fn p7in7(&self) -> P7in7R {
        P7in7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7IN0"]
    #[inline(always)]
    pub fn p7in0(&mut self) -> P7in0W<'_, P7inSpec> {
        P7in0W::new(self, 0)
    }
    #[doc = "Bit 1 - P7IN1"]
    #[inline(always)]
    pub fn p7in1(&mut self) -> P7in1W<'_, P7inSpec> {
        P7in1W::new(self, 1)
    }
    #[doc = "Bit 2 - P7IN2"]
    #[inline(always)]
    pub fn p7in2(&mut self) -> P7in2W<'_, P7inSpec> {
        P7in2W::new(self, 2)
    }
    #[doc = "Bit 3 - P7IN3"]
    #[inline(always)]
    pub fn p7in3(&mut self) -> P7in3W<'_, P7inSpec> {
        P7in3W::new(self, 3)
    }
    #[doc = "Bit 4 - P7IN4"]
    #[inline(always)]
    pub fn p7in4(&mut self) -> P7in4W<'_, P7inSpec> {
        P7in4W::new(self, 4)
    }
    #[doc = "Bit 5 - P7IN5"]
    #[inline(always)]
    pub fn p7in5(&mut self) -> P7in5W<'_, P7inSpec> {
        P7in5W::new(self, 5)
    }
    #[doc = "Bit 6 - P7IN6"]
    #[inline(always)]
    pub fn p7in6(&mut self) -> P7in6W<'_, P7inSpec> {
        P7in6W::new(self, 6)
    }
    #[doc = "Bit 7 - P7IN7"]
    #[inline(always)]
    pub fn p7in7(&mut self) -> P7in7W<'_, P7inSpec> {
        P7in7W::new(self, 7)
    }
}
#[doc = "Port 7 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p7in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7inSpec;
impl crate::RegisterSpec for P7inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7in::R`](R) reader structure"]
impl crate::Readable for P7inSpec {}
#[doc = "`write(|w| ..)` method takes [`p7in::W`](W) writer structure"]
impl crate::Writable for P7inSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P7IN to value 0"]
impl crate::Resettable for P7inSpec {}
