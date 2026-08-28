#[doc = "Register `UCB1CTL0` reader"]
pub type R = crate::R<Ucb1ctl0Spec>;
#[doc = "Register `UCB1CTL0` writer"]
pub type W = crate::W<Ucb1ctl0Spec>;
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
#[doc = "Field `UCMM` reader - Multi-Master Environment"]
pub type UcmmR = crate::BitReader;
#[doc = "Field `UCMM` writer - Multi-Master Environment"]
pub type UcmmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSLA10` reader - 10-bit Slave Address Mode"]
pub type Ucsla10R = crate::BitReader;
#[doc = "Field `UCSLA10` writer - 10-bit Slave Address Mode"]
pub type Ucsla10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCA10` reader - 10-bit Address Mode"]
pub type Uca10R = crate::BitReader;
#[doc = "Field `UCA10` writer - 10-bit Address Mode"]
pub type Uca10W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 5 - Multi-Master Environment"]
    #[inline(always)]
    pub fn ucmm(&self) -> UcmmR {
        UcmmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 10-bit Slave Address Mode"]
    #[inline(always)]
    pub fn ucsla10(&self) -> Ucsla10R {
        Ucsla10R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 10-bit Address Mode"]
    #[inline(always)]
    pub fn uca10(&self) -> Uca10R {
        Uca10R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UcsyncW<'_, Ucb1ctl0Spec> {
        UcsyncW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Sync. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UcmodeW<'_, Ucb1ctl0Spec> {
        UcmodeW::new(self, 1)
    }
    #[doc = "Bit 3 - Sync. Mode: Master Select"]
    #[inline(always)]
    pub fn ucmst(&mut self) -> UcmstW<'_, Ucb1ctl0Spec> {
        UcmstW::new(self, 3)
    }
    #[doc = "Bit 5 - Multi-Master Environment"]
    #[inline(always)]
    pub fn ucmm(&mut self) -> UcmmW<'_, Ucb1ctl0Spec> {
        UcmmW::new(self, 5)
    }
    #[doc = "Bit 6 - 10-bit Slave Address Mode"]
    #[inline(always)]
    pub fn ucsla10(&mut self) -> Ucsla10W<'_, Ucb1ctl0Spec> {
        Ucsla10W::new(self, 6)
    }
    #[doc = "Bit 7 - 10-bit Address Mode"]
    #[inline(always)]
    pub fn uca10(&mut self) -> Uca10W<'_, Ucb1ctl0Spec> {
        Uca10W::new(self, 7)
    }
}
#[doc = "USCI B1 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb1ctl0Spec;
impl crate::RegisterSpec for Ucb1ctl0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb1ctl0::R`](R) reader structure"]
impl crate::Readable for Ucb1ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ucb1ctl0::W`](W) writer structure"]
impl crate::Writable for Ucb1ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB1CTL0 to value 0"]
impl crate::Resettable for Ucb1ctl0Spec {}
