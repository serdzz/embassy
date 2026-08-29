#[doc = "Register `MPUIPSEGB1` reader"]
pub type R = crate::R<Mpuipsegb1Spec>;
#[doc = "Register `MPUIPSEGB1` writer"]
pub type W = crate::W<Mpuipsegb1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Memory Protection Unit IP Encapsulation Segment Border 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpuipsegb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpuipsegb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpuipsegb1Spec;
impl crate::RegisterSpec for Mpuipsegb1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpuipsegb1::R`](R) reader structure"]
impl crate::Readable for Mpuipsegb1Spec {}
#[doc = "`write(|w| ..)` method takes [`mpuipsegb1::W`](W) writer structure"]
impl crate::Writable for Mpuipsegb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPUIPSEGB1 to value 0"]
impl crate::Resettable for Mpuipsegb1Spec {}
