#[doc = "Register `SAPH_APGHPER` reader"]
pub type R = crate::R<SaphApghperSpec>;
#[doc = "Register `SAPH_APGHPER` writer"]
pub type W = crate::W<SaphApghperSpec>;
#[doc = "Field `HPER` reader - High phase period of PPG excitation pulses. This value defines the length of the high phase of the pulses. The minimum count is two regardless of the value set in this register."]
pub type HperR = crate::FieldReader;
#[doc = "Field `HPER` writer - High phase period of PPG excitation pulses. This value defines the length of the high phase of the pulses. The minimum count is two regardless of the value set in this register."]
pub type HperW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - High phase period of PPG excitation pulses. This value defines the length of the high phase of the pulses. The minimum count is two regardless of the value set in this register."]
    #[inline(always)]
    pub fn hper(&self) -> HperR {
        HperR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High phase period of PPG excitation pulses. This value defines the length of the high phase of the pulses. The minimum count is two regardless of the value set in this register."]
    #[inline(always)]
    pub fn hper(&mut self) -> HperW<'_, SaphApghperSpec> {
        HperW::new(self, 0)
    }
}
#[doc = "Pulse Generator High Period\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_apghper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_apghper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphApghperSpec;
impl crate::RegisterSpec for SaphApghperSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_apghper::R`](R) reader structure"]
impl crate::Readable for SaphApghperSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_apghper::W`](W) writer structure"]
impl crate::Writable for SaphApghperSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_APGHPER to value 0"]
impl crate::Resettable for SaphApghperSpec {}
