#[doc = "Register `DMAIV` reader"]
pub type R = crate::R<DmaivSpec>;
#[doc = "Register `DMAIV` writer"]
pub type W = crate::W<DmaivSpec>;
#[doc = "DMA interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Dmaiv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: DMA channel 0; Interrupt Flag: DMA0IFG; Interrupt Priority: Highest"]
    Dma0ifg = 2,
    #[doc = "4: Interrupt Source: DMA channel 1; Interrupt Flag: DMA1IFG"]
    Dma1ifg = 4,
    #[doc = "6: Interrupt Source: DMA channel 2; Interrupt Flag: DMA2IFG"]
    Dma2ifg = 6,
    #[doc = "8: Interrupt Source: DMA channel 3; Interrupt Flag: DMA3IFG"]
    Dma3ifg = 8,
    #[doc = "10: Interrupt Source: DMA channel 4; Interrupt Flag: DMA4IFG"]
    Dma4ifg = 10,
    #[doc = "12: Interrupt Source: DMA channel 5; Interrupt Flag: DMA5IFG"]
    Dma5ifg = 12,
    #[doc = "14: Interrupt Source: DMA channel 6; Interrupt Flag: DMA6IFG"]
    Dma6ifg = 14,
    #[doc = "16: Interrupt Source: DMA channel 7; Interrupt Flag: DMA7IFG; Interrupt Priority: Lowest"]
    Dma7ifg = 16,
}
impl From<Dmaiv> for u16 {
    #[inline(always)]
    fn from(variant: Dmaiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmaiv {
    type Ux = u16;
}
impl crate::IsEnum for Dmaiv {}
#[doc = "Field `DMAIV` reader - DMA interrupt vector value"]
pub type DmaivR = crate::FieldReader<Dmaiv>;
impl DmaivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmaiv> {
        match self.bits {
            0 => Some(Dmaiv::None),
            2 => Some(Dmaiv::Dma0ifg),
            4 => Some(Dmaiv::Dma1ifg),
            6 => Some(Dmaiv::Dma2ifg),
            8 => Some(Dmaiv::Dma3ifg),
            10 => Some(Dmaiv::Dma4ifg),
            12 => Some(Dmaiv::Dma5ifg),
            14 => Some(Dmaiv::Dma6ifg),
            16 => Some(Dmaiv::Dma7ifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Dmaiv::None
    }
    #[doc = "Interrupt Source: DMA channel 0; Interrupt Flag: DMA0IFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_dma0ifg(&self) -> bool {
        *self == Dmaiv::Dma0ifg
    }
    #[doc = "Interrupt Source: DMA channel 1; Interrupt Flag: DMA1IFG"]
    #[inline(always)]
    pub fn is_dma1ifg(&self) -> bool {
        *self == Dmaiv::Dma1ifg
    }
    #[doc = "Interrupt Source: DMA channel 2; Interrupt Flag: DMA2IFG"]
    #[inline(always)]
    pub fn is_dma2ifg(&self) -> bool {
        *self == Dmaiv::Dma2ifg
    }
    #[doc = "Interrupt Source: DMA channel 3; Interrupt Flag: DMA3IFG"]
    #[inline(always)]
    pub fn is_dma3ifg(&self) -> bool {
        *self == Dmaiv::Dma3ifg
    }
    #[doc = "Interrupt Source: DMA channel 4; Interrupt Flag: DMA4IFG"]
    #[inline(always)]
    pub fn is_dma4ifg(&self) -> bool {
        *self == Dmaiv::Dma4ifg
    }
    #[doc = "Interrupt Source: DMA channel 5; Interrupt Flag: DMA5IFG"]
    #[inline(always)]
    pub fn is_dma5ifg(&self) -> bool {
        *self == Dmaiv::Dma5ifg
    }
    #[doc = "Interrupt Source: DMA channel 6; Interrupt Flag: DMA6IFG"]
    #[inline(always)]
    pub fn is_dma6ifg(&self) -> bool {
        *self == Dmaiv::Dma6ifg
    }
    #[doc = "Interrupt Source: DMA channel 7; Interrupt Flag: DMA7IFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_dma7ifg(&self) -> bool {
        *self == Dmaiv::Dma7ifg
    }
}
impl R {
    #[doc = "Bits 0:15 - DMA interrupt vector value"]
    #[inline(always)]
    pub fn dmaiv(&self) -> DmaivR {
        DmaivR::new(self.bits)
    }
}
impl W {}
#[doc = "DMA Interrupt Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaivSpec;
impl crate::RegisterSpec for DmaivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmaiv::R`](R) reader structure"]
impl crate::Readable for DmaivSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaiv::W`](W) writer structure"]
impl crate::Writable for DmaivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAIV to value 0"]
impl crate::Resettable for DmaivSpec {}
