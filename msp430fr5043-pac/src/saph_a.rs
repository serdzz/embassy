#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    saph_aiidx: SaphAiidx,
    saph_amis: SaphAmis,
    saph_aris: SaphAris,
    saph_aimsc: SaphAimsc,
    saph_aicr: SaphAicr,
    saph_aisr: SaphAisr,
    saph_adesclo: SaphAdesclo,
    saph_adeschi: SaphAdeschi,
    saph_akey: SaphAkey,
    saph_aoctl0: SaphAoctl0,
    saph_aoctl1: SaphAoctl1,
    saph_aosel: SaphAosel,
    _reserved12: [u8; 0x08],
    saph_ach0put: SaphAch0put,
    saph_ach0pdt: SaphAch0pdt,
    saph_ach0tt: SaphAch0tt,
    saph_ach1put: SaphAch1put,
    saph_ach1pdt: SaphAch1pdt,
    saph_ach1tt: SaphAch1tt,
    saph_amcnf: SaphAmcnf,
    saph_atactl: SaphAtactl,
    saph_aictl0: SaphAictl0,
    _reserved21: [u8; 0x02],
    saph_abctl: SaphAbctl,
    _reserved22: [u8; 0x0a],
    saph_apgc: SaphApgc,
    saph_apglper: SaphApglper,
    saph_apghper: SaphApghper,
    saph_apgctl: SaphApgctl,
    saph_appgtrig: SaphAppgtrig,
    saph_axpgctl: SaphAxpgctl,
    saph_axpglper: SaphAxpglper,
    saph_axpghper: SaphAxpghper,
    _reserved30: [u8; 0x10],
    saph_aasctl0: SaphAasctl0,
    saph_aasctl1: SaphAasctl1,
    saph_aasqtrig: SaphAasqtrig,
    _reserved33: [u8; 0x01],
    saph_aapol: SaphAapol,
    saph_aaplev: SaphAaplev,
    saph_aaphiz: SaphAaphiz,
    _reserved36: [u8; 0x02],
    saph_aatm_a: SaphAatmA,
    saph_aatm_b: SaphAatmB,
    saph_aatm_c: SaphAatmC,
    saph_aatm_d: SaphAatmD,
    saph_aatm_e: SaphAatmE,
    saph_aatm_f: SaphAatmF,
    saph_atbctl: SaphAtbctl,
    saph_aatimlo: SaphAatimlo,
    saph_aatimhi: SaphAatimhi,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Index"]
    #[inline(always)]
    pub const fn saph_aiidx(&self) -> &SaphAiidx {
        &self.saph_aiidx
    }
    #[doc = "0x02 - Masked Interrupt Satus"]
    #[inline(always)]
    pub const fn saph_amis(&self) -> &SaphAmis {
        &self.saph_amis
    }
    #[doc = "0x04 - Raw Interrupt Status"]
    #[inline(always)]
    pub const fn saph_aris(&self) -> &SaphAris {
        &self.saph_aris
    }
    #[doc = "0x06 - Interrupt Mask"]
    #[inline(always)]
    pub const fn saph_aimsc(&self) -> &SaphAimsc {
        &self.saph_aimsc
    }
    #[doc = "0x08 - Interrupt Clear"]
    #[inline(always)]
    pub const fn saph_aicr(&self) -> &SaphAicr {
        &self.saph_aicr
    }
    #[doc = "0x0a - Interrupt Set"]
    #[inline(always)]
    pub const fn saph_aisr(&self) -> &SaphAisr {
        &self.saph_aisr
    }
    #[doc = "0x0c - Module-Descriptor Low Word"]
    #[inline(always)]
    pub const fn saph_adesclo(&self) -> &SaphAdesclo {
        &self.saph_adesclo
    }
    #[doc = "0x0e - Module-Descriptor High Word"]
    #[inline(always)]
    pub const fn saph_adeschi(&self) -> &SaphAdeschi {
        &self.saph_adeschi
    }
    #[doc = "0x10 - Key"]
    #[inline(always)]
    pub const fn saph_akey(&self) -> &SaphAkey {
        &self.saph_akey
    }
    #[doc = "0x12 - Physical Interface Output Control #0"]
    #[inline(always)]
    pub const fn saph_aoctl0(&self) -> &SaphAoctl0 {
        &self.saph_aoctl0
    }
    #[doc = "0x14 - Physical Interface Output Control #1"]
    #[inline(always)]
    pub const fn saph_aoctl1(&self) -> &SaphAoctl1 {
        &self.saph_aoctl1
    }
    #[doc = "0x16 - Physical Interface Output Function Select"]
    #[inline(always)]
    pub const fn saph_aosel(&self) -> &SaphAosel {
        &self.saph_aosel
    }
    #[doc = "0x20 - Channel 0 Pull UpTrim Register"]
    #[inline(always)]
    pub const fn saph_ach0put(&self) -> &SaphAch0put {
        &self.saph_ach0put
    }
    #[doc = "0x22 - Channel 0 Pull DownTrim Register"]
    #[inline(always)]
    pub const fn saph_ach0pdt(&self) -> &SaphAch0pdt {
        &self.saph_ach0pdt
    }
    #[doc = "0x24 - Channel 0 Termination Trim"]
    #[inline(always)]
    pub const fn saph_ach0tt(&self) -> &SaphAch0tt {
        &self.saph_ach0tt
    }
    #[doc = "0x26 - Channel 1 Pull UpTrim"]
    #[inline(always)]
    pub const fn saph_ach1put(&self) -> &SaphAch1put {
        &self.saph_ach1put
    }
    #[doc = "0x28 - Channel 1 Pull DownTrim"]
    #[inline(always)]
    pub const fn saph_ach1pdt(&self) -> &SaphAch1pdt {
        &self.saph_ach1pdt
    }
    #[doc = "0x2a - Channel 1 Termination Trim"]
    #[inline(always)]
    pub const fn saph_ach1tt(&self) -> &SaphAch1tt {
        &self.saph_ach1tt
    }
    #[doc = "0x2c - Mode Configuration Register"]
    #[inline(always)]
    pub const fn saph_amcnf(&self) -> &SaphAmcnf {
        &self.saph_amcnf
    }
    #[doc = "0x2e - Trim Access Control"]
    #[inline(always)]
    pub const fn saph_atactl(&self) -> &SaphAtactl {
        &self.saph_atactl
    }
    #[doc = "0x30 - Physical Interface Input Control #0"]
    #[inline(always)]
    pub const fn saph_aictl0(&self) -> &SaphAictl0 {
        &self.saph_aictl0
    }
    #[doc = "0x34 - Bias Control"]
    #[inline(always)]
    pub const fn saph_abctl(&self) -> &SaphAbctl {
        &self.saph_abctl
    }
    #[doc = "0x40 - PPG Count"]
    #[inline(always)]
    pub const fn saph_apgc(&self) -> &SaphApgc {
        &self.saph_apgc
    }
    #[doc = "0x42 - Pulse Generator Low Period"]
    #[inline(always)]
    pub const fn saph_apglper(&self) -> &SaphApglper {
        &self.saph_apglper
    }
    #[doc = "0x44 - Pulse Generator High Period"]
    #[inline(always)]
    pub const fn saph_apghper(&self) -> &SaphApghper {
        &self.saph_apghper
    }
    #[doc = "0x46 - PPG Control"]
    #[inline(always)]
    pub const fn saph_apgctl(&self) -> &SaphApgctl {
        &self.saph_apgctl
    }
    #[doc = "0x48 - PPG Software Trigger"]
    #[inline(always)]
    pub const fn saph_appgtrig(&self) -> &SaphAppgtrig {
        &self.saph_appgtrig
    }
    #[doc = "0x4a - Extended Pulse Control Register"]
    #[inline(always)]
    pub const fn saph_axpgctl(&self) -> &SaphAxpgctl {
        &self.saph_axpgctl
    }
    #[doc = "0x4c - Extra Pulse Low Period Register"]
    #[inline(always)]
    pub const fn saph_axpglper(&self) -> &SaphAxpglper {
        &self.saph_axpglper
    }
    #[doc = "0x4e - Extra Pulse High Period Register"]
    #[inline(always)]
    pub const fn saph_axpghper(&self) -> &SaphAxpghper {
        &self.saph_axpghper
    }
    #[doc = "0x60 - A-SEQ control register 0"]
    #[inline(always)]
    pub const fn saph_aasctl0(&self) -> &SaphAasctl0 {
        &self.saph_aasctl0
    }
    #[doc = "0x62 - A-SEQ control register 1"]
    #[inline(always)]
    pub const fn saph_aasctl1(&self) -> &SaphAasctl1 {
        &self.saph_aasctl1
    }
    #[doc = "0x64 - ASQ Software Trigger"]
    #[inline(always)]
    pub const fn saph_aasqtrig(&self) -> &SaphAasqtrig {
        &self.saph_aasqtrig
    }
    #[doc = "0x66 - ASQ ping output polarity"]
    #[inline(always)]
    pub const fn saph_aapol(&self) -> &SaphAapol {
        &self.saph_aapol
    }
    #[doc = "0x68 - ASQ ping pause level"]
    #[inline(always)]
    pub const fn saph_aaplev(&self) -> &SaphAaplev {
        &self.saph_aaplev
    }
    #[doc = "0x6a - ASQ ping pause impedance"]
    #[inline(always)]
    pub const fn saph_aaphiz(&self) -> &SaphAaphiz {
        &self.saph_aaphiz
    }
    #[doc = "0x6e - A-SEQ start to 1st ping"]
    #[inline(always)]
    pub const fn saph_aatm_a(&self) -> &SaphAatmA {
        &self.saph_aatm_a
    }
    #[doc = "0x70 - ASQ start to ADC arm"]
    #[inline(always)]
    pub const fn saph_aatm_b(&self) -> &SaphAatmB {
        &self.saph_aatm_b
    }
    #[doc = "0x72 - Count for the TIMEMARK C Event"]
    #[inline(always)]
    pub const fn saph_aatm_c(&self) -> &SaphAatmC {
        &self.saph_aatm_c
    }
    #[doc = "0x74 - ASQ start to ADC trig"]
    #[inline(always)]
    pub const fn saph_aatm_d(&self) -> &SaphAatmD {
        &self.saph_aatm_d
    }
    #[doc = "0x76 - ASQ start to restart"]
    #[inline(always)]
    pub const fn saph_aatm_e(&self) -> &SaphAatmE {
        &self.saph_aatm_e
    }
    #[doc = "0x78 - ASQ start to timeout"]
    #[inline(always)]
    pub const fn saph_aatm_f(&self) -> &SaphAatmF {
        &self.saph_aatm_f
    }
    #[doc = "0x7a - Time Base Control"]
    #[inline(always)]
    pub const fn saph_atbctl(&self) -> &SaphAtbctl {
        &self.saph_atbctl
    }
    #[doc = "0x7c - Acquisition Timer Low Part"]
    #[inline(always)]
    pub const fn saph_aatimlo(&self) -> &SaphAatimlo {
        &self.saph_aatimlo
    }
    #[doc = "0x7e - Acquisition Timer High Part"]
    #[inline(always)]
    pub const fn saph_aatimhi(&self) -> &SaphAatimhi {
        &self.saph_aatimhi
    }
}
#[doc = "SAPH_AASQTRIG (rw) register accessor: ASQ Software Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aasqtrig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aasqtrig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aasqtrig`] module"]
#[doc(alias = "SAPH_AASQTRIG")]
pub type SaphAasqtrig = crate::Reg<saph_aasqtrig::SaphAasqtrigSpec>;
#[doc = "ASQ Software Trigger"]
pub mod saph_aasqtrig;
#[doc = "SAPH_AIIDX (rw) register accessor: Interrupt Index\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aiidx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aiidx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aiidx`] module"]
#[doc(alias = "SAPH_AIIDX")]
pub type SaphAiidx = crate::Reg<saph_aiidx::SaphAiidxSpec>;
#[doc = "Interrupt Index"]
pub mod saph_aiidx;
#[doc = "SAPH_AMIS (rw) register accessor: Masked Interrupt Satus\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_amis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_amis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_amis`] module"]
#[doc(alias = "SAPH_AMIS")]
pub type SaphAmis = crate::Reg<saph_amis::SaphAmisSpec>;
#[doc = "Masked Interrupt Satus"]
pub mod saph_amis;
#[doc = "SAPH_ARIS (rw) register accessor: Raw Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aris::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aris::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aris`] module"]
#[doc(alias = "SAPH_ARIS")]
pub type SaphAris = crate::Reg<saph_aris::SaphArisSpec>;
#[doc = "Raw Interrupt Status"]
pub mod saph_aris;
#[doc = "SAPH_AIMSC (rw) register accessor: Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aimsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aimsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aimsc`] module"]
#[doc(alias = "SAPH_AIMSC")]
pub type SaphAimsc = crate::Reg<saph_aimsc::SaphAimscSpec>;
#[doc = "Interrupt Mask"]
pub mod saph_aimsc;
#[doc = "SAPH_AICR (rw) register accessor: Interrupt Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aicr`] module"]
#[doc(alias = "SAPH_AICR")]
pub type SaphAicr = crate::Reg<saph_aicr::SaphAicrSpec>;
#[doc = "Interrupt Clear"]
pub mod saph_aicr;
#[doc = "SAPH_AISR (rw) register accessor: Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aisr`] module"]
#[doc(alias = "SAPH_AISR")]
pub type SaphAisr = crate::Reg<saph_aisr::SaphAisrSpec>;
#[doc = "Interrupt Set"]
pub mod saph_aisr;
#[doc = "SAPH_ADESCLO (rw) register accessor: Module-Descriptor Low Word\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_adesclo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_adesclo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_adesclo`] module"]
#[doc(alias = "SAPH_ADESCLO")]
pub type SaphAdesclo = crate::Reg<saph_adesclo::SaphAdescloSpec>;
#[doc = "Module-Descriptor Low Word"]
pub mod saph_adesclo;
#[doc = "SAPH_ADESCHI (rw) register accessor: Module-Descriptor High Word\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_adeschi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_adeschi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_adeschi`] module"]
#[doc(alias = "SAPH_ADESCHI")]
pub type SaphAdeschi = crate::Reg<saph_adeschi::SaphAdeschiSpec>;
#[doc = "Module-Descriptor High Word"]
pub mod saph_adeschi;
#[doc = "SAPH_AKEY (rw) register accessor: Key\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_akey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_akey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_akey`] module"]
#[doc(alias = "SAPH_AKEY")]
pub type SaphAkey = crate::Reg<saph_akey::SaphAkeySpec>;
#[doc = "Key"]
pub mod saph_akey;
#[doc = "SAPH_AOCTL0 (rw) register accessor: Physical Interface Output Control #0\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aoctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aoctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aoctl0`] module"]
#[doc(alias = "SAPH_AOCTL0")]
pub type SaphAoctl0 = crate::Reg<saph_aoctl0::SaphAoctl0Spec>;
#[doc = "Physical Interface Output Control #0"]
pub mod saph_aoctl0;
#[doc = "SAPH_AOCTL1 (rw) register accessor: Physical Interface Output Control #1\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aoctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aoctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aoctl1`] module"]
#[doc(alias = "SAPH_AOCTL1")]
pub type SaphAoctl1 = crate::Reg<saph_aoctl1::SaphAoctl1Spec>;
#[doc = "Physical Interface Output Control #1"]
pub mod saph_aoctl1;
#[doc = "SAPH_AOSEL (rw) register accessor: Physical Interface Output Function Select\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aosel`] module"]
#[doc(alias = "SAPH_AOSEL")]
pub type SaphAosel = crate::Reg<saph_aosel::SaphAoselSpec>;
#[doc = "Physical Interface Output Function Select"]
pub mod saph_aosel;
#[doc = "SAPH_ACH0PUT (rw) register accessor: Channel 0 Pull UpTrim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_ach0put::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_ach0put::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_ach0put`] module"]
#[doc(alias = "SAPH_ACH0PUT")]
pub type SaphAch0put = crate::Reg<saph_ach0put::SaphAch0putSpec>;
#[doc = "Channel 0 Pull UpTrim Register"]
pub mod saph_ach0put;
#[doc = "SAPH_ACH0PDT (rw) register accessor: Channel 0 Pull DownTrim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_ach0pdt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_ach0pdt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_ach0pdt`] module"]
#[doc(alias = "SAPH_ACH0PDT")]
pub type SaphAch0pdt = crate::Reg<saph_ach0pdt::SaphAch0pdtSpec>;
#[doc = "Channel 0 Pull DownTrim Register"]
pub mod saph_ach0pdt;
#[doc = "SAPH_ACH0TT (rw) register accessor: Channel 0 Termination Trim\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_ach0tt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_ach0tt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_ach0tt`] module"]
#[doc(alias = "SAPH_ACH0TT")]
pub type SaphAch0tt = crate::Reg<saph_ach0tt::SaphAch0ttSpec>;
#[doc = "Channel 0 Termination Trim"]
pub mod saph_ach0tt;
#[doc = "SAPH_ACH1PUT (rw) register accessor: Channel 1 Pull UpTrim\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_ach1put::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_ach1put::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_ach1put`] module"]
#[doc(alias = "SAPH_ACH1PUT")]
pub type SaphAch1put = crate::Reg<saph_ach1put::SaphAch1putSpec>;
#[doc = "Channel 1 Pull UpTrim"]
pub mod saph_ach1put;
#[doc = "SAPH_ACH1PDT (rw) register accessor: Channel 1 Pull DownTrim\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_ach1pdt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_ach1pdt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_ach1pdt`] module"]
#[doc(alias = "SAPH_ACH1PDT")]
pub type SaphAch1pdt = crate::Reg<saph_ach1pdt::SaphAch1pdtSpec>;
#[doc = "Channel 1 Pull DownTrim"]
pub mod saph_ach1pdt;
#[doc = "SAPH_ACH1TT (rw) register accessor: Channel 1 Termination Trim\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_ach1tt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_ach1tt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_ach1tt`] module"]
#[doc(alias = "SAPH_ACH1TT")]
pub type SaphAch1tt = crate::Reg<saph_ach1tt::SaphAch1ttSpec>;
#[doc = "Channel 1 Termination Trim"]
pub mod saph_ach1tt;
#[doc = "SAPH_AMCNF (rw) register accessor: Mode Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_amcnf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_amcnf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_amcnf`] module"]
#[doc(alias = "SAPH_AMCNF")]
pub type SaphAmcnf = crate::Reg<saph_amcnf::SaphAmcnfSpec>;
#[doc = "Mode Configuration Register"]
pub mod saph_amcnf;
#[doc = "SAPH_ATACTL (rw) register accessor: Trim Access Control\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_atactl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_atactl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_atactl`] module"]
#[doc(alias = "SAPH_ATACTL")]
pub type SaphAtactl = crate::Reg<saph_atactl::SaphAtactlSpec>;
#[doc = "Trim Access Control"]
pub mod saph_atactl;
#[doc = "SAPH_AICTL0 (rw) register accessor: Physical Interface Input Control #0\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aictl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aictl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aictl0`] module"]
#[doc(alias = "SAPH_AICTL0")]
pub type SaphAictl0 = crate::Reg<saph_aictl0::SaphAictl0Spec>;
#[doc = "Physical Interface Input Control #0"]
pub mod saph_aictl0;
#[doc = "SAPH_ABCTL (rw) register accessor: Bias Control\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_abctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_abctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_abctl`] module"]
#[doc(alias = "SAPH_ABCTL")]
pub type SaphAbctl = crate::Reg<saph_abctl::SaphAbctlSpec>;
#[doc = "Bias Control"]
pub mod saph_abctl;
#[doc = "SAPH_APGC (rw) register accessor: PPG Count\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_apgc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_apgc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_apgc`] module"]
#[doc(alias = "SAPH_APGC")]
pub type SaphApgc = crate::Reg<saph_apgc::SaphApgcSpec>;
#[doc = "PPG Count"]
pub mod saph_apgc;
#[doc = "SAPH_APGLPER (rw) register accessor: Pulse Generator Low Period\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_apglper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_apglper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_apglper`] module"]
#[doc(alias = "SAPH_APGLPER")]
pub type SaphApglper = crate::Reg<saph_apglper::SaphApglperSpec>;
#[doc = "Pulse Generator Low Period"]
pub mod saph_apglper;
#[doc = "SAPH_APGHPER (rw) register accessor: Pulse Generator High Period\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_apghper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_apghper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_apghper`] module"]
#[doc(alias = "SAPH_APGHPER")]
pub type SaphApghper = crate::Reg<saph_apghper::SaphApghperSpec>;
#[doc = "Pulse Generator High Period"]
pub mod saph_apghper;
#[doc = "SAPH_APGCTL (rw) register accessor: PPG Control\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_apgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_apgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_apgctl`] module"]
#[doc(alias = "SAPH_APGCTL")]
pub type SaphApgctl = crate::Reg<saph_apgctl::SaphApgctlSpec>;
#[doc = "PPG Control"]
pub mod saph_apgctl;
#[doc = "SAPH_APPGTRIG (rw) register accessor: PPG Software Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_appgtrig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_appgtrig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_appgtrig`] module"]
#[doc(alias = "SAPH_APPGTRIG")]
pub type SaphAppgtrig = crate::Reg<saph_appgtrig::SaphAppgtrigSpec>;
#[doc = "PPG Software Trigger"]
pub mod saph_appgtrig;
#[doc = "SAPH_AXPGCTL (rw) register accessor: Extended Pulse Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_axpgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_axpgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_axpgctl`] module"]
#[doc(alias = "SAPH_AXPGCTL")]
pub type SaphAxpgctl = crate::Reg<saph_axpgctl::SaphAxpgctlSpec>;
#[doc = "Extended Pulse Control Register"]
pub mod saph_axpgctl;
#[doc = "SAPH_AXPGLPER (rw) register accessor: Extra Pulse Low Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_axpglper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_axpglper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_axpglper`] module"]
#[doc(alias = "SAPH_AXPGLPER")]
pub type SaphAxpglper = crate::Reg<saph_axpglper::SaphAxpglperSpec>;
#[doc = "Extra Pulse Low Period Register"]
pub mod saph_axpglper;
#[doc = "SAPH_AXPGHPER (rw) register accessor: Extra Pulse High Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_axpghper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_axpghper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_axpghper`] module"]
#[doc(alias = "SAPH_AXPGHPER")]
pub type SaphAxpghper = crate::Reg<saph_axpghper::SaphAxpghperSpec>;
#[doc = "Extra Pulse High Period Register"]
pub mod saph_axpghper;
#[doc = "SAPH_AASCTL0 (rw) register accessor: A-SEQ control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aasctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aasctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aasctl0`] module"]
#[doc(alias = "SAPH_AASCTL0")]
pub type SaphAasctl0 = crate::Reg<saph_aasctl0::SaphAasctl0Spec>;
#[doc = "A-SEQ control register 0"]
pub mod saph_aasctl0;
#[doc = "SAPH_AASCTL1 (rw) register accessor: A-SEQ control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aasctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aasctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aasctl1`] module"]
#[doc(alias = "SAPH_AASCTL1")]
pub type SaphAasctl1 = crate::Reg<saph_aasctl1::SaphAasctl1Spec>;
#[doc = "A-SEQ control register 1"]
pub mod saph_aasctl1;
#[doc = "SAPH_AAPOL (rw) register accessor: ASQ ping output polarity\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aapol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aapol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aapol`] module"]
#[doc(alias = "SAPH_AAPOL")]
pub type SaphAapol = crate::Reg<saph_aapol::SaphAapolSpec>;
#[doc = "ASQ ping output polarity"]
pub mod saph_aapol;
#[doc = "SAPH_AAPLEV (rw) register accessor: ASQ ping pause level\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aaplev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aaplev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aaplev`] module"]
#[doc(alias = "SAPH_AAPLEV")]
pub type SaphAaplev = crate::Reg<saph_aaplev::SaphAaplevSpec>;
#[doc = "ASQ ping pause level"]
pub mod saph_aaplev;
#[doc = "SAPH_AAPHIZ (rw) register accessor: ASQ ping pause impedance\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aaphiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aaphiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aaphiz`] module"]
#[doc(alias = "SAPH_AAPHIZ")]
pub type SaphAaphiz = crate::Reg<saph_aaphiz::SaphAaphizSpec>;
#[doc = "ASQ ping pause impedance"]
pub mod saph_aaphiz;
#[doc = "SAPH_AATM_A (rw) register accessor: A-SEQ start to 1st ping\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aatm_a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aatm_a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aatm_a`] module"]
#[doc(alias = "SAPH_AATM_A")]
pub type SaphAatmA = crate::Reg<saph_aatm_a::SaphAatmASpec>;
#[doc = "A-SEQ start to 1st ping"]
pub mod saph_aatm_a;
#[doc = "SAPH_AATM_B (rw) register accessor: ASQ start to ADC arm\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aatm_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aatm_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aatm_b`] module"]
#[doc(alias = "SAPH_AATM_B")]
pub type SaphAatmB = crate::Reg<saph_aatm_b::SaphAatmBSpec>;
#[doc = "ASQ start to ADC arm"]
pub mod saph_aatm_b;
#[doc = "SAPH_AATM_C (rw) register accessor: Count for the TIMEMARK C Event\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aatm_c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aatm_c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aatm_c`] module"]
#[doc(alias = "SAPH_AATM_C")]
pub type SaphAatmC = crate::Reg<saph_aatm_c::SaphAatmCSpec>;
#[doc = "Count for the TIMEMARK C Event"]
pub mod saph_aatm_c;
#[doc = "SAPH_AATM_D (rw) register accessor: ASQ start to ADC trig\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aatm_d::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aatm_d::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aatm_d`] module"]
#[doc(alias = "SAPH_AATM_D")]
pub type SaphAatmD = crate::Reg<saph_aatm_d::SaphAatmDSpec>;
#[doc = "ASQ start to ADC trig"]
pub mod saph_aatm_d;
#[doc = "SAPH_AATM_E (rw) register accessor: ASQ start to restart\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aatm_e::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aatm_e::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aatm_e`] module"]
#[doc(alias = "SAPH_AATM_E")]
pub type SaphAatmE = crate::Reg<saph_aatm_e::SaphAatmESpec>;
#[doc = "ASQ start to restart"]
pub mod saph_aatm_e;
#[doc = "SAPH_AATM_F (rw) register accessor: ASQ start to timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aatm_f::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aatm_f::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aatm_f`] module"]
#[doc(alias = "SAPH_AATM_F")]
pub type SaphAatmF = crate::Reg<saph_aatm_f::SaphAatmFSpec>;
#[doc = "ASQ start to timeout"]
pub mod saph_aatm_f;
#[doc = "SAPH_ATBCTL (rw) register accessor: Time Base Control\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_atbctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_atbctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_atbctl`] module"]
#[doc(alias = "SAPH_ATBCTL")]
pub type SaphAtbctl = crate::Reg<saph_atbctl::SaphAtbctlSpec>;
#[doc = "Time Base Control"]
pub mod saph_atbctl;
#[doc = "SAPH_AATIMLO (rw) register accessor: Acquisition Timer Low Part\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aatimlo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aatimlo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aatimlo`] module"]
#[doc(alias = "SAPH_AATIMLO")]
pub type SaphAatimlo = crate::Reg<saph_aatimlo::SaphAatimloSpec>;
#[doc = "Acquisition Timer Low Part"]
pub mod saph_aatimlo;
#[doc = "SAPH_AATIMHI (rw) register accessor: Acquisition Timer High Part\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aatimhi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aatimhi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saph_aatimhi`] module"]
#[doc(alias = "SAPH_AATIMHI")]
pub type SaphAatimhi = crate::Reg<saph_aatimhi::SaphAatimhiSpec>;
#[doc = "Acquisition Timer High Part"]
pub mod saph_aatimhi;
