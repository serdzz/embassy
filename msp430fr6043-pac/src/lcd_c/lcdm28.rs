#[doc = "Register `LCDM28` reader"]
pub type R = crate::R<Lcdm28Spec>;
#[doc = "Register `LCDM28` writer"]
pub type W = crate::W<Lcdm28Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 28\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm28Spec;
impl crate::RegisterSpec for Lcdm28Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm28::R`](R) reader structure"]
impl crate::Readable for Lcdm28Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm28::W`](W) writer structure"]
impl crate::Writable for Lcdm28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM28 to value 0"]
impl crate::Resettable for Lcdm28Spec {}
