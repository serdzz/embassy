#[doc = "Register `P7DIR` reader"]
pub type R = crate::R<P7dirSpec>;
#[doc = "Register `P7DIR` writer"]
pub type W = crate::W<P7dirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 7 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p7dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7dirSpec;
impl crate::RegisterSpec for P7dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7dir::R`](R) reader structure"]
impl crate::Readable for P7dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p7dir::W`](W) writer structure"]
impl crate::Writable for P7dirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P7DIR to value 0"]
impl crate::Resettable for P7dirSpec {}
