#[doc = "Register `HSPLLCTL` reader"]
pub type R = crate::R<HspllctlSpec>;
#[doc = "Register `HSPLLCTL` writer"]
pub type W = crate::W<HspllctlSpec>;
#[doc = "PLL Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllLock {
    #[doc = "0: PLL is not running or not locked"]
    PllLock0 = 0,
    #[doc = "1: PLL is locked"]
    PllLock1 = 1,
}
impl From<PllLock> for bool {
    #[inline(always)]
    fn from(variant: PllLock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_LOCK` reader - PLL Lock Status"]
pub type PllLockR = crate::BitReader<PllLock>;
impl PllLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllLock {
        match self.bits {
            false => PllLock::PllLock0,
            true => PllLock::PllLock1,
        }
    }
    #[doc = "PLL is not running or not locked"]
    #[inline(always)]
    pub fn is_pll_lock_0(&self) -> bool {
        *self == PllLock::PllLock0
    }
    #[doc = "PLL is locked"]
    #[inline(always)]
    pub fn is_pll_lock_1(&self) -> bool {
        *self == PllLock::PllLock1
    }
}
#[doc = "PLL Input Frequency Selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllinfreq {
    #[doc = "0: Input frequency is equal to 6MHz or lower than 6MHz"]
    Pllinfreq0 = 0,
    #[doc = "1: Input frequency is higher than 6MHz"]
    Pllinfreq1 = 1,
}
impl From<Pllinfreq> for bool {
    #[inline(always)]
    fn from(variant: Pllinfreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLINFREQ` reader - PLL Input Frequency Selection."]
pub type PllinfreqR = crate::BitReader<Pllinfreq>;
impl PllinfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllinfreq {
        match self.bits {
            false => Pllinfreq::Pllinfreq0,
            true => Pllinfreq::Pllinfreq1,
        }
    }
    #[doc = "Input frequency is equal to 6MHz or lower than 6MHz"]
    #[inline(always)]
    pub fn is_pllinfreq_0(&self) -> bool {
        *self == Pllinfreq::Pllinfreq0
    }
    #[doc = "Input frequency is higher than 6MHz"]
    #[inline(always)]
    pub fn is_pllinfreq_1(&self) -> bool {
        *self == Pllinfreq::Pllinfreq1
    }
}
#[doc = "Field `PLLINFREQ` writer - PLL Input Frequency Selection."]
pub type PllinfreqW<'a, REG> = crate::BitWriter<'a, REG, Pllinfreq>;
impl<'a, REG> PllinfreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input frequency is equal to 6MHz or lower than 6MHz"]
    #[inline(always)]
    pub fn pllinfreq_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllinfreq::Pllinfreq0)
    }
    #[doc = "Input frequency is higher than 6MHz"]
    #[inline(always)]
    pub fn pllinfreq_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllinfreq::Pllinfreq1)
    }
}
#[doc = "PLL Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllm {
    #[doc = "16: PLLM_16"]
    Pllm16 = 16,
    #[doc = "17: PLLM_17"]
    Pllm17 = 17,
    #[doc = "18: PLLM_18"]
    Pllm18 = 18,
    #[doc = "19: PLLM_19"]
    Pllm19 = 19,
    #[doc = "20: PLLM_20"]
    Pllm20 = 20,
    #[doc = "21: PLLM_21"]
    Pllm21 = 21,
    #[doc = "22: PLLM_22"]
    Pllm22 = 22,
    #[doc = "23: PLLM_23"]
    Pllm23 = 23,
    #[doc = "24: PLLM_24"]
    Pllm24 = 24,
    #[doc = "25: PLLM_25"]
    Pllm25 = 25,
    #[doc = "26: PLLM_26"]
    Pllm26 = 26,
    #[doc = "27: PLLM_27"]
    Pllm27 = 27,
    #[doc = "28: PLLM_28"]
    Pllm28 = 28,
    #[doc = "29: PLLM_29"]
    Pllm29 = 29,
    #[doc = "30: PLLM_30"]
    Pllm30 = 30,
    #[doc = "31: PLLM_31"]
    Pllm31 = 31,
    #[doc = "32: PLLM_32"]
    Pllm32 = 32,
    #[doc = "33: PLLM_33"]
    Pllm33 = 33,
    #[doc = "34: PLLM_34"]
    Pllm34 = 34,
    #[doc = "35: PLLM_35"]
    Pllm35 = 35,
    #[doc = "36: PLLM_36"]
    Pllm36 = 36,
    #[doc = "37: PLLM_37"]
    Pllm37 = 37,
    #[doc = "38: PLLM_38"]
    Pllm38 = 38,
    #[doc = "39: PLLM_39"]
    Pllm39 = 39,
}
impl From<Pllm> for u8 {
    #[inline(always)]
    fn from(variant: Pllm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllm {
    type Ux = u8;
}
impl crate::IsEnum for Pllm {}
#[doc = "Field `PLLM` reader - PLL Multiplier"]
pub type PllmR = crate::FieldReader<Pllm>;
impl PllmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pllm> {
        match self.bits {
            16 => Some(Pllm::Pllm16),
            17 => Some(Pllm::Pllm17),
            18 => Some(Pllm::Pllm18),
            19 => Some(Pllm::Pllm19),
            20 => Some(Pllm::Pllm20),
            21 => Some(Pllm::Pllm21),
            22 => Some(Pllm::Pllm22),
            23 => Some(Pllm::Pllm23),
            24 => Some(Pllm::Pllm24),
            25 => Some(Pllm::Pllm25),
            26 => Some(Pllm::Pllm26),
            27 => Some(Pllm::Pllm27),
            28 => Some(Pllm::Pllm28),
            29 => Some(Pllm::Pllm29),
            30 => Some(Pllm::Pllm30),
            31 => Some(Pllm::Pllm31),
            32 => Some(Pllm::Pllm32),
            33 => Some(Pllm::Pllm33),
            34 => Some(Pllm::Pllm34),
            35 => Some(Pllm::Pllm35),
            36 => Some(Pllm::Pllm36),
            37 => Some(Pllm::Pllm37),
            38 => Some(Pllm::Pllm38),
            39 => Some(Pllm::Pllm39),
            _ => None,
        }
    }
    #[doc = "PLLM_16"]
    #[inline(always)]
    pub fn is_pllm_16(&self) -> bool {
        *self == Pllm::Pllm16
    }
    #[doc = "PLLM_17"]
    #[inline(always)]
    pub fn is_pllm_17(&self) -> bool {
        *self == Pllm::Pllm17
    }
    #[doc = "PLLM_18"]
    #[inline(always)]
    pub fn is_pllm_18(&self) -> bool {
        *self == Pllm::Pllm18
    }
    #[doc = "PLLM_19"]
    #[inline(always)]
    pub fn is_pllm_19(&self) -> bool {
        *self == Pllm::Pllm19
    }
    #[doc = "PLLM_20"]
    #[inline(always)]
    pub fn is_pllm_20(&self) -> bool {
        *self == Pllm::Pllm20
    }
    #[doc = "PLLM_21"]
    #[inline(always)]
    pub fn is_pllm_21(&self) -> bool {
        *self == Pllm::Pllm21
    }
    #[doc = "PLLM_22"]
    #[inline(always)]
    pub fn is_pllm_22(&self) -> bool {
        *self == Pllm::Pllm22
    }
    #[doc = "PLLM_23"]
    #[inline(always)]
    pub fn is_pllm_23(&self) -> bool {
        *self == Pllm::Pllm23
    }
    #[doc = "PLLM_24"]
    #[inline(always)]
    pub fn is_pllm_24(&self) -> bool {
        *self == Pllm::Pllm24
    }
    #[doc = "PLLM_25"]
    #[inline(always)]
    pub fn is_pllm_25(&self) -> bool {
        *self == Pllm::Pllm25
    }
    #[doc = "PLLM_26"]
    #[inline(always)]
    pub fn is_pllm_26(&self) -> bool {
        *self == Pllm::Pllm26
    }
    #[doc = "PLLM_27"]
    #[inline(always)]
    pub fn is_pllm_27(&self) -> bool {
        *self == Pllm::Pllm27
    }
    #[doc = "PLLM_28"]
    #[inline(always)]
    pub fn is_pllm_28(&self) -> bool {
        *self == Pllm::Pllm28
    }
    #[doc = "PLLM_29"]
    #[inline(always)]
    pub fn is_pllm_29(&self) -> bool {
        *self == Pllm::Pllm29
    }
    #[doc = "PLLM_30"]
    #[inline(always)]
    pub fn is_pllm_30(&self) -> bool {
        *self == Pllm::Pllm30
    }
    #[doc = "PLLM_31"]
    #[inline(always)]
    pub fn is_pllm_31(&self) -> bool {
        *self == Pllm::Pllm31
    }
    #[doc = "PLLM_32"]
    #[inline(always)]
    pub fn is_pllm_32(&self) -> bool {
        *self == Pllm::Pllm32
    }
    #[doc = "PLLM_33"]
    #[inline(always)]
    pub fn is_pllm_33(&self) -> bool {
        *self == Pllm::Pllm33
    }
    #[doc = "PLLM_34"]
    #[inline(always)]
    pub fn is_pllm_34(&self) -> bool {
        *self == Pllm::Pllm34
    }
    #[doc = "PLLM_35"]
    #[inline(always)]
    pub fn is_pllm_35(&self) -> bool {
        *self == Pllm::Pllm35
    }
    #[doc = "PLLM_36"]
    #[inline(always)]
    pub fn is_pllm_36(&self) -> bool {
        *self == Pllm::Pllm36
    }
    #[doc = "PLLM_37"]
    #[inline(always)]
    pub fn is_pllm_37(&self) -> bool {
        *self == Pllm::Pllm37
    }
    #[doc = "PLLM_38"]
    #[inline(always)]
    pub fn is_pllm_38(&self) -> bool {
        *self == Pllm::Pllm38
    }
    #[doc = "PLLM_39"]
    #[inline(always)]
    pub fn is_pllm_39(&self) -> bool {
        *self == Pllm::Pllm39
    }
}
#[doc = "Field `PLLM` writer - PLL Multiplier"]
pub type PllmW<'a, REG> = crate::FieldWriter<'a, REG, 6, Pllm>;
impl<'a, REG> PllmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLM_16"]
    #[inline(always)]
    pub fn pllm_16(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm16)
    }
    #[doc = "PLLM_17"]
    #[inline(always)]
    pub fn pllm_17(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm17)
    }
    #[doc = "PLLM_18"]
    #[inline(always)]
    pub fn pllm_18(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm18)
    }
    #[doc = "PLLM_19"]
    #[inline(always)]
    pub fn pllm_19(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm19)
    }
    #[doc = "PLLM_20"]
    #[inline(always)]
    pub fn pllm_20(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm20)
    }
    #[doc = "PLLM_21"]
    #[inline(always)]
    pub fn pllm_21(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm21)
    }
    #[doc = "PLLM_22"]
    #[inline(always)]
    pub fn pllm_22(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm22)
    }
    #[doc = "PLLM_23"]
    #[inline(always)]
    pub fn pllm_23(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm23)
    }
    #[doc = "PLLM_24"]
    #[inline(always)]
    pub fn pllm_24(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm24)
    }
    #[doc = "PLLM_25"]
    #[inline(always)]
    pub fn pllm_25(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm25)
    }
    #[doc = "PLLM_26"]
    #[inline(always)]
    pub fn pllm_26(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm26)
    }
    #[doc = "PLLM_27"]
    #[inline(always)]
    pub fn pllm_27(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm27)
    }
    #[doc = "PLLM_28"]
    #[inline(always)]
    pub fn pllm_28(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm28)
    }
    #[doc = "PLLM_29"]
    #[inline(always)]
    pub fn pllm_29(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm29)
    }
    #[doc = "PLLM_30"]
    #[inline(always)]
    pub fn pllm_30(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm30)
    }
    #[doc = "PLLM_31"]
    #[inline(always)]
    pub fn pllm_31(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm31)
    }
    #[doc = "PLLM_32"]
    #[inline(always)]
    pub fn pllm_32(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm32)
    }
    #[doc = "PLLM_33"]
    #[inline(always)]
    pub fn pllm_33(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm33)
    }
    #[doc = "PLLM_34"]
    #[inline(always)]
    pub fn pllm_34(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm34)
    }
    #[doc = "PLLM_35"]
    #[inline(always)]
    pub fn pllm_35(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm35)
    }
    #[doc = "PLLM_36"]
    #[inline(always)]
    pub fn pllm_36(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm36)
    }
    #[doc = "PLLM_37"]
    #[inline(always)]
    pub fn pllm_37(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm37)
    }
    #[doc = "PLLM_38"]
    #[inline(always)]
    pub fn pllm_38(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm38)
    }
    #[doc = "PLLM_39"]
    #[inline(always)]
    pub fn pllm_39(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::Pllm39)
    }
}
impl R {
    #[doc = "Bit 0 - PLL Lock Status"]
    #[inline(always)]
    pub fn pll_lock(&self) -> PllLockR {
        PllLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - PLL Input Frequency Selection."]
    #[inline(always)]
    pub fn pllinfreq(&self) -> PllinfreqR {
        PllinfreqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:15 - PLL Multiplier"]
    #[inline(always)]
    pub fn pllm(&self) -> PllmR {
        PllmR::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - PLL Input Frequency Selection."]
    #[inline(always)]
    pub fn pllinfreq(&mut self) -> PllinfreqW<'_, HspllctlSpec> {
        PllinfreqW::new(self, 8)
    }
    #[doc = "Bits 10:15 - PLL Multiplier"]
    #[inline(always)]
    pub fn pllm(&mut self) -> PllmW<'_, HspllctlSpec> {
        PllmW::new(self, 10)
    }
}
#[doc = "HSPLL Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hspllctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspllctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HspllctlSpec;
impl crate::RegisterSpec for HspllctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hspllctl::R`](R) reader structure"]
impl crate::Readable for HspllctlSpec {}
#[doc = "`write(|w| ..)` method takes [`hspllctl::W`](W) writer structure"]
impl crate::Writable for HspllctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSPLLCTL to value 0"]
impl crate::Resettable for HspllctlSpec {}
