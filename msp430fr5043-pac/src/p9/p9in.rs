#[doc = "Register `P9IN` reader"]
pub type R = crate::R<P9inSpec>;
#[doc = "Register `P9IN` writer"]
pub type W = crate::W<P9inSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 9 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p9in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P9inSpec;
impl crate::RegisterSpec for P9inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p9in::R`](R) reader structure"]
impl crate::Readable for P9inSpec {}
#[doc = "`write(|w| ..)` method takes [`p9in::W`](W) writer structure"]
impl crate::Writable for P9inSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P9IN to value 0"]
impl crate::Resettable for P9inSpec {}
