#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tlv_checksum: TlvChecksum,
    _reserved1: [u8; 0x18],
    tlv_adc12_1_tag: TlvAdc12_1Tag,
    tlv_adc12_1_len: TlvAdc12_1Len,
    _reserved3: [u8; 0x1a],
    tlv_dco_30_tag: TlvDco30Tag,
    tlv_dco_30_len: TlvDco30Len,
}
impl RegisterBlock {
    #[doc = "0x00 - TLV CHECK SUM"]
    #[inline(always)]
    pub const fn tlv_checksum(&self) -> &TlvChecksum {
        &self.tlv_checksum
    }
    #[doc = "0x1a - TLV ADC12_1 TAG"]
    #[inline(always)]
    pub const fn tlv_adc12_1_tag(&self) -> &TlvAdc12_1Tag {
        &self.tlv_adc12_1_tag
    }
    #[doc = "0x1b - TLV ADC12_1 LEN"]
    #[inline(always)]
    pub const fn tlv_adc12_1_len(&self) -> &TlvAdc12_1Len {
        &self.tlv_adc12_1_len
    }
    #[doc = "0x36 - TLV TAG_DCO30 TAG"]
    #[inline(always)]
    pub const fn tlv_dco_30_tag(&self) -> &TlvDco30Tag {
        &self.tlv_dco_30_tag
    }
    #[doc = "0x37 - TLV TAG_DCO30 LEN"]
    #[inline(always)]
    pub const fn tlv_dco_30_len(&self) -> &TlvDco30Len {
        &self.tlv_dco_30_len
    }
}
#[doc = "TLV_ADC12_1_TAG (rw) register accessor: TLV ADC12_1 TAG\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_adc12_1_tag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_adc12_1_tag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlv_adc12_1_tag`] module"]
#[doc(alias = "TLV_ADC12_1_TAG")]
pub type TlvAdc12_1Tag = crate::Reg<tlv_adc12_1_tag::TlvAdc12_1TagSpec>;
#[doc = "TLV ADC12_1 TAG"]
pub mod tlv_adc12_1_tag;
#[doc = "TLV_ADC12_1_LEN (rw) register accessor: TLV ADC12_1 LEN\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_adc12_1_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_adc12_1_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlv_adc12_1_len`] module"]
#[doc(alias = "TLV_ADC12_1_LEN")]
pub type TlvAdc12_1Len = crate::Reg<tlv_adc12_1_len::TlvAdc12_1LenSpec>;
#[doc = "TLV ADC12_1 LEN"]
pub mod tlv_adc12_1_len;
#[doc = "TLV_DCO_30_TAG (rw) register accessor: TLV TAG_DCO30 TAG\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_dco_30_tag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_dco_30_tag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlv_dco_30_tag`] module"]
#[doc(alias = "TLV_DCO_30_TAG")]
pub type TlvDco30Tag = crate::Reg<tlv_dco_30_tag::TlvDco30TagSpec>;
#[doc = "TLV TAG_DCO30 TAG"]
pub mod tlv_dco_30_tag;
#[doc = "TLV_DCO_30_LEN (rw) register accessor: TLV TAG_DCO30 LEN\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_dco_30_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_dco_30_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlv_dco_30_len`] module"]
#[doc(alias = "TLV_DCO_30_LEN")]
pub type TlvDco30Len = crate::Reg<tlv_dco_30_len::TlvDco30LenSpec>;
#[doc = "TLV TAG_DCO30 LEN"]
pub mod tlv_dco_30_len;
#[doc = "TLV_CHECKSUM (rw) register accessor: TLV CHECK SUM\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_checksum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_checksum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlv_checksum`] module"]
#[doc(alias = "TLV_CHECKSUM")]
pub type TlvChecksum = crate::Reg<tlv_checksum::TlvChecksumSpec>;
#[doc = "TLV CHECK SUM"]
pub mod tlv_checksum;
