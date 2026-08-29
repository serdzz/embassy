#[doc = "Register `LCDM6` reader"]
pub type R = crate::R<Lcdm6Spec>;
#[doc = "Register `LCDM6` writer"]
pub type W = crate::W<Lcdm6Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 6\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm6Spec;
impl crate::RegisterSpec for Lcdm6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm6::R`](R) reader structure"]
impl crate::Readable for Lcdm6Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm6::W`](W) writer structure"]
impl crate::Writable for Lcdm6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM6 to value 0"]
impl crate::Resettable for Lcdm6Spec {}
