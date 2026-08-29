#[doc = "Register `ADC12IER1` reader"]
pub type R = crate::R<Adc12ier1Spec>;
#[doc = "Register `ADC12IER1` writer"]
pub type W = crate::W<Adc12ier1Spec>;
#[doc = "interrupt enable 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie16 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie16_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie16_1 = 1,
}
impl From<Adc12ie16> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE16` reader - interrupt enable 16"]
pub type Adc12ie16R = crate::BitReader<Adc12ie16>;
impl Adc12ie16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie16 {
        match self.bits {
            false => Adc12ie16::Adc12ie16_0,
            true => Adc12ie16::Adc12ie16_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie16_0(&self) -> bool {
        *self == Adc12ie16::Adc12ie16_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie16_1(&self) -> bool {
        *self == Adc12ie16::Adc12ie16_1
    }
}
#[doc = "Field `ADC12IE16` writer - interrupt enable 16"]
pub type Adc12ie16W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie16>;
impl<'a, REG> Adc12ie16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie16_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie16::Adc12ie16_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie16_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie16::Adc12ie16_1)
    }
}
#[doc = "interrupt enable 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie17 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie17_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie17_1 = 1,
}
impl From<Adc12ie17> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE17` reader - interrupt enable 17"]
pub type Adc12ie17R = crate::BitReader<Adc12ie17>;
impl Adc12ie17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie17 {
        match self.bits {
            false => Adc12ie17::Adc12ie17_0,
            true => Adc12ie17::Adc12ie17_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie17_0(&self) -> bool {
        *self == Adc12ie17::Adc12ie17_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie17_1(&self) -> bool {
        *self == Adc12ie17::Adc12ie17_1
    }
}
#[doc = "Field `ADC12IE17` writer - interrupt enable 17"]
pub type Adc12ie17W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie17>;
impl<'a, REG> Adc12ie17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie17_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie17::Adc12ie17_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie17_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie17::Adc12ie17_1)
    }
}
#[doc = "interrupt enable 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie18 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie18_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie18_1 = 1,
}
impl From<Adc12ie18> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE18` reader - interrupt enable 18"]
pub type Adc12ie18R = crate::BitReader<Adc12ie18>;
impl Adc12ie18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie18 {
        match self.bits {
            false => Adc12ie18::Adc12ie18_0,
            true => Adc12ie18::Adc12ie18_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie18_0(&self) -> bool {
        *self == Adc12ie18::Adc12ie18_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie18_1(&self) -> bool {
        *self == Adc12ie18::Adc12ie18_1
    }
}
#[doc = "Field `ADC12IE18` writer - interrupt enable 18"]
pub type Adc12ie18W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie18>;
impl<'a, REG> Adc12ie18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie18_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie18::Adc12ie18_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie18_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie18::Adc12ie18_1)
    }
}
#[doc = "interrupt enable 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie19 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie19_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie19_1 = 1,
}
impl From<Adc12ie19> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE19` reader - interrupt enable 19"]
pub type Adc12ie19R = crate::BitReader<Adc12ie19>;
impl Adc12ie19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie19 {
        match self.bits {
            false => Adc12ie19::Adc12ie19_0,
            true => Adc12ie19::Adc12ie19_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie19_0(&self) -> bool {
        *self == Adc12ie19::Adc12ie19_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie19_1(&self) -> bool {
        *self == Adc12ie19::Adc12ie19_1
    }
}
#[doc = "Field `ADC12IE19` writer - interrupt enable 19"]
pub type Adc12ie19W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie19>;
impl<'a, REG> Adc12ie19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie19_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie19::Adc12ie19_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie19_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie19::Adc12ie19_1)
    }
}
#[doc = "interrupt enable 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie20 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie20_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie20_1 = 1,
}
impl From<Adc12ie20> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE20` reader - interrupt enable 19"]
pub type Adc12ie20R = crate::BitReader<Adc12ie20>;
impl Adc12ie20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie20 {
        match self.bits {
            false => Adc12ie20::Adc12ie20_0,
            true => Adc12ie20::Adc12ie20_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie20_0(&self) -> bool {
        *self == Adc12ie20::Adc12ie20_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie20_1(&self) -> bool {
        *self == Adc12ie20::Adc12ie20_1
    }
}
#[doc = "Field `ADC12IE20` writer - interrupt enable 19"]
pub type Adc12ie20W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie20>;
impl<'a, REG> Adc12ie20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie20_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie20::Adc12ie20_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie20_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie20::Adc12ie20_1)
    }
}
#[doc = "interrupt enable 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie21 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie21_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie21_1 = 1,
}
impl From<Adc12ie21> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE21` reader - interrupt enable 21"]
pub type Adc12ie21R = crate::BitReader<Adc12ie21>;
impl Adc12ie21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie21 {
        match self.bits {
            false => Adc12ie21::Adc12ie21_0,
            true => Adc12ie21::Adc12ie21_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie21_0(&self) -> bool {
        *self == Adc12ie21::Adc12ie21_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie21_1(&self) -> bool {
        *self == Adc12ie21::Adc12ie21_1
    }
}
#[doc = "Field `ADC12IE21` writer - interrupt enable 21"]
pub type Adc12ie21W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie21>;
impl<'a, REG> Adc12ie21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie21_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie21::Adc12ie21_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie21_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie21::Adc12ie21_1)
    }
}
#[doc = "interrupt enable 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie22 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie22_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie22_1 = 1,
}
impl From<Adc12ie22> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE22` reader - interrupt enable 22"]
pub type Adc12ie22R = crate::BitReader<Adc12ie22>;
impl Adc12ie22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie22 {
        match self.bits {
            false => Adc12ie22::Adc12ie22_0,
            true => Adc12ie22::Adc12ie22_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie22_0(&self) -> bool {
        *self == Adc12ie22::Adc12ie22_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie22_1(&self) -> bool {
        *self == Adc12ie22::Adc12ie22_1
    }
}
#[doc = "Field `ADC12IE22` writer - interrupt enable 22"]
pub type Adc12ie22W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie22>;
impl<'a, REG> Adc12ie22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie22_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie22::Adc12ie22_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie22_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie22::Adc12ie22_1)
    }
}
#[doc = "interrupt enable 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie23 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie23_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie23_1 = 1,
}
impl From<Adc12ie23> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE23` reader - interrupt enable 23"]
pub type Adc12ie23R = crate::BitReader<Adc12ie23>;
impl Adc12ie23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie23 {
        match self.bits {
            false => Adc12ie23::Adc12ie23_0,
            true => Adc12ie23::Adc12ie23_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie23_0(&self) -> bool {
        *self == Adc12ie23::Adc12ie23_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie23_1(&self) -> bool {
        *self == Adc12ie23::Adc12ie23_1
    }
}
#[doc = "Field `ADC12IE23` writer - interrupt enable 23"]
pub type Adc12ie23W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie23>;
impl<'a, REG> Adc12ie23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie23_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie23::Adc12ie23_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie23_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie23::Adc12ie23_1)
    }
}
#[doc = "interrupt enable 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie24 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie24_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie24_1 = 1,
}
impl From<Adc12ie24> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE24` reader - interrupt enable 24"]
pub type Adc12ie24R = crate::BitReader<Adc12ie24>;
impl Adc12ie24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie24 {
        match self.bits {
            false => Adc12ie24::Adc12ie24_0,
            true => Adc12ie24::Adc12ie24_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie24_0(&self) -> bool {
        *self == Adc12ie24::Adc12ie24_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie24_1(&self) -> bool {
        *self == Adc12ie24::Adc12ie24_1
    }
}
#[doc = "Field `ADC12IE24` writer - interrupt enable 24"]
pub type Adc12ie24W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie24>;
impl<'a, REG> Adc12ie24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie24_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie24::Adc12ie24_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie24_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie24::Adc12ie24_1)
    }
}
#[doc = "interrupt enable 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie25 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie25_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie25_1 = 1,
}
impl From<Adc12ie25> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE25` reader - interrupt enable 25"]
pub type Adc12ie25R = crate::BitReader<Adc12ie25>;
impl Adc12ie25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie25 {
        match self.bits {
            false => Adc12ie25::Adc12ie25_0,
            true => Adc12ie25::Adc12ie25_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie25_0(&self) -> bool {
        *self == Adc12ie25::Adc12ie25_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie25_1(&self) -> bool {
        *self == Adc12ie25::Adc12ie25_1
    }
}
#[doc = "Field `ADC12IE25` writer - interrupt enable 25"]
pub type Adc12ie25W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie25>;
impl<'a, REG> Adc12ie25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie25_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie25::Adc12ie25_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie25_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie25::Adc12ie25_1)
    }
}
#[doc = "interrupt enable 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie26 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie26_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie26_1 = 1,
}
impl From<Adc12ie26> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE26` reader - interrupt enable 26"]
pub type Adc12ie26R = crate::BitReader<Adc12ie26>;
impl Adc12ie26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie26 {
        match self.bits {
            false => Adc12ie26::Adc12ie26_0,
            true => Adc12ie26::Adc12ie26_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie26_0(&self) -> bool {
        *self == Adc12ie26::Adc12ie26_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie26_1(&self) -> bool {
        *self == Adc12ie26::Adc12ie26_1
    }
}
#[doc = "Field `ADC12IE26` writer - interrupt enable 26"]
pub type Adc12ie26W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie26>;
impl<'a, REG> Adc12ie26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie26_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie26::Adc12ie26_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie26_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie26::Adc12ie26_1)
    }
}
#[doc = "interrupt enable 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie27 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie27_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie27_1 = 1,
}
impl From<Adc12ie27> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE27` reader - interrupt enable 27"]
pub type Adc12ie27R = crate::BitReader<Adc12ie27>;
impl Adc12ie27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie27 {
        match self.bits {
            false => Adc12ie27::Adc12ie27_0,
            true => Adc12ie27::Adc12ie27_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie27_0(&self) -> bool {
        *self == Adc12ie27::Adc12ie27_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie27_1(&self) -> bool {
        *self == Adc12ie27::Adc12ie27_1
    }
}
#[doc = "Field `ADC12IE27` writer - interrupt enable 27"]
pub type Adc12ie27W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie27>;
impl<'a, REG> Adc12ie27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie27_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie27::Adc12ie27_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie27_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie27::Adc12ie27_1)
    }
}
#[doc = "interrupt enable 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie28 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie28_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie28_1 = 1,
}
impl From<Adc12ie28> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE28` reader - interrupt enable 28"]
pub type Adc12ie28R = crate::BitReader<Adc12ie28>;
impl Adc12ie28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie28 {
        match self.bits {
            false => Adc12ie28::Adc12ie28_0,
            true => Adc12ie28::Adc12ie28_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie28_0(&self) -> bool {
        *self == Adc12ie28::Adc12ie28_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie28_1(&self) -> bool {
        *self == Adc12ie28::Adc12ie28_1
    }
}
#[doc = "Field `ADC12IE28` writer - interrupt enable 28"]
pub type Adc12ie28W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie28>;
impl<'a, REG> Adc12ie28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie28_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie28::Adc12ie28_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie28_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie28::Adc12ie28_1)
    }
}
#[doc = "interrupt enable 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie29 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie29_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie29_1 = 1,
}
impl From<Adc12ie29> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE29` reader - interrupt enable 29"]
pub type Adc12ie29R = crate::BitReader<Adc12ie29>;
impl Adc12ie29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie29 {
        match self.bits {
            false => Adc12ie29::Adc12ie29_0,
            true => Adc12ie29::Adc12ie29_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie29_0(&self) -> bool {
        *self == Adc12ie29::Adc12ie29_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie29_1(&self) -> bool {
        *self == Adc12ie29::Adc12ie29_1
    }
}
#[doc = "Field `ADC12IE29` writer - interrupt enable 29"]
pub type Adc12ie29W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie29>;
impl<'a, REG> Adc12ie29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie29_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie29::Adc12ie29_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie29_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie29::Adc12ie29_1)
    }
}
#[doc = "interrupt enable 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie30 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie30_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie30_1 = 1,
}
impl From<Adc12ie30> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE30` reader - interrupt enable 30"]
pub type Adc12ie30R = crate::BitReader<Adc12ie30>;
impl Adc12ie30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie30 {
        match self.bits {
            false => Adc12ie30::Adc12ie30_0,
            true => Adc12ie30::Adc12ie30_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie30_0(&self) -> bool {
        *self == Adc12ie30::Adc12ie30_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie30_1(&self) -> bool {
        *self == Adc12ie30::Adc12ie30_1
    }
}
#[doc = "Field `ADC12IE30` writer - interrupt enable 30"]
pub type Adc12ie30W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie30>;
impl<'a, REG> Adc12ie30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie30_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie30::Adc12ie30_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie30_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie30::Adc12ie30_1)
    }
}
#[doc = "interrupt enable 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ie31 {
    #[doc = "0: Interrupt disabled"]
    Adc12ie31_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ie31_1 = 1,
}
impl From<Adc12ie31> for bool {
    #[inline(always)]
    fn from(variant: Adc12ie31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IE31` reader - interrupt enable 30"]
pub type Adc12ie31R = crate::BitReader<Adc12ie31>;
impl Adc12ie31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ie31 {
        match self.bits {
            false => Adc12ie31::Adc12ie31_0,
            true => Adc12ie31::Adc12ie31_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ie31_0(&self) -> bool {
        *self == Adc12ie31::Adc12ie31_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ie31_1(&self) -> bool {
        *self == Adc12ie31::Adc12ie31_1
    }
}
#[doc = "Field `ADC12IE31` writer - interrupt enable 30"]
pub type Adc12ie31W<'a, REG> = crate::BitWriter<'a, REG, Adc12ie31>;
impl<'a, REG> Adc12ie31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie31_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie31::Adc12ie31_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie31_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ie31::Adc12ie31_1)
    }
}
impl R {
    #[doc = "Bit 0 - interrupt enable 16"]
    #[inline(always)]
    pub fn adc12ie16(&self) -> Adc12ie16R {
        Adc12ie16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt enable 17"]
    #[inline(always)]
    pub fn adc12ie17(&self) -> Adc12ie17R {
        Adc12ie17R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt enable 18"]
    #[inline(always)]
    pub fn adc12ie18(&self) -> Adc12ie18R {
        Adc12ie18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt enable 19"]
    #[inline(always)]
    pub fn adc12ie19(&self) -> Adc12ie19R {
        Adc12ie19R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - interrupt enable 19"]
    #[inline(always)]
    pub fn adc12ie20(&self) -> Adc12ie20R {
        Adc12ie20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - interrupt enable 21"]
    #[inline(always)]
    pub fn adc12ie21(&self) -> Adc12ie21R {
        Adc12ie21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - interrupt enable 22"]
    #[inline(always)]
    pub fn adc12ie22(&self) -> Adc12ie22R {
        Adc12ie22R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - interrupt enable 23"]
    #[inline(always)]
    pub fn adc12ie23(&self) -> Adc12ie23R {
        Adc12ie23R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - interrupt enable 24"]
    #[inline(always)]
    pub fn adc12ie24(&self) -> Adc12ie24R {
        Adc12ie24R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - interrupt enable 25"]
    #[inline(always)]
    pub fn adc12ie25(&self) -> Adc12ie25R {
        Adc12ie25R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - interrupt enable 26"]
    #[inline(always)]
    pub fn adc12ie26(&self) -> Adc12ie26R {
        Adc12ie26R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - interrupt enable 27"]
    #[inline(always)]
    pub fn adc12ie27(&self) -> Adc12ie27R {
        Adc12ie27R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - interrupt enable 28"]
    #[inline(always)]
    pub fn adc12ie28(&self) -> Adc12ie28R {
        Adc12ie28R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - interrupt enable 29"]
    #[inline(always)]
    pub fn adc12ie29(&self) -> Adc12ie29R {
        Adc12ie29R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - interrupt enable 30"]
    #[inline(always)]
    pub fn adc12ie30(&self) -> Adc12ie30R {
        Adc12ie30R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - interrupt enable 30"]
    #[inline(always)]
    pub fn adc12ie31(&self) -> Adc12ie31R {
        Adc12ie31R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interrupt enable 16"]
    #[inline(always)]
    pub fn adc12ie16(&mut self) -> Adc12ie16W<'_, Adc12ier1Spec> {
        Adc12ie16W::new(self, 0)
    }
    #[doc = "Bit 1 - interrupt enable 17"]
    #[inline(always)]
    pub fn adc12ie17(&mut self) -> Adc12ie17W<'_, Adc12ier1Spec> {
        Adc12ie17W::new(self, 1)
    }
    #[doc = "Bit 2 - interrupt enable 18"]
    #[inline(always)]
    pub fn adc12ie18(&mut self) -> Adc12ie18W<'_, Adc12ier1Spec> {
        Adc12ie18W::new(self, 2)
    }
    #[doc = "Bit 3 - interrupt enable 19"]
    #[inline(always)]
    pub fn adc12ie19(&mut self) -> Adc12ie19W<'_, Adc12ier1Spec> {
        Adc12ie19W::new(self, 3)
    }
    #[doc = "Bit 4 - interrupt enable 19"]
    #[inline(always)]
    pub fn adc12ie20(&mut self) -> Adc12ie20W<'_, Adc12ier1Spec> {
        Adc12ie20W::new(self, 4)
    }
    #[doc = "Bit 5 - interrupt enable 21"]
    #[inline(always)]
    pub fn adc12ie21(&mut self) -> Adc12ie21W<'_, Adc12ier1Spec> {
        Adc12ie21W::new(self, 5)
    }
    #[doc = "Bit 6 - interrupt enable 22"]
    #[inline(always)]
    pub fn adc12ie22(&mut self) -> Adc12ie22W<'_, Adc12ier1Spec> {
        Adc12ie22W::new(self, 6)
    }
    #[doc = "Bit 7 - interrupt enable 23"]
    #[inline(always)]
    pub fn adc12ie23(&mut self) -> Adc12ie23W<'_, Adc12ier1Spec> {
        Adc12ie23W::new(self, 7)
    }
    #[doc = "Bit 8 - interrupt enable 24"]
    #[inline(always)]
    pub fn adc12ie24(&mut self) -> Adc12ie24W<'_, Adc12ier1Spec> {
        Adc12ie24W::new(self, 8)
    }
    #[doc = "Bit 9 - interrupt enable 25"]
    #[inline(always)]
    pub fn adc12ie25(&mut self) -> Adc12ie25W<'_, Adc12ier1Spec> {
        Adc12ie25W::new(self, 9)
    }
    #[doc = "Bit 10 - interrupt enable 26"]
    #[inline(always)]
    pub fn adc12ie26(&mut self) -> Adc12ie26W<'_, Adc12ier1Spec> {
        Adc12ie26W::new(self, 10)
    }
    #[doc = "Bit 11 - interrupt enable 27"]
    #[inline(always)]
    pub fn adc12ie27(&mut self) -> Adc12ie27W<'_, Adc12ier1Spec> {
        Adc12ie27W::new(self, 11)
    }
    #[doc = "Bit 12 - interrupt enable 28"]
    #[inline(always)]
    pub fn adc12ie28(&mut self) -> Adc12ie28W<'_, Adc12ier1Spec> {
        Adc12ie28W::new(self, 12)
    }
    #[doc = "Bit 13 - interrupt enable 29"]
    #[inline(always)]
    pub fn adc12ie29(&mut self) -> Adc12ie29W<'_, Adc12ier1Spec> {
        Adc12ie29W::new(self, 13)
    }
    #[doc = "Bit 14 - interrupt enable 30"]
    #[inline(always)]
    pub fn adc12ie30(&mut self) -> Adc12ie30W<'_, Adc12ier1Spec> {
        Adc12ie30W::new(self, 14)
    }
    #[doc = "Bit 15 - interrupt enable 30"]
    #[inline(always)]
    pub fn adc12ie31(&mut self) -> Adc12ie31W<'_, Adc12ier1Spec> {
        Adc12ie31W::new(self, 15)
    }
}
#[doc = "ADC12_B Interrupt Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12ier1Spec;
impl crate::RegisterSpec for Adc12ier1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12ier1::R`](R) reader structure"]
impl crate::Readable for Adc12ier1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc12ier1::W`](W) writer structure"]
impl crate::Writable for Adc12ier1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADC12IER1 to value 0"]
impl crate::Resettable for Adc12ier1Spec {}
