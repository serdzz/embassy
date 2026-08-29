#[doc = "Register `LCDM40_LCDBM8` reader"]
pub type R = crate::R<Lcdm40Lcdbm8Spec>;
#[doc = "Register `LCDM40_LCDBM8` writer"]
pub type W = crate::W<Lcdm40Lcdbm8Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 40 / LCD blinking memory 8\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm40_lcdbm8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm40_lcdbm8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm40Lcdbm8Spec;
impl crate::RegisterSpec for Lcdm40Lcdbm8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm40_lcdbm8::R`](R) reader structure"]
impl crate::Readable for Lcdm40Lcdbm8Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm40_lcdbm8::W`](W) writer structure"]
impl crate::Writable for Lcdm40Lcdbm8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM40_LCDBM8 to value 0"]
impl crate::Resettable for Lcdm40Lcdbm8Spec {}
