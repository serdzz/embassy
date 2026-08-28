#[doc = "Register `U1TXBUF` reader"]
pub type R = crate::R<U1txbufSpec>;
#[doc = "Register `U1TXBUF` writer"]
pub type W = crate::W<U1txbufSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USART 1 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`u1txbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u1txbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U1txbufSpec;
impl crate::RegisterSpec for U1txbufSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`u1txbuf::R`](R) reader structure"]
impl crate::Readable for U1txbufSpec {}
#[doc = "`write(|w| ..)` method takes [`u1txbuf::W`](W) writer structure"]
impl crate::Writable for U1txbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets U1TXBUF to value 0"]
impl crate::Resettable for U1txbufSpec {}
