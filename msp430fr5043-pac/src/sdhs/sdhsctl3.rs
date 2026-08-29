#[doc = "Register `SDHSCTL3` reader"]
pub type R = crate::R<Sdhsctl3Spec>;
#[doc = "Register `SDHSCTL3` writer"]
pub type W = crate::W<Sdhsctl3Spec>;
#[doc = "SDHS Trigger Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigen {
    #[doc = "0: SDHS Trigger is disabled. Once this bit is de-asserted, CTL0, CTL1, CTL2, CTL7,WINHITH, WINLOTH, and DTCDA registers are unlocked (allowed to be modified)."]
    Trigen0 = 0,
    #[doc = "1: SDHS Trigger is enabled. Once this bit is asserted, CTL0, CTL1, CTL2, CTL7,WINHITH, WINLOTH, and DTCDA registers are locked (not allowed to be modified)."]
    Trigen1 = 1,
}
impl From<Trigen> for bool {
    #[inline(always)]
    fn from(variant: Trigen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGEN` reader - SDHS Trigger Enable bit"]
pub type TrigenR = crate::BitReader<Trigen>;
impl TrigenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigen {
        match self.bits {
            false => Trigen::Trigen0,
            true => Trigen::Trigen1,
        }
    }
    #[doc = "SDHS Trigger is disabled. Once this bit is de-asserted, CTL0, CTL1, CTL2, CTL7,WINHITH, WINLOTH, and DTCDA registers are unlocked (allowed to be modified)."]
    #[inline(always)]
    pub fn is_trigen_0(&self) -> bool {
        *self == Trigen::Trigen0
    }
    #[doc = "SDHS Trigger is enabled. Once this bit is asserted, CTL0, CTL1, CTL2, CTL7,WINHITH, WINLOTH, and DTCDA registers are locked (not allowed to be modified)."]
    #[inline(always)]
    pub fn is_trigen_1(&self) -> bool {
        *self == Trigen::Trigen1
    }
}
#[doc = "Field `TRIGEN` writer - SDHS Trigger Enable bit"]
pub type TrigenW<'a, REG> = crate::BitWriter<'a, REG, Trigen>;
impl<'a, REG> TrigenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDHS Trigger is disabled. Once this bit is de-asserted, CTL0, CTL1, CTL2, CTL7,WINHITH, WINLOTH, and DTCDA registers are unlocked (allowed to be modified)."]
    #[inline(always)]
    pub fn trigen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigen::Trigen0)
    }
    #[doc = "SDHS Trigger is enabled. Once this bit is asserted, CTL0, CTL1, CTL2, CTL7,WINHITH, WINLOTH, and DTCDA registers are locked (not allowed to be modified)."]
    #[inline(always)]
    pub fn trigen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigen::Trigen1)
    }
}
impl R {
    #[doc = "Bit 0 - SDHS Trigger Enable bit"]
    #[inline(always)]
    pub fn trigen(&self) -> TrigenR {
        TrigenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDHS Trigger Enable bit"]
    #[inline(always)]
    pub fn trigen(&mut self) -> TrigenW<'_, Sdhsctl3Spec> {
        TrigenW::new(self, 0)
    }
}
#[doc = "SDHS Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdhsctl3Spec;
impl crate::RegisterSpec for Sdhsctl3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsctl3::R`](R) reader structure"]
impl crate::Readable for Sdhsctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`sdhsctl3::W`](W) writer structure"]
impl crate::Writable for Sdhsctl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSCTL3 to value 0"]
impl crate::Resettable for Sdhsctl3Spec {}
