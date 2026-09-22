#[doc = "Register `P1IN` reader"]
pub type R = crate::R<P1inSpec>;
#[doc = "Register `P1IN` writer"]
pub type W = crate::W<P1inSpec>;
#[doc = "Field `P1IN0` reader - P1IN0"]
pub type P1in0R = crate::BitReader;
#[doc = "Field `P1IN0` writer - P1IN0"]
pub type P1in0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IN1` reader - P1IN1"]
pub type P1in1R = crate::BitReader;
#[doc = "Field `P1IN1` writer - P1IN1"]
pub type P1in1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IN2` reader - P1IN2"]
pub type P1in2R = crate::BitReader;
#[doc = "Field `P1IN2` writer - P1IN2"]
pub type P1in2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IN3` reader - P1IN3"]
pub type P1in3R = crate::BitReader;
#[doc = "Field `P1IN3` writer - P1IN3"]
pub type P1in3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IN4` reader - P1IN4"]
pub type P1in4R = crate::BitReader;
#[doc = "Field `P1IN4` writer - P1IN4"]
pub type P1in4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IN5` reader - P1IN5"]
pub type P1in5R = crate::BitReader;
#[doc = "Field `P1IN5` writer - P1IN5"]
pub type P1in5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IN6` reader - P1IN6"]
pub type P1in6R = crate::BitReader;
#[doc = "Field `P1IN6` writer - P1IN6"]
pub type P1in6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IN7` reader - P1IN7"]
pub type P1in7R = crate::BitReader;
#[doc = "Field `P1IN7` writer - P1IN7"]
pub type P1in7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P1IN0"]
    #[inline(always)]
    pub fn p1in0(&self) -> P1in0R {
        P1in0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1IN1"]
    #[inline(always)]
    pub fn p1in1(&self) -> P1in1R {
        P1in1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1IN2"]
    #[inline(always)]
    pub fn p1in2(&self) -> P1in2R {
        P1in2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1IN3"]
    #[inline(always)]
    pub fn p1in3(&self) -> P1in3R {
        P1in3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1IN4"]
    #[inline(always)]
    pub fn p1in4(&self) -> P1in4R {
        P1in4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1IN5"]
    #[inline(always)]
    pub fn p1in5(&self) -> P1in5R {
        P1in5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1IN6"]
    #[inline(always)]
    pub fn p1in6(&self) -> P1in6R {
        P1in6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1IN7"]
    #[inline(always)]
    pub fn p1in7(&self) -> P1in7R {
        P1in7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1IN0"]
    #[inline(always)]
    pub fn p1in0(&mut self) -> P1in0W<'_, P1inSpec> {
        P1in0W::new(self, 0)
    }
    #[doc = "Bit 1 - P1IN1"]
    #[inline(always)]
    pub fn p1in1(&mut self) -> P1in1W<'_, P1inSpec> {
        P1in1W::new(self, 1)
    }
    #[doc = "Bit 2 - P1IN2"]
    #[inline(always)]
    pub fn p1in2(&mut self) -> P1in2W<'_, P1inSpec> {
        P1in2W::new(self, 2)
    }
    #[doc = "Bit 3 - P1IN3"]
    #[inline(always)]
    pub fn p1in3(&mut self) -> P1in3W<'_, P1inSpec> {
        P1in3W::new(self, 3)
    }
    #[doc = "Bit 4 - P1IN4"]
    #[inline(always)]
    pub fn p1in4(&mut self) -> P1in4W<'_, P1inSpec> {
        P1in4W::new(self, 4)
    }
    #[doc = "Bit 5 - P1IN5"]
    #[inline(always)]
    pub fn p1in5(&mut self) -> P1in5W<'_, P1inSpec> {
        P1in5W::new(self, 5)
    }
    #[doc = "Bit 6 - P1IN6"]
    #[inline(always)]
    pub fn p1in6(&mut self) -> P1in6W<'_, P1inSpec> {
        P1in6W::new(self, 6)
    }
    #[doc = "Bit 7 - P1IN7"]
    #[inline(always)]
    pub fn p1in7(&mut self) -> P1in7W<'_, P1inSpec> {
        P1in7W::new(self, 7)
    }
}
#[doc = "Port 1 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p1in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1inSpec;
impl crate::RegisterSpec for P1inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1in::R`](R) reader structure"]
impl crate::Readable for P1inSpec {}
#[doc = "`write(|w| ..)` method takes [`p1in::W`](W) writer structure"]
impl crate::Writable for P1inSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P1IN to value 0"]
impl crate::Resettable for P1inSpec {}
