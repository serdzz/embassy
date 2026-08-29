#[doc = "Register `P8IE` reader"]
pub type R = crate::R<P8ieSpec>;
#[doc = "Register `P8IE` writer"]
pub type W = crate::W<P8ieSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 8 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p8ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8ieSpec;
impl crate::RegisterSpec for P8ieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8ie::R`](R) reader structure"]
impl crate::Readable for P8ieSpec {}
#[doc = "`write(|w| ..)` method takes [`p8ie::W`](W) writer structure"]
impl crate::Writable for P8ieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P8IE to value 0"]
impl crate::Resettable for P8ieSpec {}
