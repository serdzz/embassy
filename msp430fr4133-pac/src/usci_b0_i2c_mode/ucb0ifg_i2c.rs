#[doc = "Register `UCB0IFG_I2C` reader"]
pub type R = crate::R<Ucb0ifgI2cSpec>;
#[doc = "Register `UCB0IFG_I2C` writer"]
pub type W = crate::W<Ucb0ifgI2cSpec>;
#[doc = "Field `UCRXIFG0` reader - I2C Receive Interrupt Flag 0"]
pub type Ucrxifg0R = crate::BitReader;
#[doc = "Field `UCRXIFG0` writer - I2C Receive Interrupt Flag 0"]
pub type Ucrxifg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIFG0` reader - I2C Transmit Interrupt Flag 0"]
pub type Uctxifg0R = crate::BitReader;
#[doc = "Field `UCTXIFG0` writer - I2C Transmit Interrupt Flag 0"]
pub type Uctxifg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTTIFG` reader - I2C START Condition interrupt Flag"]
pub type UcsttifgR = crate::BitReader;
#[doc = "Field `UCSTTIFG` writer - I2C START Condition interrupt Flag"]
pub type UcsttifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTPIFG` reader - I2C STOP Condition interrupt Flag"]
pub type UcstpifgR = crate::BitReader;
#[doc = "Field `UCSTPIFG` writer - I2C STOP Condition interrupt Flag"]
pub type UcstpifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCALIFG` reader - I2C Arbitration Lost interrupt Flag"]
pub type UcalifgR = crate::BitReader;
#[doc = "Field `UCALIFG` writer - I2C Arbitration Lost interrupt Flag"]
pub type UcalifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCNACKIFG` reader - I2C NACK Condition interrupt Flag"]
pub type UcnackifgR = crate::BitReader;
#[doc = "Field `UCNACKIFG` writer - I2C NACK Condition interrupt Flag"]
pub type UcnackifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBCNTIFG` reader - I2C Byte counter interrupt flag"]
pub type UcbcntifgR = crate::BitReader;
#[doc = "Field `UCBCNTIFG` writer - I2C Byte counter interrupt flag"]
pub type UcbcntifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCCLTOIFG` reader - I2C Clock low Timeout interrupt Flag"]
pub type UccltoifgR = crate::BitReader;
#[doc = "Field `UCCLTOIFG` writer - I2C Clock low Timeout interrupt Flag"]
pub type UccltoifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCRXIFG1` reader - I2C Receive Interrupt Flag 1"]
pub type Ucrxifg1R = crate::BitReader;
#[doc = "Field `UCRXIFG1` writer - I2C Receive Interrupt Flag 1"]
pub type Ucrxifg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIFG1` reader - I2C Transmit Interrupt Flag 1"]
pub type Uctxifg1R = crate::BitReader;
#[doc = "Field `UCTXIFG1` writer - I2C Transmit Interrupt Flag 1"]
pub type Uctxifg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCRXIFG2` reader - I2C Receive Interrupt Flag 2"]
pub type Ucrxifg2R = crate::BitReader;
#[doc = "Field `UCRXIFG2` writer - I2C Receive Interrupt Flag 2"]
pub type Ucrxifg2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIFG2` reader - I2C Transmit Interrupt Flag 2"]
pub type Uctxifg2R = crate::BitReader;
#[doc = "Field `UCTXIFG2` writer - I2C Transmit Interrupt Flag 2"]
pub type Uctxifg2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCRXIFG3` reader - I2C Receive Interrupt Flag 3"]
pub type Ucrxifg3R = crate::BitReader;
#[doc = "Field `UCRXIFG3` writer - I2C Receive Interrupt Flag 3"]
pub type Ucrxifg3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIFG3` reader - I2C Transmit Interrupt Flag 3"]
pub type Uctxifg3R = crate::BitReader;
#[doc = "Field `UCTXIFG3` writer - I2C Transmit Interrupt Flag 3"]
pub type Uctxifg3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBIT9IFG` reader - I2C Bit 9 Possition Interrupt Flag 3"]
pub type Ucbit9ifgR = crate::BitReader;
#[doc = "Field `UCBIT9IFG` writer - I2C Bit 9 Possition Interrupt Flag 3"]
pub type Ucbit9ifgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C Receive Interrupt Flag 0"]
    #[inline(always)]
    pub fn ucrxifg0(&self) -> Ucrxifg0R {
        Ucrxifg0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Flag 0"]
    #[inline(always)]
    pub fn uctxifg0(&self) -> Uctxifg0R {
        Uctxifg0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C START Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucsttifg(&self) -> UcsttifgR {
        UcsttifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucstpifg(&self) -> UcstpifgR {
        UcstpifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt Flag"]
    #[inline(always)]
    pub fn ucalifg(&self) -> UcalifgR {
        UcalifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucnackifg(&self) -> UcnackifgR {
        UcnackifgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Byte counter interrupt flag"]
    #[inline(always)]
    pub fn ucbcntifg(&self) -> UcbcntifgR {
        UcbcntifgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Clock low Timeout interrupt Flag"]
    #[inline(always)]
    pub fn uccltoifg(&self) -> UccltoifgR {
        UccltoifgR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Flag 1"]
    #[inline(always)]
    pub fn ucrxifg1(&self) -> Ucrxifg1R {
        Ucrxifg1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Flag 1"]
    #[inline(always)]
    pub fn uctxifg1(&self) -> Uctxifg1R {
        Uctxifg1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Flag 2"]
    #[inline(always)]
    pub fn ucrxifg2(&self) -> Ucrxifg2R {
        Ucrxifg2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Flag 2"]
    #[inline(always)]
    pub fn uctxifg2(&self) -> Uctxifg2R {
        Uctxifg2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Flag 3"]
    #[inline(always)]
    pub fn ucrxifg3(&self) -> Ucrxifg3R {
        Ucrxifg3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Flag 3"]
    #[inline(always)]
    pub fn uctxifg3(&self) -> Uctxifg3R {
        Uctxifg3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - I2C Bit 9 Possition Interrupt Flag 3"]
    #[inline(always)]
    pub fn ucbit9ifg(&self) -> Ucbit9ifgR {
        Ucbit9ifgR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Receive Interrupt Flag 0"]
    #[inline(always)]
    pub fn ucrxifg0(&mut self) -> Ucrxifg0W<'_, Ucb0ifgI2cSpec> {
        Ucrxifg0W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Flag 0"]
    #[inline(always)]
    pub fn uctxifg0(&mut self) -> Uctxifg0W<'_, Ucb0ifgI2cSpec> {
        Uctxifg0W::new(self, 1)
    }
    #[doc = "Bit 2 - I2C START Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucsttifg(&mut self) -> UcsttifgW<'_, Ucb0ifgI2cSpec> {
        UcsttifgW::new(self, 2)
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucstpifg(&mut self) -> UcstpifgW<'_, Ucb0ifgI2cSpec> {
        UcstpifgW::new(self, 3)
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt Flag"]
    #[inline(always)]
    pub fn ucalifg(&mut self) -> UcalifgW<'_, Ucb0ifgI2cSpec> {
        UcalifgW::new(self, 4)
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucnackifg(&mut self) -> UcnackifgW<'_, Ucb0ifgI2cSpec> {
        UcnackifgW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Byte counter interrupt flag"]
    #[inline(always)]
    pub fn ucbcntifg(&mut self) -> UcbcntifgW<'_, Ucb0ifgI2cSpec> {
        UcbcntifgW::new(self, 6)
    }
    #[doc = "Bit 7 - I2C Clock low Timeout interrupt Flag"]
    #[inline(always)]
    pub fn uccltoifg(&mut self) -> UccltoifgW<'_, Ucb0ifgI2cSpec> {
        UccltoifgW::new(self, 7)
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Flag 1"]
    #[inline(always)]
    pub fn ucrxifg1(&mut self) -> Ucrxifg1W<'_, Ucb0ifgI2cSpec> {
        Ucrxifg1W::new(self, 8)
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Flag 1"]
    #[inline(always)]
    pub fn uctxifg1(&mut self) -> Uctxifg1W<'_, Ucb0ifgI2cSpec> {
        Uctxifg1W::new(self, 9)
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Flag 2"]
    #[inline(always)]
    pub fn ucrxifg2(&mut self) -> Ucrxifg2W<'_, Ucb0ifgI2cSpec> {
        Ucrxifg2W::new(self, 10)
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Flag 2"]
    #[inline(always)]
    pub fn uctxifg2(&mut self) -> Uctxifg2W<'_, Ucb0ifgI2cSpec> {
        Uctxifg2W::new(self, 11)
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Flag 3"]
    #[inline(always)]
    pub fn ucrxifg3(&mut self) -> Ucrxifg3W<'_, Ucb0ifgI2cSpec> {
        Ucrxifg3W::new(self, 12)
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Flag 3"]
    #[inline(always)]
    pub fn uctxifg3(&mut self) -> Uctxifg3W<'_, Ucb0ifgI2cSpec> {
        Uctxifg3W::new(self, 13)
    }
    #[doc = "Bit 14 - I2C Bit 9 Possition Interrupt Flag 3"]
    #[inline(always)]
    pub fn ucbit9ifg(&mut self) -> Ucbit9ifgW<'_, Ucb0ifgI2cSpec> {
        Ucbit9ifgW::new(self, 14)
    }
}
#[doc = "USCI B0 Interrupt Flags Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ifg_i2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ifg_i2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0ifgI2cSpec;
impl crate::RegisterSpec for Ucb0ifgI2cSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0ifg_i2c::R`](R) reader structure"]
impl crate::Readable for Ucb0ifgI2cSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0ifg_i2c::W`](W) writer structure"]
impl crate::Writable for Ucb0ifgI2cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0IFG_I2C to value 0"]
impl crate::Resettable for Ucb0ifgI2cSpec {}
