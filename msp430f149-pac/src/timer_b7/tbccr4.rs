#[doc = "Register `TBCCR4` reader"]
pub type R = crate::R<Tbccr4Spec>;
#[doc = "Register `TBCCR4` writer"]
pub type W = crate::W<Tbccr4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer B Capture/Compare 4\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbccr4Spec;
impl crate::RegisterSpec for Tbccr4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbccr4::R`](R) reader structure"]
impl crate::Readable for Tbccr4Spec {}
#[doc = "`write(|w| ..)` method takes [`tbccr4::W`](W) writer structure"]
impl crate::Writable for Tbccr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBCCR4 to value 0"]
impl crate::Resettable for Tbccr4Spec {}
