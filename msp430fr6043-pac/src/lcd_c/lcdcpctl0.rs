#[doc = "Register `LCDCPCTL0` reader"]
pub type R = crate::R<Lcdcpctl0Spec>;
#[doc = "Register `LCDCPCTL0` writer"]
pub type W = crate::W<Lcdcpctl0Spec>;
#[doc = "LCD segment line 0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds0 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds0_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds0_1 = 1,
}
impl From<Lcds0> for bool {
    #[inline(always)]
    fn from(variant: Lcds0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS0` reader - LCD segment line 0 enable"]
pub type Lcds0R = crate::BitReader<Lcds0>;
impl Lcds0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds0 {
        match self.bits {
            false => Lcds0::Lcds0_0,
            true => Lcds0::Lcds0_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds0_0(&self) -> bool {
        *self == Lcds0::Lcds0_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds0_1(&self) -> bool {
        *self == Lcds0::Lcds0_1
    }
}
#[doc = "Field `LCDS0` writer - LCD segment line 0 enable"]
pub type Lcds0W<'a, REG> = crate::BitWriter<'a, REG, Lcds0>;
impl<'a, REG> Lcds0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds0::Lcds0_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds0::Lcds0_1)
    }
}
#[doc = "LCD segment line 1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds1 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds1_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds1_1 = 1,
}
impl From<Lcds1> for bool {
    #[inline(always)]
    fn from(variant: Lcds1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS1` reader - LCD segment line 1 enable"]
pub type Lcds1R = crate::BitReader<Lcds1>;
impl Lcds1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds1 {
        match self.bits {
            false => Lcds1::Lcds1_0,
            true => Lcds1::Lcds1_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds1_0(&self) -> bool {
        *self == Lcds1::Lcds1_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds1_1(&self) -> bool {
        *self == Lcds1::Lcds1_1
    }
}
#[doc = "Field `LCDS1` writer - LCD segment line 1 enable"]
pub type Lcds1W<'a, REG> = crate::BitWriter<'a, REG, Lcds1>;
impl<'a, REG> Lcds1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds1::Lcds1_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds1::Lcds1_1)
    }
}
#[doc = "LCD segment line 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds2 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds2_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds2_1 = 1,
}
impl From<Lcds2> for bool {
    #[inline(always)]
    fn from(variant: Lcds2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS2` reader - LCD segment line 2 enable"]
pub type Lcds2R = crate::BitReader<Lcds2>;
impl Lcds2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds2 {
        match self.bits {
            false => Lcds2::Lcds2_0,
            true => Lcds2::Lcds2_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds2_0(&self) -> bool {
        *self == Lcds2::Lcds2_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds2_1(&self) -> bool {
        *self == Lcds2::Lcds2_1
    }
}
#[doc = "Field `LCDS2` writer - LCD segment line 2 enable"]
pub type Lcds2W<'a, REG> = crate::BitWriter<'a, REG, Lcds2>;
impl<'a, REG> Lcds2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds2::Lcds2_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds2::Lcds2_1)
    }
}
#[doc = "LCD segment line 3 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds3 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds3_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds3_1 = 1,
}
impl From<Lcds3> for bool {
    #[inline(always)]
    fn from(variant: Lcds3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS3` reader - LCD segment line 3 enable"]
pub type Lcds3R = crate::BitReader<Lcds3>;
impl Lcds3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds3 {
        match self.bits {
            false => Lcds3::Lcds3_0,
            true => Lcds3::Lcds3_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds3_0(&self) -> bool {
        *self == Lcds3::Lcds3_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds3_1(&self) -> bool {
        *self == Lcds3::Lcds3_1
    }
}
#[doc = "Field `LCDS3` writer - LCD segment line 3 enable"]
pub type Lcds3W<'a, REG> = crate::BitWriter<'a, REG, Lcds3>;
impl<'a, REG> Lcds3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds3_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds3::Lcds3_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds3::Lcds3_1)
    }
}
#[doc = "LCD segment line 4 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds4 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds4_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds4_1 = 1,
}
impl From<Lcds4> for bool {
    #[inline(always)]
    fn from(variant: Lcds4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS4` reader - LCD segment line 4 enable"]
pub type Lcds4R = crate::BitReader<Lcds4>;
impl Lcds4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds4 {
        match self.bits {
            false => Lcds4::Lcds4_0,
            true => Lcds4::Lcds4_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds4_0(&self) -> bool {
        *self == Lcds4::Lcds4_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds4_1(&self) -> bool {
        *self == Lcds4::Lcds4_1
    }
}
#[doc = "Field `LCDS4` writer - LCD segment line 4 enable"]
pub type Lcds4W<'a, REG> = crate::BitWriter<'a, REG, Lcds4>;
impl<'a, REG> Lcds4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds4_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds4::Lcds4_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds4_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds4::Lcds4_1)
    }
}
#[doc = "LCD segment line 5 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds5 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds5_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds5_1 = 1,
}
impl From<Lcds5> for bool {
    #[inline(always)]
    fn from(variant: Lcds5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS5` reader - LCD segment line 5 enable"]
pub type Lcds5R = crate::BitReader<Lcds5>;
impl Lcds5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds5 {
        match self.bits {
            false => Lcds5::Lcds5_0,
            true => Lcds5::Lcds5_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds5_0(&self) -> bool {
        *self == Lcds5::Lcds5_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds5_1(&self) -> bool {
        *self == Lcds5::Lcds5_1
    }
}
#[doc = "Field `LCDS5` writer - LCD segment line 5 enable"]
pub type Lcds5W<'a, REG> = crate::BitWriter<'a, REG, Lcds5>;
impl<'a, REG> Lcds5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds5_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds5::Lcds5_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds5_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds5::Lcds5_1)
    }
}
#[doc = "LCD segment line 6 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds6 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds6_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds6_1 = 1,
}
impl From<Lcds6> for bool {
    #[inline(always)]
    fn from(variant: Lcds6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS6` reader - LCD segment line 6 enable"]
pub type Lcds6R = crate::BitReader<Lcds6>;
impl Lcds6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds6 {
        match self.bits {
            false => Lcds6::Lcds6_0,
            true => Lcds6::Lcds6_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds6_0(&self) -> bool {
        *self == Lcds6::Lcds6_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds6_1(&self) -> bool {
        *self == Lcds6::Lcds6_1
    }
}
#[doc = "Field `LCDS6` writer - LCD segment line 6 enable"]
pub type Lcds6W<'a, REG> = crate::BitWriter<'a, REG, Lcds6>;
impl<'a, REG> Lcds6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds6_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds6::Lcds6_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds6_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds6::Lcds6_1)
    }
}
#[doc = "LCD segment line 7 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds7 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds7_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds7_1 = 1,
}
impl From<Lcds7> for bool {
    #[inline(always)]
    fn from(variant: Lcds7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS7` reader - LCD segment line 7 enable"]
pub type Lcds7R = crate::BitReader<Lcds7>;
impl Lcds7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds7 {
        match self.bits {
            false => Lcds7::Lcds7_0,
            true => Lcds7::Lcds7_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds7_0(&self) -> bool {
        *self == Lcds7::Lcds7_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds7_1(&self) -> bool {
        *self == Lcds7::Lcds7_1
    }
}
#[doc = "Field `LCDS7` writer - LCD segment line 7 enable"]
pub type Lcds7W<'a, REG> = crate::BitWriter<'a, REG, Lcds7>;
impl<'a, REG> Lcds7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds7_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds7::Lcds7_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds7_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds7::Lcds7_1)
    }
}
#[doc = "LCD segment line 8 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds8 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds8_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds8_1 = 1,
}
impl From<Lcds8> for bool {
    #[inline(always)]
    fn from(variant: Lcds8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS8` reader - LCD segment line 8 enable"]
pub type Lcds8R = crate::BitReader<Lcds8>;
impl Lcds8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds8 {
        match self.bits {
            false => Lcds8::Lcds8_0,
            true => Lcds8::Lcds8_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds8_0(&self) -> bool {
        *self == Lcds8::Lcds8_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds8_1(&self) -> bool {
        *self == Lcds8::Lcds8_1
    }
}
#[doc = "Field `LCDS8` writer - LCD segment line 8 enable"]
pub type Lcds8W<'a, REG> = crate::BitWriter<'a, REG, Lcds8>;
impl<'a, REG> Lcds8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds8_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds8::Lcds8_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds8_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds8::Lcds8_1)
    }
}
#[doc = "LCD segment line 9 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds9 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds9_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds9_1 = 1,
}
impl From<Lcds9> for bool {
    #[inline(always)]
    fn from(variant: Lcds9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS9` reader - LCD segment line 9 enable"]
pub type Lcds9R = crate::BitReader<Lcds9>;
impl Lcds9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds9 {
        match self.bits {
            false => Lcds9::Lcds9_0,
            true => Lcds9::Lcds9_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds9_0(&self) -> bool {
        *self == Lcds9::Lcds9_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds9_1(&self) -> bool {
        *self == Lcds9::Lcds9_1
    }
}
#[doc = "Field `LCDS9` writer - LCD segment line 9 enable"]
pub type Lcds9W<'a, REG> = crate::BitWriter<'a, REG, Lcds9>;
impl<'a, REG> Lcds9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds9_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds9::Lcds9_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds9_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds9::Lcds9_1)
    }
}
#[doc = "LCD segment line 10 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds10 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds10_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds10_1 = 1,
}
impl From<Lcds10> for bool {
    #[inline(always)]
    fn from(variant: Lcds10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS10` reader - LCD segment line 10 enable"]
pub type Lcds10R = crate::BitReader<Lcds10>;
impl Lcds10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds10 {
        match self.bits {
            false => Lcds10::Lcds10_0,
            true => Lcds10::Lcds10_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds10_0(&self) -> bool {
        *self == Lcds10::Lcds10_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds10_1(&self) -> bool {
        *self == Lcds10::Lcds10_1
    }
}
#[doc = "Field `LCDS10` writer - LCD segment line 10 enable"]
pub type Lcds10W<'a, REG> = crate::BitWriter<'a, REG, Lcds10>;
impl<'a, REG> Lcds10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds10_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds10::Lcds10_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds10_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds10::Lcds10_1)
    }
}
#[doc = "LCD segment line 11 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds11 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds11_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds11_1 = 1,
}
impl From<Lcds11> for bool {
    #[inline(always)]
    fn from(variant: Lcds11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS11` reader - LCD segment line 11 enable"]
pub type Lcds11R = crate::BitReader<Lcds11>;
impl Lcds11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds11 {
        match self.bits {
            false => Lcds11::Lcds11_0,
            true => Lcds11::Lcds11_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds11_0(&self) -> bool {
        *self == Lcds11::Lcds11_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds11_1(&self) -> bool {
        *self == Lcds11::Lcds11_1
    }
}
#[doc = "Field `LCDS11` writer - LCD segment line 11 enable"]
pub type Lcds11W<'a, REG> = crate::BitWriter<'a, REG, Lcds11>;
impl<'a, REG> Lcds11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds11_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds11::Lcds11_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds11_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds11::Lcds11_1)
    }
}
#[doc = "LCD segment line 12 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds12 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds12_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds12_1 = 1,
}
impl From<Lcds12> for bool {
    #[inline(always)]
    fn from(variant: Lcds12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS12` reader - LCD segment line 12 enable"]
pub type Lcds12R = crate::BitReader<Lcds12>;
impl Lcds12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds12 {
        match self.bits {
            false => Lcds12::Lcds12_0,
            true => Lcds12::Lcds12_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds12_0(&self) -> bool {
        *self == Lcds12::Lcds12_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds12_1(&self) -> bool {
        *self == Lcds12::Lcds12_1
    }
}
#[doc = "Field `LCDS12` writer - LCD segment line 12 enable"]
pub type Lcds12W<'a, REG> = crate::BitWriter<'a, REG, Lcds12>;
impl<'a, REG> Lcds12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds12_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds12::Lcds12_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds12_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds12::Lcds12_1)
    }
}
#[doc = "LCD segment line 13 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds13 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds13_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds13_1 = 1,
}
impl From<Lcds13> for bool {
    #[inline(always)]
    fn from(variant: Lcds13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS13` reader - LCD segment line 13 enable"]
pub type Lcds13R = crate::BitReader<Lcds13>;
impl Lcds13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds13 {
        match self.bits {
            false => Lcds13::Lcds13_0,
            true => Lcds13::Lcds13_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds13_0(&self) -> bool {
        *self == Lcds13::Lcds13_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds13_1(&self) -> bool {
        *self == Lcds13::Lcds13_1
    }
}
#[doc = "Field `LCDS13` writer - LCD segment line 13 enable"]
pub type Lcds13W<'a, REG> = crate::BitWriter<'a, REG, Lcds13>;
impl<'a, REG> Lcds13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds13_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds13::Lcds13_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds13_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds13::Lcds13_1)
    }
}
#[doc = "LCD segment line 14 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds14 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds14_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds14_1 = 1,
}
impl From<Lcds14> for bool {
    #[inline(always)]
    fn from(variant: Lcds14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS14` reader - LCD segment line 14 enable"]
pub type Lcds14R = crate::BitReader<Lcds14>;
impl Lcds14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds14 {
        match self.bits {
            false => Lcds14::Lcds14_0,
            true => Lcds14::Lcds14_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds14_0(&self) -> bool {
        *self == Lcds14::Lcds14_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds14_1(&self) -> bool {
        *self == Lcds14::Lcds14_1
    }
}
#[doc = "Field `LCDS14` writer - LCD segment line 14 enable"]
pub type Lcds14W<'a, REG> = crate::BitWriter<'a, REG, Lcds14>;
impl<'a, REG> Lcds14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds14_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds14::Lcds14_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds14_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds14::Lcds14_1)
    }
}
#[doc = "LCD segment line 15 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds15 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds15_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds15_1 = 1,
}
impl From<Lcds15> for bool {
    #[inline(always)]
    fn from(variant: Lcds15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS15` reader - LCD segment line 15 enable"]
pub type Lcds15R = crate::BitReader<Lcds15>;
impl Lcds15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds15 {
        match self.bits {
            false => Lcds15::Lcds15_0,
            true => Lcds15::Lcds15_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds15_0(&self) -> bool {
        *self == Lcds15::Lcds15_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds15_1(&self) -> bool {
        *self == Lcds15::Lcds15_1
    }
}
#[doc = "Field `LCDS15` writer - LCD segment line 15 enable"]
pub type Lcds15W<'a, REG> = crate::BitWriter<'a, REG, Lcds15>;
impl<'a, REG> Lcds15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds15_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds15::Lcds15_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds15_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds15::Lcds15_1)
    }
}
impl R {
    #[doc = "Bit 0 - LCD segment line 0 enable"]
    #[inline(always)]
    pub fn lcds0(&self) -> Lcds0R {
        Lcds0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD segment line 1 enable"]
    #[inline(always)]
    pub fn lcds1(&self) -> Lcds1R {
        Lcds1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD segment line 2 enable"]
    #[inline(always)]
    pub fn lcds2(&self) -> Lcds2R {
        Lcds2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD segment line 3 enable"]
    #[inline(always)]
    pub fn lcds3(&self) -> Lcds3R {
        Lcds3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD segment line 4 enable"]
    #[inline(always)]
    pub fn lcds4(&self) -> Lcds4R {
        Lcds4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD segment line 5 enable"]
    #[inline(always)]
    pub fn lcds5(&self) -> Lcds5R {
        Lcds5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD segment line 6 enable"]
    #[inline(always)]
    pub fn lcds6(&self) -> Lcds6R {
        Lcds6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD segment line 7 enable"]
    #[inline(always)]
    pub fn lcds7(&self) -> Lcds7R {
        Lcds7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD segment line 8 enable"]
    #[inline(always)]
    pub fn lcds8(&self) -> Lcds8R {
        Lcds8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD segment line 9 enable"]
    #[inline(always)]
    pub fn lcds9(&self) -> Lcds9R {
        Lcds9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD segment line 10 enable"]
    #[inline(always)]
    pub fn lcds10(&self) -> Lcds10R {
        Lcds10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCD segment line 11 enable"]
    #[inline(always)]
    pub fn lcds11(&self) -> Lcds11R {
        Lcds11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LCD segment line 12 enable"]
    #[inline(always)]
    pub fn lcds12(&self) -> Lcds12R {
        Lcds12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCD segment line 13 enable"]
    #[inline(always)]
    pub fn lcds13(&self) -> Lcds13R {
        Lcds13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LCD segment line 14 enable"]
    #[inline(always)]
    pub fn lcds14(&self) -> Lcds14R {
        Lcds14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LCD segment line 15 enable"]
    #[inline(always)]
    pub fn lcds15(&self) -> Lcds15R {
        Lcds15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD segment line 0 enable"]
    #[inline(always)]
    pub fn lcds0(&mut self) -> Lcds0W<'_, Lcdcpctl0Spec> {
        Lcds0W::new(self, 0)
    }
    #[doc = "Bit 1 - LCD segment line 1 enable"]
    #[inline(always)]
    pub fn lcds1(&mut self) -> Lcds1W<'_, Lcdcpctl0Spec> {
        Lcds1W::new(self, 1)
    }
    #[doc = "Bit 2 - LCD segment line 2 enable"]
    #[inline(always)]
    pub fn lcds2(&mut self) -> Lcds2W<'_, Lcdcpctl0Spec> {
        Lcds2W::new(self, 2)
    }
    #[doc = "Bit 3 - LCD segment line 3 enable"]
    #[inline(always)]
    pub fn lcds3(&mut self) -> Lcds3W<'_, Lcdcpctl0Spec> {
        Lcds3W::new(self, 3)
    }
    #[doc = "Bit 4 - LCD segment line 4 enable"]
    #[inline(always)]
    pub fn lcds4(&mut self) -> Lcds4W<'_, Lcdcpctl0Spec> {
        Lcds4W::new(self, 4)
    }
    #[doc = "Bit 5 - LCD segment line 5 enable"]
    #[inline(always)]
    pub fn lcds5(&mut self) -> Lcds5W<'_, Lcdcpctl0Spec> {
        Lcds5W::new(self, 5)
    }
    #[doc = "Bit 6 - LCD segment line 6 enable"]
    #[inline(always)]
    pub fn lcds6(&mut self) -> Lcds6W<'_, Lcdcpctl0Spec> {
        Lcds6W::new(self, 6)
    }
    #[doc = "Bit 7 - LCD segment line 7 enable"]
    #[inline(always)]
    pub fn lcds7(&mut self) -> Lcds7W<'_, Lcdcpctl0Spec> {
        Lcds7W::new(self, 7)
    }
    #[doc = "Bit 8 - LCD segment line 8 enable"]
    #[inline(always)]
    pub fn lcds8(&mut self) -> Lcds8W<'_, Lcdcpctl0Spec> {
        Lcds8W::new(self, 8)
    }
    #[doc = "Bit 9 - LCD segment line 9 enable"]
    #[inline(always)]
    pub fn lcds9(&mut self) -> Lcds9W<'_, Lcdcpctl0Spec> {
        Lcds9W::new(self, 9)
    }
    #[doc = "Bit 10 - LCD segment line 10 enable"]
    #[inline(always)]
    pub fn lcds10(&mut self) -> Lcds10W<'_, Lcdcpctl0Spec> {
        Lcds10W::new(self, 10)
    }
    #[doc = "Bit 11 - LCD segment line 11 enable"]
    #[inline(always)]
    pub fn lcds11(&mut self) -> Lcds11W<'_, Lcdcpctl0Spec> {
        Lcds11W::new(self, 11)
    }
    #[doc = "Bit 12 - LCD segment line 12 enable"]
    #[inline(always)]
    pub fn lcds12(&mut self) -> Lcds12W<'_, Lcdcpctl0Spec> {
        Lcds12W::new(self, 12)
    }
    #[doc = "Bit 13 - LCD segment line 13 enable"]
    #[inline(always)]
    pub fn lcds13(&mut self) -> Lcds13W<'_, Lcdcpctl0Spec> {
        Lcds13W::new(self, 13)
    }
    #[doc = "Bit 14 - LCD segment line 14 enable"]
    #[inline(always)]
    pub fn lcds14(&mut self) -> Lcds14W<'_, Lcdcpctl0Spec> {
        Lcds14W::new(self, 14)
    }
    #[doc = "Bit 15 - LCD segment line 15 enable"]
    #[inline(always)]
    pub fn lcds15(&mut self) -> Lcds15W<'_, Lcdcpctl0Spec> {
        Lcds15W::new(self, 15)
    }
}
#[doc = "LCD_C port control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcpctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcpctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdcpctl0Spec;
impl crate::RegisterSpec for Lcdcpctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdcpctl0::R`](R) reader structure"]
impl crate::Readable for Lcdcpctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdcpctl0::W`](W) writer structure"]
impl crate::Writable for Lcdcpctl0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LCDCPCTL0 to value 0"]
impl crate::Resettable for Lcdcpctl0Spec {}
