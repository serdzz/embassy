#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dac12_0ctl: Dac12_0ctl,
    dac12_1ctl: Dac12_1ctl,
    _reserved2: [u8; 0x04],
    dac12_0dat: Dac12_0dat,
    dac12_1dat: Dac12_1dat,
}
impl RegisterBlock {
    #[doc = "0x00 - DAC12_0 Control"]
    #[inline(always)]
    pub const fn dac12_0ctl(&self) -> &Dac12_0ctl {
        &self.dac12_0ctl
    }
    #[doc = "0x02 - DAC12_1 Control"]
    #[inline(always)]
    pub const fn dac12_1ctl(&self) -> &Dac12_1ctl {
        &self.dac12_1ctl
    }
    #[doc = "0x08 - DAC12_0 Data"]
    #[inline(always)]
    pub const fn dac12_0dat(&self) -> &Dac12_0dat {
        &self.dac12_0dat
    }
    #[doc = "0x0a - DAC12_1 Data"]
    #[inline(always)]
    pub const fn dac12_1dat(&self) -> &Dac12_1dat {
        &self.dac12_1dat
    }
}
#[doc = "DAC12_0CTL (rw) register accessor: DAC12_0 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dac12_0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac12_0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac12_0ctl`] module"]
#[doc(alias = "DAC12_0CTL")]
pub type Dac12_0ctl = crate::Reg<dac12_0ctl::Dac12_0ctlSpec>;
#[doc = "DAC12_0 Control"]
pub mod dac12_0ctl;
#[doc = "DAC12_1CTL (rw) register accessor: DAC12_1 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dac12_1ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac12_1ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac12_1ctl`] module"]
#[doc(alias = "DAC12_1CTL")]
pub type Dac12_1ctl = crate::Reg<dac12_1ctl::Dac12_1ctlSpec>;
#[doc = "DAC12_1 Control"]
pub mod dac12_1ctl;
#[doc = "DAC12_0DAT (rw) register accessor: DAC12_0 Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dac12_0dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac12_0dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac12_0dat`] module"]
#[doc(alias = "DAC12_0DAT")]
pub type Dac12_0dat = crate::Reg<dac12_0dat::Dac12_0datSpec>;
#[doc = "DAC12_0 Data"]
pub mod dac12_0dat;
#[doc = "DAC12_1DAT (rw) register accessor: DAC12_1 Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dac12_1dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac12_1dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac12_1dat`] module"]
#[doc(alias = "DAC12_1DAT")]
pub type Dac12_1dat = crate::Reg<dac12_1dat::Dac12_1datSpec>;
#[doc = "DAC12_1 Data"]
pub mod dac12_1dat;
