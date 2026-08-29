#[doc = "Register `LCDM22` reader"]
pub type R = crate::R<Lcdm22Spec>;
#[doc = "Register `LCDM22` writer"]
pub type W = crate::W<Lcdm22Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 22\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm22Spec;
impl crate::RegisterSpec for Lcdm22Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm22::R`](R) reader structure"]
impl crate::Readable for Lcdm22Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm22::W`](W) writer structure"]
impl crate::Writable for Lcdm22Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM22 to value 0"]
impl crate::Resettable for Lcdm22Spec {}
