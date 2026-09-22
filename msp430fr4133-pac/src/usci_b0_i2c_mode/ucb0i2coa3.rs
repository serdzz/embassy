#[doc = "Register `UCB0I2COA3` reader"]
pub type R = crate::R<Ucb0i2coa3Spec>;
#[doc = "Register `UCB0I2COA3` writer"]
pub type W = crate::W<Ucb0i2coa3Spec>;
#[doc = "Field `UCOA0` reader - I2C Own Address Bit 0"]
pub type Ucoa0R = crate::BitReader;
#[doc = "Field `UCOA0` writer - I2C Own Address Bit 0"]
pub type Ucoa0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA1` reader - I2C Own Address Bit 1"]
pub type Ucoa1R = crate::BitReader;
#[doc = "Field `UCOA1` writer - I2C Own Address Bit 1"]
pub type Ucoa1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA2` reader - I2C Own Address Bit 2"]
pub type Ucoa2R = crate::BitReader;
#[doc = "Field `UCOA2` writer - I2C Own Address Bit 2"]
pub type Ucoa2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA3` reader - I2C Own Address Bit 3"]
pub type Ucoa3R = crate::BitReader;
#[doc = "Field `UCOA3` writer - I2C Own Address Bit 3"]
pub type Ucoa3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA4` reader - I2C Own Address Bit 4"]
pub type Ucoa4R = crate::BitReader;
#[doc = "Field `UCOA4` writer - I2C Own Address Bit 4"]
pub type Ucoa4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA5` reader - I2C Own Address Bit 5"]
pub type Ucoa5R = crate::BitReader;
#[doc = "Field `UCOA5` writer - I2C Own Address Bit 5"]
pub type Ucoa5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA6` reader - I2C Own Address Bit 6"]
pub type Ucoa6R = crate::BitReader;
#[doc = "Field `UCOA6` writer - I2C Own Address Bit 6"]
pub type Ucoa6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA7` reader - I2C Own Address Bit 7"]
pub type Ucoa7R = crate::BitReader;
#[doc = "Field `UCOA7` writer - I2C Own Address Bit 7"]
pub type Ucoa7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA8` reader - I2C Own Address Bit 8"]
pub type Ucoa8R = crate::BitReader;
#[doc = "Field `UCOA8` writer - I2C Own Address Bit 8"]
pub type Ucoa8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA9` reader - I2C Own Address Bit 9"]
pub type Ucoa9R = crate::BitReader;
#[doc = "Field `UCOA9` writer - I2C Own Address Bit 9"]
pub type Ucoa9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOAEN` reader - I2C Own Address enable"]
pub type UcoaenR = crate::BitReader;
#[doc = "Field `UCOAEN` writer - I2C Own Address enable"]
pub type UcoaenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C Own Address Bit 0"]
    #[inline(always)]
    pub fn ucoa0(&self) -> Ucoa0R {
        Ucoa0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Own Address Bit 1"]
    #[inline(always)]
    pub fn ucoa1(&self) -> Ucoa1R {
        Ucoa1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Own Address Bit 2"]
    #[inline(always)]
    pub fn ucoa2(&self) -> Ucoa2R {
        Ucoa2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Own Address Bit 3"]
    #[inline(always)]
    pub fn ucoa3(&self) -> Ucoa3R {
        Ucoa3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Own Address Bit 4"]
    #[inline(always)]
    pub fn ucoa4(&self) -> Ucoa4R {
        Ucoa4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Own Address Bit 5"]
    #[inline(always)]
    pub fn ucoa5(&self) -> Ucoa5R {
        Ucoa5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Own Address Bit 6"]
    #[inline(always)]
    pub fn ucoa6(&self) -> Ucoa6R {
        Ucoa6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Own Address Bit 7"]
    #[inline(always)]
    pub fn ucoa7(&self) -> Ucoa7R {
        Ucoa7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Own Address Bit 8"]
    #[inline(always)]
    pub fn ucoa8(&self) -> Ucoa8R {
        Ucoa8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Own Address Bit 9"]
    #[inline(always)]
    pub fn ucoa9(&self) -> Ucoa9R {
        Ucoa9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C Own Address enable"]
    #[inline(always)]
    pub fn ucoaen(&self) -> UcoaenR {
        UcoaenR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Own Address Bit 0"]
    #[inline(always)]
    pub fn ucoa0(&mut self) -> Ucoa0W<'_, Ucb0i2coa3Spec> {
        Ucoa0W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C Own Address Bit 1"]
    #[inline(always)]
    pub fn ucoa1(&mut self) -> Ucoa1W<'_, Ucb0i2coa3Spec> {
        Ucoa1W::new(self, 1)
    }
    #[doc = "Bit 2 - I2C Own Address Bit 2"]
    #[inline(always)]
    pub fn ucoa2(&mut self) -> Ucoa2W<'_, Ucb0i2coa3Spec> {
        Ucoa2W::new(self, 2)
    }
    #[doc = "Bit 3 - I2C Own Address Bit 3"]
    #[inline(always)]
    pub fn ucoa3(&mut self) -> Ucoa3W<'_, Ucb0i2coa3Spec> {
        Ucoa3W::new(self, 3)
    }
    #[doc = "Bit 4 - I2C Own Address Bit 4"]
    #[inline(always)]
    pub fn ucoa4(&mut self) -> Ucoa4W<'_, Ucb0i2coa3Spec> {
        Ucoa4W::new(self, 4)
    }
    #[doc = "Bit 5 - I2C Own Address Bit 5"]
    #[inline(always)]
    pub fn ucoa5(&mut self) -> Ucoa5W<'_, Ucb0i2coa3Spec> {
        Ucoa5W::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Own Address Bit 6"]
    #[inline(always)]
    pub fn ucoa6(&mut self) -> Ucoa6W<'_, Ucb0i2coa3Spec> {
        Ucoa6W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C Own Address Bit 7"]
    #[inline(always)]
    pub fn ucoa7(&mut self) -> Ucoa7W<'_, Ucb0i2coa3Spec> {
        Ucoa7W::new(self, 7)
    }
    #[doc = "Bit 8 - I2C Own Address Bit 8"]
    #[inline(always)]
    pub fn ucoa8(&mut self) -> Ucoa8W<'_, Ucb0i2coa3Spec> {
        Ucoa8W::new(self, 8)
    }
    #[doc = "Bit 9 - I2C Own Address Bit 9"]
    #[inline(always)]
    pub fn ucoa9(&mut self) -> Ucoa9W<'_, Ucb0i2coa3Spec> {
        Ucoa9W::new(self, 9)
    }
    #[doc = "Bit 10 - I2C Own Address enable"]
    #[inline(always)]
    pub fn ucoaen(&mut self) -> UcoaenW<'_, Ucb0i2coa3Spec> {
        UcoaenW::new(self, 10)
    }
}
#[doc = "USCI B0 I2C Own Address 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2coa3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2coa3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0i2coa3Spec;
impl crate::RegisterSpec for Ucb0i2coa3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0i2coa3::R`](R) reader structure"]
impl crate::Readable for Ucb0i2coa3Spec {}
#[doc = "`write(|w| ..)` method takes [`ucb0i2coa3::W`](W) writer structure"]
impl crate::Writable for Ucb0i2coa3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0I2COA3 to value 0"]
impl crate::Resettable for Ucb0i2coa3Spec {}
