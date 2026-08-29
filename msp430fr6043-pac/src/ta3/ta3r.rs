#[doc = "Register `TA3R` reader"]
pub type R = crate::R<Ta3rSpec>;
#[doc = "Register `TA3R` writer"]
pub type W = crate::W<Ta3rSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TimerA register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta3r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta3r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta3rSpec;
impl crate::RegisterSpec for Ta3rSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta3r::R`](R) reader structure"]
impl crate::Readable for Ta3rSpec {}
#[doc = "`write(|w| ..)` method takes [`ta3r::W`](W) writer structure"]
impl crate::Writable for Ta3rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA3R to value 0"]
impl crate::Resettable for Ta3rSpec {}
