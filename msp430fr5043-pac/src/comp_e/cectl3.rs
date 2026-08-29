#[doc = "Register `CECTL3` reader"]
pub type R = crate::R<Cectl3Spec>;
#[doc = "Register `CECTL3` writer"]
pub type W = crate::W<Cectl3Spec>;
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd0 {
    #[doc = "0: The input buffer is enabled"]
    Cepd0_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd0_1 = 1,
}
impl From<Cepd0> for bool {
    #[inline(always)]
    fn from(variant: Cepd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD0` reader - Port disable"]
pub type Cepd0R = crate::BitReader<Cepd0>;
impl Cepd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd0 {
        match self.bits {
            false => Cepd0::Cepd0_0,
            true => Cepd0::Cepd0_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd0_0(&self) -> bool {
        *self == Cepd0::Cepd0_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd0_1(&self) -> bool {
        *self == Cepd0::Cepd0_1
    }
}
#[doc = "Field `CEPD0` writer - Port disable"]
pub type Cepd0W<'a, REG> = crate::BitWriter<'a, REG, Cepd0>;
impl<'a, REG> Cepd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd0::Cepd0_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd0::Cepd0_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd1 {
    #[doc = "0: The input buffer is enabled"]
    Cepd1_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd1_1 = 1,
}
impl From<Cepd1> for bool {
    #[inline(always)]
    fn from(variant: Cepd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD1` reader - Port disable"]
pub type Cepd1R = crate::BitReader<Cepd1>;
impl Cepd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd1 {
        match self.bits {
            false => Cepd1::Cepd1_0,
            true => Cepd1::Cepd1_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd1_0(&self) -> bool {
        *self == Cepd1::Cepd1_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd1_1(&self) -> bool {
        *self == Cepd1::Cepd1_1
    }
}
#[doc = "Field `CEPD1` writer - Port disable"]
pub type Cepd1W<'a, REG> = crate::BitWriter<'a, REG, Cepd1>;
impl<'a, REG> Cepd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd1::Cepd1_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd1::Cepd1_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd2 {
    #[doc = "0: The input buffer is enabled"]
    Cepd2_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd2_1 = 1,
}
impl From<Cepd2> for bool {
    #[inline(always)]
    fn from(variant: Cepd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD2` reader - Port disable"]
pub type Cepd2R = crate::BitReader<Cepd2>;
impl Cepd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd2 {
        match self.bits {
            false => Cepd2::Cepd2_0,
            true => Cepd2::Cepd2_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd2_0(&self) -> bool {
        *self == Cepd2::Cepd2_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd2_1(&self) -> bool {
        *self == Cepd2::Cepd2_1
    }
}
#[doc = "Field `CEPD2` writer - Port disable"]
pub type Cepd2W<'a, REG> = crate::BitWriter<'a, REG, Cepd2>;
impl<'a, REG> Cepd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd2::Cepd2_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd2::Cepd2_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd3 {
    #[doc = "0: The input buffer is enabled"]
    Cepd3_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd3_1 = 1,
}
impl From<Cepd3> for bool {
    #[inline(always)]
    fn from(variant: Cepd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD3` reader - Port disable"]
pub type Cepd3R = crate::BitReader<Cepd3>;
impl Cepd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd3 {
        match self.bits {
            false => Cepd3::Cepd3_0,
            true => Cepd3::Cepd3_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd3_0(&self) -> bool {
        *self == Cepd3::Cepd3_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd3_1(&self) -> bool {
        *self == Cepd3::Cepd3_1
    }
}
#[doc = "Field `CEPD3` writer - Port disable"]
pub type Cepd3W<'a, REG> = crate::BitWriter<'a, REG, Cepd3>;
impl<'a, REG> Cepd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd3_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd3::Cepd3_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd3::Cepd3_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd4 {
    #[doc = "0: The input buffer is enabled"]
    Cepd4_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd4_1 = 1,
}
impl From<Cepd4> for bool {
    #[inline(always)]
    fn from(variant: Cepd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD4` reader - Port disable"]
pub type Cepd4R = crate::BitReader<Cepd4>;
impl Cepd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd4 {
        match self.bits {
            false => Cepd4::Cepd4_0,
            true => Cepd4::Cepd4_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd4_0(&self) -> bool {
        *self == Cepd4::Cepd4_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd4_1(&self) -> bool {
        *self == Cepd4::Cepd4_1
    }
}
#[doc = "Field `CEPD4` writer - Port disable"]
pub type Cepd4W<'a, REG> = crate::BitWriter<'a, REG, Cepd4>;
impl<'a, REG> Cepd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd4_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd4::Cepd4_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd4_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd4::Cepd4_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd5 {
    #[doc = "0: The input buffer is enabled"]
    Cepd5_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd5_1 = 1,
}
impl From<Cepd5> for bool {
    #[inline(always)]
    fn from(variant: Cepd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD5` reader - Port disable"]
pub type Cepd5R = crate::BitReader<Cepd5>;
impl Cepd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd5 {
        match self.bits {
            false => Cepd5::Cepd5_0,
            true => Cepd5::Cepd5_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd5_0(&self) -> bool {
        *self == Cepd5::Cepd5_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd5_1(&self) -> bool {
        *self == Cepd5::Cepd5_1
    }
}
#[doc = "Field `CEPD5` writer - Port disable"]
pub type Cepd5W<'a, REG> = crate::BitWriter<'a, REG, Cepd5>;
impl<'a, REG> Cepd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd5_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd5::Cepd5_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd5_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd5::Cepd5_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd6 {
    #[doc = "0: The input buffer is enabled"]
    Cepd6_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd6_1 = 1,
}
impl From<Cepd6> for bool {
    #[inline(always)]
    fn from(variant: Cepd6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD6` reader - Port disable"]
pub type Cepd6R = crate::BitReader<Cepd6>;
impl Cepd6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd6 {
        match self.bits {
            false => Cepd6::Cepd6_0,
            true => Cepd6::Cepd6_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd6_0(&self) -> bool {
        *self == Cepd6::Cepd6_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd6_1(&self) -> bool {
        *self == Cepd6::Cepd6_1
    }
}
#[doc = "Field `CEPD6` writer - Port disable"]
pub type Cepd6W<'a, REG> = crate::BitWriter<'a, REG, Cepd6>;
impl<'a, REG> Cepd6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd6_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd6::Cepd6_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd6_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd6::Cepd6_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd7 {
    #[doc = "0: The input buffer is enabled"]
    Cepd7_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd7_1 = 1,
}
impl From<Cepd7> for bool {
    #[inline(always)]
    fn from(variant: Cepd7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD7` reader - Port disable"]
pub type Cepd7R = crate::BitReader<Cepd7>;
impl Cepd7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd7 {
        match self.bits {
            false => Cepd7::Cepd7_0,
            true => Cepd7::Cepd7_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd7_0(&self) -> bool {
        *self == Cepd7::Cepd7_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd7_1(&self) -> bool {
        *self == Cepd7::Cepd7_1
    }
}
#[doc = "Field `CEPD7` writer - Port disable"]
pub type Cepd7W<'a, REG> = crate::BitWriter<'a, REG, Cepd7>;
impl<'a, REG> Cepd7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd7_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd7::Cepd7_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd7_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd7::Cepd7_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd8 {
    #[doc = "0: The input buffer is enabled"]
    Cepd8_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd8_1 = 1,
}
impl From<Cepd8> for bool {
    #[inline(always)]
    fn from(variant: Cepd8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD8` reader - Port disable"]
pub type Cepd8R = crate::BitReader<Cepd8>;
impl Cepd8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd8 {
        match self.bits {
            false => Cepd8::Cepd8_0,
            true => Cepd8::Cepd8_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd8_0(&self) -> bool {
        *self == Cepd8::Cepd8_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd8_1(&self) -> bool {
        *self == Cepd8::Cepd8_1
    }
}
#[doc = "Field `CEPD8` writer - Port disable"]
pub type Cepd8W<'a, REG> = crate::BitWriter<'a, REG, Cepd8>;
impl<'a, REG> Cepd8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd8_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd8::Cepd8_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd8_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd8::Cepd8_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd9 {
    #[doc = "0: The input buffer is enabled"]
    Cepd9_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd9_1 = 1,
}
impl From<Cepd9> for bool {
    #[inline(always)]
    fn from(variant: Cepd9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD9` reader - Port disable"]
pub type Cepd9R = crate::BitReader<Cepd9>;
impl Cepd9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd9 {
        match self.bits {
            false => Cepd9::Cepd9_0,
            true => Cepd9::Cepd9_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd9_0(&self) -> bool {
        *self == Cepd9::Cepd9_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd9_1(&self) -> bool {
        *self == Cepd9::Cepd9_1
    }
}
#[doc = "Field `CEPD9` writer - Port disable"]
pub type Cepd9W<'a, REG> = crate::BitWriter<'a, REG, Cepd9>;
impl<'a, REG> Cepd9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd9_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd9::Cepd9_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd9_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd9::Cepd9_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd10 {
    #[doc = "0: The input buffer is enabled"]
    Cepd10_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd10_1 = 1,
}
impl From<Cepd10> for bool {
    #[inline(always)]
    fn from(variant: Cepd10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD10` reader - Port disable"]
pub type Cepd10R = crate::BitReader<Cepd10>;
impl Cepd10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd10 {
        match self.bits {
            false => Cepd10::Cepd10_0,
            true => Cepd10::Cepd10_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd10_0(&self) -> bool {
        *self == Cepd10::Cepd10_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd10_1(&self) -> bool {
        *self == Cepd10::Cepd10_1
    }
}
#[doc = "Field `CEPD10` writer - Port disable"]
pub type Cepd10W<'a, REG> = crate::BitWriter<'a, REG, Cepd10>;
impl<'a, REG> Cepd10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd10_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd10::Cepd10_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd10_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd10::Cepd10_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd11 {
    #[doc = "0: The input buffer is enabled"]
    Cepd11_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd11_1 = 1,
}
impl From<Cepd11> for bool {
    #[inline(always)]
    fn from(variant: Cepd11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD11` reader - Port disable"]
pub type Cepd11R = crate::BitReader<Cepd11>;
impl Cepd11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd11 {
        match self.bits {
            false => Cepd11::Cepd11_0,
            true => Cepd11::Cepd11_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd11_0(&self) -> bool {
        *self == Cepd11::Cepd11_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd11_1(&self) -> bool {
        *self == Cepd11::Cepd11_1
    }
}
#[doc = "Field `CEPD11` writer - Port disable"]
pub type Cepd11W<'a, REG> = crate::BitWriter<'a, REG, Cepd11>;
impl<'a, REG> Cepd11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd11_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd11::Cepd11_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd11_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd11::Cepd11_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd12 {
    #[doc = "0: The input buffer is enabled"]
    Cepd12_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd12_1 = 1,
}
impl From<Cepd12> for bool {
    #[inline(always)]
    fn from(variant: Cepd12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD12` reader - Port disable"]
pub type Cepd12R = crate::BitReader<Cepd12>;
impl Cepd12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd12 {
        match self.bits {
            false => Cepd12::Cepd12_0,
            true => Cepd12::Cepd12_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd12_0(&self) -> bool {
        *self == Cepd12::Cepd12_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd12_1(&self) -> bool {
        *self == Cepd12::Cepd12_1
    }
}
#[doc = "Field `CEPD12` writer - Port disable"]
pub type Cepd12W<'a, REG> = crate::BitWriter<'a, REG, Cepd12>;
impl<'a, REG> Cepd12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd12_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd12::Cepd12_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd12_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd12::Cepd12_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd13 {
    #[doc = "0: The input buffer is enabled"]
    Cepd13_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd13_1 = 1,
}
impl From<Cepd13> for bool {
    #[inline(always)]
    fn from(variant: Cepd13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD13` reader - Port disable"]
pub type Cepd13R = crate::BitReader<Cepd13>;
impl Cepd13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd13 {
        match self.bits {
            false => Cepd13::Cepd13_0,
            true => Cepd13::Cepd13_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd13_0(&self) -> bool {
        *self == Cepd13::Cepd13_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd13_1(&self) -> bool {
        *self == Cepd13::Cepd13_1
    }
}
#[doc = "Field `CEPD13` writer - Port disable"]
pub type Cepd13W<'a, REG> = crate::BitWriter<'a, REG, Cepd13>;
impl<'a, REG> Cepd13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd13_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd13::Cepd13_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd13_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd13::Cepd13_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd14 {
    #[doc = "0: The input buffer is enabled"]
    Cepd14_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd14_1 = 1,
}
impl From<Cepd14> for bool {
    #[inline(always)]
    fn from(variant: Cepd14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD14` reader - Port disable"]
pub type Cepd14R = crate::BitReader<Cepd14>;
impl Cepd14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd14 {
        match self.bits {
            false => Cepd14::Cepd14_0,
            true => Cepd14::Cepd14_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd14_0(&self) -> bool {
        *self == Cepd14::Cepd14_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd14_1(&self) -> bool {
        *self == Cepd14::Cepd14_1
    }
}
#[doc = "Field `CEPD14` writer - Port disable"]
pub type Cepd14W<'a, REG> = crate::BitWriter<'a, REG, Cepd14>;
impl<'a, REG> Cepd14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd14_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd14::Cepd14_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd14_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd14::Cepd14_1)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cepd15 {
    #[doc = "0: The input buffer is enabled"]
    Cepd15_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    Cepd15_1 = 1,
}
impl From<Cepd15> for bool {
    #[inline(always)]
    fn from(variant: Cepd15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD15` reader - Port disable"]
pub type Cepd15R = crate::BitReader<Cepd15>;
impl Cepd15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cepd15 {
        match self.bits {
            false => Cepd15::Cepd15_0,
            true => Cepd15::Cepd15_1,
        }
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn is_cepd15_0(&self) -> bool {
        *self == Cepd15::Cepd15_0
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn is_cepd15_1(&self) -> bool {
        *self == Cepd15::Cepd15_1
    }
}
#[doc = "Field `CEPD15` writer - Port disable"]
pub type Cepd15W<'a, REG> = crate::BitWriter<'a, REG, Cepd15>;
impl<'a, REG> Cepd15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd15_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd15::Cepd15_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd15_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepd15::Cepd15_1)
    }
}
impl R {
    #[doc = "Bit 0 - Port disable"]
    #[inline(always)]
    pub fn cepd0(&self) -> Cepd0R {
        Cepd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port disable"]
    #[inline(always)]
    pub fn cepd1(&self) -> Cepd1R {
        Cepd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port disable"]
    #[inline(always)]
    pub fn cepd2(&self) -> Cepd2R {
        Cepd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port disable"]
    #[inline(always)]
    pub fn cepd3(&self) -> Cepd3R {
        Cepd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port disable"]
    #[inline(always)]
    pub fn cepd4(&self) -> Cepd4R {
        Cepd4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port disable"]
    #[inline(always)]
    pub fn cepd5(&self) -> Cepd5R {
        Cepd5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port disable"]
    #[inline(always)]
    pub fn cepd6(&self) -> Cepd6R {
        Cepd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port disable"]
    #[inline(always)]
    pub fn cepd7(&self) -> Cepd7R {
        Cepd7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port disable"]
    #[inline(always)]
    pub fn cepd8(&self) -> Cepd8R {
        Cepd8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port disable"]
    #[inline(always)]
    pub fn cepd9(&self) -> Cepd9R {
        Cepd9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port disable"]
    #[inline(always)]
    pub fn cepd10(&self) -> Cepd10R {
        Cepd10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port disable"]
    #[inline(always)]
    pub fn cepd11(&self) -> Cepd11R {
        Cepd11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port disable"]
    #[inline(always)]
    pub fn cepd12(&self) -> Cepd12R {
        Cepd12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port disable"]
    #[inline(always)]
    pub fn cepd13(&self) -> Cepd13R {
        Cepd13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port disable"]
    #[inline(always)]
    pub fn cepd14(&self) -> Cepd14R {
        Cepd14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port disable"]
    #[inline(always)]
    pub fn cepd15(&self) -> Cepd15R {
        Cepd15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port disable"]
    #[inline(always)]
    pub fn cepd0(&mut self) -> Cepd0W<'_, Cectl3Spec> {
        Cepd0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port disable"]
    #[inline(always)]
    pub fn cepd1(&mut self) -> Cepd1W<'_, Cectl3Spec> {
        Cepd1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port disable"]
    #[inline(always)]
    pub fn cepd2(&mut self) -> Cepd2W<'_, Cectl3Spec> {
        Cepd2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port disable"]
    #[inline(always)]
    pub fn cepd3(&mut self) -> Cepd3W<'_, Cectl3Spec> {
        Cepd3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port disable"]
    #[inline(always)]
    pub fn cepd4(&mut self) -> Cepd4W<'_, Cectl3Spec> {
        Cepd4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port disable"]
    #[inline(always)]
    pub fn cepd5(&mut self) -> Cepd5W<'_, Cectl3Spec> {
        Cepd5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port disable"]
    #[inline(always)]
    pub fn cepd6(&mut self) -> Cepd6W<'_, Cectl3Spec> {
        Cepd6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port disable"]
    #[inline(always)]
    pub fn cepd7(&mut self) -> Cepd7W<'_, Cectl3Spec> {
        Cepd7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port disable"]
    #[inline(always)]
    pub fn cepd8(&mut self) -> Cepd8W<'_, Cectl3Spec> {
        Cepd8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port disable"]
    #[inline(always)]
    pub fn cepd9(&mut self) -> Cepd9W<'_, Cectl3Spec> {
        Cepd9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port disable"]
    #[inline(always)]
    pub fn cepd10(&mut self) -> Cepd10W<'_, Cectl3Spec> {
        Cepd10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port disable"]
    #[inline(always)]
    pub fn cepd11(&mut self) -> Cepd11W<'_, Cectl3Spec> {
        Cepd11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port disable"]
    #[inline(always)]
    pub fn cepd12(&mut self) -> Cepd12W<'_, Cectl3Spec> {
        Cepd12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port disable"]
    #[inline(always)]
    pub fn cepd13(&mut self) -> Cepd13W<'_, Cectl3Spec> {
        Cepd13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port disable"]
    #[inline(always)]
    pub fn cepd14(&mut self) -> Cepd14W<'_, Cectl3Spec> {
        Cepd14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port disable"]
    #[inline(always)]
    pub fn cepd15(&mut self) -> Cepd15W<'_, Cectl3Spec> {
        Cepd15W::new(self, 15)
    }
}
#[doc = "Comparator Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cectl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cectl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cectl3Spec;
impl crate::RegisterSpec for Cectl3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cectl3::R`](R) reader structure"]
impl crate::Readable for Cectl3Spec {}
#[doc = "`write(|w| ..)` method takes [`cectl3::W`](W) writer structure"]
impl crate::Writable for Cectl3Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CECTL3 to value 0"]
impl crate::Resettable for Cectl3Spec {}
