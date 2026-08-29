#[doc = "Register `SAPH_ADESCHI` reader"]
pub type R = crate::R<SaphAdeschiSpec>;
#[doc = "Register `SAPH_ADESCHI` writer"]
pub type W = crate::W<SaphAdeschiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Module-Descriptor High Word\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_adeschi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_adeschi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAdeschiSpec;
impl crate::RegisterSpec for SaphAdeschiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_adeschi::R`](R) reader structure"]
impl crate::Readable for SaphAdeschiSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_adeschi::W`](W) writer structure"]
impl crate::Writable for SaphAdeschiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_ADESCHI to value 0"]
impl crate::Resettable for SaphAdeschiSpec {}
