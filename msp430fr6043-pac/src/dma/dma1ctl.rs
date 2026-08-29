#[doc = "Register `DMA1CTL` reader"]
pub type R = crate::R<Dma1ctlSpec>;
#[doc = "Register `DMA1CTL` writer"]
pub type W = crate::W<Dma1ctlSpec>;
#[doc = "DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmareq {
    #[doc = "0: No DMA start"]
    Dmareq0 = 0,
    #[doc = "1: Start DMA"]
    Dmareq1 = 1,
}
impl From<Dmareq> for bool {
    #[inline(always)]
    fn from(variant: Dmareq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAREQ` reader - DMA request"]
pub type DmareqR = crate::BitReader<Dmareq>;
impl DmareqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmareq {
        match self.bits {
            false => Dmareq::Dmareq0,
            true => Dmareq::Dmareq1,
        }
    }
    #[doc = "No DMA start"]
    #[inline(always)]
    pub fn is_dmareq_0(&self) -> bool {
        *self == Dmareq::Dmareq0
    }
    #[doc = "Start DMA"]
    #[inline(always)]
    pub fn is_dmareq_1(&self) -> bool {
        *self == Dmareq::Dmareq1
    }
}
#[doc = "Field `DMAREQ` writer - DMA request"]
pub type DmareqW<'a, REG> = crate::BitWriter<'a, REG, Dmareq>;
impl<'a, REG> DmareqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA start"]
    #[inline(always)]
    pub fn dmareq_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmareq::Dmareq0)
    }
    #[doc = "Start DMA"]
    #[inline(always)]
    pub fn dmareq_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmareq::Dmareq1)
    }
}
#[doc = "DMA abort\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaabort {
    #[doc = "0: DMA transfer not interrupted"]
    Dmaabort0 = 0,
    #[doc = "1: DMA transfer interrupted by NMI"]
    Dmaabort1 = 1,
}
impl From<Dmaabort> for bool {
    #[inline(always)]
    fn from(variant: Dmaabort) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAABORT` reader - DMA abort"]
pub type DmaabortR = crate::BitReader<Dmaabort>;
impl DmaabortR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaabort {
        match self.bits {
            false => Dmaabort::Dmaabort0,
            true => Dmaabort::Dmaabort1,
        }
    }
    #[doc = "DMA transfer not interrupted"]
    #[inline(always)]
    pub fn is_dmaabort_0(&self) -> bool {
        *self == Dmaabort::Dmaabort0
    }
    #[doc = "DMA transfer interrupted by NMI"]
    #[inline(always)]
    pub fn is_dmaabort_1(&self) -> bool {
        *self == Dmaabort::Dmaabort1
    }
}
#[doc = "Field `DMAABORT` writer - DMA abort"]
pub type DmaabortW<'a, REG> = crate::BitWriter<'a, REG, Dmaabort>;
impl<'a, REG> DmaabortW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA transfer not interrupted"]
    #[inline(always)]
    pub fn dmaabort_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaabort::Dmaabort0)
    }
    #[doc = "DMA transfer interrupted by NMI"]
    #[inline(always)]
    pub fn dmaabort_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaabort::Dmaabort1)
    }
}
#[doc = "DMA interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaie {
    #[doc = "0: Disabled"]
    Disable = 0,
    #[doc = "1: Enabled"]
    Enable = 1,
}
impl From<Dmaie> for bool {
    #[inline(always)]
    fn from(variant: Dmaie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAIE` reader - DMA interrupt enable"]
pub type DmaieR = crate::BitReader<Dmaie>;
impl DmaieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaie {
        match self.bits {
            false => Dmaie::Disable,
            true => Dmaie::Enable,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmaie::Disable
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmaie::Enable
    }
}
#[doc = "Field `DMAIE` writer - DMA interrupt enable"]
pub type DmaieW<'a, REG> = crate::BitWriter<'a, REG, Dmaie>;
impl<'a, REG> DmaieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaie::Disable)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaie::Enable)
    }
}
#[doc = "DMA interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaifg {
    #[doc = "0: No interrupt pending"]
    Dmaifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Dmaifg1 = 1,
}
impl From<Dmaifg> for bool {
    #[inline(always)]
    fn from(variant: Dmaifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAIFG` reader - DMA interrupt flag"]
pub type DmaifgR = crate::BitReader<Dmaifg>;
impl DmaifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaifg {
        match self.bits {
            false => Dmaifg::Dmaifg0,
            true => Dmaifg::Dmaifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_dmaifg_0(&self) -> bool {
        *self == Dmaifg::Dmaifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_dmaifg_1(&self) -> bool {
        *self == Dmaifg::Dmaifg1
    }
}
#[doc = "Field `DMAIFG` writer - DMA interrupt flag"]
pub type DmaifgW<'a, REG> = crate::BitWriter<'a, REG, Dmaifg>;
impl<'a, REG> DmaifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn dmaifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaifg::Dmaifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn dmaifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaifg::Dmaifg1)
    }
}
#[doc = "DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: Disabled"]
    Disable = 0,
    #[doc = "1: Enabled"]
    Enable = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::Disable,
            true => Dmaen::Enable,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmaen::Disable
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmaen::Enable
    }
}
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Disable)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Enable)
    }
}
#[doc = "DMA level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmalevel {
    #[doc = "0: Edge sensitive (rising edge)"]
    Edge = 0,
    #[doc = "1: Level sensitive (high level)"]
    Level = 1,
}
impl From<Dmalevel> for bool {
    #[inline(always)]
    fn from(variant: Dmalevel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMALEVEL` reader - DMA level"]
pub type DmalevelR = crate::BitReader<Dmalevel>;
impl DmalevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmalevel {
        match self.bits {
            false => Dmalevel::Edge,
            true => Dmalevel::Level,
        }
    }
    #[doc = "Edge sensitive (rising edge)"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dmalevel::Edge
    }
    #[doc = "Level sensitive (high level)"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dmalevel::Level
    }
}
#[doc = "Field `DMALEVEL` writer - DMA level"]
pub type DmalevelW<'a, REG> = crate::BitWriter<'a, REG, Dmalevel>;
impl<'a, REG> DmalevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge sensitive (rising edge)"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dmalevel::Edge)
    }
    #[doc = "Level sensitive (high level)"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dmalevel::Level)
    }
}
#[doc = "DMA source byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmasrcbyte {
    #[doc = "0: Word"]
    Word = 0,
    #[doc = "1: Byte"]
    Byte = 1,
}
impl From<Dmasrcbyte> for bool {
    #[inline(always)]
    fn from(variant: Dmasrcbyte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASRCBYTE` reader - DMA source byte"]
pub type DmasrcbyteR = crate::BitReader<Dmasrcbyte>;
impl DmasrcbyteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmasrcbyte {
        match self.bits {
            false => Dmasrcbyte::Word,
            true => Dmasrcbyte::Byte,
        }
    }
    #[doc = "Word"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == Dmasrcbyte::Word
    }
    #[doc = "Byte"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Dmasrcbyte::Byte
    }
}
#[doc = "Field `DMASRCBYTE` writer - DMA source byte"]
pub type DmasrcbyteW<'a, REG> = crate::BitWriter<'a, REG, Dmasrcbyte>;
impl<'a, REG> DmasrcbyteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Word"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcbyte::Word)
    }
    #[doc = "Byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcbyte::Byte)
    }
}
#[doc = "DMA destination byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmadstbyte {
    #[doc = "0: Word"]
    Word = 0,
    #[doc = "1: Byte"]
    Byte = 1,
}
impl From<Dmadstbyte> for bool {
    #[inline(always)]
    fn from(variant: Dmadstbyte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMADSTBYTE` reader - DMA destination byte"]
pub type DmadstbyteR = crate::BitReader<Dmadstbyte>;
impl DmadstbyteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmadstbyte {
        match self.bits {
            false => Dmadstbyte::Word,
            true => Dmadstbyte::Byte,
        }
    }
    #[doc = "Word"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == Dmadstbyte::Word
    }
    #[doc = "Byte"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Dmadstbyte::Byte
    }
}
#[doc = "Field `DMADSTBYTE` writer - DMA destination byte"]
pub type DmadstbyteW<'a, REG> = crate::BitWriter<'a, REG, Dmadstbyte>;
impl<'a, REG> DmadstbyteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Word"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstbyte::Word)
    }
    #[doc = "Byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstbyte::Byte)
    }
}
#[doc = "DMA source increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmasrcincr {
    #[doc = "0: Source address is unchanged"]
    Dmasrcincr0 = 0,
    #[doc = "1: Source address is unchanged"]
    Dmasrcincr1 = 1,
    #[doc = "2: Source address is decremented"]
    Dmasrcincr2 = 2,
    #[doc = "3: Source address is incremented"]
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
#[doc = "Field `DMASRCINCR` reader - DMA source increment"]
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
    #[doc = "Source address is unchanged"]
    #[inline(always)]
    pub fn is_dmasrcincr_0(&self) -> bool {
        *self == Dmasrcincr::Dmasrcincr0
    }
    #[doc = "Source address is unchanged"]
    #[inline(always)]
    pub fn is_dmasrcincr_1(&self) -> bool {
        *self == Dmasrcincr::Dmasrcincr1
    }
    #[doc = "Source address is decremented"]
    #[inline(always)]
    pub fn is_dmasrcincr_2(&self) -> bool {
        *self == Dmasrcincr::Dmasrcincr2
    }
    #[doc = "Source address is incremented"]
    #[inline(always)]
    pub fn is_dmasrcincr_3(&self) -> bool {
        *self == Dmasrcincr::Dmasrcincr3
    }
}
#[doc = "Field `DMASRCINCR` writer - DMA source increment"]
pub type DmasrcincrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmasrcincr, crate::Safe>;
impl<'a, REG> DmasrcincrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Source address is unchanged"]
    #[inline(always)]
    pub fn dmasrcincr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Dmasrcincr0)
    }
    #[doc = "Source address is unchanged"]
    #[inline(always)]
    pub fn dmasrcincr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Dmasrcincr1)
    }
    #[doc = "Source address is decremented"]
    #[inline(always)]
    pub fn dmasrcincr_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Dmasrcincr2)
    }
    #[doc = "Source address is incremented"]
    #[inline(always)]
    pub fn dmasrcincr_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasrcincr::Dmasrcincr3)
    }
}
#[doc = "DMA destination increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmadstincr {
    #[doc = "0: Destination address is unchanged"]
    Dmadstincr0 = 0,
    #[doc = "1: Destination address is unchanged"]
    Dmadstincr1 = 1,
    #[doc = "2: Destination address is decremented"]
    Dmadstincr2 = 2,
    #[doc = "3: Destination address is incremented"]
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
#[doc = "Field `DMADSTINCR` reader - DMA destination increment"]
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
    #[doc = "Destination address is unchanged"]
    #[inline(always)]
    pub fn is_dmadstincr_0(&self) -> bool {
        *self == Dmadstincr::Dmadstincr0
    }
    #[doc = "Destination address is unchanged"]
    #[inline(always)]
    pub fn is_dmadstincr_1(&self) -> bool {
        *self == Dmadstincr::Dmadstincr1
    }
    #[doc = "Destination address is decremented"]
    #[inline(always)]
    pub fn is_dmadstincr_2(&self) -> bool {
        *self == Dmadstincr::Dmadstincr2
    }
    #[doc = "Destination address is incremented"]
    #[inline(always)]
    pub fn is_dmadstincr_3(&self) -> bool {
        *self == Dmadstincr::Dmadstincr3
    }
}
#[doc = "Field `DMADSTINCR` writer - DMA destination increment"]
pub type DmadstincrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmadstincr, crate::Safe>;
impl<'a, REG> DmadstincrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Destination address is unchanged"]
    #[inline(always)]
    pub fn dmadstincr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Dmadstincr0)
    }
    #[doc = "Destination address is unchanged"]
    #[inline(always)]
    pub fn dmadstincr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Dmadstincr1)
    }
    #[doc = "Destination address is decremented"]
    #[inline(always)]
    pub fn dmadstincr_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Dmadstincr2)
    }
    #[doc = "Destination address is incremented"]
    #[inline(always)]
    pub fn dmadstincr_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadstincr::Dmadstincr3)
    }
}
#[doc = "DMA transfer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmadt {
    #[doc = "0: Single transfer"]
    Dmadt0 = 0,
    #[doc = "1: Block transfer"]
    Dmadt1 = 1,
    #[doc = "2: Burst-block transfer"]
    Dmadt2 = 2,
    #[doc = "3: Burst-block transfer"]
    Dmadt3 = 3,
    #[doc = "4: Repeated single transfer"]
    Dmadt4 = 4,
    #[doc = "5: Repeated block transfer"]
    Dmadt5 = 5,
    #[doc = "6: Repeated burst-block transfer"]
    Dmadt6 = 6,
    #[doc = "7: Repeated burst-block transfer"]
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
#[doc = "Field `DMADT` reader - DMA transfer mode"]
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
    #[doc = "Single transfer"]
    #[inline(always)]
    pub fn is_dmadt_0(&self) -> bool {
        *self == Dmadt::Dmadt0
    }
    #[doc = "Block transfer"]
    #[inline(always)]
    pub fn is_dmadt_1(&self) -> bool {
        *self == Dmadt::Dmadt1
    }
    #[doc = "Burst-block transfer"]
    #[inline(always)]
    pub fn is_dmadt_2(&self) -> bool {
        *self == Dmadt::Dmadt2
    }
    #[doc = "Burst-block transfer"]
    #[inline(always)]
    pub fn is_dmadt_3(&self) -> bool {
        *self == Dmadt::Dmadt3
    }
    #[doc = "Repeated single transfer"]
    #[inline(always)]
    pub fn is_dmadt_4(&self) -> bool {
        *self == Dmadt::Dmadt4
    }
    #[doc = "Repeated block transfer"]
    #[inline(always)]
    pub fn is_dmadt_5(&self) -> bool {
        *self == Dmadt::Dmadt5
    }
    #[doc = "Repeated burst-block transfer"]
    #[inline(always)]
    pub fn is_dmadt_6(&self) -> bool {
        *self == Dmadt::Dmadt6
    }
    #[doc = "Repeated burst-block transfer"]
    #[inline(always)]
    pub fn is_dmadt_7(&self) -> bool {
        *self == Dmadt::Dmadt7
    }
}
#[doc = "Field `DMADT` writer - DMA transfer mode"]
pub type DmadtW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dmadt, crate::Safe>;
impl<'a, REG> DmadtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single transfer"]
    #[inline(always)]
    pub fn dmadt_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt0)
    }
    #[doc = "Block transfer"]
    #[inline(always)]
    pub fn dmadt_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt1)
    }
    #[doc = "Burst-block transfer"]
    #[inline(always)]
    pub fn dmadt_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt2)
    }
    #[doc = "Burst-block transfer"]
    #[inline(always)]
    pub fn dmadt_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt3)
    }
    #[doc = "Repeated single transfer"]
    #[inline(always)]
    pub fn dmadt_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt4)
    }
    #[doc = "Repeated block transfer"]
    #[inline(always)]
    pub fn dmadt_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt5)
    }
    #[doc = "Repeated burst-block transfer"]
    #[inline(always)]
    pub fn dmadt_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt6)
    }
    #[doc = "Repeated burst-block transfer"]
    #[inline(always)]
    pub fn dmadt_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadt::Dmadt7)
    }
}
impl R {
    #[doc = "Bit 0 - DMA request"]
    #[inline(always)]
    pub fn dmareq(&self) -> DmareqR {
        DmareqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA abort"]
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
    #[doc = "Bit 5 - DMA level"]
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
    #[doc = "Bits 8:9 - DMA source increment"]
    #[inline(always)]
    pub fn dmasrcincr(&self) -> DmasrcincrR {
        DmasrcincrR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DMA destination increment"]
    #[inline(always)]
    pub fn dmadstincr(&self) -> DmadstincrR {
        DmadstincrR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - DMA transfer mode"]
    #[inline(always)]
    pub fn dmadt(&self) -> DmadtR {
        DmadtR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA request"]
    #[inline(always)]
    pub fn dmareq(&mut self) -> DmareqW<'_, Dma1ctlSpec> {
        DmareqW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA abort"]
    #[inline(always)]
    pub fn dmaabort(&mut self) -> DmaabortW<'_, Dma1ctlSpec> {
        DmaabortW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA interrupt enable"]
    #[inline(always)]
    pub fn dmaie(&mut self) -> DmaieW<'_, Dma1ctlSpec> {
        DmaieW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA interrupt flag"]
    #[inline(always)]
    pub fn dmaifg(&mut self) -> DmaifgW<'_, Dma1ctlSpec> {
        DmaifgW::new(self, 3)
    }
    #[doc = "Bit 4 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, Dma1ctlSpec> {
        DmaenW::new(self, 4)
    }
    #[doc = "Bit 5 - DMA level"]
    #[inline(always)]
    pub fn dmalevel(&mut self) -> DmalevelW<'_, Dma1ctlSpec> {
        DmalevelW::new(self, 5)
    }
    #[doc = "Bit 6 - DMA source byte"]
    #[inline(always)]
    pub fn dmasrcbyte(&mut self) -> DmasrcbyteW<'_, Dma1ctlSpec> {
        DmasrcbyteW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA destination byte"]
    #[inline(always)]
    pub fn dmadstbyte(&mut self) -> DmadstbyteW<'_, Dma1ctlSpec> {
        DmadstbyteW::new(self, 7)
    }
    #[doc = "Bits 8:9 - DMA source increment"]
    #[inline(always)]
    pub fn dmasrcincr(&mut self) -> DmasrcincrW<'_, Dma1ctlSpec> {
        DmasrcincrW::new(self, 8)
    }
    #[doc = "Bits 10:11 - DMA destination increment"]
    #[inline(always)]
    pub fn dmadstincr(&mut self) -> DmadstincrW<'_, Dma1ctlSpec> {
        DmadstincrW::new(self, 10)
    }
    #[doc = "Bits 12:14 - DMA transfer mode"]
    #[inline(always)]
    pub fn dmadt(&mut self) -> DmadtW<'_, Dma1ctlSpec> {
        DmadtW::new(self, 12)
    }
}
#[doc = "DMA Channel 1 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma1ctlSpec;
impl crate::RegisterSpec for Dma1ctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma1ctl::R`](R) reader structure"]
impl crate::Readable for Dma1ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dma1ctl::W`](W) writer structure"]
impl crate::Writable for Dma1ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA1CTL to value 0"]
impl crate::Resettable for Dma1ctlSpec {}
