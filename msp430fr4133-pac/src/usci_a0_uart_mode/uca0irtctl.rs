#[doc = "Register `UCA0IRTCTL` reader"]
pub type R = crate::R<Uca0irtctlSpec>;
#[doc = "Register `UCA0IRTCTL` writer"]
pub type W = crate::W<Uca0irtctlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI A0 IrDA Transmit Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0irtctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0irtctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0irtctlSpec;
impl crate::RegisterSpec for Uca0irtctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0irtctl::R`](R) reader structure"]
impl crate::Readable for Uca0irtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0irtctl::W`](W) writer structure"]
impl crate::Writable for Uca0irtctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0IRTCTL to value 0"]
impl crate::Resettable for Uca0irtctlSpec {}
