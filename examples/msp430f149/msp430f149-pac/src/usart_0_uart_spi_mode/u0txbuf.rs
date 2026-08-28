#[doc = "Register `U0TXBUF` reader"]
pub type R = crate::R<U0txbufSpec>;
#[doc = "Register `U0TXBUF` writer"]
pub type W = crate::W<U0txbufSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USART 0 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`u0txbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u0txbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U0txbufSpec;
impl crate::RegisterSpec for U0txbufSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`u0txbuf::R`](R) reader structure"]
impl crate::Readable for U0txbufSpec {}
#[doc = "`write(|w| ..)` method takes [`u0txbuf::W`](W) writer structure"]
impl crate::Writable for U0txbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets U0TXBUF to value 0"]
impl crate::Resettable for U0txbufSpec {}
