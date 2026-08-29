#[doc = "Register `LCDM8` reader"]
pub type R = crate::R<Lcdm8Spec>;
#[doc = "Register `LCDM8` writer"]
pub type W = crate::W<Lcdm8Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 8\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm8Spec;
impl crate::RegisterSpec for Lcdm8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm8::R`](R) reader structure"]
impl crate::Readable for Lcdm8Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm8::W`](W) writer structure"]
impl crate::Writable for Lcdm8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM8 to value 0"]
impl crate::Resettable for Lcdm8Spec {}
