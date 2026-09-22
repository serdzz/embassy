#[doc = "Register `UCB0ADDRX` reader"]
pub type R = crate::R<Ucb0addrxSpec>;
#[doc = "Register `UCB0ADDRX` writer"]
pub type W = crate::W<Ucb0addrxSpec>;
#[doc = "Field `UCADDRX0` reader - I2C Receive Address Bit 0"]
pub type Ucaddrx0R = crate::BitReader;
#[doc = "Field `UCADDRX0` writer - I2C Receive Address Bit 0"]
pub type Ucaddrx0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX1` reader - I2C Receive Address Bit 1"]
pub type Ucaddrx1R = crate::BitReader;
#[doc = "Field `UCADDRX1` writer - I2C Receive Address Bit 1"]
pub type Ucaddrx1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX2` reader - I2C Receive Address Bit 2"]
pub type Ucaddrx2R = crate::BitReader;
#[doc = "Field `UCADDRX2` writer - I2C Receive Address Bit 2"]
pub type Ucaddrx2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX3` reader - I2C Receive Address Bit 3"]
pub type Ucaddrx3R = crate::BitReader;
#[doc = "Field `UCADDRX3` writer - I2C Receive Address Bit 3"]
pub type Ucaddrx3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX4` reader - I2C Receive Address Bit 4"]
pub type Ucaddrx4R = crate::BitReader;
#[doc = "Field `UCADDRX4` writer - I2C Receive Address Bit 4"]
pub type Ucaddrx4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX5` reader - I2C Receive Address Bit 5"]
pub type Ucaddrx5R = crate::BitReader;
#[doc = "Field `UCADDRX5` writer - I2C Receive Address Bit 5"]
pub type Ucaddrx5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX6` reader - I2C Receive Address Bit 6"]
pub type Ucaddrx6R = crate::BitReader;
#[doc = "Field `UCADDRX6` writer - I2C Receive Address Bit 6"]
pub type Ucaddrx6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX7` reader - I2C Receive Address Bit 7"]
pub type Ucaddrx7R = crate::BitReader;
#[doc = "Field `UCADDRX7` writer - I2C Receive Address Bit 7"]
pub type Ucaddrx7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX8` reader - I2C Receive Address Bit 8"]
pub type Ucaddrx8R = crate::BitReader;
#[doc = "Field `UCADDRX8` writer - I2C Receive Address Bit 8"]
pub type Ucaddrx8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX9` reader - I2C Receive Address Bit 9"]
pub type Ucaddrx9R = crate::BitReader;
#[doc = "Field `UCADDRX9` writer - I2C Receive Address Bit 9"]
pub type Ucaddrx9W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C Receive Address Bit 0"]
    #[inline(always)]
    pub fn ucaddrx0(&self) -> Ucaddrx0R {
        Ucaddrx0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Receive Address Bit 1"]
    #[inline(always)]
    pub fn ucaddrx1(&self) -> Ucaddrx1R {
        Ucaddrx1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Receive Address Bit 2"]
    #[inline(always)]
    pub fn ucaddrx2(&self) -> Ucaddrx2R {
        Ucaddrx2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Receive Address Bit 3"]
    #[inline(always)]
    pub fn ucaddrx3(&self) -> Ucaddrx3R {
        Ucaddrx3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Receive Address Bit 4"]
    #[inline(always)]
    pub fn ucaddrx4(&self) -> Ucaddrx4R {
        Ucaddrx4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Receive Address Bit 5"]
    #[inline(always)]
    pub fn ucaddrx5(&self) -> Ucaddrx5R {
        Ucaddrx5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Receive Address Bit 6"]
    #[inline(always)]
    pub fn ucaddrx6(&self) -> Ucaddrx6R {
        Ucaddrx6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Receive Address Bit 7"]
    #[inline(always)]
    pub fn ucaddrx7(&self) -> Ucaddrx7R {
        Ucaddrx7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Receive Address Bit 8"]
    #[inline(always)]
    pub fn ucaddrx8(&self) -> Ucaddrx8R {
        Ucaddrx8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Receive Address Bit 9"]
    #[inline(always)]
    pub fn ucaddrx9(&self) -> Ucaddrx9R {
        Ucaddrx9R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Receive Address Bit 0"]
    #[inline(always)]
    pub fn ucaddrx0(&mut self) -> Ucaddrx0W<'_, Ucb0addrxSpec> {
        Ucaddrx0W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C Receive Address Bit 1"]
    #[inline(always)]
    pub fn ucaddrx1(&mut self) -> Ucaddrx1W<'_, Ucb0addrxSpec> {
        Ucaddrx1W::new(self, 1)
    }
    #[doc = "Bit 2 - I2C Receive Address Bit 2"]
    #[inline(always)]
    pub fn ucaddrx2(&mut self) -> Ucaddrx2W<'_, Ucb0addrxSpec> {
        Ucaddrx2W::new(self, 2)
    }
    #[doc = "Bit 3 - I2C Receive Address Bit 3"]
    #[inline(always)]
    pub fn ucaddrx3(&mut self) -> Ucaddrx3W<'_, Ucb0addrxSpec> {
        Ucaddrx3W::new(self, 3)
    }
    #[doc = "Bit 4 - I2C Receive Address Bit 4"]
    #[inline(always)]
    pub fn ucaddrx4(&mut self) -> Ucaddrx4W<'_, Ucb0addrxSpec> {
        Ucaddrx4W::new(self, 4)
    }
    #[doc = "Bit 5 - I2C Receive Address Bit 5"]
    #[inline(always)]
    pub fn ucaddrx5(&mut self) -> Ucaddrx5W<'_, Ucb0addrxSpec> {
        Ucaddrx5W::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Receive Address Bit 6"]
    #[inline(always)]
    pub fn ucaddrx6(&mut self) -> Ucaddrx6W<'_, Ucb0addrxSpec> {
        Ucaddrx6W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C Receive Address Bit 7"]
    #[inline(always)]
    pub fn ucaddrx7(&mut self) -> Ucaddrx7W<'_, Ucb0addrxSpec> {
        Ucaddrx7W::new(self, 7)
    }
    #[doc = "Bit 8 - I2C Receive Address Bit 8"]
    #[inline(always)]
    pub fn ucaddrx8(&mut self) -> Ucaddrx8W<'_, Ucb0addrxSpec> {
        Ucaddrx8W::new(self, 8)
    }
    #[doc = "Bit 9 - I2C Receive Address Bit 9"]
    #[inline(always)]
    pub fn ucaddrx9(&mut self) -> Ucaddrx9W<'_, Ucb0addrxSpec> {
        Ucaddrx9W::new(self, 9)
    }
}
#[doc = "USCI B0 Received Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0addrx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0addrx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0addrxSpec;
impl crate::RegisterSpec for Ucb0addrxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0addrx::R`](R) reader structure"]
impl crate::Readable for Ucb0addrxSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0addrx::W`](W) writer structure"]
impl crate::Writable for Ucb0addrxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0ADDRX to value 0"]
impl crate::Resettable for Ucb0addrxSpec {}
