#[doc = "Register `SAPH_AXPGHPER` reader"]
pub type R = crate::R<SaphAxpghperSpec>;
#[doc = "Register `SAPH_AXPGHPER` writer"]
pub type W = crate::W<SaphAxpghperSpec>;
#[doc = "Field `XHPER` reader - XHPER high phase period of the extra pulses. This value defines the length of the high phase of the pulses in units of high speed clocks. The minimum count is two regardless of the value set in this register."]
pub type XhperR = crate::FieldReader;
#[doc = "Field `XHPER` writer - XHPER high phase period of the extra pulses. This value defines the length of the high phase of the pulses in units of high speed clocks. The minimum count is two regardless of the value set in this register."]
pub type XhperW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - XHPER high phase period of the extra pulses. This value defines the length of the high phase of the pulses in units of high speed clocks. The minimum count is two regardless of the value set in this register."]
    #[inline(always)]
    pub fn xhper(&self) -> XhperR {
        XhperR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - XHPER high phase period of the extra pulses. This value defines the length of the high phase of the pulses in units of high speed clocks. The minimum count is two regardless of the value set in this register."]
    #[inline(always)]
    pub fn xhper(&mut self) -> XhperW<'_, SaphAxpghperSpec> {
        XhperW::new(self, 0)
    }
}
#[doc = "Extra Pulse High Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_axpghper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_axpghper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAxpghperSpec;
impl crate::RegisterSpec for SaphAxpghperSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_axpghper::R`](R) reader structure"]
impl crate::Readable for SaphAxpghperSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_axpghper::W`](W) writer structure"]
impl crate::Writable for SaphAxpghperSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AXPGHPER to value 0"]
impl crate::Resettable for SaphAxpghperSpec {}
