#[doc = "Register `LCDCPCTL2` reader"]
pub type R = crate::R<Lcdcpctl2Spec>;
#[doc = "Register `LCDCPCTL2` writer"]
pub type W = crate::W<Lcdcpctl2Spec>;
#[doc = "LCD segment line 32 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds32 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds32_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds32_1 = 1,
}
impl From<Lcds32> for bool {
    #[inline(always)]
    fn from(variant: Lcds32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS32` reader - LCD segment line 32 enable"]
pub type Lcds32R = crate::BitReader<Lcds32>;
impl Lcds32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds32 {
        match self.bits {
            false => Lcds32::Lcds32_0,
            true => Lcds32::Lcds32_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds32_0(&self) -> bool {
        *self == Lcds32::Lcds32_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds32_1(&self) -> bool {
        *self == Lcds32::Lcds32_1
    }
}
#[doc = "Field `LCDS32` writer - LCD segment line 32 enable"]
pub type Lcds32W<'a, REG> = crate::BitWriter<'a, REG, Lcds32>;
impl<'a, REG> Lcds32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds32_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds32::Lcds32_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds32_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds32::Lcds32_1)
    }
}
#[doc = "LCD segment line 33 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds33 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds33_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds33_1 = 1,
}
impl From<Lcds33> for bool {
    #[inline(always)]
    fn from(variant: Lcds33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS33` reader - LCD segment line 33 enable"]
pub type Lcds33R = crate::BitReader<Lcds33>;
impl Lcds33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds33 {
        match self.bits {
            false => Lcds33::Lcds33_0,
            true => Lcds33::Lcds33_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds33_0(&self) -> bool {
        *self == Lcds33::Lcds33_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds33_1(&self) -> bool {
        *self == Lcds33::Lcds33_1
    }
}
#[doc = "Field `LCDS33` writer - LCD segment line 33 enable"]
pub type Lcds33W<'a, REG> = crate::BitWriter<'a, REG, Lcds33>;
impl<'a, REG> Lcds33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds33_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds33::Lcds33_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds33_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds33::Lcds33_1)
    }
}
#[doc = "LCD segment line 34 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds34 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds34_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds34_1 = 1,
}
impl From<Lcds34> for bool {
    #[inline(always)]
    fn from(variant: Lcds34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS34` reader - LCD segment line 34 enable"]
pub type Lcds34R = crate::BitReader<Lcds34>;
impl Lcds34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds34 {
        match self.bits {
            false => Lcds34::Lcds34_0,
            true => Lcds34::Lcds34_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds34_0(&self) -> bool {
        *self == Lcds34::Lcds34_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds34_1(&self) -> bool {
        *self == Lcds34::Lcds34_1
    }
}
#[doc = "Field `LCDS34` writer - LCD segment line 34 enable"]
pub type Lcds34W<'a, REG> = crate::BitWriter<'a, REG, Lcds34>;
impl<'a, REG> Lcds34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds34_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds34::Lcds34_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds34_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds34::Lcds34_1)
    }
}
#[doc = "LCD segment line 35 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds35 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds35_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds35_1 = 1,
}
impl From<Lcds35> for bool {
    #[inline(always)]
    fn from(variant: Lcds35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS35` reader - LCD segment line 35 enable"]
pub type Lcds35R = crate::BitReader<Lcds35>;
impl Lcds35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds35 {
        match self.bits {
            false => Lcds35::Lcds35_0,
            true => Lcds35::Lcds35_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds35_0(&self) -> bool {
        *self == Lcds35::Lcds35_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds35_1(&self) -> bool {
        *self == Lcds35::Lcds35_1
    }
}
#[doc = "Field `LCDS35` writer - LCD segment line 35 enable"]
pub type Lcds35W<'a, REG> = crate::BitWriter<'a, REG, Lcds35>;
impl<'a, REG> Lcds35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds35_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds35::Lcds35_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds35_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds35::Lcds35_1)
    }
}
#[doc = "LCD segment line 36 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds36 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds36_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds36_1 = 1,
}
impl From<Lcds36> for bool {
    #[inline(always)]
    fn from(variant: Lcds36) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS36` reader - LCD segment line 36 enable"]
pub type Lcds36R = crate::BitReader<Lcds36>;
impl Lcds36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds36 {
        match self.bits {
            false => Lcds36::Lcds36_0,
            true => Lcds36::Lcds36_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds36_0(&self) -> bool {
        *self == Lcds36::Lcds36_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds36_1(&self) -> bool {
        *self == Lcds36::Lcds36_1
    }
}
#[doc = "Field `LCDS36` writer - LCD segment line 36 enable"]
pub type Lcds36W<'a, REG> = crate::BitWriter<'a, REG, Lcds36>;
impl<'a, REG> Lcds36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds36_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds36::Lcds36_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds36_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds36::Lcds36_1)
    }
}
#[doc = "LCD segment line 37 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds37 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds37_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds37_1 = 1,
}
impl From<Lcds37> for bool {
    #[inline(always)]
    fn from(variant: Lcds37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS37` reader - LCD segment line 37 enable"]
pub type Lcds37R = crate::BitReader<Lcds37>;
impl Lcds37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds37 {
        match self.bits {
            false => Lcds37::Lcds37_0,
            true => Lcds37::Lcds37_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds37_0(&self) -> bool {
        *self == Lcds37::Lcds37_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds37_1(&self) -> bool {
        *self == Lcds37::Lcds37_1
    }
}
#[doc = "Field `LCDS37` writer - LCD segment line 37 enable"]
pub type Lcds37W<'a, REG> = crate::BitWriter<'a, REG, Lcds37>;
impl<'a, REG> Lcds37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds37_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds37::Lcds37_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds37_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds37::Lcds37_1)
    }
}
#[doc = "LCD segment line 38 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds38 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds38_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds38_1 = 1,
}
impl From<Lcds38> for bool {
    #[inline(always)]
    fn from(variant: Lcds38) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS38` reader - LCD segment line 38 enable"]
pub type Lcds38R = crate::BitReader<Lcds38>;
impl Lcds38R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds38 {
        match self.bits {
            false => Lcds38::Lcds38_0,
            true => Lcds38::Lcds38_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds38_0(&self) -> bool {
        *self == Lcds38::Lcds38_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds38_1(&self) -> bool {
        *self == Lcds38::Lcds38_1
    }
}
#[doc = "Field `LCDS38` writer - LCD segment line 38 enable"]
pub type Lcds38W<'a, REG> = crate::BitWriter<'a, REG, Lcds38>;
impl<'a, REG> Lcds38W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds38_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds38::Lcds38_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds38_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds38::Lcds38_1)
    }
}
#[doc = "LCD segment line 39 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds39 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds39_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds39_1 = 1,
}
impl From<Lcds39> for bool {
    #[inline(always)]
    fn from(variant: Lcds39) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS39` reader - LCD segment line 39 enable"]
pub type Lcds39R = crate::BitReader<Lcds39>;
impl Lcds39R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds39 {
        match self.bits {
            false => Lcds39::Lcds39_0,
            true => Lcds39::Lcds39_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds39_0(&self) -> bool {
        *self == Lcds39::Lcds39_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds39_1(&self) -> bool {
        *self == Lcds39::Lcds39_1
    }
}
#[doc = "Field `LCDS39` writer - LCD segment line 39 enable"]
pub type Lcds39W<'a, REG> = crate::BitWriter<'a, REG, Lcds39>;
impl<'a, REG> Lcds39W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds39_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds39::Lcds39_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds39_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds39::Lcds39_1)
    }
}
#[doc = "LCD segment line 40 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds40 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds40_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds40_1 = 1,
}
impl From<Lcds40> for bool {
    #[inline(always)]
    fn from(variant: Lcds40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS40` reader - LCD segment line 40 enable"]
pub type Lcds40R = crate::BitReader<Lcds40>;
impl Lcds40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds40 {
        match self.bits {
            false => Lcds40::Lcds40_0,
            true => Lcds40::Lcds40_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds40_0(&self) -> bool {
        *self == Lcds40::Lcds40_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds40_1(&self) -> bool {
        *self == Lcds40::Lcds40_1
    }
}
#[doc = "Field `LCDS40` writer - LCD segment line 40 enable"]
pub type Lcds40W<'a, REG> = crate::BitWriter<'a, REG, Lcds40>;
impl<'a, REG> Lcds40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds40_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds40::Lcds40_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds40_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds40::Lcds40_1)
    }
}
#[doc = "LCD segment line 41 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds41 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds41_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds41_1 = 1,
}
impl From<Lcds41> for bool {
    #[inline(always)]
    fn from(variant: Lcds41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS41` reader - LCD segment line 41 enable"]
pub type Lcds41R = crate::BitReader<Lcds41>;
impl Lcds41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds41 {
        match self.bits {
            false => Lcds41::Lcds41_0,
            true => Lcds41::Lcds41_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds41_0(&self) -> bool {
        *self == Lcds41::Lcds41_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds41_1(&self) -> bool {
        *self == Lcds41::Lcds41_1
    }
}
#[doc = "Field `LCDS41` writer - LCD segment line 41 enable"]
pub type Lcds41W<'a, REG> = crate::BitWriter<'a, REG, Lcds41>;
impl<'a, REG> Lcds41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds41_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds41::Lcds41_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds41_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds41::Lcds41_1)
    }
}
#[doc = "LCD segment line 42 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds42 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds42_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds42_1 = 1,
}
impl From<Lcds42> for bool {
    #[inline(always)]
    fn from(variant: Lcds42) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS42` reader - LCD segment line 42 enable"]
pub type Lcds42R = crate::BitReader<Lcds42>;
impl Lcds42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds42 {
        match self.bits {
            false => Lcds42::Lcds42_0,
            true => Lcds42::Lcds42_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds42_0(&self) -> bool {
        *self == Lcds42::Lcds42_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds42_1(&self) -> bool {
        *self == Lcds42::Lcds42_1
    }
}
#[doc = "Field `LCDS42` writer - LCD segment line 42 enable"]
pub type Lcds42W<'a, REG> = crate::BitWriter<'a, REG, Lcds42>;
impl<'a, REG> Lcds42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds42_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds42::Lcds42_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds42_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds42::Lcds42_1)
    }
}
#[doc = "LCD segment line 43 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds43 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds43_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds43_1 = 1,
}
impl From<Lcds43> for bool {
    #[inline(always)]
    fn from(variant: Lcds43) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS43` reader - LCD segment line 43 enable"]
pub type Lcds43R = crate::BitReader<Lcds43>;
impl Lcds43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds43 {
        match self.bits {
            false => Lcds43::Lcds43_0,
            true => Lcds43::Lcds43_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds43_0(&self) -> bool {
        *self == Lcds43::Lcds43_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds43_1(&self) -> bool {
        *self == Lcds43::Lcds43_1
    }
}
#[doc = "Field `LCDS43` writer - LCD segment line 43 enable"]
pub type Lcds43W<'a, REG> = crate::BitWriter<'a, REG, Lcds43>;
impl<'a, REG> Lcds43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds43_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds43::Lcds43_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds43_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds43::Lcds43_1)
    }
}
#[doc = "LCD segment line 44 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds44 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds44_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds44_1 = 1,
}
impl From<Lcds44> for bool {
    #[inline(always)]
    fn from(variant: Lcds44) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS44` reader - LCD segment line 44 enable"]
pub type Lcds44R = crate::BitReader<Lcds44>;
impl Lcds44R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds44 {
        match self.bits {
            false => Lcds44::Lcds44_0,
            true => Lcds44::Lcds44_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds44_0(&self) -> bool {
        *self == Lcds44::Lcds44_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds44_1(&self) -> bool {
        *self == Lcds44::Lcds44_1
    }
}
#[doc = "Field `LCDS44` writer - LCD segment line 44 enable"]
pub type Lcds44W<'a, REG> = crate::BitWriter<'a, REG, Lcds44>;
impl<'a, REG> Lcds44W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds44_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds44::Lcds44_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds44_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds44::Lcds44_1)
    }
}
#[doc = "LCD segment line 45 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds45 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds45_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds45_1 = 1,
}
impl From<Lcds45> for bool {
    #[inline(always)]
    fn from(variant: Lcds45) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS45` reader - LCD segment line 45 enable"]
pub type Lcds45R = crate::BitReader<Lcds45>;
impl Lcds45R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds45 {
        match self.bits {
            false => Lcds45::Lcds45_0,
            true => Lcds45::Lcds45_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds45_0(&self) -> bool {
        *self == Lcds45::Lcds45_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds45_1(&self) -> bool {
        *self == Lcds45::Lcds45_1
    }
}
#[doc = "Field `LCDS45` writer - LCD segment line 45 enable"]
pub type Lcds45W<'a, REG> = crate::BitWriter<'a, REG, Lcds45>;
impl<'a, REG> Lcds45W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds45_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds45::Lcds45_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds45_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds45::Lcds45_1)
    }
}
#[doc = "LCD segment line 46 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds46 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds46_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds46_1 = 1,
}
impl From<Lcds46> for bool {
    #[inline(always)]
    fn from(variant: Lcds46) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS46` reader - LCD segment line 46 enable"]
pub type Lcds46R = crate::BitReader<Lcds46>;
impl Lcds46R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds46 {
        match self.bits {
            false => Lcds46::Lcds46_0,
            true => Lcds46::Lcds46_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds46_0(&self) -> bool {
        *self == Lcds46::Lcds46_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds46_1(&self) -> bool {
        *self == Lcds46::Lcds46_1
    }
}
#[doc = "Field `LCDS46` writer - LCD segment line 46 enable"]
pub type Lcds46W<'a, REG> = crate::BitWriter<'a, REG, Lcds46>;
impl<'a, REG> Lcds46W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds46_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds46::Lcds46_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds46_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds46::Lcds46_1)
    }
}
#[doc = "LCD segment line 47 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds47 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds47_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds47_1 = 1,
}
impl From<Lcds47> for bool {
    #[inline(always)]
    fn from(variant: Lcds47) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS47` reader - LCD segment line 47 enable"]
pub type Lcds47R = crate::BitReader<Lcds47>;
impl Lcds47R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds47 {
        match self.bits {
            false => Lcds47::Lcds47_0,
            true => Lcds47::Lcds47_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds47_0(&self) -> bool {
        *self == Lcds47::Lcds47_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds47_1(&self) -> bool {
        *self == Lcds47::Lcds47_1
    }
}
#[doc = "Field `LCDS47` writer - LCD segment line 47 enable"]
pub type Lcds47W<'a, REG> = crate::BitWriter<'a, REG, Lcds47>;
impl<'a, REG> Lcds47W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds47_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds47::Lcds47_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds47_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds47::Lcds47_1)
    }
}
impl R {
    #[doc = "Bit 0 - LCD segment line 32 enable"]
    #[inline(always)]
    pub fn lcds32(&self) -> Lcds32R {
        Lcds32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD segment line 33 enable"]
    #[inline(always)]
    pub fn lcds33(&self) -> Lcds33R {
        Lcds33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD segment line 34 enable"]
    #[inline(always)]
    pub fn lcds34(&self) -> Lcds34R {
        Lcds34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD segment line 35 enable"]
    #[inline(always)]
    pub fn lcds35(&self) -> Lcds35R {
        Lcds35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD segment line 36 enable"]
    #[inline(always)]
    pub fn lcds36(&self) -> Lcds36R {
        Lcds36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD segment line 37 enable"]
    #[inline(always)]
    pub fn lcds37(&self) -> Lcds37R {
        Lcds37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD segment line 38 enable"]
    #[inline(always)]
    pub fn lcds38(&self) -> Lcds38R {
        Lcds38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD segment line 39 enable"]
    #[inline(always)]
    pub fn lcds39(&self) -> Lcds39R {
        Lcds39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD segment line 40 enable"]
    #[inline(always)]
    pub fn lcds40(&self) -> Lcds40R {
        Lcds40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD segment line 41 enable"]
    #[inline(always)]
    pub fn lcds41(&self) -> Lcds41R {
        Lcds41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD segment line 42 enable"]
    #[inline(always)]
    pub fn lcds42(&self) -> Lcds42R {
        Lcds42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCD segment line 43 enable"]
    #[inline(always)]
    pub fn lcds43(&self) -> Lcds43R {
        Lcds43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LCD segment line 44 enable"]
    #[inline(always)]
    pub fn lcds44(&self) -> Lcds44R {
        Lcds44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCD segment line 45 enable"]
    #[inline(always)]
    pub fn lcds45(&self) -> Lcds45R {
        Lcds45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LCD segment line 46 enable"]
    #[inline(always)]
    pub fn lcds46(&self) -> Lcds46R {
        Lcds46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LCD segment line 47 enable"]
    #[inline(always)]
    pub fn lcds47(&self) -> Lcds47R {
        Lcds47R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD segment line 32 enable"]
    #[inline(always)]
    pub fn lcds32(&mut self) -> Lcds32W<'_, Lcdcpctl2Spec> {
        Lcds32W::new(self, 0)
    }
    #[doc = "Bit 1 - LCD segment line 33 enable"]
    #[inline(always)]
    pub fn lcds33(&mut self) -> Lcds33W<'_, Lcdcpctl2Spec> {
        Lcds33W::new(self, 1)
    }
    #[doc = "Bit 2 - LCD segment line 34 enable"]
    #[inline(always)]
    pub fn lcds34(&mut self) -> Lcds34W<'_, Lcdcpctl2Spec> {
        Lcds34W::new(self, 2)
    }
    #[doc = "Bit 3 - LCD segment line 35 enable"]
    #[inline(always)]
    pub fn lcds35(&mut self) -> Lcds35W<'_, Lcdcpctl2Spec> {
        Lcds35W::new(self, 3)
    }
    #[doc = "Bit 4 - LCD segment line 36 enable"]
    #[inline(always)]
    pub fn lcds36(&mut self) -> Lcds36W<'_, Lcdcpctl2Spec> {
        Lcds36W::new(self, 4)
    }
    #[doc = "Bit 5 - LCD segment line 37 enable"]
    #[inline(always)]
    pub fn lcds37(&mut self) -> Lcds37W<'_, Lcdcpctl2Spec> {
        Lcds37W::new(self, 5)
    }
    #[doc = "Bit 6 - LCD segment line 38 enable"]
    #[inline(always)]
    pub fn lcds38(&mut self) -> Lcds38W<'_, Lcdcpctl2Spec> {
        Lcds38W::new(self, 6)
    }
    #[doc = "Bit 7 - LCD segment line 39 enable"]
    #[inline(always)]
    pub fn lcds39(&mut self) -> Lcds39W<'_, Lcdcpctl2Spec> {
        Lcds39W::new(self, 7)
    }
    #[doc = "Bit 8 - LCD segment line 40 enable"]
    #[inline(always)]
    pub fn lcds40(&mut self) -> Lcds40W<'_, Lcdcpctl2Spec> {
        Lcds40W::new(self, 8)
    }
    #[doc = "Bit 9 - LCD segment line 41 enable"]
    #[inline(always)]
    pub fn lcds41(&mut self) -> Lcds41W<'_, Lcdcpctl2Spec> {
        Lcds41W::new(self, 9)
    }
    #[doc = "Bit 10 - LCD segment line 42 enable"]
    #[inline(always)]
    pub fn lcds42(&mut self) -> Lcds42W<'_, Lcdcpctl2Spec> {
        Lcds42W::new(self, 10)
    }
    #[doc = "Bit 11 - LCD segment line 43 enable"]
    #[inline(always)]
    pub fn lcds43(&mut self) -> Lcds43W<'_, Lcdcpctl2Spec> {
        Lcds43W::new(self, 11)
    }
    #[doc = "Bit 12 - LCD segment line 44 enable"]
    #[inline(always)]
    pub fn lcds44(&mut self) -> Lcds44W<'_, Lcdcpctl2Spec> {
        Lcds44W::new(self, 12)
    }
    #[doc = "Bit 13 - LCD segment line 45 enable"]
    #[inline(always)]
    pub fn lcds45(&mut self) -> Lcds45W<'_, Lcdcpctl2Spec> {
        Lcds45W::new(self, 13)
    }
    #[doc = "Bit 14 - LCD segment line 46 enable"]
    #[inline(always)]
    pub fn lcds46(&mut self) -> Lcds46W<'_, Lcdcpctl2Spec> {
        Lcds46W::new(self, 14)
    }
    #[doc = "Bit 15 - LCD segment line 47 enable"]
    #[inline(always)]
    pub fn lcds47(&mut self) -> Lcds47W<'_, Lcdcpctl2Spec> {
        Lcds47W::new(self, 15)
    }
}
#[doc = "LCD_C port control 2 (256 segments)\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcpctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcpctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdcpctl2Spec;
impl crate::RegisterSpec for Lcdcpctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdcpctl2::R`](R) reader structure"]
impl crate::Readable for Lcdcpctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdcpctl2::W`](W) writer structure"]
impl crate::Writable for Lcdcpctl2Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LCDCPCTL2 to value 0"]
impl crate::Resettable for Lcdcpctl2Spec {}
