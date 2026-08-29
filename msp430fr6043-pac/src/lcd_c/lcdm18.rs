#[doc = "Register `LCDM18` reader"]
pub type R = crate::R<Lcdm18Spec>;
#[doc = "Register `LCDM18` writer"]
pub type W = crate::W<Lcdm18Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 18\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm18Spec;
impl crate::RegisterSpec for Lcdm18Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm18::R`](R) reader structure"]
impl crate::Readable for Lcdm18Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm18::W`](W) writer structure"]
impl crate::Writable for Lcdm18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM18 to value 0"]
impl crate::Resettable for Lcdm18Spec {}
