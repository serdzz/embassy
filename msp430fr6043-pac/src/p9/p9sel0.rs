#[doc = "Register `P9SEL0` reader"]
pub type R = crate::R<P9sel0Spec>;
#[doc = "Register `P9SEL0` writer"]
pub type W = crate::W<P9sel0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 9 Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p9sel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9sel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P9sel0Spec;
impl crate::RegisterSpec for P9sel0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p9sel0::R`](R) reader structure"]
impl crate::Readable for P9sel0Spec {}
#[doc = "`write(|w| ..)` method takes [`p9sel0::W`](W) writer structure"]
impl crate::Writable for P9sel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P9SEL0 to value 0"]
impl crate::Resettable for P9sel0Spec {}
