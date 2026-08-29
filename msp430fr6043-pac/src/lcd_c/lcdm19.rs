#[doc = "Register `LCDM19` reader"]
pub type R = crate::R<Lcdm19Spec>;
#[doc = "Register `LCDM19` writer"]
pub type W = crate::W<Lcdm19Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 19\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm19Spec;
impl crate::RegisterSpec for Lcdm19Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm19::R`](R) reader structure"]
impl crate::Readable for Lcdm19Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm19::W`](W) writer structure"]
impl crate::Writable for Lcdm19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM19 to value 0"]
impl crate::Resettable for Lcdm19Spec {}
