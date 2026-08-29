#[doc = "Register `MPUSEGB1` reader"]
pub type R = crate::R<Mpusegb1Spec>;
#[doc = "Register `MPUSEGB1` writer"]
pub type W = crate::W<Mpusegb1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Memory Protection Unit Segmentation Border 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpusegb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpusegb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpusegb1Spec;
impl crate::RegisterSpec for Mpusegb1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpusegb1::R`](R) reader structure"]
impl crate::Readable for Mpusegb1Spec {}
#[doc = "`write(|w| ..)` method takes [`mpusegb1::W`](W) writer structure"]
impl crate::Writable for Mpusegb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPUSEGB1 to value 0"]
impl crate::Resettable for Mpusegb1Spec {}
