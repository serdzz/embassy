#[doc = "Register `TA0IV` reader"]
pub type R = crate::R<Ta0ivSpec>;
#[doc = "Register `TA0IV` writer"]
pub type W = crate::W<Ta0ivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer0_A3 Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ta0iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta0iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta0ivSpec;
impl crate::RegisterSpec for Ta0ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta0iv::R`](R) reader structure"]
impl crate::Readable for Ta0ivSpec {}
#[doc = "`write(|w| ..)` method takes [`ta0iv::W`](W) writer structure"]
impl crate::Writable for Ta0ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA0IV to value 0"]
impl crate::Resettable for Ta0ivSpec {}
