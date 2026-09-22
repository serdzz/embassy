#[doc = "Register `P5IN` reader"]
pub type R = crate::R<P5inSpec>;
#[doc = "Register `P5IN` writer"]
pub type W = crate::W<P5inSpec>;
#[doc = "Field `P5IN0` reader - P5IN0"]
pub type P5in0R = crate::BitReader;
#[doc = "Field `P5IN0` writer - P5IN0"]
pub type P5in0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5IN1` reader - P5IN1"]
pub type P5in1R = crate::BitReader;
#[doc = "Field `P5IN1` writer - P5IN1"]
pub type P5in1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5IN2` reader - P5IN2"]
pub type P5in2R = crate::BitReader;
#[doc = "Field `P5IN2` writer - P5IN2"]
pub type P5in2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5IN3` reader - P5IN3"]
pub type P5in3R = crate::BitReader;
#[doc = "Field `P5IN3` writer - P5IN3"]
pub type P5in3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5IN4` reader - P5IN4"]
pub type P5in4R = crate::BitReader;
#[doc = "Field `P5IN4` writer - P5IN4"]
pub type P5in4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5IN5` reader - P5IN5"]
pub type P5in5R = crate::BitReader;
#[doc = "Field `P5IN5` writer - P5IN5"]
pub type P5in5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5IN6` reader - P5IN6"]
pub type P5in6R = crate::BitReader;
#[doc = "Field `P5IN6` writer - P5IN6"]
pub type P5in6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5IN7` reader - P5IN7"]
pub type P5in7R = crate::BitReader;
#[doc = "Field `P5IN7` writer - P5IN7"]
pub type P5in7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P5IN0"]
    #[inline(always)]
    pub fn p5in0(&self) -> P5in0R {
        P5in0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P5IN1"]
    #[inline(always)]
    pub fn p5in1(&self) -> P5in1R {
        P5in1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P5IN2"]
    #[inline(always)]
    pub fn p5in2(&self) -> P5in2R {
        P5in2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P5IN3"]
    #[inline(always)]
    pub fn p5in3(&self) -> P5in3R {
        P5in3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P5IN4"]
    #[inline(always)]
    pub fn p5in4(&self) -> P5in4R {
        P5in4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P5IN5"]
    #[inline(always)]
    pub fn p5in5(&self) -> P5in5R {
        P5in5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P5IN6"]
    #[inline(always)]
    pub fn p5in6(&self) -> P5in6R {
        P5in6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P5IN7"]
    #[inline(always)]
    pub fn p5in7(&self) -> P5in7R {
        P5in7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5IN0"]
    #[inline(always)]
    pub fn p5in0(&mut self) -> P5in0W<'_, P5inSpec> {
        P5in0W::new(self, 0)
    }
    #[doc = "Bit 1 - P5IN1"]
    #[inline(always)]
    pub fn p5in1(&mut self) -> P5in1W<'_, P5inSpec> {
        P5in1W::new(self, 1)
    }
    #[doc = "Bit 2 - P5IN2"]
    #[inline(always)]
    pub fn p5in2(&mut self) -> P5in2W<'_, P5inSpec> {
        P5in2W::new(self, 2)
    }
    #[doc = "Bit 3 - P5IN3"]
    #[inline(always)]
    pub fn p5in3(&mut self) -> P5in3W<'_, P5inSpec> {
        P5in3W::new(self, 3)
    }
    #[doc = "Bit 4 - P5IN4"]
    #[inline(always)]
    pub fn p5in4(&mut self) -> P5in4W<'_, P5inSpec> {
        P5in4W::new(self, 4)
    }
    #[doc = "Bit 5 - P5IN5"]
    #[inline(always)]
    pub fn p5in5(&mut self) -> P5in5W<'_, P5inSpec> {
        P5in5W::new(self, 5)
    }
    #[doc = "Bit 6 - P5IN6"]
    #[inline(always)]
    pub fn p5in6(&mut self) -> P5in6W<'_, P5inSpec> {
        P5in6W::new(self, 6)
    }
    #[doc = "Bit 7 - P5IN7"]
    #[inline(always)]
    pub fn p5in7(&mut self) -> P5in7W<'_, P5inSpec> {
        P5in7W::new(self, 7)
    }
}
#[doc = "Port 5 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p5in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5inSpec;
impl crate::RegisterSpec for P5inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5in::R`](R) reader structure"]
impl crate::Readable for P5inSpec {}
#[doc = "`write(|w| ..)` method takes [`p5in::W`](W) writer structure"]
impl crate::Writable for P5inSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P5IN to value 0"]
impl crate::Resettable for P5inSpec {}
