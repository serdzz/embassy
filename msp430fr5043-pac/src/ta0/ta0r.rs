#[doc = "Register `TA0R` reader"]
pub type R = crate::R<Ta0rSpec>;
#[doc = "Register `TA0R` writer"]
pub type W = crate::W<Ta0rSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TimerA register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta0rSpec;
impl crate::RegisterSpec for Ta0rSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta0r::R`](R) reader structure"]
impl crate::Readable for Ta0rSpec {}
#[doc = "`write(|w| ..)` method takes [`ta0r::W`](W) writer structure"]
impl crate::Writable for Ta0rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA0R to value 0"]
impl crate::Resettable for Ta0rSpec {}
