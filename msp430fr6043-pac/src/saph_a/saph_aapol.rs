#[doc = "Register `SAPH_AAPOL` reader"]
pub type R = crate::R<SaphAapolSpec>;
#[doc = "Register `SAPH_AAPOL` writer"]
pub type W = crate::W<SaphAapolSpec>;
#[doc = "Field `PCPOL` reader - Bit 0 defines the PPG pulse polarity for the first measurement. Bit 1 defines the PPG pulse polarity for the second measurement. Bit 2 defines the PPG pulse polarity for the third measurement. Bit 3 defines the PPG pulse polarity for the fourth measurement. 0 = PPG output pulses starts with logical high polarity. 1 = PPG output pulses starts with logical low polarity."]
pub type PcpolR = crate::FieldReader;
#[doc = "Field `PCPOL` writer - Bit 0 defines the PPG pulse polarity for the first measurement. Bit 1 defines the PPG pulse polarity for the second measurement. Bit 2 defines the PPG pulse polarity for the third measurement. Bit 3 defines the PPG pulse polarity for the fourth measurement. 0 = PPG output pulses starts with logical high polarity. 1 = PPG output pulses starts with logical low polarity."]
pub type PcpolW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Bit 0 defines the PPG pulse polarity for the first measurement. Bit 1 defines the PPG pulse polarity for the second measurement. Bit 2 defines the PPG pulse polarity for the third measurement. Bit 3 defines the PPG pulse polarity for the fourth measurement. 0 = PPG output pulses starts with logical high polarity. 1 = PPG output pulses starts with logical low polarity."]
    #[inline(always)]
    pub fn pcpol(&self) -> PcpolR {
        PcpolR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bit 0 defines the PPG pulse polarity for the first measurement. Bit 1 defines the PPG pulse polarity for the second measurement. Bit 2 defines the PPG pulse polarity for the third measurement. Bit 3 defines the PPG pulse polarity for the fourth measurement. 0 = PPG output pulses starts with logical high polarity. 1 = PPG output pulses starts with logical low polarity."]
    #[inline(always)]
    pub fn pcpol(&mut self) -> PcpolW<'_, SaphAapolSpec> {
        PcpolW::new(self, 0)
    }
}
#[doc = "ASQ ping output polarity\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aapol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aapol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAapolSpec;
impl crate::RegisterSpec for SaphAapolSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aapol::R`](R) reader structure"]
impl crate::Readable for SaphAapolSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_aapol::W`](W) writer structure"]
impl crate::Writable for SaphAapolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AAPOL to value 0"]
impl crate::Resettable for SaphAapolSpec {}
