#[doc = "Register `P3IN` reader"]
pub type R = crate::R<P3inSpec>;
#[doc = "Register `P3IN` writer"]
pub type W = crate::W<P3inSpec>;
#[doc = "Field `P3IN0` reader - P3IN0"]
pub type P3in0R = crate::BitReader;
#[doc = "Field `P3IN0` writer - P3IN0"]
pub type P3in0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3IN1` reader - P3IN1"]
pub type P3in1R = crate::BitReader;
#[doc = "Field `P3IN1` writer - P3IN1"]
pub type P3in1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3IN2` reader - P3IN2"]
pub type P3in2R = crate::BitReader;
#[doc = "Field `P3IN2` writer - P3IN2"]
pub type P3in2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3IN3` reader - P3IN3"]
pub type P3in3R = crate::BitReader;
#[doc = "Field `P3IN3` writer - P3IN3"]
pub type P3in3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3IN4` reader - P3IN4"]
pub type P3in4R = crate::BitReader;
#[doc = "Field `P3IN4` writer - P3IN4"]
pub type P3in4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3IN5` reader - P3IN5"]
pub type P3in5R = crate::BitReader;
#[doc = "Field `P3IN5` writer - P3IN5"]
pub type P3in5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3IN6` reader - P3IN6"]
pub type P3in6R = crate::BitReader;
#[doc = "Field `P3IN6` writer - P3IN6"]
pub type P3in6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3IN7` reader - P3IN7"]
pub type P3in7R = crate::BitReader;
#[doc = "Field `P3IN7` writer - P3IN7"]
pub type P3in7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P3IN0"]
    #[inline(always)]
    pub fn p3in0(&self) -> P3in0R {
        P3in0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P3IN1"]
    #[inline(always)]
    pub fn p3in1(&self) -> P3in1R {
        P3in1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P3IN2"]
    #[inline(always)]
    pub fn p3in2(&self) -> P3in2R {
        P3in2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3IN3"]
    #[inline(always)]
    pub fn p3in3(&self) -> P3in3R {
        P3in3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P3IN4"]
    #[inline(always)]
    pub fn p3in4(&self) -> P3in4R {
        P3in4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P3IN5"]
    #[inline(always)]
    pub fn p3in5(&self) -> P3in5R {
        P3in5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P3IN6"]
    #[inline(always)]
    pub fn p3in6(&self) -> P3in6R {
        P3in6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P3IN7"]
    #[inline(always)]
    pub fn p3in7(&self) -> P3in7R {
        P3in7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3IN0"]
    #[inline(always)]
    pub fn p3in0(&mut self) -> P3in0W<'_, P3inSpec> {
        P3in0W::new(self, 0)
    }
    #[doc = "Bit 1 - P3IN1"]
    #[inline(always)]
    pub fn p3in1(&mut self) -> P3in1W<'_, P3inSpec> {
        P3in1W::new(self, 1)
    }
    #[doc = "Bit 2 - P3IN2"]
    #[inline(always)]
    pub fn p3in2(&mut self) -> P3in2W<'_, P3inSpec> {
        P3in2W::new(self, 2)
    }
    #[doc = "Bit 3 - P3IN3"]
    #[inline(always)]
    pub fn p3in3(&mut self) -> P3in3W<'_, P3inSpec> {
        P3in3W::new(self, 3)
    }
    #[doc = "Bit 4 - P3IN4"]
    #[inline(always)]
    pub fn p3in4(&mut self) -> P3in4W<'_, P3inSpec> {
        P3in4W::new(self, 4)
    }
    #[doc = "Bit 5 - P3IN5"]
    #[inline(always)]
    pub fn p3in5(&mut self) -> P3in5W<'_, P3inSpec> {
        P3in5W::new(self, 5)
    }
    #[doc = "Bit 6 - P3IN6"]
    #[inline(always)]
    pub fn p3in6(&mut self) -> P3in6W<'_, P3inSpec> {
        P3in6W::new(self, 6)
    }
    #[doc = "Bit 7 - P3IN7"]
    #[inline(always)]
    pub fn p3in7(&mut self) -> P3in7W<'_, P3inSpec> {
        P3in7W::new(self, 7)
    }
}
#[doc = "Port 3 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p3in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3inSpec;
impl crate::RegisterSpec for P3inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3in::R`](R) reader structure"]
impl crate::Readable for P3inSpec {}
#[doc = "`write(|w| ..)` method takes [`p3in::W`](W) writer structure"]
impl crate::Writable for P3inSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P3IN to value 0"]
impl crate::Resettable for P3inSpec {}
