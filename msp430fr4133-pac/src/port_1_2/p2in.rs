#[doc = "Register `P2IN` reader"]
pub type R = crate::R<P2inSpec>;
#[doc = "Register `P2IN` writer"]
pub type W = crate::W<P2inSpec>;
#[doc = "Field `P2IN0` reader - P2IN0"]
pub type P2in0R = crate::BitReader;
#[doc = "Field `P2IN0` writer - P2IN0"]
pub type P2in0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IN1` reader - P2IN1"]
pub type P2in1R = crate::BitReader;
#[doc = "Field `P2IN1` writer - P2IN1"]
pub type P2in1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IN2` reader - P2IN2"]
pub type P2in2R = crate::BitReader;
#[doc = "Field `P2IN2` writer - P2IN2"]
pub type P2in2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IN3` reader - P2IN3"]
pub type P2in3R = crate::BitReader;
#[doc = "Field `P2IN3` writer - P2IN3"]
pub type P2in3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IN4` reader - P2IN4"]
pub type P2in4R = crate::BitReader;
#[doc = "Field `P2IN4` writer - P2IN4"]
pub type P2in4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IN5` reader - P2IN5"]
pub type P2in5R = crate::BitReader;
#[doc = "Field `P2IN5` writer - P2IN5"]
pub type P2in5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IN6` reader - P2IN6"]
pub type P2in6R = crate::BitReader;
#[doc = "Field `P2IN6` writer - P2IN6"]
pub type P2in6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2IN7` reader - P2IN7"]
pub type P2in7R = crate::BitReader;
#[doc = "Field `P2IN7` writer - P2IN7"]
pub type P2in7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P2IN0"]
    #[inline(always)]
    pub fn p2in0(&self) -> P2in0R {
        P2in0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2IN1"]
    #[inline(always)]
    pub fn p2in1(&self) -> P2in1R {
        P2in1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2IN2"]
    #[inline(always)]
    pub fn p2in2(&self) -> P2in2R {
        P2in2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2IN3"]
    #[inline(always)]
    pub fn p2in3(&self) -> P2in3R {
        P2in3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2IN4"]
    #[inline(always)]
    pub fn p2in4(&self) -> P2in4R {
        P2in4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2IN5"]
    #[inline(always)]
    pub fn p2in5(&self) -> P2in5R {
        P2in5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2IN6"]
    #[inline(always)]
    pub fn p2in6(&self) -> P2in6R {
        P2in6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2IN7"]
    #[inline(always)]
    pub fn p2in7(&self) -> P2in7R {
        P2in7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2IN0"]
    #[inline(always)]
    pub fn p2in0(&mut self) -> P2in0W<'_, P2inSpec> {
        P2in0W::new(self, 0)
    }
    #[doc = "Bit 1 - P2IN1"]
    #[inline(always)]
    pub fn p2in1(&mut self) -> P2in1W<'_, P2inSpec> {
        P2in1W::new(self, 1)
    }
    #[doc = "Bit 2 - P2IN2"]
    #[inline(always)]
    pub fn p2in2(&mut self) -> P2in2W<'_, P2inSpec> {
        P2in2W::new(self, 2)
    }
    #[doc = "Bit 3 - P2IN3"]
    #[inline(always)]
    pub fn p2in3(&mut self) -> P2in3W<'_, P2inSpec> {
        P2in3W::new(self, 3)
    }
    #[doc = "Bit 4 - P2IN4"]
    #[inline(always)]
    pub fn p2in4(&mut self) -> P2in4W<'_, P2inSpec> {
        P2in4W::new(self, 4)
    }
    #[doc = "Bit 5 - P2IN5"]
    #[inline(always)]
    pub fn p2in5(&mut self) -> P2in5W<'_, P2inSpec> {
        P2in5W::new(self, 5)
    }
    #[doc = "Bit 6 - P2IN6"]
    #[inline(always)]
    pub fn p2in6(&mut self) -> P2in6W<'_, P2inSpec> {
        P2in6W::new(self, 6)
    }
    #[doc = "Bit 7 - P2IN7"]
    #[inline(always)]
    pub fn p2in7(&mut self) -> P2in7W<'_, P2inSpec> {
        P2in7W::new(self, 7)
    }
}
#[doc = "Port 2 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p2in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2inSpec;
impl crate::RegisterSpec for P2inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2in::R`](R) reader structure"]
impl crate::Readable for P2inSpec {}
#[doc = "`write(|w| ..)` method takes [`p2in::W`](W) writer structure"]
impl crate::Writable for P2inSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2IN to value 0"]
impl crate::Resettable for P2inSpec {}
