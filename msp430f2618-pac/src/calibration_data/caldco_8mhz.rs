#[doc = "Register `CALDCO_8MHZ` reader"]
pub type R = crate::R<Caldco8mhzSpec>;
#[doc = "Register `CALDCO_8MHZ` writer"]
pub type W = crate::W<Caldco8mhzSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DCOCTL Calibration Data for 8MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_8mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_8mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Caldco8mhzSpec;
impl crate::RegisterSpec for Caldco8mhzSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`caldco_8mhz::R`](R) reader structure"]
impl crate::Readable for Caldco8mhzSpec {}
#[doc = "`write(|w| ..)` method takes [`caldco_8mhz::W`](W) writer structure"]
impl crate::Writable for Caldco8mhzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALDCO_8MHZ to value 0"]
impl crate::Resettable for Caldco8mhzSpec {}
