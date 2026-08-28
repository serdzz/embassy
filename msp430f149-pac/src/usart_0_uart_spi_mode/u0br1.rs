#[doc = "Register `U0BR1` reader"]
pub type R = crate::R<U0br1Spec>;
#[doc = "Register `U0BR1` writer"]
pub type W = crate::W<U0br1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USART 0 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`u0br1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u0br1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U0br1Spec;
impl crate::RegisterSpec for U0br1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`u0br1::R`](R) reader structure"]
impl crate::Readable for U0br1Spec {}
#[doc = "`write(|w| ..)` method takes [`u0br1::W`](W) writer structure"]
impl crate::Writable for U0br1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets U0BR1 to value 0"]
impl crate::Resettable for U0br1Spec {}
