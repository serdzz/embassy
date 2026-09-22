#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ucb0ctl1: Ucb0ctl1,
    ucb0ctl0: Ucb0ctl0,
    ucb0ctlw1: Ucb0ctlw1,
    _reserved3: [u8; 0x02],
    ucb0br0: Ucb0br0,
    ucb0br1: Ucb0br1,
    ucb0stat_i2c: Ucb0statI2c,
    ucb0bcnt_i2c: Ucb0bcntI2c,
    ucb0tbcnt: Ucb0tbcnt,
    ucb0rxbuf: Ucb0rxbuf,
    ucb0txbuf: Ucb0txbuf,
    _reserved10: [u8; 0x04],
    ucb0i2coa0: Ucb0i2coa0,
    ucb0i2coa1: Ucb0i2coa1,
    ucb0i2coa2: Ucb0i2coa2,
    ucb0i2coa3: Ucb0i2coa3,
    ucb0addrx: Ucb0addrx,
    ucb0addmask: Ucb0addmask,
    ucb0i2csa: Ucb0i2csa,
    _reserved17: [u8; 0x08],
    _reserved_17_ucb0: [u8; 0x02],
    _reserved_18_ucb0: [u8; 0x02],
    ucb0iv: Ucb0iv,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 1"]
    #[inline(always)]
    pub const fn ucb0ctl1(&self) -> &Ucb0ctl1 {
        &self.ucb0ctl1
    }
    #[doc = "0x01 - USCI B0 Control Register 0"]
    #[inline(always)]
    pub const fn ucb0ctl0(&self) -> &Ucb0ctl0 {
        &self.ucb0ctl0
    }
    #[doc = "0x02 - USCI B0 Control Word Register 1"]
    #[inline(always)]
    pub const fn ucb0ctlw1(&self) -> &Ucb0ctlw1 {
        &self.ucb0ctlw1
    }
    #[doc = "0x06 - USCI B0 Baud Rate 0"]
    #[inline(always)]
    pub const fn ucb0br0(&self) -> &Ucb0br0 {
        &self.ucb0br0
    }
    #[doc = "0x07 - USCI B0 Baud Rate 1"]
    #[inline(always)]
    pub const fn ucb0br1(&self) -> &Ucb0br1 {
        &self.ucb0br1
    }
    #[doc = "0x08 - USCI B0 Status Register"]
    #[inline(always)]
    pub const fn ucb0stat_i2c(&self) -> &Ucb0statI2c {
        &self.ucb0stat_i2c
    }
    #[doc = "0x09 - USCI B0 Byte Counter Register"]
    #[inline(always)]
    pub const fn ucb0bcnt_i2c(&self) -> &Ucb0bcntI2c {
        &self.ucb0bcnt_i2c
    }
    #[doc = "0x0a - USCI B0 Byte Counter Threshold Register"]
    #[inline(always)]
    pub const fn ucb0tbcnt(&self) -> &Ucb0tbcnt {
        &self.ucb0tbcnt
    }
    #[doc = "0x0c - USCI B0 Receive Buffer"]
    #[inline(always)]
    pub const fn ucb0rxbuf(&self) -> &Ucb0rxbuf {
        &self.ucb0rxbuf
    }
    #[doc = "0x0e - USCI B0 Transmit Buffer"]
    #[inline(always)]
    pub const fn ucb0txbuf(&self) -> &Ucb0txbuf {
        &self.ucb0txbuf
    }
    #[doc = "0x14 - USCI B0 I2C Own Address 0"]
    #[inline(always)]
    pub const fn ucb0i2coa0(&self) -> &Ucb0i2coa0 {
        &self.ucb0i2coa0
    }
    #[doc = "0x16 - USCI B0 I2C Own Address 1"]
    #[inline(always)]
    pub const fn ucb0i2coa1(&self) -> &Ucb0i2coa1 {
        &self.ucb0i2coa1
    }
    #[doc = "0x18 - USCI B0 I2C Own Address 2"]
    #[inline(always)]
    pub const fn ucb0i2coa2(&self) -> &Ucb0i2coa2 {
        &self.ucb0i2coa2
    }
    #[doc = "0x1a - USCI B0 I2C Own Address 3"]
    #[inline(always)]
    pub const fn ucb0i2coa3(&self) -> &Ucb0i2coa3 {
        &self.ucb0i2coa3
    }
    #[doc = "0x1c - USCI B0 Received Address Register"]
    #[inline(always)]
    pub const fn ucb0addrx(&self) -> &Ucb0addrx {
        &self.ucb0addrx
    }
    #[doc = "0x1e - USCI B0 Address Mask Register"]
    #[inline(always)]
    pub const fn ucb0addmask(&self) -> &Ucb0addmask {
        &self.ucb0addmask
    }
    #[doc = "0x20 - USCI B0 I2C Slave Address"]
    #[inline(always)]
    pub const fn ucb0i2csa(&self) -> &Ucb0i2csa {
        &self.ucb0i2csa
    }
    #[doc = "0x2a - USCI B0 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb0ie_i2c(&self) -> &Ucb0ieI2c {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2a - USCI B0 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb0ie(&self) -> &Ucb0ie {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2c - USCI B0 Interrupt Flags Register"]
    #[inline(always)]
    pub const fn ucb0ifg_i2c(&self) -> &Ucb0ifgI2c {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - USCI B0 Interrupt Flags Register"]
    #[inline(always)]
    pub const fn ucb0ifg(&self) -> &Ucb0ifg {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2e - USCI B0 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucb0iv(&self) -> &Ucb0iv {
        &self.ucb0iv
    }
}
#[doc = "UCB0CTL1 (rw) register accessor: USCI B0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl1`] module"]
#[doc(alias = "UCB0CTL1")]
pub type Ucb0ctl1 = crate::Reg<ucb0ctl1::Ucb0ctl1Spec>;
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1;
#[doc = "UCB0CTL0 (rw) register accessor: USCI B0 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl0`] module"]
#[doc(alias = "UCB0CTL0")]
pub type Ucb0ctl0 = crate::Reg<ucb0ctl0::Ucb0ctl0Spec>;
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0;
#[doc = "UCB0BR0 (rw) register accessor: USCI B0 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0br0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0br0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0br0`] module"]
#[doc(alias = "UCB0BR0")]
pub type Ucb0br0 = crate::Reg<ucb0br0::Ucb0br0Spec>;
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0;
#[doc = "UCB0BR1 (rw) register accessor: USCI B0 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0br1`] module"]
#[doc(alias = "UCB0BR1")]
pub type Ucb0br1 = crate::Reg<ucb0br1::Ucb0br1Spec>;
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1;
#[doc = "UCB0STAT_I2C (rw) register accessor: USCI B0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0stat_i2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0stat_i2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0stat_i2c`] module"]
#[doc(alias = "UCB0STAT_I2C")]
pub type Ucb0statI2c = crate::Reg<ucb0stat_i2c::Ucb0statI2cSpec>;
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat_i2c;
#[doc = "UCB0BCNT_I2C (rw) register accessor: USCI B0 Byte Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0bcnt_i2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0bcnt_i2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0bcnt_i2c`] module"]
#[doc(alias = "UCB0BCNT_I2C")]
pub type Ucb0bcntI2c = crate::Reg<ucb0bcnt_i2c::Ucb0bcntI2cSpec>;
#[doc = "USCI B0 Byte Counter Register"]
pub mod ucb0bcnt_i2c;
#[doc = "UCB0CTLW1 (rw) register accessor: USCI B0 Control Word Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctlw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctlw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctlw1`] module"]
#[doc(alias = "UCB0CTLW1")]
pub type Ucb0ctlw1 = crate::Reg<ucb0ctlw1::Ucb0ctlw1Spec>;
#[doc = "USCI B0 Control Word Register 1"]
pub mod ucb0ctlw1;
#[doc = "UCB0TBCNT (rw) register accessor: USCI B0 Byte Counter Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0tbcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0tbcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0tbcnt`] module"]
#[doc(alias = "UCB0TBCNT")]
pub type Ucb0tbcnt = crate::Reg<ucb0tbcnt::Ucb0tbcntSpec>;
#[doc = "USCI B0 Byte Counter Threshold Register"]
pub mod ucb0tbcnt;
#[doc = "UCB0RXBUF (rw) register accessor: USCI B0 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0rxbuf`] module"]
#[doc(alias = "UCB0RXBUF")]
pub type Ucb0rxbuf = crate::Reg<ucb0rxbuf::Ucb0rxbufSpec>;
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf;
#[doc = "UCB0TXBUF (rw) register accessor: USCI B0 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0txbuf`] module"]
#[doc(alias = "UCB0TXBUF")]
pub type Ucb0txbuf = crate::Reg<ucb0txbuf::Ucb0txbufSpec>;
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf;
#[doc = "UCB0I2COA0 (rw) register accessor: USCI B0 I2C Own Address 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2coa0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2coa0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2coa0`] module"]
#[doc(alias = "UCB0I2COA0")]
pub type Ucb0i2coa0 = crate::Reg<ucb0i2coa0::Ucb0i2coa0Spec>;
#[doc = "USCI B0 I2C Own Address 0"]
pub mod ucb0i2coa0;
#[doc = "UCB0I2COA1 (rw) register accessor: USCI B0 I2C Own Address 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2coa1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2coa1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2coa1`] module"]
#[doc(alias = "UCB0I2COA1")]
pub type Ucb0i2coa1 = crate::Reg<ucb0i2coa1::Ucb0i2coa1Spec>;
#[doc = "USCI B0 I2C Own Address 1"]
pub mod ucb0i2coa1;
#[doc = "UCB0I2COA2 (rw) register accessor: USCI B0 I2C Own Address 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2coa2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2coa2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2coa2`] module"]
#[doc(alias = "UCB0I2COA2")]
pub type Ucb0i2coa2 = crate::Reg<ucb0i2coa2::Ucb0i2coa2Spec>;
#[doc = "USCI B0 I2C Own Address 2"]
pub mod ucb0i2coa2;
#[doc = "UCB0I2COA3 (rw) register accessor: USCI B0 I2C Own Address 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2coa3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2coa3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2coa3`] module"]
#[doc(alias = "UCB0I2COA3")]
pub type Ucb0i2coa3 = crate::Reg<ucb0i2coa3::Ucb0i2coa3Spec>;
#[doc = "USCI B0 I2C Own Address 3"]
pub mod ucb0i2coa3;
#[doc = "UCB0ADDRX (rw) register accessor: USCI B0 Received Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0addrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0addrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0addrx`] module"]
#[doc(alias = "UCB0ADDRX")]
pub type Ucb0addrx = crate::Reg<ucb0addrx::Ucb0addrxSpec>;
#[doc = "USCI B0 Received Address Register"]
pub mod ucb0addrx;
#[doc = "UCB0ADDMASK (rw) register accessor: USCI B0 Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0addmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0addmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0addmask`] module"]
#[doc(alias = "UCB0ADDMASK")]
pub type Ucb0addmask = crate::Reg<ucb0addmask::Ucb0addmaskSpec>;
#[doc = "USCI B0 Address Mask Register"]
pub mod ucb0addmask;
#[doc = "UCB0I2CSA (rw) register accessor: USCI B0 I2C Slave Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2csa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2csa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2csa`] module"]
#[doc(alias = "UCB0I2CSA")]
pub type Ucb0i2csa = crate::Reg<ucb0i2csa::Ucb0i2csaSpec>;
#[doc = "USCI B0 I2C Slave Address"]
pub mod ucb0i2csa;
#[doc = "UCB0IE (rw) register accessor: USCI B0 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ie`] module"]
#[doc(alias = "UCB0IE")]
pub type Ucb0ie = crate::Reg<ucb0ie::Ucb0ieSpec>;
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie;
#[doc = "UCB0IE_I2C (rw) register accessor: USCI B0 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ie_i2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ie_i2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ie_i2c`] module"]
#[doc(alias = "UCB0IE_I2C")]
pub type Ucb0ieI2c = crate::Reg<ucb0ie_i2c::Ucb0ieI2cSpec>;
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie_i2c;
#[doc = "UCB0IFG (rw) register accessor: USCI B0 Interrupt Flags Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ifg`] module"]
#[doc(alias = "UCB0IFG")]
pub type Ucb0ifg = crate::Reg<ucb0ifg::Ucb0ifgSpec>;
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg;
#[doc = "UCB0IFG_I2C (rw) register accessor: USCI B0 Interrupt Flags Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ifg_i2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ifg_i2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ifg_i2c`] module"]
#[doc(alias = "UCB0IFG_I2C")]
pub type Ucb0ifgI2c = crate::Reg<ucb0ifg_i2c::Ucb0ifgI2cSpec>;
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg_i2c;
#[doc = "UCB0IV (rw) register accessor: USCI B0 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0iv`] module"]
#[doc(alias = "UCB0IV")]
pub type Ucb0iv = crate::Reg<ucb0iv::Ucb0ivSpec>;
#[doc = "USCI B0 Interrupt Vector Register"]
pub mod ucb0iv;
