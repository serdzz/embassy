#[doc = "Register `TAIV` reader"]
pub type R = crate::R<TaivSpec>;
#[doc = "Register `TAIV` writer"]
pub type W = crate::W<TaivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer A Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`taiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`taiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaivSpec;
impl crate::RegisterSpec for TaivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`taiv::R`](R) reader structure"]
impl crate::Readable for TaivSpec {}
#[doc = "`write(|w| ..)` method takes [`taiv::W`](W) writer structure"]
impl crate::Writable for TaivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TAIV to value 0"]
impl crate::Resettable for TaivSpec {}
