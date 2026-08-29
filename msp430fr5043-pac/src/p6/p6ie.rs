#[doc = "Register `P6IE` reader"]
pub type R = crate::R<P6ieSpec>;
#[doc = "Register `P6IE` writer"]
pub type W = crate::W<P6ieSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 6 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p6ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6ieSpec;
impl crate::RegisterSpec for P6ieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6ie::R`](R) reader structure"]
impl crate::Readable for P6ieSpec {}
#[doc = "`write(|w| ..)` method takes [`p6ie::W`](W) writer structure"]
impl crate::Writable for P6ieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P6IE to value 0"]
impl crate::Resettable for P6ieSpec {}
