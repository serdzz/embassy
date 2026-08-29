#[doc = "Register `LCDM7` reader"]
pub type R = crate::R<Lcdm7Spec>;
#[doc = "Register `LCDM7` writer"]
pub type W = crate::W<Lcdm7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 7\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm7Spec;
impl crate::RegisterSpec for Lcdm7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm7::R`](R) reader structure"]
impl crate::Readable for Lcdm7Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm7::W`](W) writer structure"]
impl crate::Writable for Lcdm7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM7 to value 0"]
impl crate::Resettable for Lcdm7Spec {}
