#[doc = "Register `RT1PS` reader"]
pub type R = crate::R<Rt1psSpec>;
#[doc = "Register `RT1PS` writer"]
pub type W = crate::W<Rt1psSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Prescale timer 1 counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`rt1ps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rt1ps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rt1psSpec;
impl crate::RegisterSpec for Rt1psSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rt1ps::R`](R) reader structure"]
impl crate::Readable for Rt1psSpec {}
#[doc = "`write(|w| ..)` method takes [`rt1ps::W`](W) writer structure"]
impl crate::Writable for Rt1psSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RT1PS to value 0"]
impl crate::Resettable for Rt1psSpec {}
