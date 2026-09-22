#[doc = "Register `SYSJMBO1` reader"]
pub type R = crate::R<Sysjmbo1Spec>;
#[doc = "Register `SYSJMBO1` writer"]
pub type W = crate::W<Sysjmbo1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "JTAG mailbox output 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sysjmbo1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysjmbo1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sysjmbo1Spec;
impl crate::RegisterSpec for Sysjmbo1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysjmbo1::R`](R) reader structure"]
impl crate::Readable for Sysjmbo1Spec {}
#[doc = "`write(|w| ..)` method takes [`sysjmbo1::W`](W) writer structure"]
impl crate::Writable for Sysjmbo1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSJMBO1 to value 0"]
impl crate::Resettable for Sysjmbo1Spec {}
