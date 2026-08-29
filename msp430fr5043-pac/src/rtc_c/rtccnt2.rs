#[doc = "Register `RTCCNT2` reader"]
pub type R = crate::R<Rtccnt2Spec>;
#[doc = "Register `RTCCNT2` writer"]
pub type W = crate::W<Rtccnt2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The RTCCNT2 register is the count of RTCCNT2\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccnt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccnt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtccnt2Spec;
impl crate::RegisterSpec for Rtccnt2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtccnt2::R`](R) reader structure"]
impl crate::Readable for Rtccnt2Spec {}
#[doc = "`write(|w| ..)` method takes [`rtccnt2::W`](W) writer structure"]
impl crate::Writable for Rtccnt2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCNT2 to value 0"]
impl crate::Resettable for Rtccnt2Spec {}
