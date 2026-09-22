#[doc = "Register `RTCIV` reader"]
pub type R = crate::R<RtcivSpec>;
#[doc = "Register `RTCIV` writer"]
pub type W = crate::W<RtcivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RTC interrupt vector\n\nYou can [`read`](crate::Reg::read) this register and get [`rtciv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtciv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcivSpec;
impl crate::RegisterSpec for RtcivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtciv::R`](R) reader structure"]
impl crate::Readable for RtcivSpec {}
#[doc = "`write(|w| ..)` method takes [`rtciv::W`](W) writer structure"]
impl crate::Writable for RtcivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCIV to value 0"]
impl crate::Resettable for RtcivSpec {}
