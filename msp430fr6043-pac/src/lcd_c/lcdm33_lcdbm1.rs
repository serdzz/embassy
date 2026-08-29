#[doc = "Register `LCDM33_LCDBM1` reader"]
pub type R = crate::R<Lcdm33Lcdbm1Spec>;
#[doc = "Register `LCDM33_LCDBM1` writer"]
pub type W = crate::W<Lcdm33Lcdbm1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 33 / LCD blinking memory 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm33_lcdbm1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm33_lcdbm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm33Lcdbm1Spec;
impl crate::RegisterSpec for Lcdm33Lcdbm1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm33_lcdbm1::R`](R) reader structure"]
impl crate::Readable for Lcdm33Lcdbm1Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm33_lcdbm1::W`](W) writer structure"]
impl crate::Writable for Lcdm33Lcdbm1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM33_LCDBM1 to value 0"]
impl crate::Resettable for Lcdm33Lcdbm1Spec {}
