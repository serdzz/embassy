#[doc = "Register `P8DIR` reader"]
pub type R = crate::R<P8dirSpec>;
#[doc = "Register `P8DIR` writer"]
pub type W = crate::W<P8dirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 8 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p8dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8dirSpec;
impl crate::RegisterSpec for P8dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8dir::R`](R) reader structure"]
impl crate::Readable for P8dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p8dir::W`](W) writer structure"]
impl crate::Writable for P8dirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P8DIR to value 0"]
impl crate::Resettable for P8dirSpec {}
