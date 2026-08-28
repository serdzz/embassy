#[doc = "Register `U0RXBUF` reader"]
pub type R = crate::R<U0rxbufSpec>;
#[doc = "Register `U0RXBUF` writer"]
pub type W = crate::W<U0rxbufSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USART 0 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`u0rxbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u0rxbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U0rxbufSpec;
impl crate::RegisterSpec for U0rxbufSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`u0rxbuf::R`](R) reader structure"]
impl crate::Readable for U0rxbufSpec {}
#[doc = "`write(|w| ..)` method takes [`u0rxbuf::W`](W) writer structure"]
impl crate::Writable for U0rxbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets U0RXBUF to value 0"]
impl crate::Resettable for U0rxbufSpec {}
