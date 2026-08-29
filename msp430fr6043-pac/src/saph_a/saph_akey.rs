#[doc = "Register `SAPH_AKEY` reader"]
pub type R = crate::R<SaphAkeySpec>;
#[doc = "Register `SAPH_AKEY` writer"]
pub type W = crate::W<SaphAkeySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Key\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_akey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_akey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAkeySpec;
impl crate::RegisterSpec for SaphAkeySpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_akey::R`](R) reader structure"]
impl crate::Readable for SaphAkeySpec {}
#[doc = "`write(|w| ..)` method takes [`saph_akey::W`](W) writer structure"]
impl crate::Writable for SaphAkeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AKEY to value 0"]
impl crate::Resettable for SaphAkeySpec {}
