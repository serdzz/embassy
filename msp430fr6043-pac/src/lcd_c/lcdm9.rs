#[doc = "Register `LCDM9` reader"]
pub type R = crate::R<Lcdm9Spec>;
#[doc = "Register `LCDM9` writer"]
pub type W = crate::W<Lcdm9Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 9\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm9Spec;
impl crate::RegisterSpec for Lcdm9Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm9::R`](R) reader structure"]
impl crate::Readable for Lcdm9Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm9::W`](W) writer structure"]
impl crate::Writable for Lcdm9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM9 to value 0"]
impl crate::Resettable for Lcdm9Spec {}
