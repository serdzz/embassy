#[doc = "Register `SAPH_ATACTL` reader"]
pub type R = crate::R<SaphAtactlSpec>;
#[doc = "Register `SAPH_ATACTL` writer"]
pub type W = crate::W<SaphAtactlSpec>;
#[doc = "Field `UNLOCK` reader - When UNLOCK = 1, the trim registers are allowed to be updated (CH0PUT, CH0PDT, CH0TT, CH1PUT, CH1PDT, and CH1TT)."]
pub type UnlockR = crate::BitReader;
#[doc = "Field `UNLOCK` writer - When UNLOCK = 1, the trim registers are allowed to be updated (CH0PUT, CH0PDT, CH0TT, CH1PUT, CH1PDT, and CH1TT)."]
pub type UnlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When UNLOCK = 1, the trim registers are allowed to be updated (CH0PUT, CH0PDT, CH0TT, CH1PUT, CH1PDT, and CH1TT)."]
    #[inline(always)]
    pub fn unlock(&self) -> UnlockR {
        UnlockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When UNLOCK = 1, the trim registers are allowed to be updated (CH0PUT, CH0PDT, CH0TT, CH1PUT, CH1PDT, and CH1TT)."]
    #[inline(always)]
    pub fn unlock(&mut self) -> UnlockW<'_, SaphAtactlSpec> {
        UnlockW::new(self, 0)
    }
}
#[doc = "Trim Access Control\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_atactl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_atactl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAtactlSpec;
impl crate::RegisterSpec for SaphAtactlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_atactl::R`](R) reader structure"]
impl crate::Readable for SaphAtactlSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_atactl::W`](W) writer structure"]
impl crate::Writable for SaphAtactlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_ATACTL to value 0"]
impl crate::Resettable for SaphAtactlSpec {}
