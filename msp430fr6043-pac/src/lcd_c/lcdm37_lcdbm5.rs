#[doc = "Register `LCDM37_LCDBM5` reader"]
pub type R = crate::R<Lcdm37Lcdbm5Spec>;
#[doc = "Register `LCDM37_LCDBM5` writer"]
pub type W = crate::W<Lcdm37Lcdbm5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 37 / LCD blinking memory 5\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm37_lcdbm5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm37_lcdbm5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm37Lcdbm5Spec;
impl crate::RegisterSpec for Lcdm37Lcdbm5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm37_lcdbm5::R`](R) reader structure"]
impl crate::Readable for Lcdm37Lcdbm5Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm37_lcdbm5::W`](W) writer structure"]
impl crate::Writable for Lcdm37Lcdbm5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM37_LCDBM5 to value 0"]
impl crate::Resettable for Lcdm37Lcdbm5Spec {}
