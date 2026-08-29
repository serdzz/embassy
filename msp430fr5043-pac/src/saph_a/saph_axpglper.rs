#[doc = "Register `SAPH_AXPGLPER` reader"]
pub type R = crate::R<SaphAxpglperSpec>;
#[doc = "Register `SAPH_AXPGLPER` writer"]
pub type W = crate::W<SaphAxpglperSpec>;
#[doc = "Field `XLPER` reader - XLPER low phase period of the extra pulses. This value defines the length of the low phase of the extra pulses in units of high speed clocks. The minimum count is two regardless of the value set in this register."]
pub type XlperR = crate::FieldReader;
#[doc = "Field `XLPER` writer - XLPER low phase period of the extra pulses. This value defines the length of the low phase of the extra pulses in units of high speed clocks. The minimum count is two regardless of the value set in this register."]
pub type XlperW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - XLPER low phase period of the extra pulses. This value defines the length of the low phase of the extra pulses in units of high speed clocks. The minimum count is two regardless of the value set in this register."]
    #[inline(always)]
    pub fn xlper(&self) -> XlperR {
        XlperR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - XLPER low phase period of the extra pulses. This value defines the length of the low phase of the extra pulses in units of high speed clocks. The minimum count is two regardless of the value set in this register."]
    #[inline(always)]
    pub fn xlper(&mut self) -> XlperW<'_, SaphAxpglperSpec> {
        XlperW::new(self, 0)
    }
}
#[doc = "Extra Pulse Low Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_axpglper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_axpglper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAxpglperSpec;
impl crate::RegisterSpec for SaphAxpglperSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_axpglper::R`](R) reader structure"]
impl crate::Readable for SaphAxpglperSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_axpglper::W`](W) writer structure"]
impl crate::Writable for SaphAxpglperSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AXPGLPER to value 0"]
impl crate::Resettable for SaphAxpglperSpec {}
