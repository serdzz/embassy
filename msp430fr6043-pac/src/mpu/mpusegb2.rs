#[doc = "Register `MPUSEGB2` reader"]
pub type R = crate::R<Mpusegb2Spec>;
#[doc = "Register `MPUSEGB2` writer"]
pub type W = crate::W<Mpusegb2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Memory Protection Unit Segmentation Border 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpusegb2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpusegb2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpusegb2Spec;
impl crate::RegisterSpec for Mpusegb2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpusegb2::R`](R) reader structure"]
impl crate::Readable for Mpusegb2Spec {}
#[doc = "`write(|w| ..)` method takes [`mpusegb2::W`](W) writer structure"]
impl crate::Writable for Mpusegb2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPUSEGB2 to value 0"]
impl crate::Resettable for Mpusegb2Spec {}
