#[doc = "Register `P7OUT` reader"]
pub type R = crate::R<P7outSpec>;
#[doc = "Register `P7OUT` writer"]
pub type W = crate::W<P7outSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 7 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p7out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7outSpec;
impl crate::RegisterSpec for P7outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7out::R`](R) reader structure"]
impl crate::Readable for P7outSpec {}
#[doc = "`write(|w| ..)` method takes [`p7out::W`](W) writer structure"]
impl crate::Writable for P7outSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P7OUT to value 0"]
impl crate::Resettable for P7outSpec {}
