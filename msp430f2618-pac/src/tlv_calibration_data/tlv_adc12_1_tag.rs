#[doc = "Register `TLV_ADC12_1_TAG` reader"]
pub type R = crate::R<TlvAdc12_1TagSpec>;
#[doc = "Register `TLV_ADC12_1_TAG` writer"]
pub type W = crate::W<TlvAdc12_1TagSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TLV ADC12_1 TAG\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_adc12_1_tag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_adc12_1_tag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TlvAdc12_1TagSpec;
impl crate::RegisterSpec for TlvAdc12_1TagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tlv_adc12_1_tag::R`](R) reader structure"]
impl crate::Readable for TlvAdc12_1TagSpec {}
#[doc = "`write(|w| ..)` method takes [`tlv_adc12_1_tag::W`](W) writer structure"]
impl crate::Writable for TlvAdc12_1TagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TLV_ADC12_1_TAG to value 0"]
impl crate::Resettable for TlvAdc12_1TagSpec {}
