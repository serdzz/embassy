#[doc = "Register `P8OUT` reader"]
pub type R = crate::R<P8outSpec>;
#[doc = "Register `P8OUT` writer"]
pub type W = crate::W<P8outSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 8 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p8out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8outSpec;
impl crate::RegisterSpec for P8outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8out::R`](R) reader structure"]
impl crate::Readable for P8outSpec {}
#[doc = "`write(|w| ..)` method takes [`p8out::W`](W) writer structure"]
impl crate::Writable for P8outSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P8OUT to value 0"]
impl crate::Resettable for P8outSpec {}
