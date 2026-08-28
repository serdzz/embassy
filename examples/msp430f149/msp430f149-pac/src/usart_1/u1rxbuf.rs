#[doc = "Register `U1RXBUF` reader"]
pub type R = crate::R<U1rxbufSpec>;
#[doc = "Register `U1RXBUF` writer"]
pub type W = crate::W<U1rxbufSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USART 1 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`u1rxbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u1rxbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U1rxbufSpec;
impl crate::RegisterSpec for U1rxbufSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`u1rxbuf::R`](R) reader structure"]
impl crate::Readable for U1rxbufSpec {}
#[doc = "`write(|w| ..)` method takes [`u1rxbuf::W`](W) writer structure"]
impl crate::Writable for U1rxbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets U1RXBUF to value 0"]
impl crate::Resettable for U1rxbufSpec {}
