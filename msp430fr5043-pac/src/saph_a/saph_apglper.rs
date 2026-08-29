#[doc = "Register `SAPH_APGLPER` reader"]
pub type R = crate::R<SaphApglperSpec>;
#[doc = "Register `SAPH_APGLPER` writer"]
pub type W = crate::W<SaphApglperSpec>;
#[doc = "Field `LPER` reader - Low phase period of PPG excitation pulses. This value defines the length of the low phase of the pulses. The minimum count is two regardless of the value set in this register."]
pub type LperR = crate::FieldReader;
#[doc = "Field `LPER` writer - Low phase period of PPG excitation pulses. This value defines the length of the low phase of the pulses. The minimum count is two regardless of the value set in this register."]
pub type LperW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Low phase period of PPG excitation pulses. This value defines the length of the low phase of the pulses. The minimum count is two regardless of the value set in this register."]
    #[inline(always)]
    pub fn lper(&self) -> LperR {
        LperR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low phase period of PPG excitation pulses. This value defines the length of the low phase of the pulses. The minimum count is two regardless of the value set in this register."]
    #[inline(always)]
    pub fn lper(&mut self) -> LperW<'_, SaphApglperSpec> {
        LperW::new(self, 0)
    }
}
#[doc = "Pulse Generator Low Period\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_apglper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_apglper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphApglperSpec;
impl crate::RegisterSpec for SaphApglperSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_apglper::R`](R) reader structure"]
impl crate::Readable for SaphApglperSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_apglper::W`](W) writer structure"]
impl crate::Writable for SaphApglperSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_APGLPER to value 0"]
impl crate::Resettable for SaphApglperSpec {}
