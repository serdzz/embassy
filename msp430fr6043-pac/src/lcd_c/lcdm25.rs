#[doc = "Register `LCDM25` reader"]
pub type R = crate::R<Lcdm25Spec>;
#[doc = "Register `LCDM25` writer"]
pub type W = crate::W<Lcdm25Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 25\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm25Spec;
impl crate::RegisterSpec for Lcdm25Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm25::R`](R) reader structure"]
impl crate::Readable for Lcdm25Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm25::W`](W) writer structure"]
impl crate::Writable for Lcdm25Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM25 to value 0"]
impl crate::Resettable for Lcdm25Spec {}
