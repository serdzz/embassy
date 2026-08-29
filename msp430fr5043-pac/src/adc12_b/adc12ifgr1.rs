#[doc = "Register `ADC12IFGR1` reader"]
pub type R = crate::R<Adc12ifgr1Spec>;
#[doc = "Register `ADC12IFGR1` writer"]
pub type W = crate::W<Adc12ifgr1Spec>;
#[doc = "ADC12MEM16 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg16 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg16_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg16_1 = 1,
}
impl From<Adc12ifg16> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG16` reader - ADC12MEM16 interrupt flag"]
pub type Adc12ifg16R = crate::BitReader<Adc12ifg16>;
impl Adc12ifg16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg16 {
        match self.bits {
            false => Adc12ifg16::Adc12ifg16_0,
            true => Adc12ifg16::Adc12ifg16_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg16_0(&self) -> bool {
        *self == Adc12ifg16::Adc12ifg16_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg16_1(&self) -> bool {
        *self == Adc12ifg16::Adc12ifg16_1
    }
}
#[doc = "Field `ADC12IFG16` writer - ADC12MEM16 interrupt flag"]
pub type Adc12ifg16W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg16>;
impl<'a, REG> Adc12ifg16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg16_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg16::Adc12ifg16_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg16_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg16::Adc12ifg16_1)
    }
}
#[doc = "ADC12MEM17 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg17 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg17_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg17_1 = 1,
}
impl From<Adc12ifg17> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG17` reader - ADC12MEM17 interrupt flag"]
pub type Adc12ifg17R = crate::BitReader<Adc12ifg17>;
impl Adc12ifg17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg17 {
        match self.bits {
            false => Adc12ifg17::Adc12ifg17_0,
            true => Adc12ifg17::Adc12ifg17_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg17_0(&self) -> bool {
        *self == Adc12ifg17::Adc12ifg17_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg17_1(&self) -> bool {
        *self == Adc12ifg17::Adc12ifg17_1
    }
}
#[doc = "Field `ADC12IFG17` writer - ADC12MEM17 interrupt flag"]
pub type Adc12ifg17W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg17>;
impl<'a, REG> Adc12ifg17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg17_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg17::Adc12ifg17_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg17_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg17::Adc12ifg17_1)
    }
}
#[doc = "ADC12MEM18 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg18 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg18_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg18_1 = 1,
}
impl From<Adc12ifg18> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG18` reader - ADC12MEM18 interrupt flag"]
pub type Adc12ifg18R = crate::BitReader<Adc12ifg18>;
impl Adc12ifg18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg18 {
        match self.bits {
            false => Adc12ifg18::Adc12ifg18_0,
            true => Adc12ifg18::Adc12ifg18_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg18_0(&self) -> bool {
        *self == Adc12ifg18::Adc12ifg18_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg18_1(&self) -> bool {
        *self == Adc12ifg18::Adc12ifg18_1
    }
}
#[doc = "Field `ADC12IFG18` writer - ADC12MEM18 interrupt flag"]
pub type Adc12ifg18W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg18>;
impl<'a, REG> Adc12ifg18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg18_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg18::Adc12ifg18_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg18_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg18::Adc12ifg18_1)
    }
}
#[doc = "ADC12MEM19 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg19 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg19_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg19_1 = 1,
}
impl From<Adc12ifg19> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG19` reader - ADC12MEM19 interrupt flag"]
pub type Adc12ifg19R = crate::BitReader<Adc12ifg19>;
impl Adc12ifg19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg19 {
        match self.bits {
            false => Adc12ifg19::Adc12ifg19_0,
            true => Adc12ifg19::Adc12ifg19_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg19_0(&self) -> bool {
        *self == Adc12ifg19::Adc12ifg19_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg19_1(&self) -> bool {
        *self == Adc12ifg19::Adc12ifg19_1
    }
}
#[doc = "Field `ADC12IFG19` writer - ADC12MEM19 interrupt flag"]
pub type Adc12ifg19W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg19>;
impl<'a, REG> Adc12ifg19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg19_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg19::Adc12ifg19_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg19_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg19::Adc12ifg19_1)
    }
}
#[doc = "ADC12MEM20 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg20 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg20_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg20_1 = 1,
}
impl From<Adc12ifg20> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG20` reader - ADC12MEM20 interrupt flag"]
pub type Adc12ifg20R = crate::BitReader<Adc12ifg20>;
impl Adc12ifg20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg20 {
        match self.bits {
            false => Adc12ifg20::Adc12ifg20_0,
            true => Adc12ifg20::Adc12ifg20_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg20_0(&self) -> bool {
        *self == Adc12ifg20::Adc12ifg20_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg20_1(&self) -> bool {
        *self == Adc12ifg20::Adc12ifg20_1
    }
}
#[doc = "Field `ADC12IFG20` writer - ADC12MEM20 interrupt flag"]
pub type Adc12ifg20W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg20>;
impl<'a, REG> Adc12ifg20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg20_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg20::Adc12ifg20_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg20_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg20::Adc12ifg20_1)
    }
}
#[doc = "ADC12MEM21 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg21 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg21_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg21_1 = 1,
}
impl From<Adc12ifg21> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG21` reader - ADC12MEM21 interrupt flag"]
pub type Adc12ifg21R = crate::BitReader<Adc12ifg21>;
impl Adc12ifg21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg21 {
        match self.bits {
            false => Adc12ifg21::Adc12ifg21_0,
            true => Adc12ifg21::Adc12ifg21_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg21_0(&self) -> bool {
        *self == Adc12ifg21::Adc12ifg21_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg21_1(&self) -> bool {
        *self == Adc12ifg21::Adc12ifg21_1
    }
}
#[doc = "Field `ADC12IFG21` writer - ADC12MEM21 interrupt flag"]
pub type Adc12ifg21W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg21>;
impl<'a, REG> Adc12ifg21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg21_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg21::Adc12ifg21_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg21_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg21::Adc12ifg21_1)
    }
}
#[doc = "ADC12MEM22 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg22 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg22_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg22_1 = 1,
}
impl From<Adc12ifg22> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG22` reader - ADC12MEM22 interrupt flag"]
pub type Adc12ifg22R = crate::BitReader<Adc12ifg22>;
impl Adc12ifg22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg22 {
        match self.bits {
            false => Adc12ifg22::Adc12ifg22_0,
            true => Adc12ifg22::Adc12ifg22_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg22_0(&self) -> bool {
        *self == Adc12ifg22::Adc12ifg22_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg22_1(&self) -> bool {
        *self == Adc12ifg22::Adc12ifg22_1
    }
}
#[doc = "Field `ADC12IFG22` writer - ADC12MEM22 interrupt flag"]
pub type Adc12ifg22W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg22>;
impl<'a, REG> Adc12ifg22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg22_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg22::Adc12ifg22_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg22_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg22::Adc12ifg22_1)
    }
}
#[doc = "ADC12MEM23 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg23 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg23_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg23_1 = 1,
}
impl From<Adc12ifg23> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG23` reader - ADC12MEM23 interrupt flag"]
pub type Adc12ifg23R = crate::BitReader<Adc12ifg23>;
impl Adc12ifg23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg23 {
        match self.bits {
            false => Adc12ifg23::Adc12ifg23_0,
            true => Adc12ifg23::Adc12ifg23_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg23_0(&self) -> bool {
        *self == Adc12ifg23::Adc12ifg23_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg23_1(&self) -> bool {
        *self == Adc12ifg23::Adc12ifg23_1
    }
}
#[doc = "Field `ADC12IFG23` writer - ADC12MEM23 interrupt flag"]
pub type Adc12ifg23W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg23>;
impl<'a, REG> Adc12ifg23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg23_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg23::Adc12ifg23_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg23_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg23::Adc12ifg23_1)
    }
}
#[doc = "ADC12MEM24 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg24 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg24_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg24_1 = 1,
}
impl From<Adc12ifg24> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG24` reader - ADC12MEM24 interrupt flag"]
pub type Adc12ifg24R = crate::BitReader<Adc12ifg24>;
impl Adc12ifg24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg24 {
        match self.bits {
            false => Adc12ifg24::Adc12ifg24_0,
            true => Adc12ifg24::Adc12ifg24_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg24_0(&self) -> bool {
        *self == Adc12ifg24::Adc12ifg24_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg24_1(&self) -> bool {
        *self == Adc12ifg24::Adc12ifg24_1
    }
}
#[doc = "Field `ADC12IFG24` writer - ADC12MEM24 interrupt flag"]
pub type Adc12ifg24W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg24>;
impl<'a, REG> Adc12ifg24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg24_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg24::Adc12ifg24_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg24_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg24::Adc12ifg24_1)
    }
}
#[doc = "ADC12MEM25 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg25 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg25_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg25_1 = 1,
}
impl From<Adc12ifg25> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG25` reader - ADC12MEM25 interrupt flag"]
pub type Adc12ifg25R = crate::BitReader<Adc12ifg25>;
impl Adc12ifg25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg25 {
        match self.bits {
            false => Adc12ifg25::Adc12ifg25_0,
            true => Adc12ifg25::Adc12ifg25_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg25_0(&self) -> bool {
        *self == Adc12ifg25::Adc12ifg25_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg25_1(&self) -> bool {
        *self == Adc12ifg25::Adc12ifg25_1
    }
}
#[doc = "Field `ADC12IFG25` writer - ADC12MEM25 interrupt flag"]
pub type Adc12ifg25W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg25>;
impl<'a, REG> Adc12ifg25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg25_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg25::Adc12ifg25_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg25_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg25::Adc12ifg25_1)
    }
}
#[doc = "ADC12MEM26 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg26 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg26_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg26_1 = 1,
}
impl From<Adc12ifg26> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG26` reader - ADC12MEM26 interrupt flag"]
pub type Adc12ifg26R = crate::BitReader<Adc12ifg26>;
impl Adc12ifg26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg26 {
        match self.bits {
            false => Adc12ifg26::Adc12ifg26_0,
            true => Adc12ifg26::Adc12ifg26_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg26_0(&self) -> bool {
        *self == Adc12ifg26::Adc12ifg26_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg26_1(&self) -> bool {
        *self == Adc12ifg26::Adc12ifg26_1
    }
}
#[doc = "Field `ADC12IFG26` writer - ADC12MEM26 interrupt flag"]
pub type Adc12ifg26W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg26>;
impl<'a, REG> Adc12ifg26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg26_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg26::Adc12ifg26_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg26_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg26::Adc12ifg26_1)
    }
}
#[doc = "ADC12MEM27 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg27 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg27_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg27_1 = 1,
}
impl From<Adc12ifg27> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG27` reader - ADC12MEM27 interrupt flag"]
pub type Adc12ifg27R = crate::BitReader<Adc12ifg27>;
impl Adc12ifg27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg27 {
        match self.bits {
            false => Adc12ifg27::Adc12ifg27_0,
            true => Adc12ifg27::Adc12ifg27_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg27_0(&self) -> bool {
        *self == Adc12ifg27::Adc12ifg27_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg27_1(&self) -> bool {
        *self == Adc12ifg27::Adc12ifg27_1
    }
}
#[doc = "Field `ADC12IFG27` writer - ADC12MEM27 interrupt flag"]
pub type Adc12ifg27W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg27>;
impl<'a, REG> Adc12ifg27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg27_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg27::Adc12ifg27_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg27_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg27::Adc12ifg27_1)
    }
}
#[doc = "ADC12MEM28 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg28 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg28_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg28_1 = 1,
}
impl From<Adc12ifg28> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG28` reader - ADC12MEM28 interrupt flag"]
pub type Adc12ifg28R = crate::BitReader<Adc12ifg28>;
impl Adc12ifg28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg28 {
        match self.bits {
            false => Adc12ifg28::Adc12ifg28_0,
            true => Adc12ifg28::Adc12ifg28_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg28_0(&self) -> bool {
        *self == Adc12ifg28::Adc12ifg28_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg28_1(&self) -> bool {
        *self == Adc12ifg28::Adc12ifg28_1
    }
}
#[doc = "Field `ADC12IFG28` writer - ADC12MEM28 interrupt flag"]
pub type Adc12ifg28W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg28>;
impl<'a, REG> Adc12ifg28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg28_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg28::Adc12ifg28_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg28_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg28::Adc12ifg28_1)
    }
}
#[doc = "ADC12MEM29 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg29 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg29_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg29_1 = 1,
}
impl From<Adc12ifg29> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG29` reader - ADC12MEM29 interrupt flag"]
pub type Adc12ifg29R = crate::BitReader<Adc12ifg29>;
impl Adc12ifg29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg29 {
        match self.bits {
            false => Adc12ifg29::Adc12ifg29_0,
            true => Adc12ifg29::Adc12ifg29_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg29_0(&self) -> bool {
        *self == Adc12ifg29::Adc12ifg29_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg29_1(&self) -> bool {
        *self == Adc12ifg29::Adc12ifg29_1
    }
}
#[doc = "Field `ADC12IFG29` writer - ADC12MEM29 interrupt flag"]
pub type Adc12ifg29W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg29>;
impl<'a, REG> Adc12ifg29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg29_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg29::Adc12ifg29_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg29_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg29::Adc12ifg29_1)
    }
}
#[doc = "ADC12MEM30 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg30 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg30_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg30_1 = 1,
}
impl From<Adc12ifg30> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG30` reader - ADC12MEM30 interrupt flag"]
pub type Adc12ifg30R = crate::BitReader<Adc12ifg30>;
impl Adc12ifg30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg30 {
        match self.bits {
            false => Adc12ifg30::Adc12ifg30_0,
            true => Adc12ifg30::Adc12ifg30_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg30_0(&self) -> bool {
        *self == Adc12ifg30::Adc12ifg30_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg30_1(&self) -> bool {
        *self == Adc12ifg30::Adc12ifg30_1
    }
}
#[doc = "Field `ADC12IFG30` writer - ADC12MEM30 interrupt flag"]
pub type Adc12ifg30W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg30>;
impl<'a, REG> Adc12ifg30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg30_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg30::Adc12ifg30_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg30_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg30::Adc12ifg30_1)
    }
}
#[doc = "ADC12MEM31 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ifg31 {
    #[doc = "0: No interrupt pending"]
    Adc12ifg31_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ifg31_1 = 1,
}
impl From<Adc12ifg31> for bool {
    #[inline(always)]
    fn from(variant: Adc12ifg31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12IFG31` reader - ADC12MEM31 interrupt flag"]
pub type Adc12ifg31R = crate::BitReader<Adc12ifg31>;
impl Adc12ifg31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ifg31 {
        match self.bits {
            false => Adc12ifg31::Adc12ifg31_0,
            true => Adc12ifg31::Adc12ifg31_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg31_0(&self) -> bool {
        *self == Adc12ifg31::Adc12ifg31_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ifg31_1(&self) -> bool {
        *self == Adc12ifg31::Adc12ifg31_1
    }
}
#[doc = "Field `ADC12IFG31` writer - ADC12MEM31 interrupt flag"]
pub type Adc12ifg31W<'a, REG> = crate::BitWriter<'a, REG, Adc12ifg31>;
impl<'a, REG> Adc12ifg31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg31_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg31::Adc12ifg31_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg31_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ifg31::Adc12ifg31_1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC12MEM16 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg16(&self) -> Adc12ifg16R {
        Adc12ifg16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC12MEM17 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg17(&self) -> Adc12ifg17R {
        Adc12ifg17R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC12MEM18 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg18(&self) -> Adc12ifg18R {
        Adc12ifg18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC12MEM19 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg19(&self) -> Adc12ifg19R {
        Adc12ifg19R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC12MEM20 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg20(&self) -> Adc12ifg20R {
        Adc12ifg20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC12MEM21 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg21(&self) -> Adc12ifg21R {
        Adc12ifg21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC12MEM22 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg22(&self) -> Adc12ifg22R {
        Adc12ifg22R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC12MEM23 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg23(&self) -> Adc12ifg23R {
        Adc12ifg23R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC12MEM24 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg24(&self) -> Adc12ifg24R {
        Adc12ifg24R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC12MEM25 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg25(&self) -> Adc12ifg25R {
        Adc12ifg25R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC12MEM26 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg26(&self) -> Adc12ifg26R {
        Adc12ifg26R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC12MEM27 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg27(&self) -> Adc12ifg27R {
        Adc12ifg27R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADC12MEM28 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg28(&self) -> Adc12ifg28R {
        Adc12ifg28R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC12MEM29 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg29(&self) -> Adc12ifg29R {
        Adc12ifg29R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC12MEM30 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg30(&self) -> Adc12ifg30R {
        Adc12ifg30R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC12MEM31 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg31(&self) -> Adc12ifg31R {
        Adc12ifg31R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12MEM16 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg16(&mut self) -> Adc12ifg16W<'_, Adc12ifgr1Spec> {
        Adc12ifg16W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC12MEM17 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg17(&mut self) -> Adc12ifg17W<'_, Adc12ifgr1Spec> {
        Adc12ifg17W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC12MEM18 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg18(&mut self) -> Adc12ifg18W<'_, Adc12ifgr1Spec> {
        Adc12ifg18W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC12MEM19 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg19(&mut self) -> Adc12ifg19W<'_, Adc12ifgr1Spec> {
        Adc12ifg19W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC12MEM20 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg20(&mut self) -> Adc12ifg20W<'_, Adc12ifgr1Spec> {
        Adc12ifg20W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC12MEM21 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg21(&mut self) -> Adc12ifg21W<'_, Adc12ifgr1Spec> {
        Adc12ifg21W::new(self, 5)
    }
    #[doc = "Bit 6 - ADC12MEM22 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg22(&mut self) -> Adc12ifg22W<'_, Adc12ifgr1Spec> {
        Adc12ifg22W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC12MEM23 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg23(&mut self) -> Adc12ifg23W<'_, Adc12ifgr1Spec> {
        Adc12ifg23W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC12MEM24 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg24(&mut self) -> Adc12ifg24W<'_, Adc12ifgr1Spec> {
        Adc12ifg24W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC12MEM25 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg25(&mut self) -> Adc12ifg25W<'_, Adc12ifgr1Spec> {
        Adc12ifg25W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC12MEM26 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg26(&mut self) -> Adc12ifg26W<'_, Adc12ifgr1Spec> {
        Adc12ifg26W::new(self, 10)
    }
    #[doc = "Bit 11 - ADC12MEM27 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg27(&mut self) -> Adc12ifg27W<'_, Adc12ifgr1Spec> {
        Adc12ifg27W::new(self, 11)
    }
    #[doc = "Bit 12 - ADC12MEM28 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg28(&mut self) -> Adc12ifg28W<'_, Adc12ifgr1Spec> {
        Adc12ifg28W::new(self, 12)
    }
    #[doc = "Bit 13 - ADC12MEM29 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg29(&mut self) -> Adc12ifg29W<'_, Adc12ifgr1Spec> {
        Adc12ifg29W::new(self, 13)
    }
    #[doc = "Bit 14 - ADC12MEM30 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg30(&mut self) -> Adc12ifg30W<'_, Adc12ifgr1Spec> {
        Adc12ifg30W::new(self, 14)
    }
    #[doc = "Bit 15 - ADC12MEM31 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg31(&mut self) -> Adc12ifg31W<'_, Adc12ifgr1Spec> {
        Adc12ifg31W::new(self, 15)
    }
}
#[doc = "ADC12_B Interrupt Flag 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ifgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ifgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12ifgr1Spec;
impl crate::RegisterSpec for Adc12ifgr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12ifgr1::R`](R) reader structure"]
impl crate::Readable for Adc12ifgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc12ifgr1::W`](W) writer structure"]
impl crate::Writable for Adc12ifgr1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ADC12IFGR1 to value 0"]
impl crate::Resettable for Adc12ifgr1Spec {}
