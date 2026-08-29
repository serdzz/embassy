#[doc = "Register `MPUIPSEGB2` reader"]
pub type R = crate::R<Mpuipsegb2Spec>;
#[doc = "Register `MPUIPSEGB2` writer"]
pub type W = crate::W<Mpuipsegb2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Memory Protection Unit IP Encapsulation Segment Border 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpuipsegb2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpuipsegb2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpuipsegb2Spec;
impl crate::RegisterSpec for Mpuipsegb2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpuipsegb2::R`](R) reader structure"]
impl crate::Readable for Mpuipsegb2Spec {}
#[doc = "`write(|w| ..)` method takes [`mpuipsegb2::W`](W) writer structure"]
impl crate::Writable for Mpuipsegb2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPUIPSEGB2 to value 0"]
impl crate::Resettable for Mpuipsegb2Spec {}
