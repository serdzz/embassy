#[doc = "Register `P9IES` reader"]
pub type R = crate::R<P9iesSpec>;
#[doc = "Register `P9IES` writer"]
pub type W = crate::W<P9iesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 9 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p9ies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9ies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P9iesSpec;
impl crate::RegisterSpec for P9iesSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p9ies::R`](R) reader structure"]
impl crate::Readable for P9iesSpec {}
#[doc = "`write(|w| ..)` method takes [`p9ies::W`](W) writer structure"]
impl crate::Writable for P9iesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P9IES to value 0"]
impl crate::Resettable for P9iesSpec {}
