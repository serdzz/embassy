#[doc = "Register `P7IE` reader"]
pub type R = crate::R<P7ieSpec>;
#[doc = "Register `P7IE` writer"]
pub type W = crate::W<P7ieSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 7 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p7ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7ieSpec;
impl crate::RegisterSpec for P7ieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7ie::R`](R) reader structure"]
impl crate::Readable for P7ieSpec {}
#[doc = "`write(|w| ..)` method takes [`p7ie::W`](W) writer structure"]
impl crate::Writable for P7ieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P7IE to value 0"]
impl crate::Resettable for P7ieSpec {}
