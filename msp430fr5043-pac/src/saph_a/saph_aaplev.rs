#[doc = "Register `SAPH_AAPLEV` reader"]
pub type R = crate::R<SaphAaplevSpec>;
#[doc = "Register `SAPH_AAPLEV` writer"]
pub type W = crate::W<SaphAaplevSpec>;
#[doc = "Field `PCPLEV` reader - Bit 0 defines the PPG output level at pause for the first measurement when PCPHIZ bit 0 = 0. Bit 1 defines the PPG output level at pause for the second measurement when PCPHIZ bit 1 = 0. Bit 2 defines the PPG output level at pause for the third measurement when PCPHIZ bit 2 = 0. Bit 3 defines the PPG output level at pause for the fourth measurement when PCPHIZ bit 3 = 0. 0 = Logical Low. 1 = Logical High."]
pub type PcplevR = crate::FieldReader;
#[doc = "Field `PCPLEV` writer - Bit 0 defines the PPG output level at pause for the first measurement when PCPHIZ bit 0 = 0. Bit 1 defines the PPG output level at pause for the second measurement when PCPHIZ bit 1 = 0. Bit 2 defines the PPG output level at pause for the third measurement when PCPHIZ bit 2 = 0. Bit 3 defines the PPG output level at pause for the fourth measurement when PCPHIZ bit 3 = 0. 0 = Logical Low. 1 = Logical High."]
pub type PcplevW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Bit 0 defines the PPG output level at pause for the first measurement when PCPHIZ bit 0 = 0. Bit 1 defines the PPG output level at pause for the second measurement when PCPHIZ bit 1 = 0. Bit 2 defines the PPG output level at pause for the third measurement when PCPHIZ bit 2 = 0. Bit 3 defines the PPG output level at pause for the fourth measurement when PCPHIZ bit 3 = 0. 0 = Logical Low. 1 = Logical High."]
    #[inline(always)]
    pub fn pcplev(&self) -> PcplevR {
        PcplevR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bit 0 defines the PPG output level at pause for the first measurement when PCPHIZ bit 0 = 0. Bit 1 defines the PPG output level at pause for the second measurement when PCPHIZ bit 1 = 0. Bit 2 defines the PPG output level at pause for the third measurement when PCPHIZ bit 2 = 0. Bit 3 defines the PPG output level at pause for the fourth measurement when PCPHIZ bit 3 = 0. 0 = Logical Low. 1 = Logical High."]
    #[inline(always)]
    pub fn pcplev(&mut self) -> PcplevW<'_, SaphAaplevSpec> {
        PcplevW::new(self, 0)
    }
}
#[doc = "ASQ ping pause level\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aaplev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aaplev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAaplevSpec;
impl crate::RegisterSpec for SaphAaplevSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aaplev::R`](R) reader structure"]
impl crate::Readable for SaphAaplevSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_aaplev::W`](W) writer structure"]
impl crate::Writable for SaphAaplevSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AAPLEV to value 0"]
impl crate::Resettable for SaphAaplevSpec {}
