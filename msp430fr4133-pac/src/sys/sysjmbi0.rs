#[doc = "Register `SYSJMBI0` reader"]
pub type R = crate::R<Sysjmbi0Spec>;
#[doc = "Register `SYSJMBI0` writer"]
pub type W = crate::W<Sysjmbi0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "JTAG mailbox input 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sysjmbi0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysjmbi0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sysjmbi0Spec;
impl crate::RegisterSpec for Sysjmbi0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysjmbi0::R`](R) reader structure"]
impl crate::Readable for Sysjmbi0Spec {}
#[doc = "`write(|w| ..)` method takes [`sysjmbi0::W`](W) writer structure"]
impl crate::Writable for Sysjmbi0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSJMBI0 to value 0"]
impl crate::Resettable for Sysjmbi0Spec {}
