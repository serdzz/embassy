#[doc = "Register `UCB0STAT_I2C` reader"]
pub type R = crate::R<Ucb0statI2cSpec>;
#[doc = "Register `UCB0STAT_I2C` writer"]
pub type W = crate::W<Ucb0statI2cSpec>;
#[doc = "Field `UCBBUSY` reader - Bus Busy Flag"]
pub type UcbbusyR = crate::BitReader;
#[doc = "Field `UCBBUSY` writer - Bus Busy Flag"]
pub type UcbbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCGC` reader - General Call address received Flag"]
pub type UcgcR = crate::BitReader;
#[doc = "Field `UCGC` writer - General Call address received Flag"]
pub type UcgcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSCLLOW` reader - SCL low"]
pub type UcscllowR = crate::BitReader;
#[doc = "Field `UCSCLLOW` writer - SCL low"]
pub type UcscllowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    pub fn ucbbusy(&self) -> UcbbusyR {
        UcbbusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    pub fn ucgc(&self) -> UcgcR {
        UcgcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&self) -> UcscllowR {
        UcscllowR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    pub fn ucbbusy(&mut self) -> UcbbusyW<'_, Ucb0statI2cSpec> {
        UcbbusyW::new(self, 4)
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    pub fn ucgc(&mut self) -> UcgcW<'_, Ucb0statI2cSpec> {
        UcgcW::new(self, 5)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&mut self) -> UcscllowW<'_, Ucb0statI2cSpec> {
        UcscllowW::new(self, 6)
    }
}
#[doc = "USCI B0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0stat_i2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0stat_i2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0statI2cSpec;
impl crate::RegisterSpec for Ucb0statI2cSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0stat_i2c::R`](R) reader structure"]
impl crate::Readable for Ucb0statI2cSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0stat_i2c::W`](W) writer structure"]
impl crate::Writable for Ucb0statI2cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0STAT_I2C to value 0"]
impl crate::Resettable for Ucb0statI2cSpec {}
