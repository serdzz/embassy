#[doc = "Register `CALBC1_1MHZ` reader"]
pub type R = crate::R<Calbc1_1mhzSpec>;
#[doc = "Register `CALBC1_1MHZ` writer"]
pub type W = crate::W<Calbc1_1mhzSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "BCSCTL1 Calibration Data for 1MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`calbc1_1mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calbc1_1mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Calbc1_1mhzSpec;
impl crate::RegisterSpec for Calbc1_1mhzSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`calbc1_1mhz::R`](R) reader structure"]
impl crate::Readable for Calbc1_1mhzSpec {}
#[doc = "`write(|w| ..)` method takes [`calbc1_1mhz::W`](W) writer structure"]
impl crate::Writable for Calbc1_1mhzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALBC1_1MHZ to value 0"]
impl crate::Resettable for Calbc1_1mhzSpec {}
