#[doc = "Register `P6IN` reader"]
pub type R = crate::R<P6inSpec>;
#[doc = "Register `P6IN` writer"]
pub type W = crate::W<P6inSpec>;
#[doc = "Field `P6IN0` reader - P6IN0"]
pub type P6in0R = crate::BitReader;
#[doc = "Field `P6IN0` writer - P6IN0"]
pub type P6in0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6IN1` reader - P6IN1"]
pub type P6in1R = crate::BitReader;
#[doc = "Field `P6IN1` writer - P6IN1"]
pub type P6in1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6IN2` reader - P6IN2"]
pub type P6in2R = crate::BitReader;
#[doc = "Field `P6IN2` writer - P6IN2"]
pub type P6in2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6IN3` reader - P6IN3"]
pub type P6in3R = crate::BitReader;
#[doc = "Field `P6IN3` writer - P6IN3"]
pub type P6in3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6IN4` reader - P6IN4"]
pub type P6in4R = crate::BitReader;
#[doc = "Field `P6IN4` writer - P6IN4"]
pub type P6in4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6IN5` reader - P6IN5"]
pub type P6in5R = crate::BitReader;
#[doc = "Field `P6IN5` writer - P6IN5"]
pub type P6in5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6IN6` reader - P6IN6"]
pub type P6in6R = crate::BitReader;
#[doc = "Field `P6IN6` writer - P6IN6"]
pub type P6in6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6IN7` reader - P6IN7"]
pub type P6in7R = crate::BitReader;
#[doc = "Field `P6IN7` writer - P6IN7"]
pub type P6in7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P6IN0"]
    #[inline(always)]
    pub fn p6in0(&self) -> P6in0R {
        P6in0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P6IN1"]
    #[inline(always)]
    pub fn p6in1(&self) -> P6in1R {
        P6in1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P6IN2"]
    #[inline(always)]
    pub fn p6in2(&self) -> P6in2R {
        P6in2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P6IN3"]
    #[inline(always)]
    pub fn p6in3(&self) -> P6in3R {
        P6in3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P6IN4"]
    #[inline(always)]
    pub fn p6in4(&self) -> P6in4R {
        P6in4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P6IN5"]
    #[inline(always)]
    pub fn p6in5(&self) -> P6in5R {
        P6in5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P6IN6"]
    #[inline(always)]
    pub fn p6in6(&self) -> P6in6R {
        P6in6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P6IN7"]
    #[inline(always)]
    pub fn p6in7(&self) -> P6in7R {
        P6in7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6IN0"]
    #[inline(always)]
    pub fn p6in0(&mut self) -> P6in0W<'_, P6inSpec> {
        P6in0W::new(self, 0)
    }
    #[doc = "Bit 1 - P6IN1"]
    #[inline(always)]
    pub fn p6in1(&mut self) -> P6in1W<'_, P6inSpec> {
        P6in1W::new(self, 1)
    }
    #[doc = "Bit 2 - P6IN2"]
    #[inline(always)]
    pub fn p6in2(&mut self) -> P6in2W<'_, P6inSpec> {
        P6in2W::new(self, 2)
    }
    #[doc = "Bit 3 - P6IN3"]
    #[inline(always)]
    pub fn p6in3(&mut self) -> P6in3W<'_, P6inSpec> {
        P6in3W::new(self, 3)
    }
    #[doc = "Bit 4 - P6IN4"]
    #[inline(always)]
    pub fn p6in4(&mut self) -> P6in4W<'_, P6inSpec> {
        P6in4W::new(self, 4)
    }
    #[doc = "Bit 5 - P6IN5"]
    #[inline(always)]
    pub fn p6in5(&mut self) -> P6in5W<'_, P6inSpec> {
        P6in5W::new(self, 5)
    }
    #[doc = "Bit 6 - P6IN6"]
    #[inline(always)]
    pub fn p6in6(&mut self) -> P6in6W<'_, P6inSpec> {
        P6in6W::new(self, 6)
    }
    #[doc = "Bit 7 - P6IN7"]
    #[inline(always)]
    pub fn p6in7(&mut self) -> P6in7W<'_, P6inSpec> {
        P6in7W::new(self, 7)
    }
}
#[doc = "Port 6 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p6in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6inSpec;
impl crate::RegisterSpec for P6inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6in::R`](R) reader structure"]
impl crate::Readable for P6inSpec {}
#[doc = "`write(|w| ..)` method takes [`p6in::W`](W) writer structure"]
impl crate::Writable for P6inSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P6IN to value 0"]
impl crate::Resettable for P6inSpec {}
