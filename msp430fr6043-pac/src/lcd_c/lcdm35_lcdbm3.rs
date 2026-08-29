#[doc = "Register `LCDM35_LCDBM3` reader"]
pub type R = crate::R<Lcdm35Lcdbm3Spec>;
#[doc = "Register `LCDM35_LCDBM3` writer"]
pub type W = crate::W<Lcdm35Lcdbm3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 35 / LCD blinking memory 3\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm35_lcdbm3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm35_lcdbm3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm35Lcdbm3Spec;
impl crate::RegisterSpec for Lcdm35Lcdbm3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm35_lcdbm3::R`](R) reader structure"]
impl crate::Readable for Lcdm35Lcdbm3Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm35_lcdbm3::W`](W) writer structure"]
impl crate::Writable for Lcdm35Lcdbm3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM35_LCDBM3 to value 0"]
impl crate::Resettable for Lcdm35Lcdbm3Spec {}
