#[doc = "Register `SDHSCTL5` reader"]
pub type R = crate::R<Sdhsctl5Spec>;
#[doc = "Register `SDHSCTL5` writer"]
pub type W = crate::W<Sdhsctl5Spec>;
#[doc = "Start of conversion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sstart {
    #[doc = "0: Stop conversion"]
    Sstart0 = 0,
    #[doc = "1: Start conversion"]
    Sstart1 = 1,
}
impl From<Sstart> for bool {
    #[inline(always)]
    fn from(variant: Sstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSTART` reader - Start of conversion."]
pub type SstartR = crate::BitReader<Sstart>;
impl SstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sstart {
        match self.bits {
            false => Sstart::Sstart0,
            true => Sstart::Sstart1,
        }
    }
    #[doc = "Stop conversion"]
    #[inline(always)]
    pub fn is_sstart_0(&self) -> bool {
        *self == Sstart::Sstart0
    }
    #[doc = "Start conversion"]
    #[inline(always)]
    pub fn is_sstart_1(&self) -> bool {
        *self == Sstart::Sstart1
    }
}
#[doc = "Field `SSTART` writer - Start of conversion."]
pub type SstartW<'a, REG> = crate::BitWriter<'a, REG, Sstart>;
impl<'a, REG> SstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop conversion"]
    #[inline(always)]
    pub fn sstart_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sstart::Sstart0)
    }
    #[doc = "Start conversion"]
    #[inline(always)]
    pub fn sstart_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sstart::Sstart1)
    }
}
#[doc = "Start of conversion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdhsLock {
    #[doc = "0: CTL3 register is unlocked."]
    SdhsLock0 = 0,
    #[doc = "1: CTL3 register is locked as well as CTL0, CTL1, CTL2, CTL7,WINHITH, WINLOTH, and DTCDA registers. Only read is allowed."]
    SdhsLock1 = 1,
}
impl From<SdhsLock> for bool {
    #[inline(always)]
    fn from(variant: SdhsLock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDHS_LOCK` reader - Start of conversion."]
pub type SdhsLockR = crate::BitReader<SdhsLock>;
impl SdhsLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SdhsLock {
        match self.bits {
            false => SdhsLock::SdhsLock0,
            true => SdhsLock::SdhsLock1,
        }
    }
    #[doc = "CTL3 register is unlocked."]
    #[inline(always)]
    pub fn is_sdhs_lock_0(&self) -> bool {
        *self == SdhsLock::SdhsLock0
    }
    #[doc = "CTL3 register is locked as well as CTL0, CTL1, CTL2, CTL7,WINHITH, WINLOTH, and DTCDA registers. Only read is allowed."]
    #[inline(always)]
    pub fn is_sdhs_lock_1(&self) -> bool {
        *self == SdhsLock::SdhsLock1
    }
}
impl R {
    #[doc = "Bit 0 - Start of conversion."]
    #[inline(always)]
    pub fn sstart(&self) -> SstartR {
        SstartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Start of conversion."]
    #[inline(always)]
    pub fn sdhs_lock(&self) -> SdhsLockR {
        SdhsLockR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of conversion."]
    #[inline(always)]
    pub fn sstart(&mut self) -> SstartW<'_, Sdhsctl5Spec> {
        SstartW::new(self, 0)
    }
}
#[doc = "SDHS Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdhsctl5Spec;
impl crate::RegisterSpec for Sdhsctl5Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsctl5::R`](R) reader structure"]
impl crate::Readable for Sdhsctl5Spec {}
#[doc = "`write(|w| ..)` method takes [`sdhsctl5::W`](W) writer structure"]
impl crate::Writable for Sdhsctl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSCTL5 to value 0"]
impl crate::Resettable for Sdhsctl5Spec {}
