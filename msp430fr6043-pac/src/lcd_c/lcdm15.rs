#[doc = "Register `LCDM15` reader"]
pub type R = crate::R<Lcdm15Spec>;
#[doc = "Register `LCDM15` writer"]
pub type W = crate::W<Lcdm15Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 15\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm15Spec;
impl crate::RegisterSpec for Lcdm15Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm15::R`](R) reader structure"]
impl crate::Readable for Lcdm15Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm15::W`](W) writer structure"]
impl crate::Writable for Lcdm15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM15 to value 0"]
impl crate::Resettable for Lcdm15Spec {}
