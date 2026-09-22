#[doc = "Register `TA1IV` reader"]
pub type R = crate::R<Ta1ivSpec>;
#[doc = "Register `TA1IV` writer"]
pub type W = crate::W<Ta1ivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer1_A3 Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ta1iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta1iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta1ivSpec;
impl crate::RegisterSpec for Ta1ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta1iv::R`](R) reader structure"]
impl crate::Readable for Ta1ivSpec {}
#[doc = "`write(|w| ..)` method takes [`ta1iv::W`](W) writer structure"]
impl crate::Writable for Ta1ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA1IV to value 0"]
impl crate::Resettable for Ta1ivSpec {}
