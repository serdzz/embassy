#[doc = "Register `UCA0CTL1` reader"]
pub type R = crate::R<Uca0ctl1Spec>;
#[doc = "Register `UCA0CTL1` writer"]
pub type W = crate::W<Uca0ctl1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI A0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0ctl1Spec;
impl crate::RegisterSpec for Uca0ctl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0ctl1::R`](R) reader structure"]
impl crate::Readable for Uca0ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`uca0ctl1::W`](W) writer structure"]
impl crate::Writable for Uca0ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0CTL1 to value 0"]
impl crate::Resettable for Uca0ctl1Spec {}
