#[doc = "Register `U1MCTL` reader"]
pub type R = crate::R<U1mctlSpec>;
#[doc = "Register `U1MCTL` writer"]
pub type W = crate::W<U1mctlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USART 1 Modulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`u1mctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u1mctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U1mctlSpec;
impl crate::RegisterSpec for U1mctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`u1mctl::R`](R) reader structure"]
impl crate::Readable for U1mctlSpec {}
#[doc = "`write(|w| ..)` method takes [`u1mctl::W`](W) writer structure"]
impl crate::Writable for U1mctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets U1MCTL to value 0"]
impl crate::Resettable for U1mctlSpec {}
