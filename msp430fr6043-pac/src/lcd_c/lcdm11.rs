#[doc = "Register `LCDM11` reader"]
pub type R = crate::R<Lcdm11Spec>;
#[doc = "Register `LCDM11` writer"]
pub type W = crate::W<Lcdm11Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 11\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm11Spec;
impl crate::RegisterSpec for Lcdm11Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm11::R`](R) reader structure"]
impl crate::Readable for Lcdm11Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm11::W`](W) writer structure"]
impl crate::Writable for Lcdm11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM11 to value 0"]
impl crate::Resettable for Lcdm11Spec {}
