#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sysctl: Sysctl,
    sysbslc: Sysbslc,
    _reserved2: [u8; 0x02],
    sysjmbc: Sysjmbc,
    sysjmbi0: Sysjmbi0,
    sysjmbi1: Sysjmbi1,
    sysjmbo0: Sysjmbo0,
    sysjmbo1: Sysjmbo1,
    _reserved7: [u8; 0x08],
    sysberriv: Sysberriv,
    sysuniv: Sysuniv,
    syssniv: Syssniv,
    sysrstiv: Sysrstiv,
    syscfg0: Syscfg0,
    syscfg1: Syscfg1,
    syscfg2: Syscfg2,
}
impl RegisterBlock {
    #[doc = "0x00 - System control"]
    #[inline(always)]
    pub const fn sysctl(&self) -> &Sysctl {
        &self.sysctl
    }
    #[doc = "0x02 - Boot strap configuration area"]
    #[inline(always)]
    pub const fn sysbslc(&self) -> &Sysbslc {
        &self.sysbslc
    }
    #[doc = "0x06 - JTAG mailbox control"]
    #[inline(always)]
    pub const fn sysjmbc(&self) -> &Sysjmbc {
        &self.sysjmbc
    }
    #[doc = "0x08 - JTAG mailbox input 0"]
    #[inline(always)]
    pub const fn sysjmbi0(&self) -> &Sysjmbi0 {
        &self.sysjmbi0
    }
    #[doc = "0x0a - JTAG mailbox input 1"]
    #[inline(always)]
    pub const fn sysjmbi1(&self) -> &Sysjmbi1 {
        &self.sysjmbi1
    }
    #[doc = "0x0c - JTAG mailbox output 0"]
    #[inline(always)]
    pub const fn sysjmbo0(&self) -> &Sysjmbo0 {
        &self.sysjmbo0
    }
    #[doc = "0x0e - JTAG mailbox output 1"]
    #[inline(always)]
    pub const fn sysjmbo1(&self) -> &Sysjmbo1 {
        &self.sysjmbo1
    }
    #[doc = "0x18 - Bus Error vector generator"]
    #[inline(always)]
    pub const fn sysberriv(&self) -> &Sysberriv {
        &self.sysberriv
    }
    #[doc = "0x1a - User NMI vector generator"]
    #[inline(always)]
    pub const fn sysuniv(&self) -> &Sysuniv {
        &self.sysuniv
    }
    #[doc = "0x1c - System NMI vector generator"]
    #[inline(always)]
    pub const fn syssniv(&self) -> &Syssniv {
        &self.syssniv
    }
    #[doc = "0x1e - Reset vector generator"]
    #[inline(always)]
    pub const fn sysrstiv(&self) -> &Sysrstiv {
        &self.sysrstiv
    }
    #[doc = "0x20 - System Configuration 0"]
    #[inline(always)]
    pub const fn syscfg0(&self) -> &Syscfg0 {
        &self.syscfg0
    }
    #[doc = "0x22 - System Configuration 1"]
    #[inline(always)]
    pub const fn syscfg1(&self) -> &Syscfg1 {
        &self.syscfg1
    }
    #[doc = "0x24 - System Configuration 2"]
    #[inline(always)]
    pub const fn syscfg2(&self) -> &Syscfg2 {
        &self.syscfg2
    }
}
#[doc = "SYSCTL (rw) register accessor: System control\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl`] module"]
#[doc(alias = "SYSCTL")]
pub type Sysctl = crate::Reg<sysctl::SysctlSpec>;
#[doc = "System control"]
pub mod sysctl;
#[doc = "SYSBSLC (rw) register accessor: Boot strap configuration area\n\nYou can [`read`](crate::Reg::read) this register and get [`sysbslc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysbslc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysbslc`] module"]
#[doc(alias = "SYSBSLC")]
pub type Sysbslc = crate::Reg<sysbslc::SysbslcSpec>;
#[doc = "Boot strap configuration area"]
pub mod sysbslc;
#[doc = "SYSJMBC (rw) register accessor: JTAG mailbox control\n\nYou can [`read`](crate::Reg::read) this register and get [`sysjmbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysjmbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysjmbc`] module"]
#[doc(alias = "SYSJMBC")]
pub type Sysjmbc = crate::Reg<sysjmbc::SysjmbcSpec>;
#[doc = "JTAG mailbox control"]
pub mod sysjmbc;
#[doc = "SYSJMBI0 (rw) register accessor: JTAG mailbox input 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sysjmbi0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysjmbi0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysjmbi0`] module"]
#[doc(alias = "SYSJMBI0")]
pub type Sysjmbi0 = crate::Reg<sysjmbi0::Sysjmbi0Spec>;
#[doc = "JTAG mailbox input 0"]
pub mod sysjmbi0;
#[doc = "SYSJMBI1 (rw) register accessor: JTAG mailbox input 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sysjmbi1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysjmbi1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysjmbi1`] module"]
#[doc(alias = "SYSJMBI1")]
pub type Sysjmbi1 = crate::Reg<sysjmbi1::Sysjmbi1Spec>;
#[doc = "JTAG mailbox input 1"]
pub mod sysjmbi1;
#[doc = "SYSJMBO0 (rw) register accessor: JTAG mailbox output 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sysjmbo0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysjmbo0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysjmbo0`] module"]
#[doc(alias = "SYSJMBO0")]
pub type Sysjmbo0 = crate::Reg<sysjmbo0::Sysjmbo0Spec>;
#[doc = "JTAG mailbox output 0"]
pub mod sysjmbo0;
#[doc = "SYSJMBO1 (rw) register accessor: JTAG mailbox output 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sysjmbo1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysjmbo1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysjmbo1`] module"]
#[doc(alias = "SYSJMBO1")]
pub type Sysjmbo1 = crate::Reg<sysjmbo1::Sysjmbo1Spec>;
#[doc = "JTAG mailbox output 1"]
pub mod sysjmbo1;
#[doc = "SYSBERRIV (rw) register accessor: Bus Error vector generator\n\nYou can [`read`](crate::Reg::read) this register and get [`sysberriv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysberriv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysberriv`] module"]
#[doc(alias = "SYSBERRIV")]
pub type Sysberriv = crate::Reg<sysberriv::SysberrivSpec>;
#[doc = "Bus Error vector generator"]
pub mod sysberriv;
#[doc = "SYSUNIV (rw) register accessor: User NMI vector generator\n\nYou can [`read`](crate::Reg::read) this register and get [`sysuniv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysuniv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysuniv`] module"]
#[doc(alias = "SYSUNIV")]
pub type Sysuniv = crate::Reg<sysuniv::SysunivSpec>;
#[doc = "User NMI vector generator"]
pub mod sysuniv;
#[doc = "SYSSNIV (rw) register accessor: System NMI vector generator\n\nYou can [`read`](crate::Reg::read) this register and get [`syssniv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syssniv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syssniv`] module"]
#[doc(alias = "SYSSNIV")]
pub type Syssniv = crate::Reg<syssniv::SyssnivSpec>;
#[doc = "System NMI vector generator"]
pub mod syssniv;
#[doc = "SYSRSTIV (rw) register accessor: Reset vector generator\n\nYou can [`read`](crate::Reg::read) this register and get [`sysrstiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysrstiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysrstiv`] module"]
#[doc(alias = "SYSRSTIV")]
pub type Sysrstiv = crate::Reg<sysrstiv::SysrstivSpec>;
#[doc = "Reset vector generator"]
pub mod sysrstiv;
#[doc = "SYSCFG0 (rw) register accessor: System Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg0`] module"]
#[doc(alias = "SYSCFG0")]
pub type Syscfg0 = crate::Reg<syscfg0::Syscfg0Spec>;
#[doc = "System Configuration 0"]
pub mod syscfg0;
#[doc = "SYSCFG1 (rw) register accessor: System Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg1`] module"]
#[doc(alias = "SYSCFG1")]
pub type Syscfg1 = crate::Reg<syscfg1::Syscfg1Spec>;
#[doc = "System Configuration 1"]
pub mod syscfg1;
#[doc = "SYSCFG2 (rw) register accessor: System Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg2`] module"]
#[doc(alias = "SYSCFG2")]
pub type Syscfg2 = crate::Reg<syscfg2::Syscfg2Spec>;
#[doc = "System Configuration 2"]
pub mod syscfg2;
