#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crc32diw0: Crc32diw0,
    crc32diw1: Crc32diw1,
    crc32dirbw1: Crc32dirbw1,
    crc32dirbw0: Crc32dirbw0,
    crc32iniresw0: Crc32iniresw0,
    crc32iniresw1: Crc32iniresw1,
    crc32resrw1: Crc32resrw1,
    crc32resrw0: Crc32resrw0,
    crc16diw0: Crc16diw0,
    _reserved9: [u8; 0x04],
    crc16dirbw0: Crc16dirbw0,
    crc16iniresw0: Crc16iniresw0,
    _reserved11: [u8; 0x04],
    crc16resrw0: Crc16resrw0,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC32 Data Input Word 0"]
    #[inline(always)]
    pub const fn crc32diw0(&self) -> &Crc32diw0 {
        &self.crc32diw0
    }
    #[doc = "0x02 - CRC32 Data Input Word 1"]
    #[inline(always)]
    pub const fn crc32diw1(&self) -> &Crc32diw1 {
        &self.crc32diw1
    }
    #[doc = "0x04 - CRC32 Data In Reverse Word 1"]
    #[inline(always)]
    pub const fn crc32dirbw1(&self) -> &Crc32dirbw1 {
        &self.crc32dirbw1
    }
    #[doc = "0x06 - CRC32 Data In Reverse Word 0"]
    #[inline(always)]
    pub const fn crc32dirbw0(&self) -> &Crc32dirbw0 {
        &self.crc32dirbw0
    }
    #[doc = "0x08 - CRC32 Initialization and Result Word 0"]
    #[inline(always)]
    pub const fn crc32iniresw0(&self) -> &Crc32iniresw0 {
        &self.crc32iniresw0
    }
    #[doc = "0x0a - CRC32 Initialization and Result Word 1"]
    #[inline(always)]
    pub const fn crc32iniresw1(&self) -> &Crc32iniresw1 {
        &self.crc32iniresw1
    }
    #[doc = "0x0c - CRC32 Result Reverse Word 1"]
    #[inline(always)]
    pub const fn crc32resrw1(&self) -> &Crc32resrw1 {
        &self.crc32resrw1
    }
    #[doc = "0x0e - CRC32 Result Reverse Word 0"]
    #[inline(always)]
    pub const fn crc32resrw0(&self) -> &Crc32resrw0 {
        &self.crc32resrw0
    }
    #[doc = "0x10 - CRC16 Data Input"]
    #[inline(always)]
    pub const fn crc16diw0(&self) -> &Crc16diw0 {
        &self.crc16diw0
    }
    #[doc = "0x16 - CRC16 Data In Reverse"]
    #[inline(always)]
    pub const fn crc16dirbw0(&self) -> &Crc16dirbw0 {
        &self.crc16dirbw0
    }
    #[doc = "0x18 - CRC16 Init and Result"]
    #[inline(always)]
    pub const fn crc16iniresw0(&self) -> &Crc16iniresw0 {
        &self.crc16iniresw0
    }
    #[doc = "0x1e - CRC16 Result Reverse"]
    #[inline(always)]
    pub const fn crc16resrw0(&self) -> &Crc16resrw0 {
        &self.crc16resrw0
    }
}
#[doc = "CRC32DIW0 (rw) register accessor: CRC32 Data Input Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32diw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32diw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc32diw0`] module"]
#[doc(alias = "CRC32DIW0")]
pub type Crc32diw0 = crate::Reg<crc32diw0::Crc32diw0Spec>;
#[doc = "CRC32 Data Input Word 0"]
pub mod crc32diw0;
#[doc = "CRC32DIW1 (rw) register accessor: CRC32 Data Input Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32diw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32diw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc32diw1`] module"]
#[doc(alias = "CRC32DIW1")]
pub type Crc32diw1 = crate::Reg<crc32diw1::Crc32diw1Spec>;
#[doc = "CRC32 Data Input Word 1"]
pub mod crc32diw1;
#[doc = "CRC32DIRBW1 (rw) register accessor: CRC32 Data In Reverse Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32dirbw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32dirbw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc32dirbw1`] module"]
#[doc(alias = "CRC32DIRBW1")]
pub type Crc32dirbw1 = crate::Reg<crc32dirbw1::Crc32dirbw1Spec>;
#[doc = "CRC32 Data In Reverse Word 1"]
pub mod crc32dirbw1;
#[doc = "CRC32DIRBW0 (rw) register accessor: CRC32 Data In Reverse Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32dirbw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32dirbw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc32dirbw0`] module"]
#[doc(alias = "CRC32DIRBW0")]
pub type Crc32dirbw0 = crate::Reg<crc32dirbw0::Crc32dirbw0Spec>;
#[doc = "CRC32 Data In Reverse Word 0"]
pub mod crc32dirbw0;
#[doc = "CRC32INIRESW0 (rw) register accessor: CRC32 Initialization and Result Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32iniresw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32iniresw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc32iniresw0`] module"]
#[doc(alias = "CRC32INIRESW0")]
pub type Crc32iniresw0 = crate::Reg<crc32iniresw0::Crc32iniresw0Spec>;
#[doc = "CRC32 Initialization and Result Word 0"]
pub mod crc32iniresw0;
#[doc = "CRC32INIRESW1 (rw) register accessor: CRC32 Initialization and Result Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32iniresw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32iniresw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc32iniresw1`] module"]
#[doc(alias = "CRC32INIRESW1")]
pub type Crc32iniresw1 = crate::Reg<crc32iniresw1::Crc32iniresw1Spec>;
#[doc = "CRC32 Initialization and Result Word 1"]
pub mod crc32iniresw1;
#[doc = "CRC32RESRW1 (rw) register accessor: CRC32 Result Reverse Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32resrw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32resrw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc32resrw1`] module"]
#[doc(alias = "CRC32RESRW1")]
pub type Crc32resrw1 = crate::Reg<crc32resrw1::Crc32resrw1Spec>;
#[doc = "CRC32 Result Reverse Word 1"]
pub mod crc32resrw1;
#[doc = "CRC32RESRW0 (rw) register accessor: CRC32 Result Reverse Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32resrw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32resrw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc32resrw0`] module"]
#[doc(alias = "CRC32RESRW0")]
pub type Crc32resrw0 = crate::Reg<crc32resrw0::Crc32resrw0Spec>;
#[doc = "CRC32 Result Reverse Word 0"]
pub mod crc32resrw0;
#[doc = "CRC16DIW0 (rw) register accessor: CRC16 Data Input\n\nYou can [`read`](crate::Reg::read) this register and get [`crc16diw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc16diw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc16diw0`] module"]
#[doc(alias = "CRC16DIW0")]
pub type Crc16diw0 = crate::Reg<crc16diw0::Crc16diw0Spec>;
#[doc = "CRC16 Data Input"]
pub mod crc16diw0;
#[doc = "CRC16DIRBW0 (rw) register accessor: CRC16 Data In Reverse\n\nYou can [`read`](crate::Reg::read) this register and get [`crc16dirbw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc16dirbw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc16dirbw0`] module"]
#[doc(alias = "CRC16DIRBW0")]
pub type Crc16dirbw0 = crate::Reg<crc16dirbw0::Crc16dirbw0Spec>;
#[doc = "CRC16 Data In Reverse"]
pub mod crc16dirbw0;
#[doc = "CRC16INIRESW0 (rw) register accessor: CRC16 Init and Result\n\nYou can [`read`](crate::Reg::read) this register and get [`crc16iniresw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc16iniresw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc16iniresw0`] module"]
#[doc(alias = "CRC16INIRESW0")]
pub type Crc16iniresw0 = crate::Reg<crc16iniresw0::Crc16iniresw0Spec>;
#[doc = "CRC16 Init and Result"]
pub mod crc16iniresw0;
#[doc = "CRC16RESRW0 (rw) register accessor: CRC16 Result Reverse\n\nYou can [`read`](crate::Reg::read) this register and get [`crc16resrw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc16resrw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc16resrw0`] module"]
#[doc(alias = "CRC16RESRW0")]
pub type Crc16resrw0 = crate::Reg<crc16resrw0::Crc16resrw0Spec>;
#[doc = "CRC16 Result Reverse"]
pub mod crc16resrw0;
