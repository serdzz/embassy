#[doc = "Register `P9OUT` reader"]
pub type R = crate::R<P9outSpec>;
#[doc = "Register `P9OUT` writer"]
pub type W = crate::W<P9outSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 9 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p9out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P9outSpec;
impl crate::RegisterSpec for P9outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p9out::R`](R) reader structure"]
impl crate::Readable for P9outSpec {}
#[doc = "`write(|w| ..)` method takes [`p9out::W`](W) writer structure"]
impl crate::Writable for P9outSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P9OUT to value 0"]
impl crate::Resettable for P9outSpec {}
