#[doc = "Register `LCDM30` reader"]
pub type R = crate::R<Lcdm30Spec>;
#[doc = "Register `LCDM30` writer"]
pub type W = crate::W<Lcdm30Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 30\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm30Spec;
impl crate::RegisterSpec for Lcdm30Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm30::R`](R) reader structure"]
impl crate::Readable for Lcdm30Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm30::W`](W) writer structure"]
impl crate::Writable for Lcdm30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM30 to value 0"]
impl crate::Resettable for Lcdm30Spec {}
