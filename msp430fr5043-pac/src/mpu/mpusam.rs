#[doc = "Register `MPUSAM` reader"]
pub type R = crate::R<MpusamSpec>;
#[doc = "Register `MPUSAM` writer"]
pub type W = crate::W<MpusamSpec>;
#[doc = "MPU Main Memory Segment 1 Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuseg1re {
    #[doc = "0: Read on Main Memory Segment 1 causes a violation if MPUSEG1WE = MPUSEG1XE = 0"]
    Disable = 0,
    #[doc = "1: Read on Main Memory Segment 1 is allowed"]
    Enable = 1,
}
impl From<Mpuseg1re> for bool {
    #[inline(always)]
    fn from(variant: Mpuseg1re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEG1RE` reader - MPU Main Memory Segment 1 Read Enable"]
pub type Mpuseg1reR = crate::BitReader<Mpuseg1re>;
impl Mpuseg1reR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuseg1re {
        match self.bits {
            false => Mpuseg1re::Disable,
            true => Mpuseg1re::Enable,
        }
    }
    #[doc = "Read on Main Memory Segment 1 causes a violation if MPUSEG1WE = MPUSEG1XE = 0"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpuseg1re::Disable
    }
    #[doc = "Read on Main Memory Segment 1 is allowed"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpuseg1re::Enable
    }
}
#[doc = "Field `MPUSEG1RE` writer - MPU Main Memory Segment 1 Read Enable"]
pub type Mpuseg1reW<'a, REG> = crate::BitWriter<'a, REG, Mpuseg1re>;
impl<'a, REG> Mpuseg1reW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read on Main Memory Segment 1 causes a violation if MPUSEG1WE = MPUSEG1XE = 0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg1re::Disable)
    }
    #[doc = "Read on Main Memory Segment 1 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg1re::Enable)
    }
}
#[doc = "MPU Main Memory Segment 1 Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuseg1we {
    #[doc = "0: Write on Main Memory Segment 1 causes a violation"]
    Disable = 0,
    #[doc = "1: Write on Main Memory Segment 1 is allowed"]
    Enable = 1,
}
impl From<Mpuseg1we> for bool {
    #[inline(always)]
    fn from(variant: Mpuseg1we) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEG1WE` reader - MPU Main Memory Segment 1 Write Enable"]
pub type Mpuseg1weR = crate::BitReader<Mpuseg1we>;
impl Mpuseg1weR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuseg1we {
        match self.bits {
            false => Mpuseg1we::Disable,
            true => Mpuseg1we::Enable,
        }
    }
    #[doc = "Write on Main Memory Segment 1 causes a violation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpuseg1we::Disable
    }
    #[doc = "Write on Main Memory Segment 1 is allowed"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpuseg1we::Enable
    }
}
#[doc = "Field `MPUSEG1WE` writer - MPU Main Memory Segment 1 Write Enable"]
pub type Mpuseg1weW<'a, REG> = crate::BitWriter<'a, REG, Mpuseg1we>;
impl<'a, REG> Mpuseg1weW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write on Main Memory Segment 1 causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg1we::Disable)
    }
    #[doc = "Write on Main Memory Segment 1 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg1we::Enable)
    }
}
#[doc = "MPU Main Memory Segment 1 Execute Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuseg1xe {
    #[doc = "0: Execute code on Main Memory Segment 1 causes a violation"]
    Disable = 0,
    #[doc = "1: Execute code on Main Memory Segment 1 is allowed"]
    Enable = 1,
}
impl From<Mpuseg1xe> for bool {
    #[inline(always)]
    fn from(variant: Mpuseg1xe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEG1XE` reader - MPU Main Memory Segment 1 Execute Enable"]
pub type Mpuseg1xeR = crate::BitReader<Mpuseg1xe>;
impl Mpuseg1xeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuseg1xe {
        match self.bits {
            false => Mpuseg1xe::Disable,
            true => Mpuseg1xe::Enable,
        }
    }
    #[doc = "Execute code on Main Memory Segment 1 causes a violation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpuseg1xe::Disable
    }
    #[doc = "Execute code on Main Memory Segment 1 is allowed"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpuseg1xe::Enable
    }
}
#[doc = "Field `MPUSEG1XE` writer - MPU Main Memory Segment 1 Execute Enable"]
pub type Mpuseg1xeW<'a, REG> = crate::BitWriter<'a, REG, Mpuseg1xe>;
impl<'a, REG> Mpuseg1xeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Execute code on Main Memory Segment 1 causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg1xe::Disable)
    }
    #[doc = "Execute code on Main Memory Segment 1 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg1xe::Enable)
    }
}
#[doc = "MPU Main Memory Segment 1 Violation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuseg1vs {
    #[doc = "0: Violation in Main Memory Segment 1 asserts the MPUSEG1IFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    Mpuseg1vs0 = 0,
    #[doc = "1: Violation in Main Memory Segment 1 asserts the MPUSEG1IFG bit and executes a PUC"]
    Mpuseg1vs1 = 1,
}
impl From<Mpuseg1vs> for bool {
    #[inline(always)]
    fn from(variant: Mpuseg1vs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEG1VS` reader - MPU Main Memory Segment 1 Violation Select"]
pub type Mpuseg1vsR = crate::BitReader<Mpuseg1vs>;
impl Mpuseg1vsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuseg1vs {
        match self.bits {
            false => Mpuseg1vs::Mpuseg1vs0,
            true => Mpuseg1vs::Mpuseg1vs1,
        }
    }
    #[doc = "Violation in Main Memory Segment 1 asserts the MPUSEG1IFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    #[inline(always)]
    pub fn is_mpuseg1vs_0(&self) -> bool {
        *self == Mpuseg1vs::Mpuseg1vs0
    }
    #[doc = "Violation in Main Memory Segment 1 asserts the MPUSEG1IFG bit and executes a PUC"]
    #[inline(always)]
    pub fn is_mpuseg1vs_1(&self) -> bool {
        *self == Mpuseg1vs::Mpuseg1vs1
    }
}
#[doc = "Field `MPUSEG1VS` writer - MPU Main Memory Segment 1 Violation Select"]
pub type Mpuseg1vsW<'a, REG> = crate::BitWriter<'a, REG, Mpuseg1vs>;
impl<'a, REG> Mpuseg1vsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Violation in Main Memory Segment 1 asserts the MPUSEG1IFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    #[inline(always)]
    pub fn mpuseg1vs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg1vs::Mpuseg1vs0)
    }
    #[doc = "Violation in Main Memory Segment 1 asserts the MPUSEG1IFG bit and executes a PUC"]
    #[inline(always)]
    pub fn mpuseg1vs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg1vs::Mpuseg1vs1)
    }
}
#[doc = "MPU Main Memory Segment 2 Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuseg2re {
    #[doc = "0: Read on Main Memory Segment 2 causes a violation if MPUSEG2WE = MPUSEG2XE = 0"]
    Disable = 0,
    #[doc = "1: Read on Main Memory Segment 2 is allowed"]
    Enable = 1,
}
impl From<Mpuseg2re> for bool {
    #[inline(always)]
    fn from(variant: Mpuseg2re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEG2RE` reader - MPU Main Memory Segment 2 Read Enable"]
pub type Mpuseg2reR = crate::BitReader<Mpuseg2re>;
impl Mpuseg2reR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuseg2re {
        match self.bits {
            false => Mpuseg2re::Disable,
            true => Mpuseg2re::Enable,
        }
    }
    #[doc = "Read on Main Memory Segment 2 causes a violation if MPUSEG2WE = MPUSEG2XE = 0"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpuseg2re::Disable
    }
    #[doc = "Read on Main Memory Segment 2 is allowed"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpuseg2re::Enable
    }
}
#[doc = "Field `MPUSEG2RE` writer - MPU Main Memory Segment 2 Read Enable"]
pub type Mpuseg2reW<'a, REG> = crate::BitWriter<'a, REG, Mpuseg2re>;
impl<'a, REG> Mpuseg2reW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read on Main Memory Segment 2 causes a violation if MPUSEG2WE = MPUSEG2XE = 0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg2re::Disable)
    }
    #[doc = "Read on Main Memory Segment 2 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg2re::Enable)
    }
}
#[doc = "MPU Main Memory Segment 2 Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuseg2we {
    #[doc = "0: Write on Main Memory Segment 2 causes a violation"]
    Disable = 0,
    #[doc = "1: Write on Main Memory Segment 2 is allowed"]
    Enable = 1,
}
impl From<Mpuseg2we> for bool {
    #[inline(always)]
    fn from(variant: Mpuseg2we) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEG2WE` reader - MPU Main Memory Segment 2 Write Enable"]
pub type Mpuseg2weR = crate::BitReader<Mpuseg2we>;
impl Mpuseg2weR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuseg2we {
        match self.bits {
            false => Mpuseg2we::Disable,
            true => Mpuseg2we::Enable,
        }
    }
    #[doc = "Write on Main Memory Segment 2 causes a violation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpuseg2we::Disable
    }
    #[doc = "Write on Main Memory Segment 2 is allowed"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpuseg2we::Enable
    }
}
#[doc = "Field `MPUSEG2WE` writer - MPU Main Memory Segment 2 Write Enable"]
pub type Mpuseg2weW<'a, REG> = crate::BitWriter<'a, REG, Mpuseg2we>;
impl<'a, REG> Mpuseg2weW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write on Main Memory Segment 2 causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg2we::Disable)
    }
    #[doc = "Write on Main Memory Segment 2 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg2we::Enable)
    }
}
#[doc = "MPU Main Memory Segment 2 Execute Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuseg2xe {
    #[doc = "0: Execute code on Main Memory Segment 2 causes a violation"]
    Disable = 0,
    #[doc = "1: Execute code on Main Memory Segment 2 is allowed"]
    Enable = 1,
}
impl From<Mpuseg2xe> for bool {
    #[inline(always)]
    fn from(variant: Mpuseg2xe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEG2XE` reader - MPU Main Memory Segment 2 Execute Enable"]
pub type Mpuseg2xeR = crate::BitReader<Mpuseg2xe>;
impl Mpuseg2xeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuseg2xe {
        match self.bits {
            false => Mpuseg2xe::Disable,
            true => Mpuseg2xe::Enable,
        }
    }
    #[doc = "Execute code on Main Memory Segment 2 causes a violation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpuseg2xe::Disable
    }
    #[doc = "Execute code on Main Memory Segment 2 is allowed"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpuseg2xe::Enable
    }
}
#[doc = "Field `MPUSEG2XE` writer - MPU Main Memory Segment 2 Execute Enable"]
pub type Mpuseg2xeW<'a, REG> = crate::BitWriter<'a, REG, Mpuseg2xe>;
impl<'a, REG> Mpuseg2xeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Execute code on Main Memory Segment 2 causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg2xe::Disable)
    }
    #[doc = "Execute code on Main Memory Segment 2 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg2xe::Enable)
    }
}
#[doc = "MPU Main Memory Segment 2 Violation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuseg2vs {
    #[doc = "0: Violation in Main Memory Segment 2 asserts the MPUSEG2IFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    Mpuseg2vs0 = 0,
    #[doc = "1: Violation in Main Memory Segment 2 asserts the MPUSEG2IFG bit and executes a PUC"]
    Mpuseg2vs1 = 1,
}
impl From<Mpuseg2vs> for bool {
    #[inline(always)]
    fn from(variant: Mpuseg2vs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEG2VS` reader - MPU Main Memory Segment 2 Violation Select"]
pub type Mpuseg2vsR = crate::BitReader<Mpuseg2vs>;
impl Mpuseg2vsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuseg2vs {
        match self.bits {
            false => Mpuseg2vs::Mpuseg2vs0,
            true => Mpuseg2vs::Mpuseg2vs1,
        }
    }
    #[doc = "Violation in Main Memory Segment 2 asserts the MPUSEG2IFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    #[inline(always)]
    pub fn is_mpuseg2vs_0(&self) -> bool {
        *self == Mpuseg2vs::Mpuseg2vs0
    }
    #[doc = "Violation in Main Memory Segment 2 asserts the MPUSEG2IFG bit and executes a PUC"]
    #[inline(always)]
    pub fn is_mpuseg2vs_1(&self) -> bool {
        *self == Mpuseg2vs::Mpuseg2vs1
    }
}
#[doc = "Field `MPUSEG2VS` writer - MPU Main Memory Segment 2 Violation Select"]
pub type Mpuseg2vsW<'a, REG> = crate::BitWriter<'a, REG, Mpuseg2vs>;
impl<'a, REG> Mpuseg2vsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Violation in Main Memory Segment 2 asserts the MPUSEG2IFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    #[inline(always)]
    pub fn mpuseg2vs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg2vs::Mpuseg2vs0)
    }
    #[doc = "Violation in Main Memory Segment 2 asserts the MPUSEG2IFG bit and executes a PUC"]
    #[inline(always)]
    pub fn mpuseg2vs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg2vs::Mpuseg2vs1)
    }
}
#[doc = "MPU Main Memory Segment 3 Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuseg3re {
    #[doc = "0: Read on Main Memory Segment 3 causes a violation if MPUSEG3WE = MPUSEG3XE = 0"]
    Disable = 0,
    #[doc = "1: Read on Main Memory Segment 3 is allowed"]
    Enable = 1,
}
impl From<Mpuseg3re> for bool {
    #[inline(always)]
    fn from(variant: Mpuseg3re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEG3RE` reader - MPU Main Memory Segment 3 Read Enable"]
pub type Mpuseg3reR = crate::BitReader<Mpuseg3re>;
impl Mpuseg3reR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuseg3re {
        match self.bits {
            false => Mpuseg3re::Disable,
            true => Mpuseg3re::Enable,
        }
    }
    #[doc = "Read on Main Memory Segment 3 causes a violation if MPUSEG3WE = MPUSEG3XE = 0"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpuseg3re::Disable
    }
    #[doc = "Read on Main Memory Segment 3 is allowed"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpuseg3re::Enable
    }
}
#[doc = "Field `MPUSEG3RE` writer - MPU Main Memory Segment 3 Read Enable"]
pub type Mpuseg3reW<'a, REG> = crate::BitWriter<'a, REG, Mpuseg3re>;
impl<'a, REG> Mpuseg3reW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read on Main Memory Segment 3 causes a violation if MPUSEG3WE = MPUSEG3XE = 0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg3re::Disable)
    }
    #[doc = "Read on Main Memory Segment 3 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg3re::Enable)
    }
}
#[doc = "MPU Main Memory Segment 3 Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuseg3we {
    #[doc = "0: Write on Main Memory Segment 3 causes a violation"]
    Disable = 0,
    #[doc = "1: Write on Main Memory Segment 3 is allowed"]
    Enable = 1,
}
impl From<Mpuseg3we> for bool {
    #[inline(always)]
    fn from(variant: Mpuseg3we) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEG3WE` reader - MPU Main Memory Segment 3 Write Enable"]
pub type Mpuseg3weR = crate::BitReader<Mpuseg3we>;
impl Mpuseg3weR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuseg3we {
        match self.bits {
            false => Mpuseg3we::Disable,
            true => Mpuseg3we::Enable,
        }
    }
    #[doc = "Write on Main Memory Segment 3 causes a violation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpuseg3we::Disable
    }
    #[doc = "Write on Main Memory Segment 3 is allowed"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpuseg3we::Enable
    }
}
#[doc = "Field `MPUSEG3WE` writer - MPU Main Memory Segment 3 Write Enable"]
pub type Mpuseg3weW<'a, REG> = crate::BitWriter<'a, REG, Mpuseg3we>;
impl<'a, REG> Mpuseg3weW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write on Main Memory Segment 3 causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg3we::Disable)
    }
    #[doc = "Write on Main Memory Segment 3 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg3we::Enable)
    }
}
#[doc = "MPU Main Memory Segment 3 Execute Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuseg3xe {
    #[doc = "0: Execute code on Main Memory Segment 3 causes a violation"]
    Disable = 0,
    #[doc = "1: Execute code on Main Memory Segment 3 is allowed"]
    Enable = 1,
}
impl From<Mpuseg3xe> for bool {
    #[inline(always)]
    fn from(variant: Mpuseg3xe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEG3XE` reader - MPU Main Memory Segment 3 Execute Enable"]
pub type Mpuseg3xeR = crate::BitReader<Mpuseg3xe>;
impl Mpuseg3xeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuseg3xe {
        match self.bits {
            false => Mpuseg3xe::Disable,
            true => Mpuseg3xe::Enable,
        }
    }
    #[doc = "Execute code on Main Memory Segment 3 causes a violation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpuseg3xe::Disable
    }
    #[doc = "Execute code on Main Memory Segment 3 is allowed"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpuseg3xe::Enable
    }
}
#[doc = "Field `MPUSEG3XE` writer - MPU Main Memory Segment 3 Execute Enable"]
pub type Mpuseg3xeW<'a, REG> = crate::BitWriter<'a, REG, Mpuseg3xe>;
impl<'a, REG> Mpuseg3xeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Execute code on Main Memory Segment 3 causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg3xe::Disable)
    }
    #[doc = "Execute code on Main Memory Segment 3 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg3xe::Enable)
    }
}
#[doc = "MPU Main Memory Segment 3 Violation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuseg3vs {
    #[doc = "0: Violation in Main Memory Segment 3 asserts the MPUSEG3IFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    Mpuseg3vs0 = 0,
    #[doc = "1: Violation in Main Memory Segment 3 asserts the MPUSEG3IFG bit and executes a PUC"]
    Mpuseg3vs1 = 1,
}
impl From<Mpuseg3vs> for bool {
    #[inline(always)]
    fn from(variant: Mpuseg3vs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEG3VS` reader - MPU Main Memory Segment 3 Violation Select"]
pub type Mpuseg3vsR = crate::BitReader<Mpuseg3vs>;
impl Mpuseg3vsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuseg3vs {
        match self.bits {
            false => Mpuseg3vs::Mpuseg3vs0,
            true => Mpuseg3vs::Mpuseg3vs1,
        }
    }
    #[doc = "Violation in Main Memory Segment 3 asserts the MPUSEG3IFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    #[inline(always)]
    pub fn is_mpuseg3vs_0(&self) -> bool {
        *self == Mpuseg3vs::Mpuseg3vs0
    }
    #[doc = "Violation in Main Memory Segment 3 asserts the MPUSEG3IFG bit and executes a PUC"]
    #[inline(always)]
    pub fn is_mpuseg3vs_1(&self) -> bool {
        *self == Mpuseg3vs::Mpuseg3vs1
    }
}
#[doc = "Field `MPUSEG3VS` writer - MPU Main Memory Segment 3 Violation Select"]
pub type Mpuseg3vsW<'a, REG> = crate::BitWriter<'a, REG, Mpuseg3vs>;
impl<'a, REG> Mpuseg3vsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Violation in Main Memory Segment 3 asserts the MPUSEG3IFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    #[inline(always)]
    pub fn mpuseg3vs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg3vs::Mpuseg3vs0)
    }
    #[doc = "Violation in Main Memory Segment 3 asserts the MPUSEG3IFG bit and executes a PUC"]
    #[inline(always)]
    pub fn mpuseg3vs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg3vs::Mpuseg3vs1)
    }
}
#[doc = "MPU User Information Memory Segment Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpusegire {
    #[doc = "0: Read on User Information Memory causes a violation if MPUSEGIWE=MPUSEGIXE=0"]
    Disable = 0,
    #[doc = "1: Read on User Information Memory is allowed"]
    Enable = 1,
}
impl From<Mpusegire> for bool {
    #[inline(always)]
    fn from(variant: Mpusegire) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEGIRE` reader - MPU User Information Memory Segment Read Enable"]
pub type MpusegireR = crate::BitReader<Mpusegire>;
impl MpusegireR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpusegire {
        match self.bits {
            false => Mpusegire::Disable,
            true => Mpusegire::Enable,
        }
    }
    #[doc = "Read on User Information Memory causes a violation if MPUSEGIWE=MPUSEGIXE=0"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpusegire::Disable
    }
    #[doc = "Read on User Information Memory is allowed"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpusegire::Enable
    }
}
#[doc = "Field `MPUSEGIRE` writer - MPU User Information Memory Segment Read Enable"]
pub type MpusegireW<'a, REG> = crate::BitWriter<'a, REG, Mpusegire>;
impl<'a, REG> MpusegireW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read on User Information Memory causes a violation if MPUSEGIWE=MPUSEGIXE=0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpusegire::Disable)
    }
    #[doc = "Read on User Information Memory is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpusegire::Enable)
    }
}
#[doc = "MPU User Information Memory Segment Write Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpusegiwe {
    #[doc = "0: Write on User Information Memory causes a violation"]
    Disable = 0,
    #[doc = "1: Write on User Information Memory is allowed"]
    Enable = 1,
}
impl From<Mpusegiwe> for bool {
    #[inline(always)]
    fn from(variant: Mpusegiwe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEGIWE` reader - MPU User Information Memory Segment Write Enable."]
pub type MpusegiweR = crate::BitReader<Mpusegiwe>;
impl MpusegiweR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpusegiwe {
        match self.bits {
            false => Mpusegiwe::Disable,
            true => Mpusegiwe::Enable,
        }
    }
    #[doc = "Write on User Information Memory causes a violation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpusegiwe::Disable
    }
    #[doc = "Write on User Information Memory is allowed"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpusegiwe::Enable
    }
}
#[doc = "Field `MPUSEGIWE` writer - MPU User Information Memory Segment Write Enable."]
pub type MpusegiweW<'a, REG> = crate::BitWriter<'a, REG, Mpusegiwe>;
impl<'a, REG> MpusegiweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write on User Information Memory causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpusegiwe::Disable)
    }
    #[doc = "Write on User Information Memory is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpusegiwe::Enable)
    }
}
#[doc = "MPU User Information Memory Segment Execute Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpusegixe {
    #[doc = "0: Execute code on User Information Memory causes a violation"]
    Disable = 0,
    #[doc = "1: Execute code on User Information Memory is allowed"]
    Enable = 1,
}
impl From<Mpusegixe> for bool {
    #[inline(always)]
    fn from(variant: Mpusegixe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEGIXE` reader - MPU User Information Memory Segment Execute Enable"]
pub type MpusegixeR = crate::BitReader<Mpusegixe>;
impl MpusegixeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpusegixe {
        match self.bits {
            false => Mpusegixe::Disable,
            true => Mpusegixe::Enable,
        }
    }
    #[doc = "Execute code on User Information Memory causes a violation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpusegixe::Disable
    }
    #[doc = "Execute code on User Information Memory is allowed"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpusegixe::Enable
    }
}
#[doc = "Field `MPUSEGIXE` writer - MPU User Information Memory Segment Execute Enable"]
pub type MpusegixeW<'a, REG> = crate::BitWriter<'a, REG, Mpusegixe>;
impl<'a, REG> MpusegixeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Execute code on User Information Memory causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpusegixe::Disable)
    }
    #[doc = "Execute code on User Information Memory is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpusegixe::Enable)
    }
}
#[doc = "MPU User Information Memory Segment Violation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpusegivs {
    #[doc = "0: Violation in User Information Memory asserts the MPUSEGIIFG bit and executes a SNMI if enabled by MPUSEGIE =1"]
    Mpusegivs0 = 0,
    #[doc = "1: Violation in User Information Memory asserts the MPUSEGIIFG bit and executes a PUC"]
    Mpusegivs1 = 1,
}
impl From<Mpusegivs> for bool {
    #[inline(always)]
    fn from(variant: Mpusegivs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEGIVS` reader - MPU User Information Memory Segment Violation Select"]
pub type MpusegivsR = crate::BitReader<Mpusegivs>;
impl MpusegivsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpusegivs {
        match self.bits {
            false => Mpusegivs::Mpusegivs0,
            true => Mpusegivs::Mpusegivs1,
        }
    }
    #[doc = "Violation in User Information Memory asserts the MPUSEGIIFG bit and executes a SNMI if enabled by MPUSEGIE =1"]
    #[inline(always)]
    pub fn is_mpusegivs_0(&self) -> bool {
        *self == Mpusegivs::Mpusegivs0
    }
    #[doc = "Violation in User Information Memory asserts the MPUSEGIIFG bit and executes a PUC"]
    #[inline(always)]
    pub fn is_mpusegivs_1(&self) -> bool {
        *self == Mpusegivs::Mpusegivs1
    }
}
#[doc = "Field `MPUSEGIVS` writer - MPU User Information Memory Segment Violation Select"]
pub type MpusegivsW<'a, REG> = crate::BitWriter<'a, REG, Mpusegivs>;
impl<'a, REG> MpusegivsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Violation in User Information Memory asserts the MPUSEGIIFG bit and executes a SNMI if enabled by MPUSEGIE =1"]
    #[inline(always)]
    pub fn mpusegivs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpusegivs::Mpusegivs0)
    }
    #[doc = "Violation in User Information Memory asserts the MPUSEGIIFG bit and executes a PUC"]
    #[inline(always)]
    pub fn mpusegivs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpusegivs::Mpusegivs1)
    }
}
impl R {
    #[doc = "Bit 0 - MPU Main Memory Segment 1 Read Enable"]
    #[inline(always)]
    pub fn mpuseg1re(&self) -> Mpuseg1reR {
        Mpuseg1reR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MPU Main Memory Segment 1 Write Enable"]
    #[inline(always)]
    pub fn mpuseg1we(&self) -> Mpuseg1weR {
        Mpuseg1weR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MPU Main Memory Segment 1 Execute Enable"]
    #[inline(always)]
    pub fn mpuseg1xe(&self) -> Mpuseg1xeR {
        Mpuseg1xeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MPU Main Memory Segment 1 Violation Select"]
    #[inline(always)]
    pub fn mpuseg1vs(&self) -> Mpuseg1vsR {
        Mpuseg1vsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MPU Main Memory Segment 2 Read Enable"]
    #[inline(always)]
    pub fn mpuseg2re(&self) -> Mpuseg2reR {
        Mpuseg2reR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MPU Main Memory Segment 2 Write Enable"]
    #[inline(always)]
    pub fn mpuseg2we(&self) -> Mpuseg2weR {
        Mpuseg2weR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MPU Main Memory Segment 2 Execute Enable"]
    #[inline(always)]
    pub fn mpuseg2xe(&self) -> Mpuseg2xeR {
        Mpuseg2xeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MPU Main Memory Segment 2 Violation Select"]
    #[inline(always)]
    pub fn mpuseg2vs(&self) -> Mpuseg2vsR {
        Mpuseg2vsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MPU Main Memory Segment 3 Read Enable"]
    #[inline(always)]
    pub fn mpuseg3re(&self) -> Mpuseg3reR {
        Mpuseg3reR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MPU Main Memory Segment 3 Write Enable"]
    #[inline(always)]
    pub fn mpuseg3we(&self) -> Mpuseg3weR {
        Mpuseg3weR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MPU Main Memory Segment 3 Execute Enable"]
    #[inline(always)]
    pub fn mpuseg3xe(&self) -> Mpuseg3xeR {
        Mpuseg3xeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MPU Main Memory Segment 3 Violation Select"]
    #[inline(always)]
    pub fn mpuseg3vs(&self) -> Mpuseg3vsR {
        Mpuseg3vsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MPU User Information Memory Segment Read Enable"]
    #[inline(always)]
    pub fn mpusegire(&self) -> MpusegireR {
        MpusegireR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MPU User Information Memory Segment Write Enable."]
    #[inline(always)]
    pub fn mpusegiwe(&self) -> MpusegiweR {
        MpusegiweR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MPU User Information Memory Segment Execute Enable"]
    #[inline(always)]
    pub fn mpusegixe(&self) -> MpusegixeR {
        MpusegixeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MPU User Information Memory Segment Violation Select"]
    #[inline(always)]
    pub fn mpusegivs(&self) -> MpusegivsR {
        MpusegivsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU Main Memory Segment 1 Read Enable"]
    #[inline(always)]
    pub fn mpuseg1re(&mut self) -> Mpuseg1reW<'_, MpusamSpec> {
        Mpuseg1reW::new(self, 0)
    }
    #[doc = "Bit 1 - MPU Main Memory Segment 1 Write Enable"]
    #[inline(always)]
    pub fn mpuseg1we(&mut self) -> Mpuseg1weW<'_, MpusamSpec> {
        Mpuseg1weW::new(self, 1)
    }
    #[doc = "Bit 2 - MPU Main Memory Segment 1 Execute Enable"]
    #[inline(always)]
    pub fn mpuseg1xe(&mut self) -> Mpuseg1xeW<'_, MpusamSpec> {
        Mpuseg1xeW::new(self, 2)
    }
    #[doc = "Bit 3 - MPU Main Memory Segment 1 Violation Select"]
    #[inline(always)]
    pub fn mpuseg1vs(&mut self) -> Mpuseg1vsW<'_, MpusamSpec> {
        Mpuseg1vsW::new(self, 3)
    }
    #[doc = "Bit 4 - MPU Main Memory Segment 2 Read Enable"]
    #[inline(always)]
    pub fn mpuseg2re(&mut self) -> Mpuseg2reW<'_, MpusamSpec> {
        Mpuseg2reW::new(self, 4)
    }
    #[doc = "Bit 5 - MPU Main Memory Segment 2 Write Enable"]
    #[inline(always)]
    pub fn mpuseg2we(&mut self) -> Mpuseg2weW<'_, MpusamSpec> {
        Mpuseg2weW::new(self, 5)
    }
    #[doc = "Bit 6 - MPU Main Memory Segment 2 Execute Enable"]
    #[inline(always)]
    pub fn mpuseg2xe(&mut self) -> Mpuseg2xeW<'_, MpusamSpec> {
        Mpuseg2xeW::new(self, 6)
    }
    #[doc = "Bit 7 - MPU Main Memory Segment 2 Violation Select"]
    #[inline(always)]
    pub fn mpuseg2vs(&mut self) -> Mpuseg2vsW<'_, MpusamSpec> {
        Mpuseg2vsW::new(self, 7)
    }
    #[doc = "Bit 8 - MPU Main Memory Segment 3 Read Enable"]
    #[inline(always)]
    pub fn mpuseg3re(&mut self) -> Mpuseg3reW<'_, MpusamSpec> {
        Mpuseg3reW::new(self, 8)
    }
    #[doc = "Bit 9 - MPU Main Memory Segment 3 Write Enable"]
    #[inline(always)]
    pub fn mpuseg3we(&mut self) -> Mpuseg3weW<'_, MpusamSpec> {
        Mpuseg3weW::new(self, 9)
    }
    #[doc = "Bit 10 - MPU Main Memory Segment 3 Execute Enable"]
    #[inline(always)]
    pub fn mpuseg3xe(&mut self) -> Mpuseg3xeW<'_, MpusamSpec> {
        Mpuseg3xeW::new(self, 10)
    }
    #[doc = "Bit 11 - MPU Main Memory Segment 3 Violation Select"]
    #[inline(always)]
    pub fn mpuseg3vs(&mut self) -> Mpuseg3vsW<'_, MpusamSpec> {
        Mpuseg3vsW::new(self, 11)
    }
    #[doc = "Bit 12 - MPU User Information Memory Segment Read Enable"]
    #[inline(always)]
    pub fn mpusegire(&mut self) -> MpusegireW<'_, MpusamSpec> {
        MpusegireW::new(self, 12)
    }
    #[doc = "Bit 13 - MPU User Information Memory Segment Write Enable."]
    #[inline(always)]
    pub fn mpusegiwe(&mut self) -> MpusegiweW<'_, MpusamSpec> {
        MpusegiweW::new(self, 13)
    }
    #[doc = "Bit 14 - MPU User Information Memory Segment Execute Enable"]
    #[inline(always)]
    pub fn mpusegixe(&mut self) -> MpusegixeW<'_, MpusamSpec> {
        MpusegixeW::new(self, 14)
    }
    #[doc = "Bit 15 - MPU User Information Memory Segment Violation Select"]
    #[inline(always)]
    pub fn mpusegivs(&mut self) -> MpusegivsW<'_, MpusamSpec> {
        MpusegivsW::new(self, 15)
    }
}
#[doc = "Memory Protection Unit Segmentation Access Management Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpusam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpusam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpusamSpec;
impl crate::RegisterSpec for MpusamSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpusam::R`](R) reader structure"]
impl crate::Readable for MpusamSpec {}
#[doc = "`write(|w| ..)` method takes [`mpusam::W`](W) writer structure"]
impl crate::Writable for MpusamSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MPUSAM to value 0"]
impl crate::Resettable for MpusamSpec {}
