#[doc = "Register `UCA0CTL0` reader"]
pub type R = crate::R<Uca0ctl0Spec>;
#[doc = "Register `UCA0CTL0` writer"]
pub type W = crate::W<Uca0ctl0Spec>;
#[doc = "Field `UCSYNC` reader - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub type UcsyncR = crate::BitReader;
#[doc = "Field `UCSYNC` writer - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub type UcsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Async. Mode: USCI Mode 1\n\nValue on reset: 0"]
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
#[doc = "Field `UCMODE` reader - Async. Mode: USCI Mode 1"]
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
#[doc = "Field `UCMODE` writer - Async. Mode: USCI Mode 1"]
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
#[doc = "Field `UCSPB` reader - Async. Mode: Stop Bits 0:one / 1: two"]
pub type UcspbR = crate::BitReader;
#[doc = "Field `UCSPB` writer - Async. Mode: Stop Bits 0:one / 1: two"]
pub type UcspbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UC7BIT` reader - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
pub type Uc7bitR = crate::BitReader;
#[doc = "Field `UC7BIT` writer - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
pub type Uc7bitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCMSB` reader - Async. Mode: MSB first 0:LSB / 1:MSB"]
pub type UcmsbR = crate::BitReader;
#[doc = "Field `UCMSB` writer - Async. Mode: MSB first 0:LSB / 1:MSB"]
pub type UcmsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPAR` reader - Async. Mode: Parity 0:odd / 1:even"]
pub type UcparR = crate::BitReader;
#[doc = "Field `UCPAR` writer - Async. Mode: Parity 0:odd / 1:even"]
pub type UcparW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPEN` reader - Async. Mode: Parity enable"]
pub type UcpenR = crate::BitReader;
#[doc = "Field `UCPEN` writer - Async. Mode: Parity enable"]
pub type UcpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&self) -> UcsyncR {
        UcsyncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Async. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&self) -> UcmodeR {
        UcmodeR::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Async. Mode: Stop Bits 0:one / 1: two"]
    #[inline(always)]
    pub fn ucspb(&self) -> UcspbR {
        UcspbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
    #[inline(always)]
    pub fn uc7bit(&self) -> Uc7bitR {
        Uc7bitR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Async. Mode: MSB first 0:LSB / 1:MSB"]
    #[inline(always)]
    pub fn ucmsb(&self) -> UcmsbR {
        UcmsbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Async. Mode: Parity 0:odd / 1:even"]
    #[inline(always)]
    pub fn ucpar(&self) -> UcparR {
        UcparR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Async. Mode: Parity enable"]
    #[inline(always)]
    pub fn ucpen(&self) -> UcpenR {
        UcpenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UcsyncW<'_, Uca0ctl0Spec> {
        UcsyncW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Async. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UcmodeW<'_, Uca0ctl0Spec> {
        UcmodeW::new(self, 1)
    }
    #[doc = "Bit 3 - Async. Mode: Stop Bits 0:one / 1: two"]
    #[inline(always)]
    pub fn ucspb(&mut self) -> UcspbW<'_, Uca0ctl0Spec> {
        UcspbW::new(self, 3)
    }
    #[doc = "Bit 4 - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
    #[inline(always)]
    pub fn uc7bit(&mut self) -> Uc7bitW<'_, Uca0ctl0Spec> {
        Uc7bitW::new(self, 4)
    }
    #[doc = "Bit 5 - Async. Mode: MSB first 0:LSB / 1:MSB"]
    #[inline(always)]
    pub fn ucmsb(&mut self) -> UcmsbW<'_, Uca0ctl0Spec> {
        UcmsbW::new(self, 5)
    }
    #[doc = "Bit 6 - Async. Mode: Parity 0:odd / 1:even"]
    #[inline(always)]
    pub fn ucpar(&mut self) -> UcparW<'_, Uca0ctl0Spec> {
        UcparW::new(self, 6)
    }
    #[doc = "Bit 7 - Async. Mode: Parity enable"]
    #[inline(always)]
    pub fn ucpen(&mut self) -> UcpenW<'_, Uca0ctl0Spec> {
        UcpenW::new(self, 7)
    }
}
#[doc = "USCI A0 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0ctl0Spec;
impl crate::RegisterSpec for Uca0ctl0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0ctl0::R`](R) reader structure"]
impl crate::Readable for Uca0ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`uca0ctl0::W`](W) writer structure"]
impl crate::Writable for Uca0ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0CTL0 to value 0"]
impl crate::Resettable for Uca0ctl0Spec {}
