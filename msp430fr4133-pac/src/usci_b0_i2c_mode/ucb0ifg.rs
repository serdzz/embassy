#[doc = "Register `UCB0IFG` reader"]
pub type R = crate::R<Ucb0ifgSpec>;
#[doc = "Register `UCB0IFG` writer"]
pub type W = crate::W<Ucb0ifgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI B0 Interrupt Flags Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0ifgSpec;
impl crate::RegisterSpec for Ucb0ifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0ifg::R`](R) reader structure"]
impl crate::Readable for Ucb0ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0ifg::W`](W) writer structure"]
impl crate::Writable for Ucb0ifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0IFG to value 0"]
impl crate::Resettable for Ucb0ifgSpec {}
