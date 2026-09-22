#[doc = "Register `P8IN` reader"]
pub type R = crate::R<P8inSpec>;
#[doc = "Register `P8IN` writer"]
pub type W = crate::W<P8inSpec>;
#[doc = "Field `P8IN0` reader - P8IN0"]
pub type P8in0R = crate::BitReader;
#[doc = "Field `P8IN0` writer - P8IN0"]
pub type P8in0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8IN1` reader - P8IN1"]
pub type P8in1R = crate::BitReader;
#[doc = "Field `P8IN1` writer - P8IN1"]
pub type P8in1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8IN2` reader - P8IN2"]
pub type P8in2R = crate::BitReader;
#[doc = "Field `P8IN2` writer - P8IN2"]
pub type P8in2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8IN3` reader - P8IN3"]
pub type P8in3R = crate::BitReader;
#[doc = "Field `P8IN3` writer - P8IN3"]
pub type P8in3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8IN4` reader - P8IN4"]
pub type P8in4R = crate::BitReader;
#[doc = "Field `P8IN4` writer - P8IN4"]
pub type P8in4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8IN5` reader - P8IN5"]
pub type P8in5R = crate::BitReader;
#[doc = "Field `P8IN5` writer - P8IN5"]
pub type P8in5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8IN6` reader - P8IN6"]
pub type P8in6R = crate::BitReader;
#[doc = "Field `P8IN6` writer - P8IN6"]
pub type P8in6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8IN7` reader - P8IN7"]
pub type P8in7R = crate::BitReader;
#[doc = "Field `P8IN7` writer - P8IN7"]
pub type P8in7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P8IN0"]
    #[inline(always)]
    pub fn p8in0(&self) -> P8in0R {
        P8in0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P8IN1"]
    #[inline(always)]
    pub fn p8in1(&self) -> P8in1R {
        P8in1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P8IN2"]
    #[inline(always)]
    pub fn p8in2(&self) -> P8in2R {
        P8in2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P8IN3"]
    #[inline(always)]
    pub fn p8in3(&self) -> P8in3R {
        P8in3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P8IN4"]
    #[inline(always)]
    pub fn p8in4(&self) -> P8in4R {
        P8in4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P8IN5"]
    #[inline(always)]
    pub fn p8in5(&self) -> P8in5R {
        P8in5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P8IN6"]
    #[inline(always)]
    pub fn p8in6(&self) -> P8in6R {
        P8in6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P8IN7"]
    #[inline(always)]
    pub fn p8in7(&self) -> P8in7R {
        P8in7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8IN0"]
    #[inline(always)]
    pub fn p8in0(&mut self) -> P8in0W<'_, P8inSpec> {
        P8in0W::new(self, 0)
    }
    #[doc = "Bit 1 - P8IN1"]
    #[inline(always)]
    pub fn p8in1(&mut self) -> P8in1W<'_, P8inSpec> {
        P8in1W::new(self, 1)
    }
    #[doc = "Bit 2 - P8IN2"]
    #[inline(always)]
    pub fn p8in2(&mut self) -> P8in2W<'_, P8inSpec> {
        P8in2W::new(self, 2)
    }
    #[doc = "Bit 3 - P8IN3"]
    #[inline(always)]
    pub fn p8in3(&mut self) -> P8in3W<'_, P8inSpec> {
        P8in3W::new(self, 3)
    }
    #[doc = "Bit 4 - P8IN4"]
    #[inline(always)]
    pub fn p8in4(&mut self) -> P8in4W<'_, P8inSpec> {
        P8in4W::new(self, 4)
    }
    #[doc = "Bit 5 - P8IN5"]
    #[inline(always)]
    pub fn p8in5(&mut self) -> P8in5W<'_, P8inSpec> {
        P8in5W::new(self, 5)
    }
    #[doc = "Bit 6 - P8IN6"]
    #[inline(always)]
    pub fn p8in6(&mut self) -> P8in6W<'_, P8inSpec> {
        P8in6W::new(self, 6)
    }
    #[doc = "Bit 7 - P8IN7"]
    #[inline(always)]
    pub fn p8in7(&mut self) -> P8in7W<'_, P8inSpec> {
        P8in7W::new(self, 7)
    }
}
#[doc = "Port 8 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p8in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8inSpec;
impl crate::RegisterSpec for P8inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8in::R`](R) reader structure"]
impl crate::Readable for P8inSpec {}
#[doc = "`write(|w| ..)` method takes [`p8in::W`](W) writer structure"]
impl crate::Writable for P8inSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P8IN to value 0"]
impl crate::Resettable for P8inSpec {}
