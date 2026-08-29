#[doc = "Register `ADC12IFGR0` reader"]
pub type R = crate::R<Adc12ifgr0Spec>;
#[doc = "Register `ADC12IFGR0` writer"]
pub type W = crate::W<Adc12ifgr0Spec>;
#[doc = "ADC12MEM0 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg0 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg0_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg0_1 = 1,
}
impl From<Adc12ifg0> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG0` reader - ADC12MEM0 interrupt flag"]
pub type Adc12ifg0R = crate::BitReader<Adc12ifg0>;
impl Adc12ifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg0 {
        match self.bits {
            false => Adc12ifg0::Adc12ifg0_0,
            true => Adc12ifg0::Adc12ifg0_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg0_0(&self) -> bool {
        *self == Adc12ifg0::Adc12ifg0_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg0_1(&self) -> bool {
        *self == Adc12ifg0::Adc12ifg0_1
    }
}
#[doc = "Field `ADC12IFG0` writer - ADC12MEM0 interrupt flag"]
pub type Adc12ifg0W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg0>;
impl<'a, REG> Adc12ifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg0::Adc12ifg0_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg0::Adc12ifg0_1)
    }
}
#[doc = "ADC12MEM1 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg1 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg1_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg1_1 = 1,
}
impl From<Adc12ifg1> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG1` reader - ADC12MEM1 interrupt flag"]
pub type Adc12ifg1R = crate::BitReader<Adc12ifg1>;
impl Adc12ifg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg1 {
        match self.bits {
            false => Adc12ifg1::Adc12ifg1_0,
            true => Adc12ifg1::Adc12ifg1_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg1_0(&self) -> bool {
        *self == Adc12ifg1::Adc12ifg1_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg1_1(&self) -> bool {
        *self == Adc12ifg1::Adc12ifg1_1
    }
}
#[doc = "Field `ADC12IFG1` writer - ADC12MEM1 interrupt flag"]
pub type Adc12ifg1W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg1>;
impl<'a, REG> Adc12ifg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg1::Adc12ifg1_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg1::Adc12ifg1_1)
    }
}
#[doc = "ADC12MEM2 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg2 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg2_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg2_1 = 1,
}
impl From<Adc12ifg2> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG2` reader - ADC12MEM2 interrupt flag"]
pub type Adc12ifg2R = crate::BitReader<Adc12ifg2>;
impl Adc12ifg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg2 {
        match self.bits {
            false => Adc12ifg2::Adc12ifg2_0,
            true => Adc12ifg2::Adc12ifg2_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg2_0(&self) -> bool {
        *self == Adc12ifg2::Adc12ifg2_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg2_1(&self) -> bool {
        *self == Adc12ifg2::Adc12ifg2_1
    }
}
#[doc = "Field `ADC12IFG2` writer - ADC12MEM2 interrupt flag"]
pub type Adc12ifg2W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg2>;
impl<'a, REG> Adc12ifg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg2::Adc12ifg2_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg2::Adc12ifg2_1)
    }
}
#[doc = "ADC12MEM3 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg3 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg3_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg3_1 = 1,
}
impl From<Adc12ifg3> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG3` reader - ADC12MEM3 interrupt flag"]
pub type Adc12ifg3R = crate::BitReader<Adc12ifg3>;
impl Adc12ifg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg3 {
        match self.bits {
            false => Adc12ifg3::Adc12ifg3_0,
            true => Adc12ifg3::Adc12ifg3_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg3_0(&self) -> bool {
        *self == Adc12ifg3::Adc12ifg3_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg3_1(&self) -> bool {
        *self == Adc12ifg3::Adc12ifg3_1
    }
}
#[doc = "Field `ADC12IFG3` writer - ADC12MEM3 interrupt flag"]
pub type Adc12ifg3W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg3>;
impl<'a, REG> Adc12ifg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg3_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg3::Adc12ifg3_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg3::Adc12ifg3_1)
    }
}
#[doc = "ADC12MEM4 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg4 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg4_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg4_1 = 1,
}
impl From<Adc12ifg4> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG4` reader - ADC12MEM4 interrupt flag"]
pub type Adc12ifg4R = crate::BitReader<Adc12ifg4>;
impl Adc12ifg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg4 {
        match self.bits {
            false => Adc12ifg4::Adc12ifg4_0,
            true => Adc12ifg4::Adc12ifg4_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg4_0(&self) -> bool {
        *self == Adc12ifg4::Adc12ifg4_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg4_1(&self) -> bool {
        *self == Adc12ifg4::Adc12ifg4_1
    }
}
#[doc = "Field `ADC12IFG4` writer - ADC12MEM4 interrupt flag"]
pub type Adc12ifg4W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg4>;
impl<'a, REG> Adc12ifg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg4_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg4::Adc12ifg4_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg4_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg4::Adc12ifg4_1)
    }
}
#[doc = "ADC12MEM5 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg5 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg5_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg5_1 = 1,
}
impl From<Adc12ifg5> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG5` reader - ADC12MEM5 interrupt flag"]
pub type Adc12ifg5R = crate::BitReader<Adc12ifg5>;
impl Adc12ifg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg5 {
        match self.bits {
            false => Adc12ifg5::Adc12ifg5_0,
            true => Adc12ifg5::Adc12ifg5_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg5_0(&self) -> bool {
        *self == Adc12ifg5::Adc12ifg5_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg5_1(&self) -> bool {
        *self == Adc12ifg5::Adc12ifg5_1
    }
}
#[doc = "Field `ADC12IFG5` writer - ADC12MEM5 interrupt flag"]
pub type Adc12ifg5W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg5>;
impl<'a, REG> Adc12ifg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg5_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg5::Adc12ifg5_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg5_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg5::Adc12ifg5_1)
    }
}
#[doc = "ADC12MEM6 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg6 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg6_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg6_1 = 1,
}
impl From<Adc12ifg6> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG6` reader - ADC12MEM6 interrupt flag"]
pub type Adc12ifg6R = crate::BitReader<Adc12ifg6>;
impl Adc12ifg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg6 {
        match self.bits {
            false => Adc12ifg6::Adc12ifg6_0,
            true => Adc12ifg6::Adc12ifg6_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg6_0(&self) -> bool {
        *self == Adc12ifg6::Adc12ifg6_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg6_1(&self) -> bool {
        *self == Adc12ifg6::Adc12ifg6_1
    }
}
#[doc = "Field `ADC12IFG6` writer - ADC12MEM6 interrupt flag"]
pub type Adc12ifg6W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg6>;
impl<'a, REG> Adc12ifg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg6_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg6::Adc12ifg6_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg6_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg6::Adc12ifg6_1)
    }
}
#[doc = "ADC12MEM7 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg7 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg7_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg7_1 = 1,
}
impl From<Adc12ifg7> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG7` reader - ADC12MEM7 interrupt flag"]
pub type Adc12ifg7R = crate::BitReader<Adc12ifg7>;
impl Adc12ifg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg7 {
        match self.bits {
            false => Adc12ifg7::Adc12ifg7_0,
            true => Adc12ifg7::Adc12ifg7_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg7_0(&self) -> bool {
        *self == Adc12ifg7::Adc12ifg7_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg7_1(&self) -> bool {
        *self == Adc12ifg7::Adc12ifg7_1
    }
}
#[doc = "Field `ADC12IFG7` writer - ADC12MEM7 interrupt flag"]
pub type Adc12ifg7W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg7>;
impl<'a, REG> Adc12ifg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg7_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg7::Adc12ifg7_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg7_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg7::Adc12ifg7_1)
    }
}
#[doc = "ADC12MEM8 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg8 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg8_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg8_1 = 1,
}
impl From<Adc12ifg8> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG8` reader - ADC12MEM8 interrupt flag"]
pub type Adc12ifg8R = crate::BitReader<Adc12ifg8>;
impl Adc12ifg8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg8 {
        match self.bits {
            false => Adc12ifg8::Adc12ifg8_0,
            true => Adc12ifg8::Adc12ifg8_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg8_0(&self) -> bool {
        *self == Adc12ifg8::Adc12ifg8_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg8_1(&self) -> bool {
        *self == Adc12ifg8::Adc12ifg8_1
    }
}
#[doc = "Field `ADC12IFG8` writer - ADC12MEM8 interrupt flag"]
pub type Adc12ifg8W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg8>;
impl<'a, REG> Adc12ifg8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg8_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg8::Adc12ifg8_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg8_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg8::Adc12ifg8_1)
    }
}
#[doc = "ADC12MEM9 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg9 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg9_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg9_1 = 1,
}
impl From<Adc12ifg9> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG9` reader - ADC12MEM9 interrupt flag"]
pub type Adc12ifg9R = crate::BitReader<Adc12ifg9>;
impl Adc12ifg9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg9 {
        match self.bits {
            false => Adc12ifg9::Adc12ifg9_0,
            true => Adc12ifg9::Adc12ifg9_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg9_0(&self) -> bool {
        *self == Adc12ifg9::Adc12ifg9_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg9_1(&self) -> bool {
        *self == Adc12ifg9::Adc12ifg9_1
    }
}
#[doc = "Field `ADC12IFG9` writer - ADC12MEM9 interrupt flag"]
pub type Adc12ifg9W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg9>;
impl<'a, REG> Adc12ifg9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg9_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg9::Adc12ifg9_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg9_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg9::Adc12ifg9_1)
    }
}
#[doc = "ADC12MEM10 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg10 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg10_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg10_1 = 1,
}
impl From<Adc12ifg10> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG10` reader - ADC12MEM10 interrupt flag"]
pub type Adc12ifg10R = crate::BitReader<Adc12ifg10>;
impl Adc12ifg10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg10 {
        match self.bits {
            false => Adc12ifg10::Adc12ifg10_0,
            true => Adc12ifg10::Adc12ifg10_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg10_0(&self) -> bool {
        *self == Adc12ifg10::Adc12ifg10_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg10_1(&self) -> bool {
        *self == Adc12ifg10::Adc12ifg10_1
    }
}
#[doc = "Field `ADC12IFG10` writer - ADC12MEM10 interrupt flag"]
pub type Adc12ifg10W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg10>;
impl<'a, REG> Adc12ifg10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg10_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg10::Adc12ifg10_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg10_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg10::Adc12ifg10_1)
    }
}
#[doc = "ADC12MEM11 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg11 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg11_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg11_1 = 1,
}
impl From<Adc12ifg11> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG11` reader - ADC12MEM11 interrupt flag"]
pub type Adc12ifg11R = crate::BitReader<Adc12ifg11>;
impl Adc12ifg11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg11 {
        match self.bits {
            false => Adc12ifg11::Adc12ifg11_0,
            true => Adc12ifg11::Adc12ifg11_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg11_0(&self) -> bool {
        *self == Adc12ifg11::Adc12ifg11_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg11_1(&self) -> bool {
        *self == Adc12ifg11::Adc12ifg11_1
    }
}
#[doc = "Field `ADC12IFG11` writer - ADC12MEM11 interrupt flag"]
pub type Adc12ifg11W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg11>;
impl<'a, REG> Adc12ifg11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg11_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg11::Adc12ifg11_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg11_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg11::Adc12ifg11_1)
    }
}
#[doc = "ADC12MEM12 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg12 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg12_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg12_1 = 1,
}
impl From<Adc12ifg12> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG12` reader - ADC12MEM12 interrupt flag"]
pub type Adc12ifg12R = crate::BitReader<Adc12ifg12>;
impl Adc12ifg12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg12 {
        match self.bits {
            false => Adc12ifg12::Adc12ifg12_0,
            true => Adc12ifg12::Adc12ifg12_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg12_0(&self) -> bool {
        *self == Adc12ifg12::Adc12ifg12_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg12_1(&self) -> bool {
        *self == Adc12ifg12::Adc12ifg12_1
    }
}
#[doc = "Field `ADC12IFG12` writer - ADC12MEM12 interrupt flag"]
pub type Adc12ifg12W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg12>;
impl<'a, REG> Adc12ifg12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg12_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg12::Adc12ifg12_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg12_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg12::Adc12ifg12_1)
    }
}
#[doc = "ADC12MEM13 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg13 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg13_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg13_1 = 1,
}
impl From<Adc12ifg13> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG13` reader - ADC12MEM13 interrupt flag"]
pub type Adc12ifg13R = crate::BitReader<Adc12ifg13>;
impl Adc12ifg13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg13 {
        match self.bits {
            false => Adc12ifg13::Adc12ifg13_0,
            true => Adc12ifg13::Adc12ifg13_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg13_0(&self) -> bool {
        *self == Adc12ifg13::Adc12ifg13_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg13_1(&self) -> bool {
        *self == Adc12ifg13::Adc12ifg13_1
    }
}
#[doc = "Field `ADC12IFG13` writer - ADC12MEM13 interrupt flag"]
pub type Adc12ifg13W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg13>;
impl<'a, REG> Adc12ifg13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg13_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg13::Adc12ifg13_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg13_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg13::Adc12ifg13_1)
    }
}
#[doc = "ADC12MEM14 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg14 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg14_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg14_1 = 1,
}
impl From<Adc12ifg14> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG14` reader - ADC12MEM14 interrupt flag"]
pub type Adc12ifg14R = crate::BitReader<Adc12ifg14>;
impl Adc12ifg14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg14 {
        match self.bits {
            false => Adc12ifg14::Adc12ifg14_0,
            true => Adc12ifg14::Adc12ifg14_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg14_0(&self) -> bool {
        *self == Adc12ifg14::Adc12ifg14_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg14_1(&self) -> bool {
        *self == Adc12ifg14::Adc12ifg14_1
    }
}
#[doc = "Field `ADC12IFG14` writer - ADC12MEM14 interrupt flag"]
pub type Adc12ifg14W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg14>;
impl<'a, REG> Adc12ifg14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg14_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg14::Adc12ifg14_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg14_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg14::Adc12ifg14_1)
    }
}
#[doc = "ADC12MEM15 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg15 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg15_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg15_1 = 1,
}
impl From<Adc12ifg15> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG15` reader - ADC12MEM15 interrupt flag"]
pub type Adc12ifg15R = crate::BitReader<Adc12ifg15>;
impl Adc12ifg15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg15 {
        match self.bits {
            false => Adc12ifg15::Adc12ifg15_0,
            true => Adc12ifg15::Adc12ifg15_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg15_0(&self) -> bool {
        *self == Adc12ifg15::Adc12ifg15_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg15_1(&self) -> bool {
        *self == Adc12ifg15::Adc12ifg15_1
    }
}
#[doc = "Field `ADC12IFG15` writer - ADC12MEM15 interrupt flag"]
pub type Adc12ifg15W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg15>;
impl<'a, REG> Adc12ifg15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg15_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg15::Adc12ifg15_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg15_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg15::Adc12ifg15_1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC12MEM0 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg0(&self) -> Adc12ifg0R {
        Adc12ifg0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC12MEM1 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg1(&self) -> Adc12ifg1R {
        Adc12ifg1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC12MEM2 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg2(&self) -> Adc12ifg2R {
        Adc12ifg2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC12MEM3 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg3(&self) -> Adc12ifg3R {
        Adc12ifg3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC12MEM4 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg4(&self) -> Adc12ifg4R {
        Adc12ifg4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC12MEM5 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg5(&self) -> Adc12ifg5R {
        Adc12ifg5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC12MEM6 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg6(&self) -> Adc12ifg6R {
        Adc12ifg6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC12MEM7 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg7(&self) -> Adc12ifg7R {
        Adc12ifg7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC12MEM8 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg8(&self) -> Adc12ifg8R {
        Adc12ifg8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC12MEM9 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg9(&self) -> Adc12ifg9R {
        Adc12ifg9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC12MEM10 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg10(&self) -> Adc12ifg10R {
        Adc12ifg10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC12MEM11 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg11(&self) -> Adc12ifg11R {
        Adc12ifg11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADC12MEM12 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg12(&self) -> Adc12ifg12R {
        Adc12ifg12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC12MEM13 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg13(&self) -> Adc12ifg13R {
        Adc12ifg13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC12MEM14 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg14(&self) -> Adc12ifg14R {
        Adc12ifg14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC12MEM15 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg15(&self) -> Adc12ifg15R {
        Adc12ifg15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12MEM0 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg0(&mut self) -> Adc12ifg0W<'_, Adc12ifgr0Spec> {
        Adc12ifg0W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC12MEM1 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg1(&mut self) -> Adc12ifg1W<'_, Adc12ifgr0Spec> {
        Adc12ifg1W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC12MEM2 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg2(&mut self) -> Adc12ifg2W<'_, Adc12ifgr0Spec> {
        Adc12ifg2W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC12MEM3 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg3(&mut self) -> Adc12ifg3W<'_, Adc12ifgr0Spec> {
        Adc12ifg3W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC12MEM4 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg4(&mut self) -> Adc12ifg4W<'_, Adc12ifgr0Spec> {
        Adc12ifg4W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC12MEM5 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg5(&mut self) -> Adc12ifg5W<'_, Adc12ifgr0Spec> {
        Adc12ifg5W::new(self, 5)
    }
    #[doc = "Bit 6 - ADC12MEM6 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg6(&mut self) -> Adc12ifg6W<'_, Adc12ifgr0Spec> {
        Adc12ifg6W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC12MEM7 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg7(&mut self) -> Adc12ifg7W<'_, Adc12ifgr0Spec> {
        Adc12ifg7W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC12MEM8 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg8(&mut self) -> Adc12ifg8W<'_, Adc12ifgr0Spec> {
        Adc12ifg8W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC12MEM9 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg9(&mut self) -> Adc12ifg9W<'_, Adc12ifgr0Spec> {
        Adc12ifg9W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC12MEM10 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg10(&mut self) -> Adc12ifg10W<'_, Adc12ifgr0Spec> {
        Adc12ifg10W::new(self, 10)
    }
    #[doc = "Bit 11 - ADC12MEM11 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg11(&mut self) -> Adc12ifg11W<'_, Adc12ifgr0Spec> {
        Adc12ifg11W::new(self, 11)
    }
    #[doc = "Bit 12 - ADC12MEM12 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg12(&mut self) -> Adc12ifg12W<'_, Adc12ifgr0Spec> {
        Adc12ifg12W::new(self, 12)
    }
    #[doc = "Bit 13 - ADC12MEM13 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg13(&mut self) -> Adc12ifg13W<'_, Adc12ifgr0Spec> {
        Adc12ifg13W::new(self, 13)
    }
    #[doc = "Bit 14 - ADC12MEM14 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg14(&mut self) -> Adc12ifg14W<'_, Adc12ifgr0Spec> {
        Adc12ifg14W::new(self, 14)
    }
    #[doc = "Bit 15 - ADC12MEM15 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg15(&mut self) -> Adc12ifg15W<'_, Adc12ifgr0Spec> {
        Adc12ifg15W::new(self, 15)
    }
}
#[doc = "ADC12_B Interrupt Flag 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ifgr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ifgr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12ifgr0Spec;
impl crate::RegisterSpec for Adc12ifgr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12ifgr0::R`](R) reader structure"]
impl crate::Readable for Adc12ifgr0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc12ifgr0::W`](W) writer structure"]
impl crate::Writable for Adc12ifgr0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADC12IFGR0 to value 0"]
impl crate::Resettable for Adc12ifgr0Spec {}
