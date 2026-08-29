#[doc = "Register `P9IE` reader"]
pub type R = crate::R<P9ieSpec>;
#[doc = "Register `P9IE` writer"]
pub type W = crate::W<P9ieSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 9 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p9ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P9ieSpec;
impl crate::RegisterSpec for P9ieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p9ie::R`](R) reader structure"]
impl crate::Readable for P9ieSpec {}
#[doc = "`write(|w| ..)` method takes [`p9ie::W`](W) writer structure"]
impl crate::Writable for P9ieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P9IE to value 0"]
impl crate::Resettable for P9ieSpec {}
