#[doc = "Register `LCDM51_LCDBM19` reader"]
pub type R = crate::R<Lcdm51Lcdbm19Spec>;
#[doc = "Register `LCDM51_LCDBM19` writer"]
pub type W = crate::W<Lcdm51Lcdbm19Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 51 / LCD blinking memory 19\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm51_lcdbm19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm51_lcdbm19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm51Lcdbm19Spec;
impl crate::RegisterSpec for Lcdm51Lcdbm19Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm51_lcdbm19::R`](R) reader structure"]
impl crate::Readable for Lcdm51Lcdbm19Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm51_lcdbm19::W`](W) writer structure"]
impl crate::Writable for Lcdm51Lcdbm19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM51_LCDBM19 to value 0"]
impl crate::Resettable for Lcdm51Lcdbm19Spec {}
