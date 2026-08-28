#[doc = "Register `TACTL` reader"]
pub type R = crate::R<TactlSpec>;
#[doc = "Register `TACTL` writer"]
pub type W = crate::W<TactlSpec>;
#[doc = "Field `TAIFG` reader - Timer A counter interrupt flag"]
pub type TaifgR = crate::BitReader;
#[doc = "Field `TAIFG` writer - Timer A counter interrupt flag"]
pub type TaifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAIE` reader - Timer A counter interrupt enable"]
pub type TaieR = crate::BitReader;
#[doc = "Field `TAIE` writer - Timer A counter interrupt enable"]
pub type TaieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TACLR` reader - Timer A counter clear"]
pub type TaclrR = crate::BitReader;
#[doc = "Field `TACLR` writer - Timer A counter clear"]
pub type TaclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer A mode control 1\n\nValue on reset: 0"]
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
#[doc = "Field `MC` reader - Timer A mode control 1"]
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
#[doc = "Field `MC` writer - Timer A mode control 1"]
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
#[doc = "Timer A clock input divider 1\n\nValue on reset: 0"]
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
#[doc = "Field `ID` reader - Timer A clock input divider 1"]
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
#[doc = "Field `ID` writer - Timer A clock input divider 1"]
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
#[doc = "Timer A clock source select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tassel {
    #[doc = "0: Timer A clock source select: 0 - TACLK"]
    Tassel0 = 0,
    #[doc = "1: Timer A clock source select: 1 - ACLK"]
    Tassel1 = 1,
    #[doc = "2: Timer A clock source select: 2 - SMCLK"]
    Tassel2 = 2,
    #[doc = "3: Timer A clock source select: 3 - INCLK"]
    Tassel3 = 3,
}
impl From<Tassel> for u8 {
    #[inline(always)]
    fn from(variant: Tassel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tassel {
    type Ux = u8;
}
impl crate::IsEnum for Tassel {}
#[doc = "Field `TASSEL` reader - Timer A clock source select 1"]
pub type TasselR = crate::FieldReader<Tassel>;
impl TasselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tassel {
        match self.bits {
            0 => Tassel::Tassel0,
            1 => Tassel::Tassel1,
            2 => Tassel::Tassel2,
            3 => Tassel::Tassel3,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer A clock source select: 0 - TACLK"]
    #[inline(always)]
    pub fn is_tassel_0(&self) -> bool {
        *self == Tassel::Tassel0
    }
    #[doc = "Timer A clock source select: 1 - ACLK"]
    #[inline(always)]
    pub fn is_tassel_1(&self) -> bool {
        *self == Tassel::Tassel1
    }
    #[doc = "Timer A clock source select: 2 - SMCLK"]
    #[inline(always)]
    pub fn is_tassel_2(&self) -> bool {
        *self == Tassel::Tassel2
    }
    #[doc = "Timer A clock source select: 3 - INCLK"]
    #[inline(always)]
    pub fn is_tassel_3(&self) -> bool {
        *self == Tassel::Tassel3
    }
}
#[doc = "Field `TASSEL` writer - Timer A clock source select 1"]
pub type TasselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tassel, crate::Safe>;
impl<'a, REG> TasselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer A clock source select: 0 - TACLK"]
    #[inline(always)]
    pub fn tassel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Tassel0)
    }
    #[doc = "Timer A clock source select: 1 - ACLK"]
    #[inline(always)]
    pub fn tassel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Tassel1)
    }
    #[doc = "Timer A clock source select: 2 - SMCLK"]
    #[inline(always)]
    pub fn tassel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Tassel2)
    }
    #[doc = "Timer A clock source select: 3 - INCLK"]
    #[inline(always)]
    pub fn tassel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Tassel3)
    }
}
impl R {
    #[doc = "Bit 0 - Timer A counter interrupt flag"]
    #[inline(always)]
    pub fn taifg(&self) -> TaifgR {
        TaifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A counter interrupt enable"]
    #[inline(always)]
    pub fn taie(&self) -> TaieR {
        TaieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer A counter clear"]
    #[inline(always)]
    pub fn taclr(&self) -> TaclrR {
        TaclrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Timer A mode control 1"]
    #[inline(always)]
    pub fn mc(&self) -> McR {
        McR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Timer A clock input divider 1"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Timer A clock source select 1"]
    #[inline(always)]
    pub fn tassel(&self) -> TasselR {
        TasselR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timer A counter interrupt flag"]
    #[inline(always)]
    pub fn taifg(&mut self) -> TaifgW<'_, TactlSpec> {
        TaifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer A counter interrupt enable"]
    #[inline(always)]
    pub fn taie(&mut self) -> TaieW<'_, TactlSpec> {
        TaieW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer A counter clear"]
    #[inline(always)]
    pub fn taclr(&mut self) -> TaclrW<'_, TactlSpec> {
        TaclrW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Timer A mode control 1"]
    #[inline(always)]
    pub fn mc(&mut self) -> McW<'_, TactlSpec> {
        McW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Timer A clock input divider 1"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<'_, TactlSpec> {
        IdW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Timer A clock source select 1"]
    #[inline(always)]
    pub fn tassel(&mut self) -> TasselW<'_, TactlSpec> {
        TasselW::new(self, 8)
    }
}
#[doc = "Timer A Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tactl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tactl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TactlSpec;
impl crate::RegisterSpec for TactlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tactl::R`](R) reader structure"]
impl crate::Readable for TactlSpec {}
#[doc = "`write(|w| ..)` method takes [`tactl::W`](W) writer structure"]
impl crate::Writable for TactlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TACTL to value 0"]
impl crate::Resettable for TactlSpec {}
