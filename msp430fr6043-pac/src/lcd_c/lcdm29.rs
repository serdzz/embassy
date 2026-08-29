#[doc = "Register `LCDM29` reader"]
pub type R = crate::R<Lcdm29Spec>;
#[doc = "Register `LCDM29` writer"]
pub type W = crate::W<Lcdm29Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 29\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm29Spec;
impl crate::RegisterSpec for Lcdm29Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm29::R`](R) reader structure"]
impl crate::Readable for Lcdm29Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm29::W`](W) writer structure"]
impl crate::Writable for Lcdm29Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM29 to value 0"]
impl crate::Resettable for Lcdm29Spec {}
