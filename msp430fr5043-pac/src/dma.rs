#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmactl0: Dmactl0,
    dmactl1: Dmactl1,
    dmactl2: Dmactl2,
    _reserved3: [u8; 0x02],
    dmactl4: Dmactl4,
    _reserved4: [u8; 0x04],
    dmaiv: Dmaiv,
    dma0ctl: Dma0ctl,
    dma0sa: Dma0sa,
    _reserved7: [u8; 0x02],
    dma0da: Dma0da,
    _reserved8: [u8; 0x02],
    dma0sz: Dma0sz,
    _reserved9: [u8; 0x04],
    dma1ctl: Dma1ctl,
    dma1sa: Dma1sa,
    _reserved11: [u8; 0x02],
    dma1da: Dma1da,
    _reserved12: [u8; 0x02],
    dma1sz: Dma1sz,
    _reserved13: [u8; 0x04],
    dma2ctl: Dma2ctl,
    dma2sa: Dma2sa,
    _reserved15: [u8; 0x02],
    dma2da: Dma2da,
    _reserved16: [u8; 0x02],
    dma2sz: Dma2sz,
    _reserved17: [u8; 0x04],
    dma3ctl: Dma3ctl,
    dma3sa: Dma3sa,
    _reserved19: [u8; 0x02],
    dma3da: Dma3da,
    _reserved20: [u8; 0x02],
    dma3sz: Dma3sz,
    _reserved21: [u8; 0x04],
    dma4ctl: Dma4ctl,
    dma4sa: Dma4sa,
    _reserved23: [u8; 0x02],
    dma4da: Dma4da,
    _reserved24: [u8; 0x02],
    dma4sz: Dma4sz,
    _reserved25: [u8; 0x04],
    dma5ctl: Dma5ctl,
    dma5sa: Dma5sa,
    _reserved27: [u8; 0x02],
    dma5da: Dma5da,
    _reserved28: [u8; 0x02],
    dma5sz: Dma5sz,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Control 0"]
    #[inline(always)]
    pub const fn dmactl0(&self) -> &Dmactl0 {
        &self.dmactl0
    }
    #[doc = "0x02 - DMA Control 1"]
    #[inline(always)]
    pub const fn dmactl1(&self) -> &Dmactl1 {
        &self.dmactl1
    }
    #[doc = "0x04 - DMA Control 2"]
    #[inline(always)]
    pub const fn dmactl2(&self) -> &Dmactl2 {
        &self.dmactl2
    }
    #[doc = "0x08 - DMA Control 4"]
    #[inline(always)]
    pub const fn dmactl4(&self) -> &Dmactl4 {
        &self.dmactl4
    }
    #[doc = "0x0e - DMA Interrupt Vector"]
    #[inline(always)]
    pub const fn dmaiv(&self) -> &Dmaiv {
        &self.dmaiv
    }
    #[doc = "0x10 - DMA Channel 0 Control"]
    #[inline(always)]
    pub const fn dma0ctl(&self) -> &Dma0ctl {
        &self.dma0ctl
    }
    #[doc = "0x12 - DMA Channel 0 Source Address"]
    #[inline(always)]
    pub const fn dma0sa(&self) -> &Dma0sa {
        &self.dma0sa
    }
    #[doc = "0x16 - DMA Channel 0 Destination Address"]
    #[inline(always)]
    pub const fn dma0da(&self) -> &Dma0da {
        &self.dma0da
    }
    #[doc = "0x1a - DMA Channel 0 Transfer Size"]
    #[inline(always)]
    pub const fn dma0sz(&self) -> &Dma0sz {
        &self.dma0sz
    }
    #[doc = "0x20 - DMA Channel 1 Control"]
    #[inline(always)]
    pub const fn dma1ctl(&self) -> &Dma1ctl {
        &self.dma1ctl
    }
    #[doc = "0x22 - DMA Channel 1 Source Address"]
    #[inline(always)]
    pub const fn dma1sa(&self) -> &Dma1sa {
        &self.dma1sa
    }
    #[doc = "0x26 - DMA Channel 1 Destination Address"]
    #[inline(always)]
    pub const fn dma1da(&self) -> &Dma1da {
        &self.dma1da
    }
    #[doc = "0x2a - DMA Channel 1 Transfer Size"]
    #[inline(always)]
    pub const fn dma1sz(&self) -> &Dma1sz {
        &self.dma1sz
    }
    #[doc = "0x30 - DMA Channel 2 Control"]
    #[inline(always)]
    pub const fn dma2ctl(&self) -> &Dma2ctl {
        &self.dma2ctl
    }
    #[doc = "0x32 - DMA Channel 2 Source Address"]
    #[inline(always)]
    pub const fn dma2sa(&self) -> &Dma2sa {
        &self.dma2sa
    }
    #[doc = "0x36 - DMA Channel 2 Destination Address"]
    #[inline(always)]
    pub const fn dma2da(&self) -> &Dma2da {
        &self.dma2da
    }
    #[doc = "0x3a - DMA Channel 2 Transfer Size"]
    #[inline(always)]
    pub const fn dma2sz(&self) -> &Dma2sz {
        &self.dma2sz
    }
    #[doc = "0x40 - DMA Channel 3 Control"]
    #[inline(always)]
    pub const fn dma3ctl(&self) -> &Dma3ctl {
        &self.dma3ctl
    }
    #[doc = "0x42 - DMA Channel 3 Source Address"]
    #[inline(always)]
    pub const fn dma3sa(&self) -> &Dma3sa {
        &self.dma3sa
    }
    #[doc = "0x46 - DMA Channel 3 Destination Address"]
    #[inline(always)]
    pub const fn dma3da(&self) -> &Dma3da {
        &self.dma3da
    }
    #[doc = "0x4a - DMA Channel 3 Transfer Size"]
    #[inline(always)]
    pub const fn dma3sz(&self) -> &Dma3sz {
        &self.dma3sz
    }
    #[doc = "0x50 - DMA Channel 4 Control"]
    #[inline(always)]
    pub const fn dma4ctl(&self) -> &Dma4ctl {
        &self.dma4ctl
    }
    #[doc = "0x52 - DMA Channel 4 Source Address"]
    #[inline(always)]
    pub const fn dma4sa(&self) -> &Dma4sa {
        &self.dma4sa
    }
    #[doc = "0x56 - DMA Channel 4 Destination Address"]
    #[inline(always)]
    pub const fn dma4da(&self) -> &Dma4da {
        &self.dma4da
    }
    #[doc = "0x5a - DMA Channel 4 Transfer Size"]
    #[inline(always)]
    pub const fn dma4sz(&self) -> &Dma4sz {
        &self.dma4sz
    }
    #[doc = "0x60 - DMA Channel 5 Control"]
    #[inline(always)]
    pub const fn dma5ctl(&self) -> &Dma5ctl {
        &self.dma5ctl
    }
    #[doc = "0x62 - DMA Channel 5 Source Address"]
    #[inline(always)]
    pub const fn dma5sa(&self) -> &Dma5sa {
        &self.dma5sa
    }
    #[doc = "0x66 - DMA Channel 5 Destination Address"]
    #[inline(always)]
    pub const fn dma5da(&self) -> &Dma5da {
        &self.dma5da
    }
    #[doc = "0x6a - DMA Channel 5 Transfer Size"]
    #[inline(always)]
    pub const fn dma5sz(&self) -> &Dma5sz {
        &self.dma5sz
    }
}
#[doc = "DMACTL0 (rw) register accessor: DMA Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl0`] module"]
#[doc(alias = "DMACTL0")]
pub type Dmactl0 = crate::Reg<dmactl0::Dmactl0Spec>;
#[doc = "DMA Control 0"]
pub mod dmactl0;
#[doc = "DMACTL1 (rw) register accessor: DMA Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl1`] module"]
#[doc(alias = "DMACTL1")]
pub type Dmactl1 = crate::Reg<dmactl1::Dmactl1Spec>;
#[doc = "DMA Control 1"]
pub mod dmactl1;
#[doc = "DMACTL2 (rw) register accessor: DMA Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl2`] module"]
#[doc(alias = "DMACTL2")]
pub type Dmactl2 = crate::Reg<dmactl2::Dmactl2Spec>;
#[doc = "DMA Control 2"]
pub mod dmactl2;
#[doc = "DMACTL4 (rw) register accessor: DMA Control 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl4`] module"]
#[doc(alias = "DMACTL4")]
pub type Dmactl4 = crate::Reg<dmactl4::Dmactl4Spec>;
#[doc = "DMA Control 4"]
pub mod dmactl4;
#[doc = "DMAIV (rw) register accessor: DMA Interrupt Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaiv`] module"]
#[doc(alias = "DMAIV")]
pub type Dmaiv = crate::Reg<dmaiv::DmaivSpec>;
#[doc = "DMA Interrupt Vector"]
pub mod dmaiv;
#[doc = "DMA0CTL (rw) register accessor: DMA Channel 0 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0ctl`] module"]
#[doc(alias = "DMA0CTL")]
pub type Dma0ctl = crate::Reg<dma0ctl::Dma0ctlSpec>;
#[doc = "DMA Channel 0 Control"]
pub mod dma0ctl;
#[doc = "DMA0SA (rw) register accessor: DMA Channel 0 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0sa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0sa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0sa`] module"]
#[doc(alias = "DMA0SA")]
pub type Dma0sa = crate::Reg<dma0sa::Dma0saSpec>;
#[doc = "DMA Channel 0 Source Address"]
pub mod dma0sa;
#[doc = "DMA0DA (rw) register accessor: DMA Channel 0 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0da::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0da::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0da`] module"]
#[doc(alias = "DMA0DA")]
pub type Dma0da = crate::Reg<dma0da::Dma0daSpec>;
#[doc = "DMA Channel 0 Destination Address"]
pub mod dma0da;
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
#[doc = "DMA1SA (rw) register accessor: DMA Channel 1 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1sa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1sa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1sa`] module"]
#[doc(alias = "DMA1SA")]
pub type Dma1sa = crate::Reg<dma1sa::Dma1saSpec>;
#[doc = "DMA Channel 1 Source Address"]
pub mod dma1sa;
#[doc = "DMA1DA (rw) register accessor: DMA Channel 1 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1da::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1da::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1da`] module"]
#[doc(alias = "DMA1DA")]
pub type Dma1da = crate::Reg<dma1da::Dma1daSpec>;
#[doc = "DMA Channel 1 Destination Address"]
pub mod dma1da;
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
#[doc = "DMA2SA (rw) register accessor: DMA Channel 2 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2sa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2sa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2sa`] module"]
#[doc(alias = "DMA2SA")]
pub type Dma2sa = crate::Reg<dma2sa::Dma2saSpec>;
#[doc = "DMA Channel 2 Source Address"]
pub mod dma2sa;
#[doc = "DMA2DA (rw) register accessor: DMA Channel 2 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2da::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2da::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2da`] module"]
#[doc(alias = "DMA2DA")]
pub type Dma2da = crate::Reg<dma2da::Dma2daSpec>;
#[doc = "DMA Channel 2 Destination Address"]
pub mod dma2da;
#[doc = "DMA2SZ (rw) register accessor: DMA Channel 2 Transfer Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2sz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2sz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2sz`] module"]
#[doc(alias = "DMA2SZ")]
pub type Dma2sz = crate::Reg<dma2sz::Dma2szSpec>;
#[doc = "DMA Channel 2 Transfer Size"]
pub mod dma2sz;
#[doc = "DMA3CTL (rw) register accessor: DMA Channel 3 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dma3ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma3ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma3ctl`] module"]
#[doc(alias = "DMA3CTL")]
pub type Dma3ctl = crate::Reg<dma3ctl::Dma3ctlSpec>;
#[doc = "DMA Channel 3 Control"]
pub mod dma3ctl;
#[doc = "DMA3SA (rw) register accessor: DMA Channel 3 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma3sa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma3sa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma3sa`] module"]
#[doc(alias = "DMA3SA")]
pub type Dma3sa = crate::Reg<dma3sa::Dma3saSpec>;
#[doc = "DMA Channel 3 Source Address"]
pub mod dma3sa;
#[doc = "DMA3DA (rw) register accessor: DMA Channel 3 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma3da::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma3da::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma3da`] module"]
#[doc(alias = "DMA3DA")]
pub type Dma3da = crate::Reg<dma3da::Dma3daSpec>;
#[doc = "DMA Channel 3 Destination Address"]
pub mod dma3da;
#[doc = "DMA3SZ (rw) register accessor: DMA Channel 3 Transfer Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dma3sz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma3sz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma3sz`] module"]
#[doc(alias = "DMA3SZ")]
pub type Dma3sz = crate::Reg<dma3sz::Dma3szSpec>;
#[doc = "DMA Channel 3 Transfer Size"]
pub mod dma3sz;
#[doc = "DMA4CTL (rw) register accessor: DMA Channel 4 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dma4ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma4ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma4ctl`] module"]
#[doc(alias = "DMA4CTL")]
pub type Dma4ctl = crate::Reg<dma4ctl::Dma4ctlSpec>;
#[doc = "DMA Channel 4 Control"]
pub mod dma4ctl;
#[doc = "DMA4SA (rw) register accessor: DMA Channel 4 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma4sa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma4sa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma4sa`] module"]
#[doc(alias = "DMA4SA")]
pub type Dma4sa = crate::Reg<dma4sa::Dma4saSpec>;
#[doc = "DMA Channel 4 Source Address"]
pub mod dma4sa;
#[doc = "DMA4DA (rw) register accessor: DMA Channel 4 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma4da::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma4da::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma4da`] module"]
#[doc(alias = "DMA4DA")]
pub type Dma4da = crate::Reg<dma4da::Dma4daSpec>;
#[doc = "DMA Channel 4 Destination Address"]
pub mod dma4da;
#[doc = "DMA4SZ (rw) register accessor: DMA Channel 4 Transfer Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dma4sz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma4sz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma4sz`] module"]
#[doc(alias = "DMA4SZ")]
pub type Dma4sz = crate::Reg<dma4sz::Dma4szSpec>;
#[doc = "DMA Channel 4 Transfer Size"]
pub mod dma4sz;
#[doc = "DMA5CTL (rw) register accessor: DMA Channel 5 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dma5ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma5ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma5ctl`] module"]
#[doc(alias = "DMA5CTL")]
pub type Dma5ctl = crate::Reg<dma5ctl::Dma5ctlSpec>;
#[doc = "DMA Channel 5 Control"]
pub mod dma5ctl;
#[doc = "DMA5SA (rw) register accessor: DMA Channel 5 Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma5sa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma5sa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma5sa`] module"]
#[doc(alias = "DMA5SA")]
pub type Dma5sa = crate::Reg<dma5sa::Dma5saSpec>;
#[doc = "DMA Channel 5 Source Address"]
pub mod dma5sa;
#[doc = "DMA5DA (rw) register accessor: DMA Channel 5 Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dma5da::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma5da::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma5da`] module"]
#[doc(alias = "DMA5DA")]
pub type Dma5da = crate::Reg<dma5da::Dma5daSpec>;
#[doc = "DMA Channel 5 Destination Address"]
pub mod dma5da;
#[doc = "DMA5SZ (rw) register accessor: DMA Channel 5 Transfer Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dma5sz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma5sz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma5sz`] module"]
#[doc(alias = "DMA5SZ")]
pub type Dma5sz = crate::Reg<dma5sz::Dma5szSpec>;
#[doc = "DMA Channel 5 Transfer Size"]
pub mod dma5sz;
