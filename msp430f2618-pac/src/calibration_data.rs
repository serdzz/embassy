#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    caldco_16mhz: Caldco16mhz,
    calbc1_16mhz: Calbc1_16mhz,
    caldco_12mhz: Caldco12mhz,
    calbc1_12mhz: Calbc1_12mhz,
    caldco_8mhz: Caldco8mhz,
    calbc1_8mhz: Calbc1_8mhz,
    caldco_1mhz: Caldco1mhz,
    calbc1_1mhz: Calbc1_1mhz,
}
impl RegisterBlock {
    #[doc = "0x00 - DCOCTL Calibration Data for 16MHz"]
    #[inline(always)]
    pub const fn caldco_16mhz(&self) -> &Caldco16mhz {
        &self.caldco_16mhz
    }
    #[doc = "0x01 - BCSCTL1 Calibration Data for 16MHz"]
    #[inline(always)]
    pub const fn calbc1_16mhz(&self) -> &Calbc1_16mhz {
        &self.calbc1_16mhz
    }
    #[doc = "0x02 - DCOCTL Calibration Data for 12MHz"]
    #[inline(always)]
    pub const fn caldco_12mhz(&self) -> &Caldco12mhz {
        &self.caldco_12mhz
    }
    #[doc = "0x03 - BCSCTL1 Calibration Data for 12MHz"]
    #[inline(always)]
    pub const fn calbc1_12mhz(&self) -> &Calbc1_12mhz {
        &self.calbc1_12mhz
    }
    #[doc = "0x04 - DCOCTL Calibration Data for 8MHz"]
    #[inline(always)]
    pub const fn caldco_8mhz(&self) -> &Caldco8mhz {
        &self.caldco_8mhz
    }
    #[doc = "0x05 - BCSCTL1 Calibration Data for 8MHz"]
    #[inline(always)]
    pub const fn calbc1_8mhz(&self) -> &Calbc1_8mhz {
        &self.calbc1_8mhz
    }
    #[doc = "0x06 - DCOCTL Calibration Data for 1MHz"]
    #[inline(always)]
    pub const fn caldco_1mhz(&self) -> &Caldco1mhz {
        &self.caldco_1mhz
    }
    #[doc = "0x07 - BCSCTL1 Calibration Data for 1MHz"]
    #[inline(always)]
    pub const fn calbc1_1mhz(&self) -> &Calbc1_1mhz {
        &self.calbc1_1mhz
    }
}
#[doc = "CALDCO_16MHZ (rw) register accessor: DCOCTL Calibration Data for 16MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_16mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_16mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caldco_16mhz`] module"]
#[doc(alias = "CALDCO_16MHZ")]
pub type Caldco16mhz = crate::Reg<caldco_16mhz::Caldco16mhzSpec>;
#[doc = "DCOCTL Calibration Data for 16MHz"]
pub mod caldco_16mhz;
#[doc = "CALBC1_16MHZ (rw) register accessor: BCSCTL1 Calibration Data for 16MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`calbc1_16mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calbc1_16mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calbc1_16mhz`] module"]
#[doc(alias = "CALBC1_16MHZ")]
pub type Calbc1_16mhz = crate::Reg<calbc1_16mhz::Calbc1_16mhzSpec>;
#[doc = "BCSCTL1 Calibration Data for 16MHz"]
pub mod calbc1_16mhz;
#[doc = "CALDCO_12MHZ (rw) register accessor: DCOCTL Calibration Data for 12MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_12mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_12mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caldco_12mhz`] module"]
#[doc(alias = "CALDCO_12MHZ")]
pub type Caldco12mhz = crate::Reg<caldco_12mhz::Caldco12mhzSpec>;
#[doc = "DCOCTL Calibration Data for 12MHz"]
pub mod caldco_12mhz;
#[doc = "CALBC1_12MHZ (rw) register accessor: BCSCTL1 Calibration Data for 12MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`calbc1_12mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calbc1_12mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calbc1_12mhz`] module"]
#[doc(alias = "CALBC1_12MHZ")]
pub type Calbc1_12mhz = crate::Reg<calbc1_12mhz::Calbc1_12mhzSpec>;
#[doc = "BCSCTL1 Calibration Data for 12MHz"]
pub mod calbc1_12mhz;
#[doc = "CALDCO_8MHZ (rw) register accessor: DCOCTL Calibration Data for 8MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_8mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_8mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caldco_8mhz`] module"]
#[doc(alias = "CALDCO_8MHZ")]
pub type Caldco8mhz = crate::Reg<caldco_8mhz::Caldco8mhzSpec>;
#[doc = "DCOCTL Calibration Data for 8MHz"]
pub mod caldco_8mhz;
#[doc = "CALBC1_8MHZ (rw) register accessor: BCSCTL1 Calibration Data for 8MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`calbc1_8mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calbc1_8mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calbc1_8mhz`] module"]
#[doc(alias = "CALBC1_8MHZ")]
pub type Calbc1_8mhz = crate::Reg<calbc1_8mhz::Calbc1_8mhzSpec>;
#[doc = "BCSCTL1 Calibration Data for 8MHz"]
pub mod calbc1_8mhz;
#[doc = "CALDCO_1MHZ (rw) register accessor: DCOCTL Calibration Data for 1MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_1mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_1mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caldco_1mhz`] module"]
#[doc(alias = "CALDCO_1MHZ")]
pub type Caldco1mhz = crate::Reg<caldco_1mhz::Caldco1mhzSpec>;
#[doc = "DCOCTL Calibration Data for 1MHz"]
pub mod caldco_1mhz;
#[doc = "CALBC1_1MHZ (rw) register accessor: BCSCTL1 Calibration Data for 1MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`calbc1_1mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calbc1_1mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calbc1_1mhz`] module"]
#[doc(alias = "CALBC1_1MHZ")]
pub type Calbc1_1mhz = crate::Reg<calbc1_1mhz::Calbc1_1mhzSpec>;
#[doc = "BCSCTL1 Calibration Data for 1MHz"]
pub mod calbc1_1mhz;
