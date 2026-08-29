#[doc = "Register `LCDM12` reader"]
pub type R = crate::R<Lcdm12Spec>;
#[doc = "Register `LCDM12` writer"]
pub type W = crate::W<Lcdm12Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 12\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm12Spec;
impl crate::RegisterSpec for Lcdm12Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm12::R`](R) reader structure"]
impl crate::Readable for Lcdm12Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm12::W`](W) writer structure"]
impl crate::Writable for Lcdm12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM12 to value 0"]
impl crate::Resettable for Lcdm12Spec {}
