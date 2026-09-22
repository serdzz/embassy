#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crcdi: Crcdi,
    crcdirb: Crcdirb,
    crcinires: Crcinires,
    crcresr: Crcresr,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC Data In Register"]
    #[inline(always)]
    pub const fn crcdi(&self) -> &Crcdi {
        &self.crcdi
    }
    #[doc = "0x02 - CRC data in reverse byte Register"]
    #[inline(always)]
    pub const fn crcdirb(&self) -> &Crcdirb {
        &self.crcdirb
    }
    #[doc = "0x04 - CRC Initialisation Register and Result Register"]
    #[inline(always)]
    pub const fn crcinires(&self) -> &Crcinires {
        &self.crcinires
    }
    #[doc = "0x06 - CRC reverse result Register"]
    #[inline(always)]
    pub const fn crcresr(&self) -> &Crcresr {
        &self.crcresr
    }
}
#[doc = "CRCDI (rw) register accessor: CRC Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdi`] module"]
#[doc(alias = "CRCDI")]
pub type Crcdi = crate::Reg<crcdi::CrcdiSpec>;
#[doc = "CRC Data In Register"]
pub mod crcdi;
#[doc = "CRCDIRB (rw) register accessor: CRC data in reverse byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdirb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdirb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdirb`] module"]
#[doc(alias = "CRCDIRB")]
pub type Crcdirb = crate::Reg<crcdirb::CrcdirbSpec>;
#[doc = "CRC data in reverse byte Register"]
pub mod crcdirb;
#[doc = "CRCINIRES (rw) register accessor: CRC Initialisation Register and Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcinires::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcinires::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcinires`] module"]
#[doc(alias = "CRCINIRES")]
pub type Crcinires = crate::Reg<crcinires::CrciniresSpec>;
#[doc = "CRC Initialisation Register and Result Register"]
pub mod crcinires;
#[doc = "CRCRESR (rw) register accessor: CRC reverse result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcresr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcresr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcresr`] module"]
#[doc(alias = "CRCRESR")]
pub type Crcresr = crate::Reg<crcresr::CrcresrSpec>;
#[doc = "CRC reverse result Register"]
pub mod crcresr;
