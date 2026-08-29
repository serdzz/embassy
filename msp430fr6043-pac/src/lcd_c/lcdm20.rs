#[doc = "Register `LCDM20` reader"]
pub type R = crate::R<Lcdm20Spec>;
#[doc = "Register `LCDM20` writer"]
pub type W = crate::W<Lcdm20Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 20\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm20Spec;
impl crate::RegisterSpec for Lcdm20Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm20::R`](R) reader structure"]
impl crate::Readable for Lcdm20Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm20::W`](W) writer structure"]
impl crate::Writable for Lcdm20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM20 to value 0"]
impl crate::Resettable for Lcdm20Spec {}
