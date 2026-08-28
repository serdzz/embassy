#[doc = "Register `CALBC1_12MHZ` reader"]
pub type R = crate::R<Calbc1_12mhzSpec>;
#[doc = "Register `CALBC1_12MHZ` writer"]
pub type W = crate::W<Calbc1_12mhzSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "BCSCTL1 Calibration Data for 12MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`calbc1_12mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calbc1_12mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Calbc1_12mhzSpec;
impl crate::RegisterSpec for Calbc1_12mhzSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`calbc1_12mhz::R`](R) reader structure"]
impl crate::Readable for Calbc1_12mhzSpec {}
#[doc = "`write(|w| ..)` method takes [`calbc1_12mhz::W`](W) writer structure"]
impl crate::Writable for Calbc1_12mhzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALBC1_12MHZ to value 0"]
impl crate::Resettable for Calbc1_12mhzSpec {}
