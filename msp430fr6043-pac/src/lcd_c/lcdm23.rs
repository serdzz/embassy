#[doc = "Register `LCDM23` reader"]
pub type R = crate::R<Lcdm23Spec>;
#[doc = "Register `LCDM23` writer"]
pub type W = crate::W<Lcdm23Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 23\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm23Spec;
impl crate::RegisterSpec for Lcdm23Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm23::R`](R) reader structure"]
impl crate::Readable for Lcdm23Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm23::W`](W) writer structure"]
impl crate::Writable for Lcdm23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM23 to value 0"]
impl crate::Resettable for Lcdm23Spec {}
