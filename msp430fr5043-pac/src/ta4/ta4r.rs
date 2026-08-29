#[doc = "Register `TA4R` reader"]
pub type R = crate::R<Ta4rSpec>;
#[doc = "Register `TA4R` writer"]
pub type W = crate::W<Ta4rSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TimerA register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta4r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta4r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta4rSpec;
impl crate::RegisterSpec for Ta4rSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta4r::R`](R) reader structure"]
impl crate::Readable for Ta4rSpec {}
#[doc = "`write(|w| ..)` method takes [`ta4r::W`](W) writer structure"]
impl crate::Writable for Ta4rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA4R to value 0"]
impl crate::Resettable for Ta4rSpec {}
