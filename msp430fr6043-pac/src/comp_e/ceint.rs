#[doc = "Register `CEINT` reader"]
pub type R = crate::R<CeintSpec>;
#[doc = "Register `CEINT` writer"]
pub type W = crate::W<CeintSpec>;
#[doc = "Comparator output interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceifg {
    #[doc = "0: No interrupt pending"]
    Ceifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Ceifg1 = 1,
}
impl From<Ceifg> for bool {
    #[inline(always)]
    fn from(variant: Ceifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIFG` reader - Comparator output interrupt flag"]
pub type CeifgR = crate::BitReader<Ceifg>;
impl CeifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceifg {
        match self.bits {
            false => Ceifg::Ceifg0,
            true => Ceifg::Ceifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ceifg_0(&self) -> bool {
        *self == Ceifg::Ceifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ceifg_1(&self) -> bool {
        *self == Ceifg::Ceifg1
    }
}
#[doc = "Field `CEIFG` writer - Comparator output interrupt flag"]
pub type CeifgW<'a, REG> = crate::BitWriter<'a, REG, Ceifg>;
impl<'a, REG> CeifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ceifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceifg::Ceifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ceifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceifg::Ceifg1)
    }
}
#[doc = "Comparator output inverted interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceiifg {
    #[doc = "0: No interrupt pending"]
    Ceiifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Ceiifg1 = 1,
}
impl From<Ceiifg> for bool {
    #[inline(always)]
    fn from(variant: Ceiifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIIFG` reader - Comparator output inverted interrupt flag"]
pub type CeiifgR = crate::BitReader<Ceiifg>;
impl CeiifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceiifg {
        match self.bits {
            false => Ceiifg::Ceiifg0,
            true => Ceiifg::Ceiifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ceiifg_0(&self) -> bool {
        *self == Ceiifg::Ceiifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ceiifg_1(&self) -> bool {
        *self == Ceiifg::Ceiifg1
    }
}
#[doc = "Field `CEIIFG` writer - Comparator output inverted interrupt flag"]
pub type CeiifgW<'a, REG> = crate::BitWriter<'a, REG, Ceiifg>;
impl<'a, REG> CeiifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ceiifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceiifg::Ceiifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ceiifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceiifg::Ceiifg1)
    }
}
#[doc = "Comparator ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cerdyifg {
    #[doc = "0: No interrupt pending"]
    Cerdyifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Cerdyifg1 = 1,
}
impl From<Cerdyifg> for bool {
    #[inline(always)]
    fn from(variant: Cerdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERDYIFG` reader - Comparator ready interrupt flag"]
pub type CerdyifgR = crate::BitReader<Cerdyifg>;
impl CerdyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cerdyifg {
        match self.bits {
            false => Cerdyifg::Cerdyifg0,
            true => Cerdyifg::Cerdyifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_cerdyifg_0(&self) -> bool {
        *self == Cerdyifg::Cerdyifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_cerdyifg_1(&self) -> bool {
        *self == Cerdyifg::Cerdyifg1
    }
}
#[doc = "Field `CERDYIFG` writer - Comparator ready interrupt flag"]
pub type CerdyifgW<'a, REG> = crate::BitWriter<'a, REG, Cerdyifg>;
impl<'a, REG> CerdyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn cerdyifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cerdyifg::Cerdyifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn cerdyifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cerdyifg::Cerdyifg1)
    }
}
#[doc = "Comparator output interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceie {
    #[doc = "0: Interrupt disabled"]
    Disable = 0,
    #[doc = "1: Interrupt enabled"]
    Enable = 1,
}
impl From<Ceie> for bool {
    #[inline(always)]
    fn from(variant: Ceie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIE` reader - Comparator output interrupt enable"]
pub type CeieR = crate::BitReader<Ceie>;
impl CeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceie {
        match self.bits {
            false => Ceie::Disable,
            true => Ceie::Enable,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ceie::Disable
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ceie::Enable
    }
}
#[doc = "Field `CEIE` writer - Comparator output interrupt enable"]
pub type CeieW<'a, REG> = crate::BitWriter<'a, REG, Ceie>;
impl<'a, REG> CeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ceie::Disable)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ceie::Enable)
    }
}
#[doc = "Comparator output interrupt enable inverted polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceiie {
    #[doc = "0: Interrupt disabled"]
    Disable = 0,
    #[doc = "1: Interrupt enabled"]
    Enable = 1,
}
impl From<Ceiie> for bool {
    #[inline(always)]
    fn from(variant: Ceiie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIIE` reader - Comparator output interrupt enable inverted polarity"]
pub type CeiieR = crate::BitReader<Ceiie>;
impl CeiieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceiie {
        match self.bits {
            false => Ceiie::Disable,
            true => Ceiie::Enable,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ceiie::Disable
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ceiie::Enable
    }
}
#[doc = "Field `CEIIE` writer - Comparator output interrupt enable inverted polarity"]
pub type CeiieW<'a, REG> = crate::BitWriter<'a, REG, Ceiie>;
impl<'a, REG> CeiieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ceiie::Disable)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ceiie::Enable)
    }
}
#[doc = "Comparator ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cerdyie {
    #[doc = "0: Interrupt disabled"]
    Disable = 0,
    #[doc = "1: Interrupt enabled"]
    Enable = 1,
}
impl From<Cerdyie> for bool {
    #[inline(always)]
    fn from(variant: Cerdyie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERDYIE` reader - Comparator ready interrupt enable"]
pub type CerdyieR = crate::BitReader<Cerdyie>;
impl CerdyieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cerdyie {
        match self.bits {
            false => Cerdyie::Disable,
            true => Cerdyie::Enable,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cerdyie::Disable
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cerdyie::Enable
    }
}
#[doc = "Field `CERDYIE` writer - Comparator ready interrupt enable"]
pub type CerdyieW<'a, REG> = crate::BitWriter<'a, REG, Cerdyie>;
impl<'a, REG> CerdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cerdyie::Disable)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cerdyie::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator output interrupt flag"]
    #[inline(always)]
    pub fn ceifg(&self) -> CeifgR {
        CeifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator output inverted interrupt flag"]
    #[inline(always)]
    pub fn ceiifg(&self) -> CeiifgR {
        CeiifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator ready interrupt flag"]
    #[inline(always)]
    pub fn cerdyifg(&self) -> CerdyifgR {
        CerdyifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Comparator output interrupt enable"]
    #[inline(always)]
    pub fn ceie(&self) -> CeieR {
        CeieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator output interrupt enable inverted polarity"]
    #[inline(always)]
    pub fn ceiie(&self) -> CeiieR {
        CeiieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Comparator ready interrupt enable"]
    #[inline(always)]
    pub fn cerdyie(&self) -> CerdyieR {
        CerdyieR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator output interrupt flag"]
    #[inline(always)]
    pub fn ceifg(&mut self) -> CeifgW<'_, CeintSpec> {
        CeifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator output inverted interrupt flag"]
    #[inline(always)]
    pub fn ceiifg(&mut self) -> CeiifgW<'_, CeintSpec> {
        CeiifgW::new(self, 1)
    }
    #[doc = "Bit 4 - Comparator ready interrupt flag"]
    #[inline(always)]
    pub fn cerdyifg(&mut self) -> CerdyifgW<'_, CeintSpec> {
        CerdyifgW::new(self, 4)
    }
    #[doc = "Bit 8 - Comparator output interrupt enable"]
    #[inline(always)]
    pub fn ceie(&mut self) -> CeieW<'_, CeintSpec> {
        CeieW::new(self, 8)
    }
    #[doc = "Bit 9 - Comparator output interrupt enable inverted polarity"]
    #[inline(always)]
    pub fn ceiie(&mut self) -> CeiieW<'_, CeintSpec> {
        CeiieW::new(self, 9)
    }
    #[doc = "Bit 12 - Comparator ready interrupt enable"]
    #[inline(always)]
    pub fn cerdyie(&mut self) -> CerdyieW<'_, CeintSpec> {
        CerdyieW::new(self, 12)
    }
}
#[doc = "Comparator Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ceint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ceint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CeintSpec;
impl crate::RegisterSpec for CeintSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ceint::R`](R) reader structure"]
impl crate::Readable for CeintSpec {}
#[doc = "`write(|w| ..)` method takes [`ceint::W`](W) writer structure"]
impl crate::Writable for CeintSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CEINT to value 0"]
impl crate::Resettable for CeintSpec {}
