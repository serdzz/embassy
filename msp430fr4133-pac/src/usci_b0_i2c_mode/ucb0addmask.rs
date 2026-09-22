#[doc = "Register `UCB0ADDMASK` reader"]
pub type R = crate::R<Ucb0addmaskSpec>;
#[doc = "Register `UCB0ADDMASK` writer"]
pub type W = crate::W<Ucb0addmaskSpec>;
#[doc = "Field `UCADDMASK0` reader - I2C Address Mask Bit 0"]
pub type Ucaddmask0R = crate::BitReader;
#[doc = "Field `UCADDMASK0` writer - I2C Address Mask Bit 0"]
pub type Ucaddmask0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDMASK1` reader - I2C Address Mask Bit 1"]
pub type Ucaddmask1R = crate::BitReader;
#[doc = "Field `UCADDMASK1` writer - I2C Address Mask Bit 1"]
pub type Ucaddmask1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDMASK2` reader - I2C Address Mask Bit 2"]
pub type Ucaddmask2R = crate::BitReader;
#[doc = "Field `UCADDMASK2` writer - I2C Address Mask Bit 2"]
pub type Ucaddmask2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDMASK3` reader - I2C Address Mask Bit 3"]
pub type Ucaddmask3R = crate::BitReader;
#[doc = "Field `UCADDMASK3` writer - I2C Address Mask Bit 3"]
pub type Ucaddmask3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDMASK4` reader - I2C Address Mask Bit 4"]
pub type Ucaddmask4R = crate::BitReader;
#[doc = "Field `UCADDMASK4` writer - I2C Address Mask Bit 4"]
pub type Ucaddmask4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDMASK5` reader - I2C Address Mask Bit 5"]
pub type Ucaddmask5R = crate::BitReader;
#[doc = "Field `UCADDMASK5` writer - I2C Address Mask Bit 5"]
pub type Ucaddmask5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDMASK6` reader - I2C Address Mask Bit 6"]
pub type Ucaddmask6R = crate::BitReader;
#[doc = "Field `UCADDMASK6` writer - I2C Address Mask Bit 6"]
pub type Ucaddmask6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDMASK7` reader - I2C Address Mask Bit 7"]
pub type Ucaddmask7R = crate::BitReader;
#[doc = "Field `UCADDMASK7` writer - I2C Address Mask Bit 7"]
pub type Ucaddmask7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDMASK8` reader - I2C Address Mask Bit 8"]
pub type Ucaddmask8R = crate::BitReader;
#[doc = "Field `UCADDMASK8` writer - I2C Address Mask Bit 8"]
pub type Ucaddmask8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDMASK9` reader - I2C Address Mask Bit 9"]
pub type Ucaddmask9R = crate::BitReader;
#[doc = "Field `UCADDMASK9` writer - I2C Address Mask Bit 9"]
pub type Ucaddmask9W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C Address Mask Bit 0"]
    #[inline(always)]
    pub fn ucaddmask0(&self) -> Ucaddmask0R {
        Ucaddmask0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Address Mask Bit 1"]
    #[inline(always)]
    pub fn ucaddmask1(&self) -> Ucaddmask1R {
        Ucaddmask1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Address Mask Bit 2"]
    #[inline(always)]
    pub fn ucaddmask2(&self) -> Ucaddmask2R {
        Ucaddmask2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Address Mask Bit 3"]
    #[inline(always)]
    pub fn ucaddmask3(&self) -> Ucaddmask3R {
        Ucaddmask3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Address Mask Bit 4"]
    #[inline(always)]
    pub fn ucaddmask4(&self) -> Ucaddmask4R {
        Ucaddmask4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Address Mask Bit 5"]
    #[inline(always)]
    pub fn ucaddmask5(&self) -> Ucaddmask5R {
        Ucaddmask5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Address Mask Bit 6"]
    #[inline(always)]
    pub fn ucaddmask6(&self) -> Ucaddmask6R {
        Ucaddmask6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Address Mask Bit 7"]
    #[inline(always)]
    pub fn ucaddmask7(&self) -> Ucaddmask7R {
        Ucaddmask7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Address Mask Bit 8"]
    #[inline(always)]
    pub fn ucaddmask8(&self) -> Ucaddmask8R {
        Ucaddmask8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Address Mask Bit 9"]
    #[inline(always)]
    pub fn ucaddmask9(&self) -> Ucaddmask9R {
        Ucaddmask9R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Address Mask Bit 0"]
    #[inline(always)]
    pub fn ucaddmask0(&mut self) -> Ucaddmask0W<'_, Ucb0addmaskSpec> {
        Ucaddmask0W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C Address Mask Bit 1"]
    #[inline(always)]
    pub fn ucaddmask1(&mut self) -> Ucaddmask1W<'_, Ucb0addmaskSpec> {
        Ucaddmask1W::new(self, 1)
    }
    #[doc = "Bit 2 - I2C Address Mask Bit 2"]
    #[inline(always)]
    pub fn ucaddmask2(&mut self) -> Ucaddmask2W<'_, Ucb0addmaskSpec> {
        Ucaddmask2W::new(self, 2)
    }
    #[doc = "Bit 3 - I2C Address Mask Bit 3"]
    #[inline(always)]
    pub fn ucaddmask3(&mut self) -> Ucaddmask3W<'_, Ucb0addmaskSpec> {
        Ucaddmask3W::new(self, 3)
    }
    #[doc = "Bit 4 - I2C Address Mask Bit 4"]
    #[inline(always)]
    pub fn ucaddmask4(&mut self) -> Ucaddmask4W<'_, Ucb0addmaskSpec> {
        Ucaddmask4W::new(self, 4)
    }
    #[doc = "Bit 5 - I2C Address Mask Bit 5"]
    #[inline(always)]
    pub fn ucaddmask5(&mut self) -> Ucaddmask5W<'_, Ucb0addmaskSpec> {
        Ucaddmask5W::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Address Mask Bit 6"]
    #[inline(always)]
    pub fn ucaddmask6(&mut self) -> Ucaddmask6W<'_, Ucb0addmaskSpec> {
        Ucaddmask6W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C Address Mask Bit 7"]
    #[inline(always)]
    pub fn ucaddmask7(&mut self) -> Ucaddmask7W<'_, Ucb0addmaskSpec> {
        Ucaddmask7W::new(self, 7)
    }
    #[doc = "Bit 8 - I2C Address Mask Bit 8"]
    #[inline(always)]
    pub fn ucaddmask8(&mut self) -> Ucaddmask8W<'_, Ucb0addmaskSpec> {
        Ucaddmask8W::new(self, 8)
    }
    #[doc = "Bit 9 - I2C Address Mask Bit 9"]
    #[inline(always)]
    pub fn ucaddmask9(&mut self) -> Ucaddmask9W<'_, Ucb0addmaskSpec> {
        Ucaddmask9W::new(self, 9)
    }
}
#[doc = "USCI B0 Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0addmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0addmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0addmaskSpec;
impl crate::RegisterSpec for Ucb0addmaskSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0addmask::R`](R) reader structure"]
impl crate::Readable for Ucb0addmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0addmask::W`](W) writer structure"]
impl crate::Writable for Ucb0addmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0ADDMASK to value 0"]
impl crate::Resettable for Ucb0addmaskSpec {}
