#[doc = "Register `UCB0IE_I2C` reader"]
pub type R = crate::R<Ucb0ieI2cSpec>;
#[doc = "Register `UCB0IE_I2C` writer"]
pub type W = crate::W<Ucb0ieI2cSpec>;
#[doc = "Field `UCRXIE0` reader - I2C Receive Interrupt Enable 0"]
pub type Ucrxie0R = crate::BitReader;
#[doc = "Field `UCRXIE0` writer - I2C Receive Interrupt Enable 0"]
pub type Ucrxie0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIE0` reader - I2C Transmit Interrupt Enable 0"]
pub type Uctxie0R = crate::BitReader;
#[doc = "Field `UCTXIE0` writer - I2C Transmit Interrupt Enable 0"]
pub type Uctxie0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTTIE` reader - I2C START Condition interrupt enable"]
pub type UcsttieR = crate::BitReader;
#[doc = "Field `UCSTTIE` writer - I2C START Condition interrupt enable"]
pub type UcsttieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTPIE` reader - I2C STOP Condition interrupt enable"]
pub type UcstpieR = crate::BitReader;
#[doc = "Field `UCSTPIE` writer - I2C STOP Condition interrupt enable"]
pub type UcstpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCALIE` reader - I2C Arbitration Lost interrupt enable"]
pub type UcalieR = crate::BitReader;
#[doc = "Field `UCALIE` writer - I2C Arbitration Lost interrupt enable"]
pub type UcalieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCNACKIE` reader - I2C NACK Condition interrupt enable"]
pub type UcnackieR = crate::BitReader;
#[doc = "Field `UCNACKIE` writer - I2C NACK Condition interrupt enable"]
pub type UcnackieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBCNTIE` reader - I2C Automatic stop assertion interrupt enable"]
pub type UcbcntieR = crate::BitReader;
#[doc = "Field `UCBCNTIE` writer - I2C Automatic stop assertion interrupt enable"]
pub type UcbcntieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCCLTOIE` reader - I2C Clock Low Timeout interrupt enable"]
pub type UccltoieR = crate::BitReader;
#[doc = "Field `UCCLTOIE` writer - I2C Clock Low Timeout interrupt enable"]
pub type UccltoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCRXIE1` reader - I2C Receive Interrupt Enable 1"]
pub type Ucrxie1R = crate::BitReader;
#[doc = "Field `UCRXIE1` writer - I2C Receive Interrupt Enable 1"]
pub type Ucrxie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIE1` reader - I2C Transmit Interrupt Enable 1"]
pub type Uctxie1R = crate::BitReader;
#[doc = "Field `UCTXIE1` writer - I2C Transmit Interrupt Enable 1"]
pub type Uctxie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCRXIE2` reader - I2C Receive Interrupt Enable 2"]
pub type Ucrxie2R = crate::BitReader;
#[doc = "Field `UCRXIE2` writer - I2C Receive Interrupt Enable 2"]
pub type Ucrxie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIE2` reader - I2C Transmit Interrupt Enable 2"]
pub type Uctxie2R = crate::BitReader;
#[doc = "Field `UCTXIE2` writer - I2C Transmit Interrupt Enable 2"]
pub type Uctxie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCRXIE3` reader - I2C Receive Interrupt Enable 3"]
pub type Ucrxie3R = crate::BitReader;
#[doc = "Field `UCRXIE3` writer - I2C Receive Interrupt Enable 3"]
pub type Ucrxie3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIE3` reader - I2C Transmit Interrupt Enable 3"]
pub type Uctxie3R = crate::BitReader;
#[doc = "Field `UCTXIE3` writer - I2C Transmit Interrupt Enable 3"]
pub type Uctxie3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBIT9IE` reader - I2C Bit 9 Position Interrupt Enable 3"]
pub type Ucbit9ieR = crate::BitReader;
#[doc = "Field `UCBIT9IE` writer - I2C Bit 9 Position Interrupt Enable 3"]
pub type Ucbit9ieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C Receive Interrupt Enable 0"]
    #[inline(always)]
    pub fn ucrxie0(&self) -> Ucrxie0R {
        Ucrxie0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Enable 0"]
    #[inline(always)]
    pub fn uctxie0(&self) -> Uctxie0R {
        Uctxie0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UcsttieR {
        UcsttieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&self) -> UcstpieR {
        UcstpieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&self) -> UcalieR {
        UcalieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&self) -> UcnackieR {
        UcnackieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Automatic stop assertion interrupt enable"]
    #[inline(always)]
    pub fn ucbcntie(&self) -> UcbcntieR {
        UcbcntieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Clock Low Timeout interrupt enable"]
    #[inline(always)]
    pub fn uccltoie(&self) -> UccltoieR {
        UccltoieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Enable 1"]
    #[inline(always)]
    pub fn ucrxie1(&self) -> Ucrxie1R {
        Ucrxie1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Enable 1"]
    #[inline(always)]
    pub fn uctxie1(&self) -> Uctxie1R {
        Uctxie1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Enable 2"]
    #[inline(always)]
    pub fn ucrxie2(&self) -> Ucrxie2R {
        Ucrxie2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Enable 2"]
    #[inline(always)]
    pub fn uctxie2(&self) -> Uctxie2R {
        Uctxie2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucrxie3(&self) -> Ucrxie3R {
        Ucrxie3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Enable 3"]
    #[inline(always)]
    pub fn uctxie3(&self) -> Uctxie3R {
        Uctxie3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - I2C Bit 9 Position Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucbit9ie(&self) -> Ucbit9ieR {
        Ucbit9ieR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Receive Interrupt Enable 0"]
    #[inline(always)]
    pub fn ucrxie0(&mut self) -> Ucrxie0W<'_, Ucb0ieI2cSpec> {
        Ucrxie0W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Enable 0"]
    #[inline(always)]
    pub fn uctxie0(&mut self) -> Uctxie0W<'_, Ucb0ieI2cSpec> {
        Uctxie0W::new(self, 1)
    }
    #[doc = "Bit 2 - I2C START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&mut self) -> UcsttieW<'_, Ucb0ieI2cSpec> {
        UcsttieW::new(self, 2)
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&mut self) -> UcstpieW<'_, Ucb0ieI2cSpec> {
        UcstpieW::new(self, 3)
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&mut self) -> UcalieW<'_, Ucb0ieI2cSpec> {
        UcalieW::new(self, 4)
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&mut self) -> UcnackieW<'_, Ucb0ieI2cSpec> {
        UcnackieW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Automatic stop assertion interrupt enable"]
    #[inline(always)]
    pub fn ucbcntie(&mut self) -> UcbcntieW<'_, Ucb0ieI2cSpec> {
        UcbcntieW::new(self, 6)
    }
    #[doc = "Bit 7 - I2C Clock Low Timeout interrupt enable"]
    #[inline(always)]
    pub fn uccltoie(&mut self) -> UccltoieW<'_, Ucb0ieI2cSpec> {
        UccltoieW::new(self, 7)
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Enable 1"]
    #[inline(always)]
    pub fn ucrxie1(&mut self) -> Ucrxie1W<'_, Ucb0ieI2cSpec> {
        Ucrxie1W::new(self, 8)
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Enable 1"]
    #[inline(always)]
    pub fn uctxie1(&mut self) -> Uctxie1W<'_, Ucb0ieI2cSpec> {
        Uctxie1W::new(self, 9)
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Enable 2"]
    #[inline(always)]
    pub fn ucrxie2(&mut self) -> Ucrxie2W<'_, Ucb0ieI2cSpec> {
        Ucrxie2W::new(self, 10)
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Enable 2"]
    #[inline(always)]
    pub fn uctxie2(&mut self) -> Uctxie2W<'_, Ucb0ieI2cSpec> {
        Uctxie2W::new(self, 11)
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucrxie3(&mut self) -> Ucrxie3W<'_, Ucb0ieI2cSpec> {
        Ucrxie3W::new(self, 12)
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Enable 3"]
    #[inline(always)]
    pub fn uctxie3(&mut self) -> Uctxie3W<'_, Ucb0ieI2cSpec> {
        Uctxie3W::new(self, 13)
    }
    #[doc = "Bit 14 - I2C Bit 9 Position Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucbit9ie(&mut self) -> Ucbit9ieW<'_, Ucb0ieI2cSpec> {
        Ucbit9ieW::new(self, 14)
    }
}
#[doc = "USCI B0 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ie_i2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ie_i2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0ieI2cSpec;
impl crate::RegisterSpec for Ucb0ieI2cSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0ie_i2c::R`](R) reader structure"]
impl crate::Readable for Ucb0ieI2cSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0ie_i2c::W`](W) writer structure"]
impl crate::Writable for Ucb0ieI2cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0IE_I2C to value 0"]
impl crate::Resettable for Ucb0ieI2cSpec {}
