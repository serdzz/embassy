#[doc = "Register `SAPH_AAPHIZ` reader"]
pub type R = crate::R<SaphAaphizSpec>;
#[doc = "Register `SAPH_AAPHIZ` writer"]
pub type W = crate::W<SaphAaphizSpec>;
#[doc = "Field `PCPHIZ` reader - Bit 0 defines the PPG output status at pause for the first measurement. Bit 1 defines the PPG output status at pause for the second measurement. Bit 2 defines the PPG output status at pause for the third measurement. Bit 3 defines the PPG output status at pause for the fourth measurement. 0 = PPG ouput level is determined by APLEV.PCPLEV bits 1 = Hi-z. regardless of APLEV.PCPLEV bits"]
pub type PcphizR = crate::FieldReader;
#[doc = "Field `PCPHIZ` writer - Bit 0 defines the PPG output status at pause for the first measurement. Bit 1 defines the PPG output status at pause for the second measurement. Bit 2 defines the PPG output status at pause for the third measurement. Bit 3 defines the PPG output status at pause for the fourth measurement. 0 = PPG ouput level is determined by APLEV.PCPLEV bits 1 = Hi-z. regardless of APLEV.PCPLEV bits"]
pub type PcphizW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Bit 0 defines the PPG output status at pause for the first measurement. Bit 1 defines the PPG output status at pause for the second measurement. Bit 2 defines the PPG output status at pause for the third measurement. Bit 3 defines the PPG output status at pause for the fourth measurement. 0 = PPG ouput level is determined by APLEV.PCPLEV bits 1 = Hi-z. regardless of APLEV.PCPLEV bits"]
    #[inline(always)]
    pub fn pcphiz(&self) -> PcphizR {
        PcphizR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bit 0 defines the PPG output status at pause for the first measurement. Bit 1 defines the PPG output status at pause for the second measurement. Bit 2 defines the PPG output status at pause for the third measurement. Bit 3 defines the PPG output status at pause for the fourth measurement. 0 = PPG ouput level is determined by APLEV.PCPLEV bits 1 = Hi-z. regardless of APLEV.PCPLEV bits"]
    #[inline(always)]
    pub fn pcphiz(&mut self) -> PcphizW<'_, SaphAaphizSpec> {
        PcphizW::new(self, 0)
    }
}
#[doc = "ASQ ping pause impedance\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aaphiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aaphiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAaphizSpec;
impl crate::RegisterSpec for SaphAaphizSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aaphiz::R`](R) reader structure"]
impl crate::Readable for SaphAaphizSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_aaphiz::W`](W) writer structure"]
impl crate::Writable for SaphAaphizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AAPHIZ to value 0"]
impl crate::Resettable for SaphAaphizSpec {}
