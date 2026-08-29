#[doc = "Register `SAPH_APPGTRIG` reader"]
pub type R = crate::R<SaphAppgtrigSpec>;
#[doc = "Register `SAPH_APPGTRIG` writer"]
pub type W = crate::W<SaphAppgtrigSpec>;
#[doc = "Field `PPGTRIG` reader - Writing '1' to this bit triggers the PPG to generate pulses when PGCTL.TRSEL =0. Note: This bit is write only. Reading always returns with zero."]
pub type PpgtrigR = crate::BitReader;
#[doc = "Field `PPGTRIG` writer - Writing '1' to this bit triggers the PPG to generate pulses when PGCTL.TRSEL =0. Note: This bit is write only. Reading always returns with zero."]
pub type PpgtrigW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing '1' to this bit triggers the PPG to generate pulses when PGCTL.TRSEL =0. Note: This bit is write only. Reading always returns with zero."]
    #[inline(always)]
    pub fn ppgtrig(&self) -> PpgtrigR {
        PpgtrigR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing '1' to this bit triggers the PPG to generate pulses when PGCTL.TRSEL =0. Note: This bit is write only. Reading always returns with zero."]
    #[inline(always)]
    pub fn ppgtrig(&mut self) -> PpgtrigW<'_, SaphAppgtrigSpec> {
        PpgtrigW::new(self, 0)
    }
}
#[doc = "PPG Software Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_appgtrig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_appgtrig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAppgtrigSpec;
impl crate::RegisterSpec for SaphAppgtrigSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_appgtrig::R`](R) reader structure"]
impl crate::Readable for SaphAppgtrigSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_appgtrig::W`](W) writer structure"]
impl crate::Writable for SaphAppgtrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_APPGTRIG to value 0"]
impl crate::Resettable for SaphAppgtrigSpec {}
