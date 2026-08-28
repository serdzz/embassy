#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmactl0: Dmactl0,
    dmactl1: Dmactl1,
    dmaiv: Dmaiv,
    _reserved3: [u8; 0xa8],
    dma0ctl: Dma0ctl,
    dma0sal: Dma0sal,
    _reserved5: [u8; 0x02],
    dma0dal: Dma0dal,
    _reserved6: [u8; 0x02],
    dma0sz: Dma0sz,
    dma1ctl: Dma1ctl,
    dma1sal: Dma1sal,
    _reserved9: [u8; 0x02],
    dma1dal: Dma1dal,
    _reserved10: [u8; 0x02],
    dma1sz: Dma1sz,
    dma2ctl: Dma2ctl,
    dma2sal: Dma2sal,
    _reserved13: [u8; 0x02],
    dma2dal: Dma2dal,
    _reserved14: [u8; 0x02],
    dma2sz: Dma2sz,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Module Control 0"]
    #[inline(always)]
    pub const fn dmactl0(&self) -> &Dmactl0 {
        &self.dmactl0
    }
    #[doc = "0x02 - DMA Module Control 1"]
    #[inline(always)]
    pub const fn dmactl1(&self) -> &Dmactl1 {
        &self.dmactl1
    }
    #[doc = "0x04 - DMA Interrupt Vector Word"]
    #[inline(always)]
    pub const fn dmaiv(&self) -> &Dmaiv {
        &self.dmaiv
    }
    #[doc = "0xae - DMA Channel 0 Control"]
    #[inline(always)]
    pub const fn dma0ctl(&self) -> &Dma0ctl {
        &self.dma0ctl
    }
    #[doc = "0xb0 - DMA Channel 0 Source Address"]
    #[inline(always)]
    pub const fn dma0sal(&self) -> &Dma0sal {
        &self.dma0sal
    }
    #[doc = "0xb4 - DMA Channel 0 Destination Address"]
    #[inline(always)]
    pub const fn dma0dal(&self) -> &Dma0dal {
        &self.dma0dal
    }
    #[doc = "0xb8 - DMA Channel 0 Transfer Size"]
    #[inline(always)]
    pub const fn dma0sz(&self) -> &Dma0sz {
        &self.dma0sz
    }
    #[doc = "0xba - DMA Channel 1 Control"]
    #[inline(always)]
    pub const fn dma1ctl(&self) -> &Dma1ctl {
        &self.dma1ctl
    }
    #[doc = "0xbc - DMA Channel 1 Source Address"]
    #[inline(always)]
    pub const fn dma1sal(&self) -> &Dma1sal {
        &self.dma1sal
    }
    #[doc = "0xc0 - DMA Channel 1 Destination Address"]
    #[inline(always)]
    pub const fn dma1dal(&self) -> &Dma1dal {
        &self.dma1dal
    }
    #[doc = "0xc4 - DMA Channel 1 Transfer Size"]
    #[inline(always)]
    pub const fn dma1sz(&self) -> &Dma1sz {
        &self.dma1sz
    }
    #[doc = "0xc6 - DMA Channel 2 Control"]
    #[inline(always)]
    pub const fn dma2ctl(&self) -> &Dma2ctl {
        &self.dma2ctl
    }
    #[doc = "0xc8 - DMA Channel 2 Source Address"]
    #[inline(always)]
    pub const fn dma2sal(&self) -> &Dma2sal {
        &self.dma2sal
    }
    #[doc = "0xcc - DMA Channel 2 Destination Address"]
    #[inline(always)]
    pub const fn dma2dal(&self) -> &Dma2dal {
        &self.dma2dal
    }
    #[doc = "0xd0 - DMA Channel 2 Transfer Size"]
    #[inline(always)]
    pub const fn dma2sz(&self) -> &Dma2sz {
        &self.dma2sz
    }
}
#[doc = "DMACTL0 (rw) register accessor: DMA Module Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl0`] module"]
#[doc(alias = "DMACTL0")]
pub type Dmactl0 = crate::Reg<dmactl0::Dmactl0Spec>;
#[doc = "DMA Module Control 0"]
pub mod dmactl0;
#[doc = "DMACTL1 (rw) register accessor: DMA Module Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl1`] module"]
#[doc(alias = "DMACTL1")]
pub type Dmactl1 = crate::Reg<dmactl1::Dmactl1Spec>;
#[doc = "DMA Module Control 1"]
pub mod dmactl1;
#[doc = "DMAIV (rw) register accessor: DMA Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaiv`] module"]
#[doc(alias = "DMAIV")]
pub type Dmaiv = crate::Reg<dmaiv::DmaivSpec>;
#[doc = "DMA Interrupt Vector Word"]
pub mod dmaiv;
#[doc = "DMA0CTL (rw) register accessor: DMA Channel 0 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0ctl`] module"]
#[doc(alias = "DMA0CTL")]
pub type Dma0ctl = crate::Reg<dma0ctl::Dma0ctlSpec>;
#[doc = "DMA Channel 0 Control"]
pub mod dma0ctl;
#[doc = "DMA0SAL (rw) register accessor: DMA Channel 0 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0sal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0sal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0sal`] module"]
#[doc(alias = "DMA0SAL")]
pub type Dma0sal = crate::Reg<dma0sal::Dma0salSpec>;
#[doc = "DMA Channel 0 Source Address"]
pub mod dma0sal;
#[doc = "DMA0DAL (rw) register accessor: DMA Channel 0 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0dal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0dal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0dal`] module"]
#[doc(alias = "DMA0DAL")]
pub type Dma0dal = crate::Reg<dma0dal::Dma0dalSpec>;
#[doc = "DMA Channel 0 Destination Address"]
pub mod dma0dal;
#[doc = "DMA0SZ (rw) register accessor: DMA Channel 0 Transfer Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0sz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0sz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0sz`] module"]
#[doc(alias = "DMA0SZ")]
pub type Dma0sz = crate::Reg<dma0sz::Dma0szSpec>;
#[doc = "DMA Channel 0 Transfer Size"]
pub mod dma0sz;
#[doc = "DMA1CTL (rw) register accessor: DMA Channel 1 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1ctl`] module"]
#[doc(alias = "DMA1CTL")]
pub type Dma1ctl = crate::Reg<dma1ctl::Dma1ctlSpec>;
#[doc = "DMA Channel 1 Control"]
pub mod dma1ctl;
#[doc = "DMA1SAL (rw) register accessor: DMA Channel 1 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1sal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1sal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1sal`] module"]
#[doc(alias = "DMA1SAL")]
pub type Dma1sal = crate::Reg<dma1sal::Dma1salSpec>;
#[doc = "DMA Channel 1 Source Address"]
pub mod dma1sal;
#[doc = "DMA1DAL (rw) register accessor: DMA Channel 1 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1dal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1dal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1dal`] module"]
#[doc(alias = "DMA1DAL")]
pub type Dma1dal = crate::Reg<dma1dal::Dma1dalSpec>;
#[doc = "DMA Channel 1 Destination Address"]
pub mod dma1dal;
#[doc = "DMA1SZ (rw) register accessor: DMA Channel 1 Transfer Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1sz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1sz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1sz`] module"]
#[doc(alias = "DMA1SZ")]
pub type Dma1sz = crate::Reg<dma1sz::Dma1szSpec>;
#[doc = "DMA Channel 1 Transfer Size"]
pub mod dma1sz;
#[doc = "DMA2CTL (rw) register accessor: DMA Channel 2 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2ctl`] module"]
#[doc(alias = "DMA2CTL")]
pub type Dma2ctl = crate::Reg<dma2ctl::Dma2ctlSpec>;
#[doc = "DMA Channel 2 Control"]
pub mod dma2ctl;
#[doc = "DMA2SAL (rw) register accessor: DMA Channel 2 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2sal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2sal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2sal`] module"]
#[doc(alias = "DMA2SAL")]
pub type Dma2sal = crate::Reg<dma2sal::Dma2salSpec>;
#[doc = "DMA Channel 2 Source Address"]
pub mod dma2sal;
#[doc = "DMA2DAL (rw) register accessor: DMA Channel 2 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2dal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2dal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2dal`] module"]
#[doc(alias = "DMA2DAL")]
pub type Dma2dal = crate::Reg<dma2dal::Dma2dalSpec>;
#[doc = "DMA Channel 2 Destination Address"]
pub mod dma2dal;
#[doc = "DMA2SZ (rw) register accessor: DMA Channel 2 Transfer Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2sz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2sz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2sz`] module"]
#[doc(alias = "DMA2SZ")]
pub type Dma2sz = crate::Reg<dma2sz::Dma2szSpec>;
#[doc = "DMA Channel 2 Transfer Size"]
pub mod dma2sz;
