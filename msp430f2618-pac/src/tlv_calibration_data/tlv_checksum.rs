#[doc = "Register `TLV_CHECKSUM` reader"]
pub type R = crate::R<TlvChecksumSpec>;
#[doc = "Register `TLV_CHECKSUM` writer"]
pub type W = crate::W<TlvChecksumSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TLV CHECK SUM\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_checksum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_checksum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TlvChecksumSpec;
impl crate::RegisterSpec for TlvChecksumSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tlv_checksum::R`](R) reader structure"]
impl crate::Readable for TlvChecksumSpec {}
#[doc = "`write(|w| ..)` method takes [`tlv_checksum::W`](W) writer structure"]
impl crate::Writable for TlvChecksumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TLV_CHECKSUM to value 0"]
impl crate::Resettable for TlvChecksumSpec {}
