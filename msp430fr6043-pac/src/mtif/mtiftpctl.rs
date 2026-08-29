#[doc = "Register `MTIFTPCTL` reader"]
pub type R = crate::R<MtiftpctlSpec>;
#[doc = "Register `MTIFTPCTL` writer"]
pub type W = crate::W<MtiftpctlSpec>;
#[doc = "Field `TPOE` reader - Test port output enable. This bit allows to enable the test pulse output when set to one"]
pub type TpoeR = crate::BitReader;
#[doc = "Field `TPOE` writer - Test port output enable. This bit allows to enable the test pulse output when set to one"]
pub type TpoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPIE` reader - Test port input enable. This bit allows to enable the test input port"]
pub type TpieR = crate::BitReader;
#[doc = "Field `TPIE` writer - Test port input enable. This bit allows to enable the test input port"]
pub type TpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Test port input select for pulse counter. This value determines the source for the pulse counter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpisel {
    #[doc = "0: The pulse generator is used as input"]
    Tpisel0 = 0,
    #[doc = "1: The test port input terminal is selected as input"]
    Tpisel1 = 1,
}
impl From<Tpisel> for bool {
    #[inline(always)]
    fn from(variant: Tpisel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPISEL` reader - Test port input select for pulse counter. This value determines the source for the pulse counter."]
pub type TpiselR = crate::BitReader<Tpisel>;
impl TpiselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpisel {
        match self.bits {
            false => Tpisel::Tpisel0,
            true => Tpisel::Tpisel1,
        }
    }
    #[doc = "The pulse generator is used as input"]
    #[inline(always)]
    pub fn is_tpisel_0(&self) -> bool {
        *self == Tpisel::Tpisel0
    }
    #[doc = "The test port input terminal is selected as input"]
    #[inline(always)]
    pub fn is_tpisel_1(&self) -> bool {
        *self == Tpisel::Tpisel1
    }
}
#[doc = "Field `TPISEL` writer - Test port input select for pulse counter. This value determines the source for the pulse counter."]
pub type TpiselW<'a, REG> = crate::BitWriter<'a, REG, Tpisel>;
impl<'a, REG> TpiselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The pulse generator is used as input"]
    #[inline(always)]
    pub fn tpisel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpisel::Tpisel0)
    }
    #[doc = "The test port input terminal is selected as input"]
    #[inline(always)]
    pub fn tpisel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpisel::Tpisel1)
    }
}
#[doc = "Test port terminal enable activation. This value determines if the testport output is enabled solely by software or by software and hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activate {
    #[doc = "0: The test port output is enabled solely by TPOE (enabled if TPOE=1)"]
    Activate0 = 0,
    #[doc = "1: The testport output requires both TPOE to be high and the MTPE pin to be high to be enabled"]
    Activate1 = 1,
}
impl From<Activate> for bool {
    #[inline(always)]
    fn from(variant: Activate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVATE` reader - Test port terminal enable activation. This value determines if the testport output is enabled solely by software or by software and hardware."]
pub type ActivateR = crate::BitReader<Activate>;
impl ActivateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Activate {
        match self.bits {
            false => Activate::Activate0,
            true => Activate::Activate1,
        }
    }
    #[doc = "The test port output is enabled solely by TPOE (enabled if TPOE=1)"]
    #[inline(always)]
    pub fn is_activate_0(&self) -> bool {
        *self == Activate::Activate0
    }
    #[doc = "The testport output requires both TPOE to be high and the MTPE pin to be high to be enabled"]
    #[inline(always)]
    pub fn is_activate_1(&self) -> bool {
        *self == Activate::Activate1
    }
}
#[doc = "Field `ACTIVATE` writer - Test port terminal enable activation. This value determines if the testport output is enabled solely by software or by software and hardware."]
pub type ActivateW<'a, REG> = crate::BitWriter<'a, REG, Activate>;
impl<'a, REG> ActivateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The test port output is enabled solely by TPOE (enabled if TPOE=1)"]
    #[inline(always)]
    pub fn activate_0(self) -> &'a mut crate::W<REG> {
        self.variant(Activate::Activate0)
    }
    #[doc = "The testport output requires both TPOE to be high and the MTPE pin to be high to be enabled"]
    #[inline(always)]
    pub fn activate_1(self) -> &'a mut crate::W<REG> {
        self.variant(Activate::Activate1)
    }
}
#[doc = "Field `TPPW` reader - Test port password. Always reads as 0x0F. Must be written as 0xC3 for register changes to be effective.This password differs from the pulse generator and pulse counter passwords"]
pub type TppwR = crate::FieldReader;
#[doc = "Field `TPPW` writer - Test port password. Always reads as 0x0F. Must be written as 0xC3 for register changes to be effective.This password differs from the pulse generator and pulse counter passwords"]
pub type TppwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Test port output enable. This bit allows to enable the test pulse output when set to one"]
    #[inline(always)]
    pub fn tpoe(&self) -> TpoeR {
        TpoeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Test port input enable. This bit allows to enable the test input port"]
    #[inline(always)]
    pub fn tpie(&self) -> TpieR {
        TpieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Test port input select for pulse counter. This value determines the source for the pulse counter."]
    #[inline(always)]
    pub fn tpisel(&self) -> TpiselR {
        TpiselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Test port terminal enable activation. This value determines if the testport output is enabled solely by software or by software and hardware."]
    #[inline(always)]
    pub fn activate(&self) -> ActivateR {
        ActivateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Test port password. Always reads as 0x0F. Must be written as 0xC3 for register changes to be effective.This password differs from the pulse generator and pulse counter passwords"]
    #[inline(always)]
    pub fn tppw(&self) -> TppwR {
        TppwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Test port output enable. This bit allows to enable the test pulse output when set to one"]
    #[inline(always)]
    pub fn tpoe(&mut self) -> TpoeW<'_, MtiftpctlSpec> {
        TpoeW::new(self, 0)
    }
    #[doc = "Bit 1 - Test port input enable. This bit allows to enable the test input port"]
    #[inline(always)]
    pub fn tpie(&mut self) -> TpieW<'_, MtiftpctlSpec> {
        TpieW::new(self, 1)
    }
    #[doc = "Bit 2 - Test port input select for pulse counter. This value determines the source for the pulse counter."]
    #[inline(always)]
    pub fn tpisel(&mut self) -> TpiselW<'_, MtiftpctlSpec> {
        TpiselW::new(self, 2)
    }
    #[doc = "Bit 3 - Test port terminal enable activation. This value determines if the testport output is enabled solely by software or by software and hardware."]
    #[inline(always)]
    pub fn activate(&mut self) -> ActivateW<'_, MtiftpctlSpec> {
        ActivateW::new(self, 3)
    }
    #[doc = "Bits 8:15 - Test port password. Always reads as 0x0F. Must be written as 0xC3 for register changes to be effective.This password differs from the pulse generator and pulse counter passwords"]
    #[inline(always)]
    pub fn tppw(&mut self) -> TppwW<'_, MtiftpctlSpec> {
        TppwW::new(self, 8)
    }
}
#[doc = "Measurement Test Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtiftpctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtiftpctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtiftpctlSpec;
impl crate::RegisterSpec for MtiftpctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mtiftpctl::R`](R) reader structure"]
impl crate::Readable for MtiftpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`mtiftpctl::W`](W) writer structure"]
impl crate::Writable for MtiftpctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIFTPCTL to value 0"]
impl crate::Resettable for MtiftpctlSpec {}
