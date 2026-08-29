#[doc = "Register `ADC12IER0` reader"]
pub type R = crate::R<Adc12ier0Spec>;
#[doc = "Register `ADC12IER0` writer"]
pub type W = crate::W<Adc12ier0Spec>;
#[doc = "Interrupt enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie0 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie0_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie0_1 = 1,
}
impl From<Adc12ie0> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE0` reader - Interrupt enable 0"]
pub type Adc12ie0R = crate::BitReader<Adc12ie0>;
impl Adc12ie0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie0 {
        match self.bits {
            false => Adc12ie0::Adc12ie0_0,
            true => Adc12ie0::Adc12ie0_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie0_0(&self) -> bool {
        *self == Adc12ie0::Adc12ie0_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie0_1(&self) -> bool {
        *self == Adc12ie0::Adc12ie0_1
    }
}
#[doc = "Field `ADC12IE0` writer - Interrupt enable 0"]
pub type Adc12ie0W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie0>;
impl<'a, REG> Adc12ie0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie0::Adc12ie0_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie0::Adc12ie0_1)
    }
}
#[doc = "interrupt enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie1 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie1_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie1_1 = 1,
}
impl From<Adc12ie1> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE1` reader - interrupt enable 1"]
pub type Adc12ie1R = crate::BitReader<Adc12ie1>;
impl Adc12ie1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie1 {
        match self.bits {
            false => Adc12ie1::Adc12ie1_0,
            true => Adc12ie1::Adc12ie1_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie1_0(&self) -> bool {
        *self == Adc12ie1::Adc12ie1_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie1_1(&self) -> bool {
        *self == Adc12ie1::Adc12ie1_1
    }
}
#[doc = "Field `ADC12IE1` writer - interrupt enable 1"]
pub type Adc12ie1W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie1>;
impl<'a, REG> Adc12ie1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie1::Adc12ie1_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie1::Adc12ie1_1)
    }
}
#[doc = "interrupt enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie2 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie2_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie2_1 = 1,
}
impl From<Adc12ie2> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE2` reader - interrupt enable 2"]
pub type Adc12ie2R = crate::BitReader<Adc12ie2>;
impl Adc12ie2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie2 {
        match self.bits {
            false => Adc12ie2::Adc12ie2_0,
            true => Adc12ie2::Adc12ie2_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie2_0(&self) -> bool {
        *self == Adc12ie2::Adc12ie2_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie2_1(&self) -> bool {
        *self == Adc12ie2::Adc12ie2_1
    }
}
#[doc = "Field `ADC12IE2` writer - interrupt enable 2"]
pub type Adc12ie2W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie2>;
impl<'a, REG> Adc12ie2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie2::Adc12ie2_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie2::Adc12ie2_1)
    }
}
#[doc = "interrupt enable 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie3 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie3_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie3_1 = 1,
}
impl From<Adc12ie3> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE3` reader - interrupt enable 3"]
pub type Adc12ie3R = crate::BitReader<Adc12ie3>;
impl Adc12ie3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie3 {
        match self.bits {
            false => Adc12ie3::Adc12ie3_0,
            true => Adc12ie3::Adc12ie3_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie3_0(&self) -> bool {
        *self == Adc12ie3::Adc12ie3_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie3_1(&self) -> bool {
        *self == Adc12ie3::Adc12ie3_1
    }
}
#[doc = "Field `ADC12IE3` writer - interrupt enable 3"]
pub type Adc12ie3W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie3>;
impl<'a, REG> Adc12ie3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie3_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie3::Adc12ie3_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie3::Adc12ie3_1)
    }
}
#[doc = "interrupt enable 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie4 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie4_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie4_1 = 1,
}
impl From<Adc12ie4> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE4` reader - interrupt enable 4"]
pub type Adc12ie4R = crate::BitReader<Adc12ie4>;
impl Adc12ie4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie4 {
        match self.bits {
            false => Adc12ie4::Adc12ie4_0,
            true => Adc12ie4::Adc12ie4_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie4_0(&self) -> bool {
        *self == Adc12ie4::Adc12ie4_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie4_1(&self) -> bool {
        *self == Adc12ie4::Adc12ie4_1
    }
}
#[doc = "Field `ADC12IE4` writer - interrupt enable 4"]
pub type Adc12ie4W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie4>;
impl<'a, REG> Adc12ie4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie4_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie4::Adc12ie4_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie4_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie4::Adc12ie4_1)
    }
}
#[doc = "interrupt enable 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie5 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie5_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie5_1 = 1,
}
impl From<Adc12ie5> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE5` reader - interrupt enable 5"]
pub type Adc12ie5R = crate::BitReader<Adc12ie5>;
impl Adc12ie5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie5 {
        match self.bits {
            false => Adc12ie5::Adc12ie5_0,
            true => Adc12ie5::Adc12ie5_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie5_0(&self) -> bool {
        *self == Adc12ie5::Adc12ie5_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie5_1(&self) -> bool {
        *self == Adc12ie5::Adc12ie5_1
    }
}
#[doc = "Field `ADC12IE5` writer - interrupt enable 5"]
pub type Adc12ie5W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie5>;
impl<'a, REG> Adc12ie5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie5_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie5::Adc12ie5_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie5_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie5::Adc12ie5_1)
    }
}
#[doc = "interrupt enable 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie6 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie6_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie6_1 = 1,
}
impl From<Adc12ie6> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE6` reader - interrupt enable 6"]
pub type Adc12ie6R = crate::BitReader<Adc12ie6>;
impl Adc12ie6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie6 {
        match self.bits {
            false => Adc12ie6::Adc12ie6_0,
            true => Adc12ie6::Adc12ie6_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie6_0(&self) -> bool {
        *self == Adc12ie6::Adc12ie6_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie6_1(&self) -> bool {
        *self == Adc12ie6::Adc12ie6_1
    }
}
#[doc = "Field `ADC12IE6` writer - interrupt enable 6"]
pub type Adc12ie6W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie6>;
impl<'a, REG> Adc12ie6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie6_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie6::Adc12ie6_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie6_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie6::Adc12ie6_1)
    }
}
#[doc = "interrupt enable 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie7 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie7_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie7_1 = 1,
}
impl From<Adc12ie7> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE7` reader - interrupt enable 7"]
pub type Adc12ie7R = crate::BitReader<Adc12ie7>;
impl Adc12ie7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie7 {
        match self.bits {
            false => Adc12ie7::Adc12ie7_0,
            true => Adc12ie7::Adc12ie7_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie7_0(&self) -> bool {
        *self == Adc12ie7::Adc12ie7_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie7_1(&self) -> bool {
        *self == Adc12ie7::Adc12ie7_1
    }
}
#[doc = "Field `ADC12IE7` writer - interrupt enable 7"]
pub type Adc12ie7W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie7>;
impl<'a, REG> Adc12ie7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie7_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie7::Adc12ie7_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie7_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie7::Adc12ie7_1)
    }
}
#[doc = "interrupt enable 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie8 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie8_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie8_1 = 1,
}
impl From<Adc12ie8> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE8` reader - interrupt enable 8"]
pub type Adc12ie8R = crate::BitReader<Adc12ie8>;
impl Adc12ie8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie8 {
        match self.bits {
            false => Adc12ie8::Adc12ie8_0,
            true => Adc12ie8::Adc12ie8_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie8_0(&self) -> bool {
        *self == Adc12ie8::Adc12ie8_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie8_1(&self) -> bool {
        *self == Adc12ie8::Adc12ie8_1
    }
}
#[doc = "Field `ADC12IE8` writer - interrupt enable 8"]
pub type Adc12ie8W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie8>;
impl<'a, REG> Adc12ie8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie8_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie8::Adc12ie8_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie8_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie8::Adc12ie8_1)
    }
}
#[doc = "interrupt enable 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie9 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie9_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie9_1 = 1,
}
impl From<Adc12ie9> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE9` reader - interrupt enable 9"]
pub type Adc12ie9R = crate::BitReader<Adc12ie9>;
impl Adc12ie9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie9 {
        match self.bits {
            false => Adc12ie9::Adc12ie9_0,
            true => Adc12ie9::Adc12ie9_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie9_0(&self) -> bool {
        *self == Adc12ie9::Adc12ie9_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie9_1(&self) -> bool {
        *self == Adc12ie9::Adc12ie9_1
    }
}
#[doc = "Field `ADC12IE9` writer - interrupt enable 9"]
pub type Adc12ie9W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie9>;
impl<'a, REG> Adc12ie9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie9_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie9::Adc12ie9_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie9_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie9::Adc12ie9_1)
    }
}
#[doc = "interrupt enable 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie10 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie10_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie10_1 = 1,
}
impl From<Adc12ie10> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE10` reader - interrupt enable 10"]
pub type Adc12ie10R = crate::BitReader<Adc12ie10>;
impl Adc12ie10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie10 {
        match self.bits {
            false => Adc12ie10::Adc12ie10_0,
            true => Adc12ie10::Adc12ie10_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie10_0(&self) -> bool {
        *self == Adc12ie10::Adc12ie10_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie10_1(&self) -> bool {
        *self == Adc12ie10::Adc12ie10_1
    }
}
#[doc = "Field `ADC12IE10` writer - interrupt enable 10"]
pub type Adc12ie10W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie10>;
impl<'a, REG> Adc12ie10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie10_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie10::Adc12ie10_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie10_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie10::Adc12ie10_1)
    }
}
#[doc = "interrupt enable 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie11 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie11_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie11_1 = 1,
}
impl From<Adc12ie11> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE11` reader - interrupt enable 11"]
pub type Adc12ie11R = crate::BitReader<Adc12ie11>;
impl Adc12ie11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie11 {
        match self.bits {
            false => Adc12ie11::Adc12ie11_0,
            true => Adc12ie11::Adc12ie11_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie11_0(&self) -> bool {
        *self == Adc12ie11::Adc12ie11_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie11_1(&self) -> bool {
        *self == Adc12ie11::Adc12ie11_1
    }
}
#[doc = "Field `ADC12IE11` writer - interrupt enable 11"]
pub type Adc12ie11W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie11>;
impl<'a, REG> Adc12ie11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie11_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie11::Adc12ie11_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie11_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie11::Adc12ie11_1)
    }
}
#[doc = "interrupt enable 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie12 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie12_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie12_1 = 1,
}
impl From<Adc12ie12> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE12` reader - interrupt enable 12"]
pub type Adc12ie12R = crate::BitReader<Adc12ie12>;
impl Adc12ie12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie12 {
        match self.bits {
            false => Adc12ie12::Adc12ie12_0,
            true => Adc12ie12::Adc12ie12_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie12_0(&self) -> bool {
        *self == Adc12ie12::Adc12ie12_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie12_1(&self) -> bool {
        *self == Adc12ie12::Adc12ie12_1
    }
}
#[doc = "Field `ADC12IE12` writer - interrupt enable 12"]
pub type Adc12ie12W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie12>;
impl<'a, REG> Adc12ie12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie12_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie12::Adc12ie12_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie12_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie12::Adc12ie12_1)
    }
}
#[doc = "interrupt enable 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie13 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie13_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie13_1 = 1,
}
impl From<Adc12ie13> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE13` reader - interrupt enable 13"]
pub type Adc12ie13R = crate::BitReader<Adc12ie13>;
impl Adc12ie13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie13 {
        match self.bits {
            false => Adc12ie13::Adc12ie13_0,
            true => Adc12ie13::Adc12ie13_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie13_0(&self) -> bool {
        *self == Adc12ie13::Adc12ie13_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie13_1(&self) -> bool {
        *self == Adc12ie13::Adc12ie13_1
    }
}
#[doc = "Field `ADC12IE13` writer - interrupt enable 13"]
pub type Adc12ie13W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie13>;
impl<'a, REG> Adc12ie13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie13_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie13::Adc12ie13_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie13_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie13::Adc12ie13_1)
    }
}
#[doc = "interrupt enable 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie14 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie14_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie14_1 = 1,
}
impl From<Adc12ie14> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE14` reader - interrupt enable 14"]
pub type Adc12ie14R = crate::BitReader<Adc12ie14>;
impl Adc12ie14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie14 {
        match self.bits {
            false => Adc12ie14::Adc12ie14_0,
            true => Adc12ie14::Adc12ie14_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie14_0(&self) -> bool {
        *self == Adc12ie14::Adc12ie14_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie14_1(&self) -> bool {
        *self == Adc12ie14::Adc12ie14_1
    }
}
#[doc = "Field `ADC12IE14` writer - interrupt enable 14"]
pub type Adc12ie14W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie14>;
impl<'a, REG> Adc12ie14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie14_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie14::Adc12ie14_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie14_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie14::Adc12ie14_1)
    }
}
#[doc = "interrupt enable 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie15 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie15_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie15_1 = 1,
}
impl From<Adc12ie15> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE15` reader - interrupt enable 15"]
pub type Adc12ie15R = crate::BitReader<Adc12ie15>;
impl Adc12ie15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie15 {
        match self.bits {
            false => Adc12ie15::Adc12ie15_0,
            true => Adc12ie15::Adc12ie15_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie15_0(&self) -> bool {
        *self == Adc12ie15::Adc12ie15_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie15_1(&self) -> bool {
        *self == Adc12ie15::Adc12ie15_1
    }
}
#[doc = "Field `ADC12IE15` writer - interrupt enable 15"]
pub type Adc12ie15W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie15>;
impl<'a, REG> Adc12ie15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie15_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie15::Adc12ie15_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie15_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie15::Adc12ie15_1)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt enable 0"]
    #[inline(always)]
    pub fn adc12ie0(&self) -> Adc12ie0R {
        Adc12ie0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt enable 1"]
    #[inline(always)]
    pub fn adc12ie1(&self) -> Adc12ie1R {
        Adc12ie1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt enable 2"]
    #[inline(always)]
    pub fn adc12ie2(&self) -> Adc12ie2R {
        Adc12ie2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt enable 3"]
    #[inline(always)]
    pub fn adc12ie3(&self) -> Adc12ie3R {
        Adc12ie3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - interrupt enable 4"]
    #[inline(always)]
    pub fn adc12ie4(&self) -> Adc12ie4R {
        Adc12ie4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - interrupt enable 5"]
    #[inline(always)]
    pub fn adc12ie5(&self) -> Adc12ie5R {
        Adc12ie5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - interrupt enable 6"]
    #[inline(always)]
    pub fn adc12ie6(&self) -> Adc12ie6R {
        Adc12ie6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - interrupt enable 7"]
    #[inline(always)]
    pub fn adc12ie7(&self) -> Adc12ie7R {
        Adc12ie7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - interrupt enable 8"]
    #[inline(always)]
    pub fn adc12ie8(&self) -> Adc12ie8R {
        Adc12ie8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - interrupt enable 9"]
    #[inline(always)]
    pub fn adc12ie9(&self) -> Adc12ie9R {
        Adc12ie9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - interrupt enable 10"]
    #[inline(always)]
    pub fn adc12ie10(&self) -> Adc12ie10R {
        Adc12ie10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - interrupt enable 11"]
    #[inline(always)]
    pub fn adc12ie11(&self) -> Adc12ie11R {
        Adc12ie11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - interrupt enable 12"]
    #[inline(always)]
    pub fn adc12ie12(&self) -> Adc12ie12R {
        Adc12ie12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - interrupt enable 13"]
    #[inline(always)]
    pub fn adc12ie13(&self) -> Adc12ie13R {
        Adc12ie13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - interrupt enable 14"]
    #[inline(always)]
    pub fn adc12ie14(&self) -> Adc12ie14R {
        Adc12ie14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - interrupt enable 15"]
    #[inline(always)]
    pub fn adc12ie15(&self) -> Adc12ie15R {
        Adc12ie15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable 0"]
    #[inline(always)]
    pub fn adc12ie0(&mut self) -> Adc12ie0W<'_, Adc12ier0Spec> {
        Adc12ie0W::new(self, 0)
    }
    #[doc = "Bit 1 - interrupt enable 1"]
    #[inline(always)]
    pub fn adc12ie1(&mut self) -> Adc12ie1W<'_, Adc12ier0Spec> {
        Adc12ie1W::new(self, 1)
    }
    #[doc = "Bit 2 - interrupt enable 2"]
    #[inline(always)]
    pub fn adc12ie2(&mut self) -> Adc12ie2W<'_, Adc12ier0Spec> {
        Adc12ie2W::new(self, 2)
    }
    #[doc = "Bit 3 - interrupt enable 3"]
    #[inline(always)]
    pub fn adc12ie3(&mut self) -> Adc12ie3W<'_, Adc12ier0Spec> {
        Adc12ie3W::new(self, 3)
    }
    #[doc = "Bit 4 - interrupt enable 4"]
    #[inline(always)]
    pub fn adc12ie4(&mut self) -> Adc12ie4W<'_, Adc12ier0Spec> {
        Adc12ie4W::new(self, 4)
    }
    #[doc = "Bit 5 - interrupt enable 5"]
    #[inline(always)]
    pub fn adc12ie5(&mut self) -> Adc12ie5W<'_, Adc12ier0Spec> {
        Adc12ie5W::new(self, 5)
    }
    #[doc = "Bit 6 - interrupt enable 6"]
    #[inline(always)]
    pub fn adc12ie6(&mut self) -> Adc12ie6W<'_, Adc12ier0Spec> {
        Adc12ie6W::new(self, 6)
    }
    #[doc = "Bit 7 - interrupt enable 7"]
    #[inline(always)]
    pub fn adc12ie7(&mut self) -> Adc12ie7W<'_, Adc12ier0Spec> {
        Adc12ie7W::new(self, 7)
    }
    #[doc = "Bit 8 - interrupt enable 8"]
    #[inline(always)]
    pub fn adc12ie8(&mut self) -> Adc12ie8W<'_, Adc12ier0Spec> {
        Adc12ie8W::new(self, 8)
    }
    #[doc = "Bit 9 - interrupt enable 9"]
    #[inline(always)]
    pub fn adc12ie9(&mut self) -> Adc12ie9W<'_, Adc12ier0Spec> {
        Adc12ie9W::new(self, 9)
    }
    #[doc = "Bit 10 - interrupt enable 10"]
    #[inline(always)]
    pub fn adc12ie10(&mut self) -> Adc12ie10W<'_, Adc12ier0Spec> {
        Adc12ie10W::new(self, 10)
    }
    #[doc = "Bit 11 - interrupt enable 11"]
    #[inline(always)]
    pub fn adc12ie11(&mut self) -> Adc12ie11W<'_, Adc12ier0Spec> {
        Adc12ie11W::new(self, 11)
    }
    #[doc = "Bit 12 - interrupt enable 12"]
    #[inline(always)]
    pub fn adc12ie12(&mut self) -> Adc12ie12W<'_, Adc12ier0Spec> {
        Adc12ie12W::new(self, 12)
    }
    #[doc = "Bit 13 - interrupt enable 13"]
    #[inline(always)]
    pub fn adc12ie13(&mut self) -> Adc12ie13W<'_, Adc12ier0Spec> {
        Adc12ie13W::new(self, 13)
    }
    #[doc = "Bit 14 - interrupt enable 14"]
    #[inline(always)]
    pub fn adc12ie14(&mut self) -> Adc12ie14W<'_, Adc12ier0Spec> {
        Adc12ie14W::new(self, 14)
    }
    #[doc = "Bit 15 - interrupt enable 15"]
    #[inline(always)]
    pub fn adc12ie15(&mut self) -> Adc12ie15W<'_, Adc12ier0Spec> {
        Adc12ie15W::new(self, 15)
    }
}
#[doc = "ADC12_B Interrupt Enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ier0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ier0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12ier0Spec;
impl crate::RegisterSpec for Adc12ier0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12ier0::R`](R) reader structure"]
impl crate::Readable for Adc12ier0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc12ier0::W`](W) writer structure"]
impl crate::Writable for Adc12ier0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADC12IER0 to value 0"]
impl crate::Resettable for Adc12ier0Spec {}
