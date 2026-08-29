#[doc = "Register `LCDM46_LCDBM14` reader"]
pub type R = crate::R<Lcdm46Lcdbm14Spec>;
#[doc = "Register `LCDM46_LCDBM14` writer"]
pub type W = crate::W<Lcdm46Lcdbm14Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 46 / LCD blinking memory 14\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm46_lcdbm14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm46_lcdbm14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm46Lcdbm14Spec;
impl crate::RegisterSpec for Lcdm46Lcdbm14Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm46_lcdbm14::R`](R) reader structure"]
impl crate::Readable for Lcdm46Lcdbm14Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm46_lcdbm14::W`](W) writer structure"]
impl crate::Writable for Lcdm46Lcdbm14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM46_LCDBM14 to value 0"]
impl crate::Resettable for Lcdm46Lcdbm14Spec {}
