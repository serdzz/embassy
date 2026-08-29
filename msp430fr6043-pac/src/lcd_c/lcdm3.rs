#[doc = "Register `LCDM3` reader"]
pub type R = crate::R<Lcdm3Spec>;
#[doc = "Register `LCDM3` writer"]
pub type W = crate::W<Lcdm3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 3\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm3Spec;
impl crate::RegisterSpec for Lcdm3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm3::R`](R) reader structure"]
impl crate::Readable for Lcdm3Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm3::W`](W) writer structure"]
impl crate::Writable for Lcdm3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM3 to value 0"]
impl crate::Resettable for Lcdm3Spec {}
