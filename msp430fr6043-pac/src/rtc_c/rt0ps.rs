#[doc = "Register `RT0PS` reader"]
pub type R = crate::R<Rt0psSpec>;
#[doc = "Register `RT0PS` writer"]
pub type W = crate::W<Rt0psSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Prescale timer 0 counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`rt0ps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rt0ps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rt0psSpec;
impl crate::RegisterSpec for Rt0psSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rt0ps::R`](R) reader structure"]
impl crate::Readable for Rt0psSpec {}
#[doc = "`write(|w| ..)` method takes [`rt0ps::W`](W) writer structure"]
impl crate::Writable for Rt0psSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RT0PS to value 0"]
impl crate::Resettable for Rt0psSpec {}
