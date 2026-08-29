#[doc = "Register `LCDM27` reader"]
pub type R = crate::R<Lcdm27Spec>;
#[doc = "Register `LCDM27` writer"]
pub type W = crate::W<Lcdm27Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 27\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm27Spec;
impl crate::RegisterSpec for Lcdm27Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm27::R`](R) reader structure"]
impl crate::Readable for Lcdm27Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm27::W`](W) writer structure"]
impl crate::Writable for Lcdm27Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM27 to value 0"]
impl crate::Resettable for Lcdm27Spec {}
