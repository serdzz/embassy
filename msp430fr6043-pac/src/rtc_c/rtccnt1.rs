#[doc = "Register `RTCCNT1` reader"]
pub type R = crate::R<Rtccnt1Spec>;
#[doc = "Register `RTCCNT1` writer"]
pub type W = crate::W<Rtccnt1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The RTCCNT1 register is the count of RTCCNT1\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtccnt1Spec;
impl crate::RegisterSpec for Rtccnt1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtccnt1::R`](R) reader structure"]
impl crate::Readable for Rtccnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`rtccnt1::W`](W) writer structure"]
impl crate::Writable for Rtccnt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCNT1 to value 0"]
impl crate::Resettable for Rtccnt1Spec {}
