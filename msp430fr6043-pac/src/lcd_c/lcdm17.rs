#[doc = "Register `LCDM17` reader"]
pub type R = crate::R<Lcdm17Spec>;
#[doc = "Register `LCDM17` writer"]
pub type W = crate::W<Lcdm17Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 17\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm17Spec;
impl crate::RegisterSpec for Lcdm17Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm17::R`](R) reader structure"]
impl crate::Readable for Lcdm17Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm17::W`](W) writer structure"]
impl crate::Writable for Lcdm17Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM17 to value 0"]
impl crate::Resettable for Lcdm17Spec {}
