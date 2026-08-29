#[doc = "Register `LCDM31` reader"]
pub type R = crate::R<Lcdm31Spec>;
#[doc = "Register `LCDM31` writer"]
pub type W = crate::W<Lcdm31Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 31\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm31Spec;
impl crate::RegisterSpec for Lcdm31Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm31::R`](R) reader structure"]
impl crate::Readable for Lcdm31Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm31::W`](W) writer structure"]
impl crate::Writable for Lcdm31Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM31 to value 0"]
impl crate::Resettable for Lcdm31Spec {}
