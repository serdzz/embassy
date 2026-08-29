#[doc = "Register `SAPH_AATM_D` reader"]
pub type R = crate::R<SaphAatmDSpec>;
#[doc = "Register `SAPH_AATM_D` writer"]
pub type W = crate::W<SaphAatmDSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ASQ start to ADC trig\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aatm_d::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aatm_d::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAatmDSpec;
impl crate::RegisterSpec for SaphAatmDSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aatm_d::R`](R) reader structure"]
impl crate::Readable for SaphAatmDSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_aatm_d::W`](W) writer structure"]
impl crate::Writable for SaphAatmDSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AATM_D to value 0"]
impl crate::Resettable for SaphAatmDSpec {}
