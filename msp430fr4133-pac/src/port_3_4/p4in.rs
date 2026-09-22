#[doc = "Register `P4IN` reader"]
pub type R = crate::R<P4inSpec>;
#[doc = "Register `P4IN` writer"]
pub type W = crate::W<P4inSpec>;
#[doc = "Field `P4IN0` reader - P4IN0"]
pub type P4in0R = crate::BitReader;
#[doc = "Field `P4IN0` writer - P4IN0"]
pub type P4in0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4IN1` reader - P4IN1"]
pub type P4in1R = crate::BitReader;
#[doc = "Field `P4IN1` writer - P4IN1"]
pub type P4in1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4IN2` reader - P4IN2"]
pub type P4in2R = crate::BitReader;
#[doc = "Field `P4IN2` writer - P4IN2"]
pub type P4in2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4IN3` reader - P4IN3"]
pub type P4in3R = crate::BitReader;
#[doc = "Field `P4IN3` writer - P4IN3"]
pub type P4in3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4IN4` reader - P4IN4"]
pub type P4in4R = crate::BitReader;
#[doc = "Field `P4IN4` writer - P4IN4"]
pub type P4in4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4IN5` reader - P4IN5"]
pub type P4in5R = crate::BitReader;
#[doc = "Field `P4IN5` writer - P4IN5"]
pub type P4in5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4IN6` reader - P4IN6"]
pub type P4in6R = crate::BitReader;
#[doc = "Field `P4IN6` writer - P4IN6"]
pub type P4in6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4IN7` reader - P4IN7"]
pub type P4in7R = crate::BitReader;
#[doc = "Field `P4IN7` writer - P4IN7"]
pub type P4in7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P4IN0"]
    #[inline(always)]
    pub fn p4in0(&self) -> P4in0R {
        P4in0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P4IN1"]
    #[inline(always)]
    pub fn p4in1(&self) -> P4in1R {
        P4in1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P4IN2"]
    #[inline(always)]
    pub fn p4in2(&self) -> P4in2R {
        P4in2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P4IN3"]
    #[inline(always)]
    pub fn p4in3(&self) -> P4in3R {
        P4in3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P4IN4"]
    #[inline(always)]
    pub fn p4in4(&self) -> P4in4R {
        P4in4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P4IN5"]
    #[inline(always)]
    pub fn p4in5(&self) -> P4in5R {
        P4in5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P4IN6"]
    #[inline(always)]
    pub fn p4in6(&self) -> P4in6R {
        P4in6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P4IN7"]
    #[inline(always)]
    pub fn p4in7(&self) -> P4in7R {
        P4in7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4IN0"]
    #[inline(always)]
    pub fn p4in0(&mut self) -> P4in0W<'_, P4inSpec> {
        P4in0W::new(self, 0)
    }
    #[doc = "Bit 1 - P4IN1"]
    #[inline(always)]
    pub fn p4in1(&mut self) -> P4in1W<'_, P4inSpec> {
        P4in1W::new(self, 1)
    }
    #[doc = "Bit 2 - P4IN2"]
    #[inline(always)]
    pub fn p4in2(&mut self) -> P4in2W<'_, P4inSpec> {
        P4in2W::new(self, 2)
    }
    #[doc = "Bit 3 - P4IN3"]
    #[inline(always)]
    pub fn p4in3(&mut self) -> P4in3W<'_, P4inSpec> {
        P4in3W::new(self, 3)
    }
    #[doc = "Bit 4 - P4IN4"]
    #[inline(always)]
    pub fn p4in4(&mut self) -> P4in4W<'_, P4inSpec> {
        P4in4W::new(self, 4)
    }
    #[doc = "Bit 5 - P4IN5"]
    #[inline(always)]
    pub fn p4in5(&mut self) -> P4in5W<'_, P4inSpec> {
        P4in5W::new(self, 5)
    }
    #[doc = "Bit 6 - P4IN6"]
    #[inline(always)]
    pub fn p4in6(&mut self) -> P4in6W<'_, P4inSpec> {
        P4in6W::new(self, 6)
    }
    #[doc = "Bit 7 - P4IN7"]
    #[inline(always)]
    pub fn p4in7(&mut self) -> P4in7W<'_, P4inSpec> {
        P4in7W::new(self, 7)
    }
}
#[doc = "Port 4 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p4in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4inSpec;
impl crate::RegisterSpec for P4inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4in::R`](R) reader structure"]
impl crate::Readable for P4inSpec {}
#[doc = "`write(|w| ..)` method takes [`p4in::W`](W) writer structure"]
impl crate::Writable for P4inSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P4IN to value 0"]
impl crate::Resettable for P4inSpec {}
