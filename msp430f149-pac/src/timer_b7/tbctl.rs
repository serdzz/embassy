#[doc = "Register `TBCTL` reader"]
pub type R = crate::R<TbctlSpec>;
#[doc = "Register `TBCTL` writer"]
pub type W = crate::W<TbctlSpec>;
#[doc = "Field `TBIFG` reader - Timer B interrupt flag"]
pub type TbifgR = crate::BitReader;
#[doc = "Field `TBIFG` writer - Timer B interrupt flag"]
pub type TbifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBIE` reader - Timer B interrupt enable"]
pub type TbieR = crate::BitReader;
#[doc = "Field `TBIE` writer - Timer B interrupt enable"]
pub type TbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBCLR` reader - Timer B counter clear"]
pub type TbclrR = crate::BitReader;
#[doc = "Field `TBCLR` writer - Timer B counter clear"]
pub type TbclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer B mode control 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mc {
    #[doc = "0: Timer A mode control: 0 - Stop"]
    Mc0 = 0,
    #[doc = "1: Timer A mode control: 1 - Up to CCR0"]
    Mc1 = 1,
    #[doc = "2: Timer A mode control: 2 - Continous up"]
    Mc2 = 2,
    #[doc = "3: Timer A mode control: 3 - Up/Down"]
    Mc3 = 3,
}
impl From<Mc> for u8 {
    #[inline(always)]
    fn from(variant: Mc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mc {
    type Ux = u8;
}
impl crate::IsEnum for Mc {}
#[doc = "Field `MC` reader - Timer B mode control 1"]
pub type McR = crate::FieldReader<Mc>;
impl McR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mc {
        match self.bits {
            0 => Mc::Mc0,
            1 => Mc::Mc1,
            2 => Mc::Mc2,
            3 => Mc::Mc3,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer A mode control: 0 - Stop"]
    #[inline(always)]
    pub fn is_mc_0(&self) -> bool {
        *self == Mc::Mc0
    }
    #[doc = "Timer A mode control: 1 - Up to CCR0"]
    #[inline(always)]
    pub fn is_mc_1(&self) -> bool {
        *self == Mc::Mc1
    }
    #[doc = "Timer A mode control: 2 - Continous up"]
    #[inline(always)]
    pub fn is_mc_2(&self) -> bool {
        *self == Mc::Mc2
    }
    #[doc = "Timer A mode control: 3 - Up/Down"]
    #[inline(always)]
    pub fn is_mc_3(&self) -> bool {
        *self == Mc::Mc3
    }
}
#[doc = "Field `MC` writer - Timer B mode control 1"]
pub type McW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mc, crate::Safe>;
impl<'a, REG> McW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer A mode control: 0 - Stop"]
    #[inline(always)]
    pub fn mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Mc0)
    }
    #[doc = "Timer A mode control: 1 - Up to CCR0"]
    #[inline(always)]
    pub fn mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Mc1)
    }
    #[doc = "Timer A mode control: 2 - Continous up"]
    #[inline(always)]
    pub fn mc_2(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Mc2)
    }
    #[doc = "Timer A mode control: 3 - Up/Down"]
    #[inline(always)]
    pub fn mc_3(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Mc3)
    }
}
#[doc = "Timer B clock input divider 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Id {
    #[doc = "0: Timer A input divider: 0 - /1"]
    Id0 = 0,
    #[doc = "1: Timer A input divider: 1 - /2"]
    Id1 = 1,
    #[doc = "2: Timer A input divider: 2 - /4"]
    Id2 = 2,
    #[doc = "3: Timer A input divider: 3 - /8"]
    Id3 = 3,
}
impl From<Id> for u8 {
    #[inline(always)]
    fn from(variant: Id) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Id {
    type Ux = u8;
}
impl crate::IsEnum for Id {}
#[doc = "Field `ID` reader - Timer B clock input divider 1"]
pub type IdR = crate::FieldReader<Id>;
impl IdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id {
        match self.bits {
            0 => Id::Id0,
            1 => Id::Id1,
            2 => Id::Id2,
            3 => Id::Id3,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer A input divider: 0 - /1"]
    #[inline(always)]
    pub fn is_id_0(&self) -> bool {
        *self == Id::Id0
    }
    #[doc = "Timer A input divider: 1 - /2"]
    #[inline(always)]
    pub fn is_id_1(&self) -> bool {
        *self == Id::Id1
    }
    #[doc = "Timer A input divider: 2 - /4"]
    #[inline(always)]
    pub fn is_id_2(&self) -> bool {
        *self == Id::Id2
    }
    #[doc = "Timer A input divider: 3 - /8"]
    #[inline(always)]
    pub fn is_id_3(&self) -> bool {
        *self == Id::Id3
    }
}
#[doc = "Field `ID` writer - Timer B clock input divider 1"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Id, crate::Safe>;
impl<'a, REG> IdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer A input divider: 0 - /1"]
    #[inline(always)]
    pub fn id_0(self) -> &'a mut crate::W<REG> {
        self.variant(Id::Id0)
    }
    #[doc = "Timer A input divider: 1 - /2"]
    #[inline(always)]
    pub fn id_1(self) -> &'a mut crate::W<REG> {
        self.variant(Id::Id1)
    }
    #[doc = "Timer A input divider: 2 - /4"]
    #[inline(always)]
    pub fn id_2(self) -> &'a mut crate::W<REG> {
        self.variant(Id::Id2)
    }
    #[doc = "Timer A input divider: 3 - /8"]
    #[inline(always)]
    pub fn id_3(self) -> &'a mut crate::W<REG> {
        self.variant(Id::Id3)
    }
}
#[doc = "Clock source 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tbssel {
    #[doc = "0: Clock Source: TBCLK"]
    Tbssel0 = 0,
    #[doc = "1: Clock Source: ACLK"]
    Tbssel1 = 1,
    #[doc = "2: Clock Source: SMCLK"]
    Tbssel2 = 2,
    #[doc = "3: Clock Source: INCLK"]
    Tbssel3 = 3,
}
impl From<Tbssel> for u8 {
    #[inline(always)]
    fn from(variant: Tbssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tbssel {
    type Ux = u8;
}
impl crate::IsEnum for Tbssel {}
#[doc = "Field `TBSSEL` reader - Clock source 1"]
pub type TbsselR = crate::FieldReader<Tbssel>;
impl TbsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbssel {
        match self.bits {
            0 => Tbssel::Tbssel0,
            1 => Tbssel::Tbssel1,
            2 => Tbssel::Tbssel2,
            3 => Tbssel::Tbssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock Source: TBCLK"]
    #[inline(always)]
    pub fn is_tbssel_0(&self) -> bool {
        *self == Tbssel::Tbssel0
    }
    #[doc = "Clock Source: ACLK"]
    #[inline(always)]
    pub fn is_tbssel_1(&self) -> bool {
        *self == Tbssel::Tbssel1
    }
    #[doc = "Clock Source: SMCLK"]
    #[inline(always)]
    pub fn is_tbssel_2(&self) -> bool {
        *self == Tbssel::Tbssel2
    }
    #[doc = "Clock Source: INCLK"]
    #[inline(always)]
    pub fn is_tbssel_3(&self) -> bool {
        *self == Tbssel::Tbssel3
    }
}
#[doc = "Field `TBSSEL` writer - Clock source 1"]
pub type TbsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tbssel, crate::Safe>;
impl<'a, REG> TbsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock Source: TBCLK"]
    #[inline(always)]
    pub fn tbssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tbssel::Tbssel0)
    }
    #[doc = "Clock Source: ACLK"]
    #[inline(always)]
    pub fn tbssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tbssel::Tbssel1)
    }
    #[doc = "Clock Source: SMCLK"]
    #[inline(always)]
    pub fn tbssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Tbssel::Tbssel2)
    }
    #[doc = "Clock Source: INCLK"]
    #[inline(always)]
    pub fn tbssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Tbssel::Tbssel3)
    }
}
#[doc = "Counter lenght 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cntl {
    #[doc = "0: Counter lenght: 16 bit"]
    Cntl0 = 0,
    #[doc = "1: Counter lenght: 12 bit"]
    Cntl1 = 1,
    #[doc = "2: Counter lenght: 10 bit"]
    Cntl2 = 2,
    #[doc = "3: Counter lenght: 8 bit"]
    Cntl3 = 3,
}
impl From<Cntl> for u8 {
    #[inline(always)]
    fn from(variant: Cntl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cntl {
    type Ux = u8;
}
impl crate::IsEnum for Cntl {}
#[doc = "Field `CNTL` reader - Counter lenght 1"]
pub type CntlR = crate::FieldReader<Cntl>;
impl CntlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntl {
        match self.bits {
            0 => Cntl::Cntl0,
            1 => Cntl::Cntl1,
            2 => Cntl::Cntl2,
            3 => Cntl::Cntl3,
            _ => unreachable!(),
        }
    }
    #[doc = "Counter lenght: 16 bit"]
    #[inline(always)]
    pub fn is_cntl_0(&self) -> bool {
        *self == Cntl::Cntl0
    }
    #[doc = "Counter lenght: 12 bit"]
    #[inline(always)]
    pub fn is_cntl_1(&self) -> bool {
        *self == Cntl::Cntl1
    }
    #[doc = "Counter lenght: 10 bit"]
    #[inline(always)]
    pub fn is_cntl_2(&self) -> bool {
        *self == Cntl::Cntl2
    }
    #[doc = "Counter lenght: 8 bit"]
    #[inline(always)]
    pub fn is_cntl_3(&self) -> bool {
        *self == Cntl::Cntl3
    }
}
#[doc = "Field `CNTL` writer - Counter lenght 1"]
pub type CntlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cntl, crate::Safe>;
impl<'a, REG> CntlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counter lenght: 16 bit"]
    #[inline(always)]
    pub fn cntl_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cntl::Cntl0)
    }
    #[doc = "Counter lenght: 12 bit"]
    #[inline(always)]
    pub fn cntl_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cntl::Cntl1)
    }
    #[doc = "Counter lenght: 10 bit"]
    #[inline(always)]
    pub fn cntl_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cntl::Cntl2)
    }
    #[doc = "Counter lenght: 8 bit"]
    #[inline(always)]
    pub fn cntl_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cntl::Cntl3)
    }
}
#[doc = "Timer B Compare latch load group 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tbclgrp {
    #[doc = "0: Timer B Group: 0 - individually"]
    Tbclgrp0 = 0,
    #[doc = "1: Timer B Group: 1 - 3 groups (1-2"]
    Tbclgrp1 = 1,
    #[doc = "2: Timer B Group: 2 - 2 groups (1-3"]
    Tbclgrp2 = 2,
    #[doc = "3: Timer B Group: 3 - 1 group (all)"]
    Tbclgrp3 = 3,
}
impl From<Tbclgrp> for u8 {
    #[inline(always)]
    fn from(variant: Tbclgrp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tbclgrp {
    type Ux = u8;
}
impl crate::IsEnum for Tbclgrp {}
#[doc = "Field `TBCLGRP` reader - Timer B Compare latch load group 1"]
pub type TbclgrpR = crate::FieldReader<Tbclgrp>;
impl TbclgrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbclgrp {
        match self.bits {
            0 => Tbclgrp::Tbclgrp0,
            1 => Tbclgrp::Tbclgrp1,
            2 => Tbclgrp::Tbclgrp2,
            3 => Tbclgrp::Tbclgrp3,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer B Group: 0 - individually"]
    #[inline(always)]
    pub fn is_tbclgrp_0(&self) -> bool {
        *self == Tbclgrp::Tbclgrp0
    }
    #[doc = "Timer B Group: 1 - 3 groups (1-2"]
    #[inline(always)]
    pub fn is_tbclgrp_1(&self) -> bool {
        *self == Tbclgrp::Tbclgrp1
    }
    #[doc = "Timer B Group: 2 - 2 groups (1-3"]
    #[inline(always)]
    pub fn is_tbclgrp_2(&self) -> bool {
        *self == Tbclgrp::Tbclgrp2
    }
    #[doc = "Timer B Group: 3 - 1 group (all)"]
    #[inline(always)]
    pub fn is_tbclgrp_3(&self) -> bool {
        *self == Tbclgrp::Tbclgrp3
    }
}
#[doc = "Field `TBCLGRP` writer - Timer B Compare latch load group 1"]
pub type TbclgrpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tbclgrp, crate::Safe>;
impl<'a, REG> TbclgrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer B Group: 0 - individually"]
    #[inline(always)]
    pub fn tbclgrp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tbclgrp::Tbclgrp0)
    }
    #[doc = "Timer B Group: 1 - 3 groups (1-2"]
    #[inline(always)]
    pub fn tbclgrp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tbclgrp::Tbclgrp1)
    }
    #[doc = "Timer B Group: 2 - 2 groups (1-3"]
    #[inline(always)]
    pub fn tbclgrp_2(self) -> &'a mut crate::W<REG> {
        self.variant(Tbclgrp::Tbclgrp2)
    }
    #[doc = "Timer B Group: 3 - 1 group (all)"]
    #[inline(always)]
    pub fn tbclgrp_3(self) -> &'a mut crate::W<REG> {
        self.variant(Tbclgrp::Tbclgrp3)
    }
}
impl R {
    #[doc = "Bit 0 - Timer B interrupt flag"]
    #[inline(always)]
    pub fn tbifg(&self) -> TbifgR {
        TbifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer B interrupt enable"]
    #[inline(always)]
    pub fn tbie(&self) -> TbieR {
        TbieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer B counter clear"]
    #[inline(always)]
    pub fn tbclr(&self) -> TbclrR {
        TbclrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Timer B mode control 1"]
    #[inline(always)]
    pub fn mc(&self) -> McR {
        McR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Timer B clock input divider 1"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Clock source 1"]
    #[inline(always)]
    pub fn tbssel(&self) -> TbsselR {
        TbsselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Counter lenght 1"]
    #[inline(always)]
    pub fn cntl(&self) -> CntlR {
        CntlR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Timer B Compare latch load group 1"]
    #[inline(always)]
    pub fn tbclgrp(&self) -> TbclgrpR {
        TbclgrpR::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timer B interrupt flag"]
    #[inline(always)]
    pub fn tbifg(&mut self) -> TbifgW<'_, TbctlSpec> {
        TbifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer B interrupt enable"]
    #[inline(always)]
    pub fn tbie(&mut self) -> TbieW<'_, TbctlSpec> {
        TbieW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer B counter clear"]
    #[inline(always)]
    pub fn tbclr(&mut self) -> TbclrW<'_, TbctlSpec> {
        TbclrW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Timer B mode control 1"]
    #[inline(always)]
    pub fn mc(&mut self) -> McW<'_, TbctlSpec> {
        McW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Timer B clock input divider 1"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<'_, TbctlSpec> {
        IdW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Clock source 1"]
    #[inline(always)]
    pub fn tbssel(&mut self) -> TbsselW<'_, TbctlSpec> {
        TbsselW::new(self, 8)
    }
    #[doc = "Bits 11:12 - Counter lenght 1"]
    #[inline(always)]
    pub fn cntl(&mut self) -> CntlW<'_, TbctlSpec> {
        CntlW::new(self, 11)
    }
    #[doc = "Bits 13:14 - Timer B Compare latch load group 1"]
    #[inline(always)]
    pub fn tbclgrp(&mut self) -> TbclgrpW<'_, TbctlSpec> {
        TbclgrpW::new(self, 13)
    }
}
#[doc = "Timer B Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbctlSpec;
impl crate::RegisterSpec for TbctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbctl::R`](R) reader structure"]
impl crate::Readable for TbctlSpec {}
#[doc = "`write(|w| ..)` method takes [`tbctl::W`](W) writer structure"]
impl crate::Writable for TbctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBCTL to value 0"]
impl crate::Resettable for TbctlSpec {}
