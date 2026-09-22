#[doc = "Register `UCB0I2CSA` reader"]
pub type R = crate::R<Ucb0i2csaSpec>;
#[doc = "Register `UCB0I2CSA` writer"]
pub type W = crate::W<Ucb0i2csaSpec>;
#[doc = "Field `UCSA0` reader - I2C Slave Address Bit 0"]
pub type Ucsa0R = crate::BitReader;
#[doc = "Field `UCSA0` writer - I2C Slave Address Bit 0"]
pub type Ucsa0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA1` reader - I2C Slave Address Bit 1"]
pub type Ucsa1R = crate::BitReader;
#[doc = "Field `UCSA1` writer - I2C Slave Address Bit 1"]
pub type Ucsa1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA2` reader - I2C Slave Address Bit 2"]
pub type Ucsa2R = crate::BitReader;
#[doc = "Field `UCSA2` writer - I2C Slave Address Bit 2"]
pub type Ucsa2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA3` reader - I2C Slave Address Bit 3"]
pub type Ucsa3R = crate::BitReader;
#[doc = "Field `UCSA3` writer - I2C Slave Address Bit 3"]
pub type Ucsa3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA4` reader - I2C Slave Address Bit 4"]
pub type Ucsa4R = crate::BitReader;
#[doc = "Field `UCSA4` writer - I2C Slave Address Bit 4"]
pub type Ucsa4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA5` reader - I2C Slave Address Bit 5"]
pub type Ucsa5R = crate::BitReader;
#[doc = "Field `UCSA5` writer - I2C Slave Address Bit 5"]
pub type Ucsa5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA6` reader - I2C Slave Address Bit 6"]
pub type Ucsa6R = crate::BitReader;
#[doc = "Field `UCSA6` writer - I2C Slave Address Bit 6"]
pub type Ucsa6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA7` reader - I2C Slave Address Bit 7"]
pub type Ucsa7R = crate::BitReader;
#[doc = "Field `UCSA7` writer - I2C Slave Address Bit 7"]
pub type Ucsa7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA8` reader - I2C Slave Address Bit 8"]
pub type Ucsa8R = crate::BitReader;
#[doc = "Field `UCSA8` writer - I2C Slave Address Bit 8"]
pub type Ucsa8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA9` reader - I2C Slave Address Bit 9"]
pub type Ucsa9R = crate::BitReader;
#[doc = "Field `UCSA9` writer - I2C Slave Address Bit 9"]
pub type Ucsa9W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C Slave Address Bit 0"]
    #[inline(always)]
    pub fn ucsa0(&self) -> Ucsa0R {
        Ucsa0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Slave Address Bit 1"]
    #[inline(always)]
    pub fn ucsa1(&self) -> Ucsa1R {
        Ucsa1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Slave Address Bit 2"]
    #[inline(always)]
    pub fn ucsa2(&self) -> Ucsa2R {
        Ucsa2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Slave Address Bit 3"]
    #[inline(always)]
    pub fn ucsa3(&self) -> Ucsa3R {
        Ucsa3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Slave Address Bit 4"]
    #[inline(always)]
    pub fn ucsa4(&self) -> Ucsa4R {
        Ucsa4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Slave Address Bit 5"]
    #[inline(always)]
    pub fn ucsa5(&self) -> Ucsa5R {
        Ucsa5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Slave Address Bit 6"]
    #[inline(always)]
    pub fn ucsa6(&self) -> Ucsa6R {
        Ucsa6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Slave Address Bit 7"]
    #[inline(always)]
    pub fn ucsa7(&self) -> Ucsa7R {
        Ucsa7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Slave Address Bit 8"]
    #[inline(always)]
    pub fn ucsa8(&self) -> Ucsa8R {
        Ucsa8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Slave Address Bit 9"]
    #[inline(always)]
    pub fn ucsa9(&self) -> Ucsa9R {
        Ucsa9R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Slave Address Bit 0"]
    #[inline(always)]
    pub fn ucsa0(&mut self) -> Ucsa0W<'_, Ucb0i2csaSpec> {
        Ucsa0W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C Slave Address Bit 1"]
    #[inline(always)]
    pub fn ucsa1(&mut self) -> Ucsa1W<'_, Ucb0i2csaSpec> {
        Ucsa1W::new(self, 1)
    }
    #[doc = "Bit 2 - I2C Slave Address Bit 2"]
    #[inline(always)]
    pub fn ucsa2(&mut self) -> Ucsa2W<'_, Ucb0i2csaSpec> {
        Ucsa2W::new(self, 2)
    }
    #[doc = "Bit 3 - I2C Slave Address Bit 3"]
    #[inline(always)]
    pub fn ucsa3(&mut self) -> Ucsa3W<'_, Ucb0i2csaSpec> {
        Ucsa3W::new(self, 3)
    }
    #[doc = "Bit 4 - I2C Slave Address Bit 4"]
    #[inline(always)]
    pub fn ucsa4(&mut self) -> Ucsa4W<'_, Ucb0i2csaSpec> {
        Ucsa4W::new(self, 4)
    }
    #[doc = "Bit 5 - I2C Slave Address Bit 5"]
    #[inline(always)]
    pub fn ucsa5(&mut self) -> Ucsa5W<'_, Ucb0i2csaSpec> {
        Ucsa5W::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Slave Address Bit 6"]
    #[inline(always)]
    pub fn ucsa6(&mut self) -> Ucsa6W<'_, Ucb0i2csaSpec> {
        Ucsa6W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C Slave Address Bit 7"]
    #[inline(always)]
    pub fn ucsa7(&mut self) -> Ucsa7W<'_, Ucb0i2csaSpec> {
        Ucsa7W::new(self, 7)
    }
    #[doc = "Bit 8 - I2C Slave Address Bit 8"]
    #[inline(always)]
    pub fn ucsa8(&mut self) -> Ucsa8W<'_, Ucb0i2csaSpec> {
        Ucsa8W::new(self, 8)
    }
    #[doc = "Bit 9 - I2C Slave Address Bit 9"]
    #[inline(always)]
    pub fn ucsa9(&mut self) -> Ucsa9W<'_, Ucb0i2csaSpec> {
        Ucsa9W::new(self, 9)
    }
}
#[doc = "USCI B0 I2C Slave Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2csa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2csa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0i2csaSpec;
impl crate::RegisterSpec for Ucb0i2csaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0i2csa::R`](R) reader structure"]
impl crate::Readable for Ucb0i2csaSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0i2csa::W`](W) writer structure"]
impl crate::Writable for Ucb0i2csaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0I2CSA to value 0"]
impl crate::Resettable for Ucb0i2csaSpec {}
