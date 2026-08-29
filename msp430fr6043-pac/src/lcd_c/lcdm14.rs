#[doc = "Register `LCDM14` reader"]
pub type R = crate::R<Lcdm14Spec>;
#[doc = "Register `LCDM14` writer"]
pub type W = crate::W<Lcdm14Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 14\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm14Spec;
impl crate::RegisterSpec for Lcdm14Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm14::R`](R) reader structure"]
impl crate::Readable for Lcdm14Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm14::W`](W) writer structure"]
impl crate::Writable for Lcdm14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM14 to value 0"]
impl crate::Resettable for Lcdm14Spec {}
