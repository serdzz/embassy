#[doc = "Register `UCA1TXBUF` reader"]
pub type R = crate::R<Uca1txbufSpec>;
#[doc = "Register `UCA1TXBUF` writer"]
pub type W = crate::W<Uca1txbufSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI A1 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1txbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1txbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca1txbufSpec;
impl crate::RegisterSpec for Uca1txbufSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1txbuf::R`](R) reader structure"]
impl crate::Readable for Uca1txbufSpec {}
#[doc = "`write(|w| ..)` method takes [`uca1txbuf::W`](W) writer structure"]
impl crate::Writable for Uca1txbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA1TXBUF to value 0"]
impl crate::Resettable for Uca1txbufSpec {}
