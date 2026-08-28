#[doc = "Register `UCB1CTL0_SPI` reader"]
pub type R = crate::R<Ucb1ctl0SpiSpec>;
#[doc = "Register `UCB1CTL0_SPI` writer"]
pub type W = crate::W<Ucb1ctl0SpiSpec>;
#[doc = "Field `UCSYNC` reader - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub type UcsyncR = crate::BitReader;
#[doc = "Field `UCSYNC` writer - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub type UcsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sync. Mode: USCI Mode 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucmode {
    #[doc = "0: Sync. Mode: USCI Mode: 0"]
    Ucmode0 = 0,
    #[doc = "1: Sync. Mode: USCI Mode: 1"]
    Ucmode1 = 1,
    #[doc = "2: Sync. Mode: USCI Mode: 2"]
    Ucmode2 = 2,
    #[doc = "3: Sync. Mode: USCI Mode: 3"]
    Ucmode3 = 3,
}
impl From<Ucmode> for u8 {
    #[inline(always)]
    fn from(variant: Ucmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucmode {
    type Ux = u8;
}
impl crate::IsEnum for Ucmode {}
#[doc = "Field `UCMODE` reader - Sync. Mode: USCI Mode 1"]
pub type UcmodeR = crate::FieldReader<Ucmode>;
impl UcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucmode {
        match self.bits {
            0 => Ucmode::Ucmode0,
            1 => Ucmode::Ucmode1,
            2 => Ucmode::Ucmode2,
            3 => Ucmode::Ucmode3,
            _ => unreachable!(),
        }
    }
    #[doc = "Sync. Mode: USCI Mode: 0"]
    #[inline(always)]
    pub fn is_ucmode_0(&self) -> bool {
        *self == Ucmode::Ucmode0
    }
    #[doc = "Sync. Mode: USCI Mode: 1"]
    #[inline(always)]
    pub fn is_ucmode_1(&self) -> bool {
        *self == Ucmode::Ucmode1
    }
    #[doc = "Sync. Mode: USCI Mode: 2"]
    #[inline(always)]
    pub fn is_ucmode_2(&self) -> bool {
        *self == Ucmode::Ucmode2
    }
    #[doc = "Sync. Mode: USCI Mode: 3"]
    #[inline(always)]
    pub fn is_ucmode_3(&self) -> bool {
        *self == Ucmode::Ucmode3
    }
}
#[doc = "Field `UCMODE` writer - Sync. Mode: USCI Mode 1"]
pub type UcmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucmode, crate::Safe>;
impl<'a, REG> UcmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sync. Mode: USCI Mode: 0"]
    #[inline(always)]
    pub fn ucmode_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode0)
    }
    #[doc = "Sync. Mode: USCI Mode: 1"]
    #[inline(always)]
    pub fn ucmode_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode1)
    }
    #[doc = "Sync. Mode: USCI Mode: 2"]
    #[inline(always)]
    pub fn ucmode_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode2)
    }
    #[doc = "Sync. Mode: USCI Mode: 3"]
    #[inline(always)]
    pub fn ucmode_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode3)
    }
}
#[doc = "Field `UCMST` reader - Sync. Mode: Master Select"]
pub type UcmstR = crate::BitReader;
#[doc = "Field `UCMST` writer - Sync. Mode: Master Select"]
pub type UcmstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UC7BIT` reader - Sync. Mode: Data Bits 0:8-bits / 1:7-bits"]
pub type Uc7bitR = crate::BitReader;
#[doc = "Field `UC7BIT` writer - Sync. Mode: Data Bits 0:8-bits / 1:7-bits"]
pub type Uc7bitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCMSB` reader - Sync. Mode: MSB first 0:LSB / 1:MSB"]
pub type UcmsbR = crate::BitReader;
#[doc = "Field `UCMSB` writer - Sync. Mode: MSB first 0:LSB / 1:MSB"]
pub type UcmsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCCKPL` reader - Sync. Mode: Clock Polarity"]
pub type UcckplR = crate::BitReader;
#[doc = "Field `UCCKPL` writer - Sync. Mode: Clock Polarity"]
pub type UcckplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCCKPH` reader - Sync. Mode: Clock Phase"]
pub type UcckphR = crate::BitReader;
#[doc = "Field `UCCKPH` writer - Sync. Mode: Clock Phase"]
pub type UcckphW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&self) -> UcsyncR {
        UcsyncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Sync. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&self) -> UcmodeR {
        UcmodeR::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Sync. Mode: Master Select"]
    #[inline(always)]
    pub fn ucmst(&self) -> UcmstR {
        UcmstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sync. Mode: Data Bits 0:8-bits / 1:7-bits"]
    #[inline(always)]
    pub fn uc7bit(&self) -> Uc7bitR {
        Uc7bitR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sync. Mode: MSB first 0:LSB / 1:MSB"]
    #[inline(always)]
    pub fn ucmsb(&self) -> UcmsbR {
        UcmsbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sync. Mode: Clock Polarity"]
    #[inline(always)]
    pub fn ucckpl(&self) -> UcckplR {
        UcckplR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sync. Mode: Clock Phase"]
    #[inline(always)]
    pub fn ucckph(&self) -> UcckphR {
        UcckphR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UcsyncW<'_, Ucb1ctl0SpiSpec> {
        UcsyncW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Sync. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UcmodeW<'_, Ucb1ctl0SpiSpec> {
        UcmodeW::new(self, 1)
    }
    #[doc = "Bit 3 - Sync. Mode: Master Select"]
    #[inline(always)]
    pub fn ucmst(&mut self) -> UcmstW<'_, Ucb1ctl0SpiSpec> {
        UcmstW::new(self, 3)
    }
    #[doc = "Bit 4 - Sync. Mode: Data Bits 0:8-bits / 1:7-bits"]
    #[inline(always)]
    pub fn uc7bit(&mut self) -> Uc7bitW<'_, Ucb1ctl0SpiSpec> {
        Uc7bitW::new(self, 4)
    }
    #[doc = "Bit 5 - Sync. Mode: MSB first 0:LSB / 1:MSB"]
    #[inline(always)]
    pub fn ucmsb(&mut self) -> UcmsbW<'_, Ucb1ctl0SpiSpec> {
        UcmsbW::new(self, 5)
    }
    #[doc = "Bit 6 - Sync. Mode: Clock Polarity"]
    #[inline(always)]
    pub fn ucckpl(&mut self) -> UcckplW<'_, Ucb1ctl0SpiSpec> {
        UcckplW::new(self, 6)
    }
    #[doc = "Bit 7 - Sync. Mode: Clock Phase"]
    #[inline(always)]
    pub fn ucckph(&mut self) -> UcckphW<'_, Ucb1ctl0SpiSpec> {
        UcckphW::new(self, 7)
    }
}
#[doc = "USCI B1 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1ctl0_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1ctl0_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb1ctl0SpiSpec;
impl crate::RegisterSpec for Ucb1ctl0SpiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb1ctl0_spi::R`](R) reader structure"]
impl crate::Readable for Ucb1ctl0SpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb1ctl0_spi::W`](W) writer structure"]
impl crate::Writable for Ucb1ctl0SpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB1CTL0_SPI to value 0"]
impl crate::Resettable for Ucb1ctl0SpiSpec {}
