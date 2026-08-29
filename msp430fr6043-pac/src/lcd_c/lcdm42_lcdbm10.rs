#[doc = "Register `LCDM42_LCDBM10` reader"]
pub type R = crate::R<Lcdm42Lcdbm10Spec>;
#[doc = "Register `LCDM42_LCDBM10` writer"]
pub type W = crate::W<Lcdm42Lcdbm10Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 42 / LCD blinking memory 10\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm42_lcdbm10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm42_lcdbm10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm42Lcdbm10Spec;
impl crate::RegisterSpec for Lcdm42Lcdbm10Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm42_lcdbm10::R`](R) reader structure"]
impl crate::Readable for Lcdm42Lcdbm10Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm42_lcdbm10::W`](W) writer structure"]
impl crate::Writable for Lcdm42Lcdbm10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM42_LCDBM10 to value 0"]
impl crate::Resettable for Lcdm42Lcdbm10Spec {}
