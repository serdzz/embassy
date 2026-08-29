#[doc = "Register `P8IES` reader"]
pub type R = crate::R<P8iesSpec>;
#[doc = "Register `P8IES` writer"]
pub type W = crate::W<P8iesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 8 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p8ies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8ies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8iesSpec;
impl crate::RegisterSpec for P8iesSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8ies::R`](R) reader structure"]
impl crate::Readable for P8iesSpec {}
#[doc = "`write(|w| ..)` method takes [`p8ies::W`](W) writer structure"]
impl crate::Writable for P8iesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P8IES to value 0"]
impl crate::Resettable for P8iesSpec {}
