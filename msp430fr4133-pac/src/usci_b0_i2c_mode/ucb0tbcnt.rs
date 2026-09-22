#[doc = "Register `UCB0TBCNT` reader"]
pub type R = crate::R<Ucb0tbcntSpec>;
#[doc = "Register `UCB0TBCNT` writer"]
pub type W = crate::W<Ucb0tbcntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI B0 Byte Counter Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0tbcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0tbcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0tbcntSpec;
impl crate::RegisterSpec for Ucb0tbcntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0tbcnt::R`](R) reader structure"]
impl crate::Readable for Ucb0tbcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0tbcnt::W`](W) writer structure"]
impl crate::Writable for Ucb0tbcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0TBCNT to value 0"]
impl crate::Resettable for Ucb0tbcntSpec {}
