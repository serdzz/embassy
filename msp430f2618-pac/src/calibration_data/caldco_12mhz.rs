#[doc = "Register `CALDCO_12MHZ` reader"]
pub type R = crate::R<Caldco12mhzSpec>;
#[doc = "Register `CALDCO_12MHZ` writer"]
pub type W = crate::W<Caldco12mhzSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DCOCTL Calibration Data for 12MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_12mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_12mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Caldco12mhzSpec;
impl crate::RegisterSpec for Caldco12mhzSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`caldco_12mhz::R`](R) reader structure"]
impl crate::Readable for Caldco12mhzSpec {}
#[doc = "`write(|w| ..)` method takes [`caldco_12mhz::W`](W) writer structure"]
impl crate::Writable for Caldco12mhzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALDCO_12MHZ to value 0"]
impl crate::Resettable for Caldco12mhzSpec {}
