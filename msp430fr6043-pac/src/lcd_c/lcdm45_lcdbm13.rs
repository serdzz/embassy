#[doc = "Register `LCDM45_LCDBM13` reader"]
pub type R = crate::R<Lcdm45Lcdbm13Spec>;
#[doc = "Register `LCDM45_LCDBM13` writer"]
pub type W = crate::W<Lcdm45Lcdbm13Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 45 / LCD blinking memory 13\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm45_lcdbm13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm45_lcdbm13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm45Lcdbm13Spec;
impl crate::RegisterSpec for Lcdm45Lcdbm13Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm45_lcdbm13::R`](R) reader structure"]
impl crate::Readable for Lcdm45Lcdbm13Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm45_lcdbm13::W`](W) writer structure"]
impl crate::Writable for Lcdm45Lcdbm13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM45_LCDBM13 to value 0"]
impl crate::Resettable for Lcdm45Lcdbm13Spec {}
