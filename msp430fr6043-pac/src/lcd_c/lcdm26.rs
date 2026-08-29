#[doc = "Register `LCDM26` reader"]
pub type R = crate::R<Lcdm26Spec>;
#[doc = "Register `LCDM26` writer"]
pub type W = crate::W<Lcdm26Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 26\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm26Spec;
impl crate::RegisterSpec for Lcdm26Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm26::R`](R) reader structure"]
impl crate::Readable for Lcdm26Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm26::W`](W) writer structure"]
impl crate::Writable for Lcdm26Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM26 to value 0"]
impl crate::Resettable for Lcdm26Spec {}
