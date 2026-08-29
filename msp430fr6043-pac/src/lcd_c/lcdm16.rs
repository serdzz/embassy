#[doc = "Register `LCDM16` reader"]
pub type R = crate::R<Lcdm16Spec>;
#[doc = "Register `LCDM16` writer"]
pub type W = crate::W<Lcdm16Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 16\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm16Spec;
impl crate::RegisterSpec for Lcdm16Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm16::R`](R) reader structure"]
impl crate::Readable for Lcdm16Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm16::W`](W) writer structure"]
impl crate::Writable for Lcdm16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM16 to value 0"]
impl crate::Resettable for Lcdm16Spec {}
