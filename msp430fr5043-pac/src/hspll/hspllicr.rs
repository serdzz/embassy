#[doc = "Register `HSPLLICR` reader"]
pub type R = crate::R<HspllicrSpec>;
#[doc = "Register `HSPLLICR` writer"]
pub type W = crate::W<HspllicrSpec>;
#[doc = "Field `PLLUNLOCK` reader - PLL Unlock Interrupt Clear bit."]
pub type PllunlockR = crate::BitReader;
#[doc = "Field `PLLUNLOCK` writer - PLL Unlock Interrupt Clear bit."]
pub type PllunlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLL Unlock Interrupt Clear bit."]
    #[inline(always)]
    pub fn pllunlock(&self) -> PllunlockR {
        PllunlockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Unlock Interrupt Clear bit."]
    #[inline(always)]
    pub fn pllunlock(&mut self) -> PllunlockW<'_, HspllicrSpec> {
        PllunlockW::new(self, 0)
    }
}
#[doc = "Interrupt Flag Clear Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hspllicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspllicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HspllicrSpec;
impl crate::RegisterSpec for HspllicrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hspllicr::R`](R) reader structure"]
impl crate::Readable for HspllicrSpec {}
#[doc = "`write(|w| ..)` method takes [`hspllicr::W`](W) writer structure"]
impl crate::Writable for HspllicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSPLLICR to value 0"]
impl crate::Resettable for HspllicrSpec {}
