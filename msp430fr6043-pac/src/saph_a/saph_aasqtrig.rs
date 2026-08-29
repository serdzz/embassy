#[doc = "Register `SAPH_AASQTRIG` reader"]
pub type R = crate::R<SaphAasqtrigSpec>;
#[doc = "Register `SAPH_AASQTRIG` writer"]
pub type W = crate::W<SaphAasqtrigSpec>;
#[doc = "Field `ASQTRIG` reader - Writing '1' to this bit trigger the ASQ when ASCTL0.TRIGSEL = 0. Note: This bit is write only. Reading always returns with zero."]
pub type AsqtrigR = crate::BitReader;
#[doc = "Field `ASQTRIG` writer - Writing '1' to this bit trigger the ASQ when ASCTL0.TRIGSEL = 0. Note: This bit is write only. Reading always returns with zero."]
pub type AsqtrigW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing '1' to this bit trigger the ASQ when ASCTL0.TRIGSEL = 0. Note: This bit is write only. Reading always returns with zero."]
    #[inline(always)]
    pub fn asqtrig(&self) -> AsqtrigR {
        AsqtrigR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing '1' to this bit trigger the ASQ when ASCTL0.TRIGSEL = 0. Note: This bit is write only. Reading always returns with zero."]
    #[inline(always)]
    pub fn asqtrig(&mut self) -> AsqtrigW<'_, SaphAasqtrigSpec> {
        AsqtrigW::new(self, 0)
    }
}
#[doc = "ASQ Software Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aasqtrig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aasqtrig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAasqtrigSpec;
impl crate::RegisterSpec for SaphAasqtrigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`saph_aasqtrig::R`](R) reader structure"]
impl crate::Readable for SaphAasqtrigSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_aasqtrig::W`](W) writer structure"]
impl crate::Writable for SaphAasqtrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AASQTRIG to value 0"]
impl crate::Resettable for SaphAasqtrigSpec {}
