#[doc = "Register `P9DIR` reader"]
pub type R = crate::R<P9dirSpec>;
#[doc = "Register `P9DIR` writer"]
pub type W = crate::W<P9dirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 9 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p9dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P9dirSpec;
impl crate::RegisterSpec for P9dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p9dir::R`](R) reader structure"]
impl crate::Readable for P9dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p9dir::W`](W) writer structure"]
impl crate::Writable for P9dirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P9DIR to value 0"]
impl crate::Resettable for P9dirSpec {}
