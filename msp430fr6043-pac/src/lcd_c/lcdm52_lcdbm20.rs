#[doc = "Register `LCDM52_LCDBM20` reader"]
pub type R = crate::R<Lcdm52Lcdbm20Spec>;
#[doc = "Register `LCDM52_LCDBM20` writer"]
pub type W = crate::W<Lcdm52Lcdbm20Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 52 / LCD blinking memory 20\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm52_lcdbm20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm52_lcdbm20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm52Lcdbm20Spec;
impl crate::RegisterSpec for Lcdm52Lcdbm20Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm52_lcdbm20::R`](R) reader structure"]
impl crate::Readable for Lcdm52Lcdbm20Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm52_lcdbm20::W`](W) writer structure"]
impl crate::Writable for Lcdm52Lcdbm20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM52_LCDBM20 to value 0"]
impl crate::Resettable for Lcdm52Lcdbm20Spec {}
