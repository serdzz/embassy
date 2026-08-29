#[doc = "Register `HSPLLRIS` reader"]
pub type R = crate::R<HspllrisSpec>;
#[doc = "Register `HSPLLRIS` writer"]
pub type W = crate::W<HspllrisSpec>;
#[doc = "PLL Unlock Raw Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllunlock {
    #[doc = "0: PLL status has not been changed"]
    Pllunlock0 = 0,
    #[doc = "1: PLL status has been changed from Lock to Unlock"]
    Pllunlock1 = 1,
}
impl From<Pllunlock> for bool {
    #[inline(always)]
    fn from(variant: Pllunlock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLUNLOCK` reader - PLL Unlock Raw Interrupt Status bit."]
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
    #[doc = "PLL status has not been changed"]
    #[inline(always)]
    pub fn is_pllunlock_0(&self) -> bool {
        *self == Pllunlock::Pllunlock0
    }
    #[doc = "PLL status has been changed from Lock to Unlock"]
    #[inline(always)]
    pub fn is_pllunlock_1(&self) -> bool {
        *self == Pllunlock::Pllunlock1
    }
}
impl R {
    #[doc = "Bit 0 - PLL Unlock Raw Interrupt Status bit."]
    #[inline(always)]
    pub fn pllunlock(&self) -> PllunlockR {
        PllunlockR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hspllris::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspllris::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HspllrisSpec;
impl crate::RegisterSpec for HspllrisSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hspllris::R`](R) reader structure"]
impl crate::Readable for HspllrisSpec {}
#[doc = "`write(|w| ..)` method takes [`hspllris::W`](W) writer structure"]
impl crate::Writable for HspllrisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSPLLRIS to value 0"]
impl crate::Resettable for HspllrisSpec {}
