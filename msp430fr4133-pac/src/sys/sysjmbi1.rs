#[doc = "Register `SYSJMBI1` reader"]
pub type R = crate::R<Sysjmbi1Spec>;
#[doc = "Register `SYSJMBI1` writer"]
pub type W = crate::W<Sysjmbi1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "JTAG mailbox input 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sysjmbi1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysjmbi1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sysjmbi1Spec;
impl crate::RegisterSpec for Sysjmbi1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysjmbi1::R`](R) reader structure"]
impl crate::Readable for Sysjmbi1Spec {}
#[doc = "`write(|w| ..)` method takes [`sysjmbi1::W`](W) writer structure"]
impl crate::Writable for Sysjmbi1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSJMBI1 to value 0"]
impl crate::Resettable for Sysjmbi1Spec {}
