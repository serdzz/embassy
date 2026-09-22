#[doc = "Register `UCB0BCNT_I2C` reader"]
pub type R = crate::R<Ucb0bcntI2cSpec>;
#[doc = "Register `UCB0BCNT_I2C` writer"]
pub type W = crate::W<Ucb0bcntI2cSpec>;
#[doc = "Field `UCBCNT0` reader - USCI Byte Counter Bit 0"]
pub type Ucbcnt0R = crate::BitReader;
#[doc = "Field `UCBCNT0` writer - USCI Byte Counter Bit 0"]
pub type Ucbcnt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBCNT1` reader - USCI Byte Counter Bit 1"]
pub type Ucbcnt1R = crate::BitReader;
#[doc = "Field `UCBCNT1` writer - USCI Byte Counter Bit 1"]
pub type Ucbcnt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBCNT2` reader - USCI Byte Counter Bit 2"]
pub type Ucbcnt2R = crate::BitReader;
#[doc = "Field `UCBCNT2` writer - USCI Byte Counter Bit 2"]
pub type Ucbcnt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBCNT3` reader - USCI Byte Counter Bit 3"]
pub type Ucbcnt3R = crate::BitReader;
#[doc = "Field `UCBCNT3` writer - USCI Byte Counter Bit 3"]
pub type Ucbcnt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBCNT4` reader - USCI Byte Counter Bit 4"]
pub type Ucbcnt4R = crate::BitReader;
#[doc = "Field `UCBCNT4` writer - USCI Byte Counter Bit 4"]
pub type Ucbcnt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBCNT5` reader - USCI Byte Counter Bit 5"]
pub type Ucbcnt5R = crate::BitReader;
#[doc = "Field `UCBCNT5` writer - USCI Byte Counter Bit 5"]
pub type Ucbcnt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBCNT6` reader - USCI Byte Counter Bit 6"]
pub type Ucbcnt6R = crate::BitReader;
#[doc = "Field `UCBCNT6` writer - USCI Byte Counter Bit 6"]
pub type Ucbcnt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBCNT7` reader - USCI Byte Counter Bit 7"]
pub type Ucbcnt7R = crate::BitReader;
#[doc = "Field `UCBCNT7` writer - USCI Byte Counter Bit 7"]
pub type Ucbcnt7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USCI Byte Counter Bit 0"]
    #[inline(always)]
    pub fn ucbcnt0(&self) -> Ucbcnt0R {
        Ucbcnt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USCI Byte Counter Bit 1"]
    #[inline(always)]
    pub fn ucbcnt1(&self) -> Ucbcnt1R {
        Ucbcnt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USCI Byte Counter Bit 2"]
    #[inline(always)]
    pub fn ucbcnt2(&self) -> Ucbcnt2R {
        Ucbcnt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USCI Byte Counter Bit 3"]
    #[inline(always)]
    pub fn ucbcnt3(&self) -> Ucbcnt3R {
        Ucbcnt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USCI Byte Counter Bit 4"]
    #[inline(always)]
    pub fn ucbcnt4(&self) -> Ucbcnt4R {
        Ucbcnt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USCI Byte Counter Bit 5"]
    #[inline(always)]
    pub fn ucbcnt5(&self) -> Ucbcnt5R {
        Ucbcnt5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USCI Byte Counter Bit 6"]
    #[inline(always)]
    pub fn ucbcnt6(&self) -> Ucbcnt6R {
        Ucbcnt6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USCI Byte Counter Bit 7"]
    #[inline(always)]
    pub fn ucbcnt7(&self) -> Ucbcnt7R {
        Ucbcnt7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Byte Counter Bit 0"]
    #[inline(always)]
    pub fn ucbcnt0(&mut self) -> Ucbcnt0W<'_, Ucb0bcntI2cSpec> {
        Ucbcnt0W::new(self, 0)
    }
    #[doc = "Bit 1 - USCI Byte Counter Bit 1"]
    #[inline(always)]
    pub fn ucbcnt1(&mut self) -> Ucbcnt1W<'_, Ucb0bcntI2cSpec> {
        Ucbcnt1W::new(self, 1)
    }
    #[doc = "Bit 2 - USCI Byte Counter Bit 2"]
    #[inline(always)]
    pub fn ucbcnt2(&mut self) -> Ucbcnt2W<'_, Ucb0bcntI2cSpec> {
        Ucbcnt2W::new(self, 2)
    }
    #[doc = "Bit 3 - USCI Byte Counter Bit 3"]
    #[inline(always)]
    pub fn ucbcnt3(&mut self) -> Ucbcnt3W<'_, Ucb0bcntI2cSpec> {
        Ucbcnt3W::new(self, 3)
    }
    #[doc = "Bit 4 - USCI Byte Counter Bit 4"]
    #[inline(always)]
    pub fn ucbcnt4(&mut self) -> Ucbcnt4W<'_, Ucb0bcntI2cSpec> {
        Ucbcnt4W::new(self, 4)
    }
    #[doc = "Bit 5 - USCI Byte Counter Bit 5"]
    #[inline(always)]
    pub fn ucbcnt5(&mut self) -> Ucbcnt5W<'_, Ucb0bcntI2cSpec> {
        Ucbcnt5W::new(self, 5)
    }
    #[doc = "Bit 6 - USCI Byte Counter Bit 6"]
    #[inline(always)]
    pub fn ucbcnt6(&mut self) -> Ucbcnt6W<'_, Ucb0bcntI2cSpec> {
        Ucbcnt6W::new(self, 6)
    }
    #[doc = "Bit 7 - USCI Byte Counter Bit 7"]
    #[inline(always)]
    pub fn ucbcnt7(&mut self) -> Ucbcnt7W<'_, Ucb0bcntI2cSpec> {
        Ucbcnt7W::new(self, 7)
    }
}
#[doc = "USCI B0 Byte Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0bcnt_i2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0bcnt_i2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0bcntI2cSpec;
impl crate::RegisterSpec for Ucb0bcntI2cSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0bcnt_i2c::R`](R) reader structure"]
impl crate::Readable for Ucb0bcntI2cSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0bcnt_i2c::W`](W) writer structure"]
impl crate::Writable for Ucb0bcntI2cSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets UCB0BCNT_I2C to value 0"]
impl crate::Resettable for Ucb0bcntI2cSpec {}
