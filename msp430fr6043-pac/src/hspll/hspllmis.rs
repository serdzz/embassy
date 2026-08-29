#[doc = "Register `HSPLLMIS` reader"]
pub type R = crate::R<HspllmisSpec>;
#[doc = "Register `HSPLLMIS` writer"]
pub type W = crate::W<HspllmisSpec>;
#[doc = "HSPLL Unlock Masked Interrupt Status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllunlock {
    #[doc = "0: No interrupt pending"]
    Pllunlock0 = 0,
    #[doc = "1: Interrupt pending"]
    Pllunlock1 = 1,
}
impl From<Pllunlock> for bool {
    #[inline(always)]
    fn from(variant: Pllunlock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLUNLOCK` reader - HSPLL Unlock Masked Interrupt Status bit"]
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
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_pllunlock_0(&self) -> bool {
        *self == Pllunlock::Pllunlock0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_pllunlock_1(&self) -> bool {
        *self == Pllunlock::Pllunlock1
    }
}
impl R {
    #[doc = "Bit 0 - HSPLL Unlock Masked Interrupt Status bit"]
    #[inline(always)]
    pub fn pllunlock(&self) -> PllunlockR {
        PllunlockR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "Masked Interrupt Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hspllmis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspllmis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HspllmisSpec;
impl crate::RegisterSpec for HspllmisSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hspllmis::R`](R) reader structure"]
impl crate::Readable for HspllmisSpec {}
#[doc = "`write(|w| ..)` method takes [`hspllmis::W`](W) writer structure"]
impl crate::Writable for HspllmisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSPLLMIS to value 0"]
impl crate::Resettable for HspllmisSpec {}
