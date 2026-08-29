#[doc = "Register `HSPLLISR` reader"]
pub type R = crate::R<HspllisrSpec>;
#[doc = "Register `HSPLLISR` writer"]
pub type W = crate::W<HspllisrSpec>;
#[doc = "Field `PLLUNLOCK` reader - PLL Unlock Interrupt Set bit."]
pub type PllunlockR = crate::BitReader;
#[doc = "Field `PLLUNLOCK` writer - PLL Unlock Interrupt Set bit."]
pub type PllunlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLL Unlock Interrupt Set bit."]
    #[inline(always)]
    pub fn pllunlock(&self) -> PllunlockR {
        PllunlockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Unlock Interrupt Set bit."]
    #[inline(always)]
    pub fn pllunlock(&mut self) -> PllunlockW<'_, HspllisrSpec> {
        PllunlockW::new(self, 0)
    }
}
#[doc = "Interrupt Flag Set Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hspllisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspllisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HspllisrSpec;
impl crate::RegisterSpec for HspllisrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hspllisr::R`](R) reader structure"]
impl crate::Readable for HspllisrSpec {}
#[doc = "`write(|w| ..)` method takes [`hspllisr::W`](W) writer structure"]
impl crate::Writable for HspllisrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSPLLISR to value 0"]
impl crate::Resettable for HspllisrSpec {}
