#[doc = "Register `LCDM24` reader"]
pub type R = crate::R<Lcdm24Spec>;
#[doc = "Register `LCDM24` writer"]
pub type W = crate::W<Lcdm24Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 24\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm24Spec;
impl crate::RegisterSpec for Lcdm24Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm24::R`](R) reader structure"]
impl crate::Readable for Lcdm24Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm24::W`](W) writer structure"]
impl crate::Writable for Lcdm24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM24 to value 0"]
impl crate::Resettable for Lcdm24Spec {}
