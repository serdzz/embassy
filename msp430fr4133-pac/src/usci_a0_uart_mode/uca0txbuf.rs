#[doc = "Register `UCA0TXBUF` reader"]
pub type R = crate::R<Uca0txbufSpec>;
#[doc = "Register `UCA0TXBUF` writer"]
pub type W = crate::W<Uca0txbufSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI A0 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0txbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0txbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0txbufSpec;
impl crate::RegisterSpec for Uca0txbufSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca0txbuf::R`](R) reader structure"]
impl crate::Readable for Uca0txbufSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0txbuf::W`](W) writer structure"]
impl crate::Writable for Uca0txbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0TXBUF to value 0"]
impl crate::Resettable for Uca0txbufSpec {}
