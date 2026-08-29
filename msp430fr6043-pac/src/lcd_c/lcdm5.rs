#[doc = "Register `LCDM5` reader"]
pub type R = crate::R<Lcdm5Spec>;
#[doc = "Register `LCDM5` writer"]
pub type W = crate::W<Lcdm5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 5\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm5Spec;
impl crate::RegisterSpec for Lcdm5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm5::R`](R) reader structure"]
impl crate::Readable for Lcdm5Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm5::W`](W) writer structure"]
impl crate::Writable for Lcdm5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM5 to value 0"]
impl crate::Resettable for Lcdm5Spec {}
