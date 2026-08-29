#[doc = "Register `RTCCNT3` reader"]
pub type R = crate::R<Rtccnt3Spec>;
#[doc = "Register `RTCCNT3` writer"]
pub type W = crate::W<Rtccnt3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The RTCCNT3 register is the count of RTCCNT3\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccnt3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccnt3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtccnt3Spec;
impl crate::RegisterSpec for Rtccnt3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtccnt3::R`](R) reader structure"]
impl crate::Readable for Rtccnt3Spec {}
#[doc = "`write(|w| ..)` method takes [`rtccnt3::W`](W) writer structure"]
impl crate::Writable for Rtccnt3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCNT3 to value 0"]
impl crate::Resettable for Rtccnt3Spec {}
