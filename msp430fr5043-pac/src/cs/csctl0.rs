#[doc = "Register `CSCTL0` reader"]
pub type R = crate::R<Csctl0Spec>;
#[doc = "Register `CSCTL0` writer"]
pub type W = crate::W<Csctl0Spec>;
#[doc = "Field `CSKEY` reader - CSKEY password"]
pub type CskeyR = crate::FieldReader;
#[doc = "Field `CSKEY` writer - CSKEY password"]
pub type CskeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - CSKEY password"]
    #[inline(always)]
    pub fn cskey(&self) -> CskeyR {
        CskeyR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - CSKEY password"]
    #[inline(always)]
    pub fn cskey(&mut self) -> CskeyW<'_, Csctl0Spec> {
        CskeyW::new(self, 8)
    }
}
#[doc = "Clock System Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl0Spec;
impl crate::RegisterSpec for Csctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl0::R`](R) reader structure"]
impl crate::Readable for Csctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl0::W`](W) writer structure"]
impl crate::Writable for Csctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL0 to value 0"]
impl crate::Resettable for Csctl0Spec {}
