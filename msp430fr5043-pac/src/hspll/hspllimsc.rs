#[doc = "Register `HSPLLIMSC` reader"]
pub type R = crate::R<HspllimscSpec>;
#[doc = "Register `HSPLLIMSC` writer"]
pub type W = crate::W<HspllimscSpec>;
#[doc = "PLL Unlock Interrupt Mask bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllunlock {
    #[doc = "0: PLL Unlock Interrupt is disabled"]
    Pllunlock0 = 0,
    #[doc = "1: PLL Unlock Interrupt is enabled"]
    Pllunlock1 = 1,
}
impl From<Pllunlock> for bool {
    #[inline(always)]
    fn from(variant: Pllunlock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLUNLOCK` reader - PLL Unlock Interrupt Mask bit."]
pub type PllunlockR = crate::BitReader<Pllunlock>;
impl PllunlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllunlock {
        match self.bits {
            false => Pllunlock::Pllunlock0,
            true => Pllunlock::Pllunlock1,
        }
    }
    #[doc = "PLL Unlock Interrupt is disabled"]
    #[inline(always)]
    pub fn is_pllunlock_0(&self) -> bool {
        *self == Pllunlock::Pllunlock0
    }
    #[doc = "PLL Unlock Interrupt is enabled"]
    #[inline(always)]
    pub fn is_pllunlock_1(&self) -> bool {
        *self == Pllunlock::Pllunlock1
    }
}
#[doc = "Field `PLLUNLOCK` writer - PLL Unlock Interrupt Mask bit."]
pub type PllunlockW<'a, REG> = crate::BitWriter<'a, REG, Pllunlock>;
impl<'a, REG> PllunlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL Unlock Interrupt is disabled"]
    #[inline(always)]
    pub fn pllunlock_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllunlock::Pllunlock0)
    }
    #[doc = "PLL Unlock Interrupt is enabled"]
    #[inline(always)]
    pub fn pllunlock_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllunlock::Pllunlock1)
    }
}
impl R {
    #[doc = "Bit 0 - PLL Unlock Interrupt Mask bit."]
    #[inline(always)]
    pub fn pllunlock(&self) -> PllunlockR {
        PllunlockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Unlock Interrupt Mask bit."]
    #[inline(always)]
    pub fn pllunlock(&mut self) -> PllunlockW<'_, HspllimscSpec> {
        PllunlockW::new(self, 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hspllimsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspllimsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HspllimscSpec;
impl crate::RegisterSpec for HspllimscSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hspllimsc::R`](R) reader structure"]
impl crate::Readable for HspllimscSpec {}
#[doc = "`write(|w| ..)` method takes [`hspllimsc::W`](W) writer structure"]
impl crate::Writable for HspllimscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSPLLIMSC to value 0"]
impl crate::Resettable for HspllimscSpec {}
