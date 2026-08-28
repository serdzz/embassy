#[doc = "Register `TLV_DCO_30_TAG` reader"]
pub type R = crate::R<TlvDco30TagSpec>;
#[doc = "Register `TLV_DCO_30_TAG` writer"]
pub type W = crate::W<TlvDco30TagSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TLV TAG_DCO30 TAG\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_dco_30_tag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_dco_30_tag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TlvDco30TagSpec;
impl crate::RegisterSpec for TlvDco30TagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tlv_dco_30_tag::R`](R) reader structure"]
impl crate::Readable for TlvDco30TagSpec {}
#[doc = "`write(|w| ..)` method takes [`tlv_dco_30_tag::W`](W) writer structure"]
impl crate::Writable for TlvDco30TagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TLV_DCO_30_TAG to value 0"]
impl crate::Resettable for TlvDco30TagSpec {}
