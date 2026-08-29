#[doc = "Register `TA2R` reader"]
pub type R = crate::R<Ta2rSpec>;
#[doc = "Register `TA2R` writer"]
pub type W = crate::W<Ta2rSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TimerA register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta2rSpec;
impl crate::RegisterSpec for Ta2rSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta2r::R`](R) reader structure"]
impl crate::Readable for Ta2rSpec {}
#[doc = "`write(|w| ..)` method takes [`ta2r::W`](W) writer structure"]
impl crate::Writable for Ta2rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA2R to value 0"]
impl crate::Resettable for Ta2rSpec {}
