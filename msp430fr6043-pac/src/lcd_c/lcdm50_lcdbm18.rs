#[doc = "Register `LCDM50_LCDBM18` reader"]
pub type R = crate::R<Lcdm50Lcdbm18Spec>;
#[doc = "Register `LCDM50_LCDBM18` writer"]
pub type W = crate::W<Lcdm50Lcdbm18Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 50 / LCD blinking memory 18\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm50_lcdbm18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm50_lcdbm18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm50Lcdbm18Spec;
impl crate::RegisterSpec for Lcdm50Lcdbm18Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm50_lcdbm18::R`](R) reader structure"]
impl crate::Readable for Lcdm50Lcdbm18Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm50_lcdbm18::W`](W) writer structure"]
impl crate::Writable for Lcdm50Lcdbm18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM50_LCDBM18 to value 0"]
impl crate::Resettable for Lcdm50Lcdbm18Spec {}
