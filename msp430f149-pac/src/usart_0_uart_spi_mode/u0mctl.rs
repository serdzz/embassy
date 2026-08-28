#[doc = "Register `U0MCTL` reader"]
pub type R = crate::R<U0mctlSpec>;
#[doc = "Register `U0MCTL` writer"]
pub type W = crate::W<U0mctlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USART 0 Modulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`u0mctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u0mctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U0mctlSpec;
impl crate::RegisterSpec for U0mctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`u0mctl::R`](R) reader structure"]
impl crate::Readable for U0mctlSpec {}
#[doc = "`write(|w| ..)` method takes [`u0mctl::W`](W) writer structure"]
impl crate::Writable for U0mctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets U0MCTL to value 0"]
impl crate::Resettable for U0mctlSpec {}
