#[doc = "Register `DMA0CTL` reader"]
pub type R = crate::R<Dma0ctlSpec>;
#[doc = "Register `DMA0CTL` writer"]
pub type W = crate::W<Dma0ctlSpec>;
#[doc = "Field `DMAREQ` reader - Initiate DMA transfer with DMATSEL"]
pub type DmareqR = crate::BitReader;
#[doc = "Field `DMAREQ` writer - Initiate DMA transfer with DMATSEL"]
pub type DmareqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAABORT` reader - DMA transfer aborted by NMI"]
pub type DmaabortR = crate::BitReader;
#[doc = "Field `DMAABORT` writer - DMA transfer aborted by NMI"]
pub type DmaabortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIE` reader - DMA interrupt enable"]
pub type DmaieR = crate::BitReader;
#[doc = "Field `DMAIE` writer - DMA interrupt enable"]
pub type DmaieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIFG` reader - DMA interrupt flag"]
pub type DmaifgR = crate::BitReader;
#[doc = "Field `DMAIFG` writer - DMA interrupt flag"]
pub type DmaifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMALEVEL` reader - DMA level sensitive trigger select"]
pub type DmalevelR = crate::BitReader;
#[doc = "Field `DMALEVEL` writer - DMA level sensitive trigger select"]
pub type DmalevelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASRCBYTE` reader - DMA source byte"]
pub type DmasrcbyteR = crate::BitReader;
#[doc = "Field `DMASRCBYTE` writer - DMA source byte"]
pub type DmasrcbyteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMADSTBYTE` reader - DMA destination byte"]
pub type DmadstbyteR = crate::BitReader;
#[doc = "Field `DMADSTBYTE` writer - DMA destination byte"]
pub type DmadstbyteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMA source increment bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmasrcincr {
    #[doc = "0: DMA source increment 0: source address unchanged"]
    Dmasrcincr0 = 0,
    #[doc = "1: DMA source increment 1: source address unchanged"]
    Dmasrcincr1 = 1,
    #[doc = "2: DMA source increment 2: source address decremented"]
    Dmasrcincr2 = 2,
    #[doc = "3: DMA source increment 3: source address incremented"]
    Dmasrcincr3 = 3,
}
impl From<Dmasrcincr> for u8 {
    #[inline(always)]
    fn from(variant: Dmasrcincr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmasrcincr {
    type Ux = u8;
}
impl crate::IsEnum for Dmasrcincr {}
#[doc = "Field `DMASRCINCR` reader - DMA source increment bit 0"]
pub type DmasrcincrR = crate::FieldReader<Dmasrcincr>;
impl DmasrcincrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmasrcincr {
        match self.bits {
            0 => Dmasrcincr::Dmasrcincr0,
            1 => Dmasrcincr::Dmasrcincr1,
            2 => Dmasrcincr::Dmasrcincr2,
            3 => Dmasrcincr::Dmasrcincr3,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA source increment 0: source address unchanged"]
    #[inline(always)]
    pub fn is_dmasrcincr_0(&self) -> bool {
        *self == Dmasrcincr::Dmasrcincr0
    }
    #[doc = "DMA source increment 1: source address unchanged"]
    #[inline(always)]
    pub fn is_dmasrcincr_1(&self) -> bool {
        *self == Dmasrcincr::Dmasrcincr1
    }
    #[doc = "DMA source increment 2: source address decremented"]
    #[inline(always)]
    pub fn is_dmasrcincr_2(&self) -> bool {
        *self == Dmasrcincr::Dmasrcincr2
    }
    #[doc = "DMA source increment 3: source address incremented"]
    #[inline(always)]
    pub fn is_dmasrcincr_3(&self) -> bool {
        *self == Dmasrcincr::Dmasrcincr3
    }
}
#[doc = "Field `DMASRCINCR` writer - DMA source increment bit 0"]
pub type DmasrcincrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmasrcincr, crate::Safe>;
impl<'a, REG> DmasrcincrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA source increment 0: source address unchanged"]
    #[inline(always)]
    pub fn dmasrcincr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Dmasrcincr0)
    }
    #[doc = "DMA source increment 1: source address unchanged"]
    #[inline(always)]
    pub fn dmasrcincr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Dmasrcincr1)
    }
    #[doc = "DMA source increment 2: source address decremented"]
    #[inline(always)]
    pub fn dmasrcincr_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Dmasrcincr2)
    }
    #[doc = "DMA source increment 3: source address incremented"]
    #[inline(always)]
    pub fn dmasrcincr_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Dmasrcincr3)
    }
}
#[doc = "DMA destination increment bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmadstincr {
    #[doc = "0: DMA destination increment 0: destination address unchanged"]
    Dmadstincr0 = 0,
    #[doc = "1: DMA destination increment 1: destination address unchanged"]
    Dmadstincr1 = 1,
    #[doc = "2: DMA destination increment 2: destination address decremented"]
    Dmadstincr2 = 2,
    #[doc = "3: DMA destination increment 3: destination address incremented"]
    Dmadstincr3 = 3,
}
impl From<Dmadstincr> for u8 {
    #[inline(always)]
    fn from(variant: Dmadstincr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmadstincr {
    type Ux = u8;
}
impl crate::IsEnum for Dmadstincr {}
#[doc = "Field `DMADSTINCR` reader - DMA destination increment bit 0"]
pub type DmadstincrR = crate::FieldReader<Dmadstincr>;
impl DmadstincrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmadstincr {
        match self.bits {
            0 => Dmadstincr::Dmadstincr0,
            1 => Dmadstincr::Dmadstincr1,
            2 => Dmadstincr::Dmadstincr2,
            3 => Dmadstincr::Dmadstincr3,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA destination increment 0: destination address unchanged"]
    #[inline(always)]
    pub fn is_dmadstincr_0(&self) -> bool {
        *self == Dmadstincr::Dmadstincr0
    }
    #[doc = "DMA destination increment 1: destination address unchanged"]
    #[inline(always)]
    pub fn is_dmadstincr_1(&self) -> bool {
        *self == Dmadstincr::Dmadstincr1
    }
    #[doc = "DMA destination increment 2: destination address decremented"]
    #[inline(always)]
    pub fn is_dmadstincr_2(&self) -> bool {
        *self == Dmadstincr::Dmadstincr2
    }
    #[doc = "DMA destination increment 3: destination address incremented"]
    #[inline(always)]
    pub fn is_dmadstincr_3(&self) -> bool {
        *self == Dmadstincr::Dmadstincr3
    }
}
#[doc = "Field `DMADSTINCR` writer - DMA destination increment bit 0"]
pub type DmadstincrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmadstincr, crate::Safe>;
impl<'a, REG> DmadstincrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA destination increment 0: destination address unchanged"]
    #[inline(always)]
    pub fn dmadstincr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Dmadstincr0)
    }
    #[doc = "DMA destination increment 1: destination address unchanged"]
    #[inline(always)]
    pub fn dmadstincr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Dmadstincr1)
    }
    #[doc = "DMA destination increment 2: destination address decremented"]
    #[inline(always)]
    pub fn dmadstincr_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Dmadstincr2)
    }
    #[doc = "DMA destination increment 3: destination address incremented"]
    #[inline(always)]
    pub fn dmadstincr_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Dmadstincr3)
    }
}
#[doc = "DMA transfer mode bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmadt {
    #[doc = "0: DMA transfer mode 0: single"]
    Dmadt0 = 0,
    #[doc = "1: DMA transfer mode 1: block"]
    Dmadt1 = 1,
    #[doc = "2: DMA transfer mode 2: interleaved"]
    Dmadt2 = 2,
    #[doc = "3: DMA transfer mode 3: interleaved"]
    Dmadt3 = 3,
    #[doc = "4: DMA transfer mode 4: single"]
    Dmadt4 = 4,
    #[doc = "5: DMA transfer mode 5: block"]
    Dmadt5 = 5,
    #[doc = "6: DMA transfer mode 6: interleaved"]
    Dmadt6 = 6,
    #[doc = "7: DMA transfer mode 7: interleaved"]
    Dmadt7 = 7,
}
impl From<Dmadt> for u8 {
    #[inline(always)]
    fn from(variant: Dmadt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmadt {
    type Ux = u8;
}
impl crate::IsEnum for Dmadt {}
#[doc = "Field `DMADT` reader - DMA transfer mode bit 0"]
pub type DmadtR = crate::FieldReader<Dmadt>;
impl DmadtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmadt {
        match self.bits {
            0 => Dmadt::Dmadt0,
            1 => Dmadt::Dmadt1,
            2 => Dmadt::Dmadt2,
            3 => Dmadt::Dmadt3,
            4 => Dmadt::Dmadt4,
            5 => Dmadt::Dmadt5,
            6 => Dmadt::Dmadt6,
            7 => Dmadt::Dmadt7,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA transfer mode 0: single"]
    #[inline(always)]
    pub fn is_dmadt_0(&self) -> bool {
        *self == Dmadt::Dmadt0
    }
    #[doc = "DMA transfer mode 1: block"]
    #[inline(always)]
    pub fn is_dmadt_1(&self) -> bool {
        *self == Dmadt::Dmadt1
    }
    #[doc = "DMA transfer mode 2: interleaved"]
    #[inline(always)]
    pub fn is_dmadt_2(&self) -> bool {
        *self == Dmadt::Dmadt2
    }
    #[doc = "DMA transfer mode 3: interleaved"]
    #[inline(always)]
    pub fn is_dmadt_3(&self) -> bool {
        *self == Dmadt::Dmadt3
    }
    #[doc = "DMA transfer mode 4: single"]
    #[inline(always)]
    pub fn is_dmadt_4(&self) -> bool {
        *self == Dmadt::Dmadt4
    }
    #[doc = "DMA transfer mode 5: block"]
    #[inline(always)]
    pub fn is_dmadt_5(&self) -> bool {
        *self == Dmadt::Dmadt5
    }
    #[doc = "DMA transfer mode 6: interleaved"]
    #[inline(always)]
    pub fn is_dmadt_6(&self) -> bool {
        *self == Dmadt::Dmadt6
    }
    #[doc = "DMA transfer mode 7: interleaved"]
    #[inline(always)]
    pub fn is_dmadt_7(&self) -> bool {
        *self == Dmadt::Dmadt7
    }
}
#[doc = "Field `DMADT` writer - DMA transfer mode bit 0"]
pub type DmadtW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dmadt, crate::Safe>;
impl<'a, REG> DmadtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA transfer mode 0: single"]
    #[inline(always)]
    pub fn dmadt_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt0)
    }
    #[doc = "DMA transfer mode 1: block"]
    #[inline(always)]
    pub fn dmadt_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt1)
    }
    #[doc = "DMA transfer mode 2: interleaved"]
    #[inline(always)]
    pub fn dmadt_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt2)
    }
    #[doc = "DMA transfer mode 3: interleaved"]
    #[inline(always)]
    pub fn dmadt_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt3)
    }
    #[doc = "DMA transfer mode 4: single"]
    #[inline(always)]
    pub fn dmadt_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt4)
    }
    #[doc = "DMA transfer mode 5: block"]
    #[inline(always)]
    pub fn dmadt_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt5)
    }
    #[doc = "DMA transfer mode 6: interleaved"]
    #[inline(always)]
    pub fn dmadt_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt6)
    }
    #[doc = "DMA transfer mode 7: interleaved"]
    #[inline(always)]
    pub fn dmadt_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt7)
    }
}
impl R {
    #[doc = "Bit 0 - Initiate DMA transfer with DMATSEL"]
    #[inline(always)]
    pub fn dmareq(&self) -> DmareqR {
        DmareqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA transfer aborted by NMI"]
    #[inline(always)]
    pub fn dmaabort(&self) -> DmaabortR {
        DmaabortR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA interrupt enable"]
    #[inline(always)]
    pub fn dmaie(&self) -> DmaieR {
        DmaieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA interrupt flag"]
    #[inline(always)]
    pub fn dmaifg(&self) -> DmaifgR {
        DmaifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA level sensitive trigger select"]
    #[inline(always)]
    pub fn dmalevel(&self) -> DmalevelR {
        DmalevelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA source byte"]
    #[inline(always)]
    pub fn dmasrcbyte(&self) -> DmasrcbyteR {
        DmasrcbyteR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA destination byte"]
    #[inline(always)]
    pub fn dmadstbyte(&self) -> DmadstbyteR {
        DmadstbyteR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - DMA source increment bit 0"]
    #[inline(always)]
    pub fn dmasrcincr(&self) -> DmasrcincrR {
        DmasrcincrR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DMA destination increment bit 0"]
    #[inline(always)]
    pub fn dmadstincr(&self) -> DmadstincrR {
        DmadstincrR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - DMA transfer mode bit 0"]
    #[inline(always)]
    pub fn dmadt(&self) -> DmadtR {
        DmadtR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Initiate DMA transfer with DMATSEL"]
    #[inline(always)]
    pub fn dmareq(&mut self) -> DmareqW<'_, Dma0ctlSpec> {
        DmareqW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA transfer aborted by NMI"]
    #[inline(always)]
    pub fn dmaabort(&mut self) -> DmaabortW<'_, Dma0ctlSpec> {
        DmaabortW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA interrupt enable"]
    #[inline(always)]
    pub fn dmaie(&mut self) -> DmaieW<'_, Dma0ctlSpec> {
        DmaieW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA interrupt flag"]
    #[inline(always)]
    pub fn dmaifg(&mut self) -> DmaifgW<'_, Dma0ctlSpec> {
        DmaifgW::new(self, 3)
    }
    #[doc = "Bit 4 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, Dma0ctlSpec> {
        DmaenW::new(self, 4)
    }
    #[doc = "Bit 5 - DMA level sensitive trigger select"]
    #[inline(always)]
    pub fn dmalevel(&mut self) -> DmalevelW<'_, Dma0ctlSpec> {
        DmalevelW::new(self, 5)
    }
    #[doc = "Bit 6 - DMA source byte"]
    #[inline(always)]
    pub fn dmasrcbyte(&mut self) -> DmasrcbyteW<'_, Dma0ctlSpec> {
        DmasrcbyteW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA destination byte"]
    #[inline(always)]
    pub fn dmadstbyte(&mut self) -> DmadstbyteW<'_, Dma0ctlSpec> {
        DmadstbyteW::new(self, 7)
    }
    #[doc = "Bits 8:9 - DMA source increment bit 0"]
    #[inline(always)]
    pub fn dmasrcincr(&mut self) -> DmasrcincrW<'_, Dma0ctlSpec> {
        DmasrcincrW::new(self, 8)
    }
    #[doc = "Bits 10:11 - DMA destination increment bit 0"]
    #[inline(always)]
    pub fn dmadstincr(&mut self) -> DmadstincrW<'_, Dma0ctlSpec> {
        DmadstincrW::new(self, 10)
    }
    #[doc = "Bits 12:14 - DMA transfer mode bit 0"]
    #[inline(always)]
    pub fn dmadt(&mut self) -> DmadtW<'_, Dma0ctlSpec> {
        DmadtW::new(self, 12)
    }
}
#[doc = "DMA Channel 0 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma0ctlSpec;
impl crate::RegisterSpec for Dma0ctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma0ctl::R`](R) reader structure"]
impl crate::Readable for Dma0ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dma0ctl::W`](W) writer structure"]
impl crate::Writable for Dma0ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA0CTL to value 0"]
impl crate::Resettable for Dma0ctlSpec {}
