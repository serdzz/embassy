#[doc = "Register `ADC12IER2` reader"]
pub type R = crate::R<Adc12ier2Spec>;
#[doc = "Register `ADC12IER2` writer"]
pub type W = crate::W<Adc12ier2Spec>;
#[doc = "interrupt enable MEMx between ADC12HI and LO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12inie {
    #[doc = "0: Interrupt disabled"]
    Adc12inie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12inie1 = 1,
}
impl From<Adc12inie> for bool {
    #[inline(always)]
    fn from(variant: Adc12inie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12INIE` reader - interrupt enable MEMx between ADC12HI and LO"]
pub type Adc12inieR = crate::BitReader<Adc12inie>;
impl Adc12inieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12inie {
        match self.bits {
            false => Adc12inie::Adc12inie0,
            true => Adc12inie::Adc12inie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12inie_0(&self) -> bool {
        *self == Adc12inie::Adc12inie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12inie_1(&self) -> bool {
        *self == Adc12inie::Adc12inie1
    }
}
#[doc = "Field `ADC12INIE` writer - interrupt enable MEMx between ADC12HI and LO"]
pub type Adc12inieW<'a, REG> = crate::BitWriter<'a, REG, Adc12inie>;
impl<'a, REG> Adc12inieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12inie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inie::Adc12inie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12inie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inie::Adc12inie1)
    }
}
#[doc = "interrupt enable MEMx ADC12LO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12loie {
    #[doc = "0: Interrupt disabled"]
    Adc12loie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12loie1 = 1,
}
impl From<Adc12loie> for bool {
    #[inline(always)]
    fn from(variant: Adc12loie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12LOIE` reader - interrupt enable MEMx ADC12LO"]
pub type Adc12loieR = crate::BitReader<Adc12loie>;
impl Adc12loieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12loie {
        match self.bits {
            false => Adc12loie::Adc12loie0,
            true => Adc12loie::Adc12loie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12loie_0(&self) -> bool {
        *self == Adc12loie::Adc12loie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12loie_1(&self) -> bool {
        *self == Adc12loie::Adc12loie1
    }
}
#[doc = "Field `ADC12LOIE` writer - interrupt enable MEMx ADC12LO"]
pub type Adc12loieW<'a, REG> = crate::BitWriter<'a, REG, Adc12loie>;
impl<'a, REG> Adc12loieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12loie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12loie::Adc12loie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12loie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12loie::Adc12loie1)
    }
}
#[doc = "interrupt enable MEMx ADC12HI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12hiie {
    #[doc = "0: Interrupt disabled"]
    Adc12hiie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12hiie1 = 1,
}
impl From<Adc12hiie> for bool {
    #[inline(always)]
    fn from(variant: Adc12hiie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12HIIE` reader - interrupt enable MEMx ADC12HI"]
pub type Adc12hiieR = crate::BitReader<Adc12hiie>;
impl Adc12hiieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12hiie {
        match self.bits {
            false => Adc12hiie::Adc12hiie0,
            true => Adc12hiie::Adc12hiie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12hiie_0(&self) -> bool {
        *self == Adc12hiie::Adc12hiie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12hiie_1(&self) -> bool {
        *self == Adc12hiie::Adc12hiie1
    }
}
#[doc = "Field `ADC12HIIE` writer - interrupt enable MEMx ADC12HI"]
pub type Adc12hiieW<'a, REG> = crate::BitWriter<'a, REG, Adc12hiie>;
impl<'a, REG> Adc12hiieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12hiie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12hiie::Adc12hiie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12hiie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12hiie::Adc12hiie1)
    }
}
#[doc = "ADC12MEMx overflow-interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ovie {
    #[doc = "0: Interrupt disabled"]
    Adc12ovie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12ovie1 = 1,
}
impl From<Adc12ovie> for bool {
    #[inline(always)]
    fn from(variant: Adc12ovie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12OVIE` reader - ADC12MEMx overflow-interrupt enable"]
pub type Adc12ovieR = crate::BitReader<Adc12ovie>;
impl Adc12ovieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ovie {
        match self.bits {
            false => Adc12ovie::Adc12ovie0,
            true => Adc12ovie::Adc12ovie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12ovie_0(&self) -> bool {
        *self == Adc12ovie::Adc12ovie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12ovie_1(&self) -> bool {
        *self == Adc12ovie::Adc12ovie1
    }
}
#[doc = "Field `ADC12OVIE` writer - ADC12MEMx overflow-interrupt enable"]
pub type Adc12ovieW<'a, REG> = crate::BitWriter<'a, REG, Adc12ovie>;
impl<'a, REG> Adc12ovieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ovie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ovie::Adc12ovie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ovie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ovie::Adc12ovie1)
    }
}
#[doc = "conversion-time-overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12tovie {
    #[doc = "0: Interrupt disabled"]
    Adc12tovie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12tovie1 = 1,
}
impl From<Adc12tovie> for bool {
    #[inline(always)]
    fn from(variant: Adc12tovie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12TOVIE` reader - conversion-time-overflow interrupt enable"]
pub type Adc12tovieR = crate::BitReader<Adc12tovie>;
impl Adc12tovieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12tovie {
        match self.bits {
            false => Adc12tovie::Adc12tovie0,
            true => Adc12tovie::Adc12tovie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12tovie_0(&self) -> bool {
        *self == Adc12tovie::Adc12tovie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12tovie_1(&self) -> bool {
        *self == Adc12tovie::Adc12tovie1
    }
}
#[doc = "Field `ADC12TOVIE` writer - conversion-time-overflow interrupt enable"]
pub type Adc12tovieW<'a, REG> = crate::BitWriter<'a, REG, Adc12tovie>;
impl<'a, REG> Adc12tovieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12tovie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12tovie::Adc12tovie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12tovie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12tovie::Adc12tovie1)
    }
}
#[doc = "interrupt enable ADC ref buffer ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12rdyie {
    #[doc = "0: Interrupt disabled"]
    Adc12rdyie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc12rdyie1 = 1,
}
impl From<Adc12rdyie> for bool {
    #[inline(always)]
    fn from(variant: Adc12rdyie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12RDYIE` reader - interrupt enable ADC ref buffer ready"]
pub type Adc12rdyieR = crate::BitReader<Adc12rdyie>;
impl Adc12rdyieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12rdyie {
        match self.bits {
            false => Adc12rdyie::Adc12rdyie0,
            true => Adc12rdyie::Adc12rdyie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc12rdyie_0(&self) -> bool {
        *self == Adc12rdyie::Adc12rdyie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc12rdyie_1(&self) -> bool {
        *self == Adc12rdyie::Adc12rdyie1
    }
}
#[doc = "Field `ADC12RDYIE` writer - interrupt enable ADC ref buffer ready"]
pub type Adc12rdyieW<'a, REG> = crate::BitWriter<'a, REG, Adc12rdyie>;
impl<'a, REG> Adc12rdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12rdyie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12rdyie::Adc12rdyie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12rdyie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12rdyie::Adc12rdyie1)
    }
}
impl R {
    #[doc = "Bit 1 - interrupt enable MEMx between ADC12HI and LO"]
    #[inline(always)]
    pub fn adc12inie(&self) -> Adc12inieR {
        Adc12inieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt enable MEMx ADC12LO"]
    #[inline(always)]
    pub fn adc12loie(&self) -> Adc12loieR {
        Adc12loieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt enable MEMx ADC12HI"]
    #[inline(always)]
    pub fn adc12hiie(&self) -> Adc12hiieR {
        Adc12hiieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC12MEMx overflow-interrupt enable"]
    #[inline(always)]
    pub fn adc12ovie(&self) -> Adc12ovieR {
        Adc12ovieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - conversion-time-overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12tovie(&self) -> Adc12tovieR {
        Adc12tovieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - interrupt enable ADC ref buffer ready"]
    #[inline(always)]
    pub fn adc12rdyie(&self) -> Adc12rdyieR {
        Adc12rdyieR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - interrupt enable MEMx between ADC12HI and LO"]
    #[inline(always)]
    pub fn adc12inie(&mut self) -> Adc12inieW<'_, Adc12ier2Spec> {
        Adc12inieW::new(self, 1)
    }
    #[doc = "Bit 2 - interrupt enable MEMx ADC12LO"]
    #[inline(always)]
    pub fn adc12loie(&mut self) -> Adc12loieW<'_, Adc12ier2Spec> {
        Adc12loieW::new(self, 2)
    }
    #[doc = "Bit 3 - interrupt enable MEMx ADC12HI"]
    #[inline(always)]
    pub fn adc12hiie(&mut self) -> Adc12hiieW<'_, Adc12ier2Spec> {
        Adc12hiieW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC12MEMx overflow-interrupt enable"]
    #[inline(always)]
    pub fn adc12ovie(&mut self) -> Adc12ovieW<'_, Adc12ier2Spec> {
        Adc12ovieW::new(self, 4)
    }
    #[doc = "Bit 5 - conversion-time-overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12tovie(&mut self) -> Adc12tovieW<'_, Adc12ier2Spec> {
        Adc12tovieW::new(self, 5)
    }
    #[doc = "Bit 6 - interrupt enable ADC ref buffer ready"]
    #[inline(always)]
    pub fn adc12rdyie(&mut self) -> Adc12rdyieW<'_, Adc12ier2Spec> {
        Adc12rdyieW::new(self, 6)
    }
}
#[doc = "ADC12_B Interrupt Enable 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12ier2Spec;
impl crate::RegisterSpec for Adc12ier2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12ier2::R`](R) reader structure"]
impl crate::Readable for Adc12ier2Spec {}
#[doc = "`write(|w| ..)` method takes [`adc12ier2::W`](W) writer structure"]
impl crate::Writable for Adc12ier2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC12IER2 to value 0"]
impl crate::Resettable for Adc12ier2Spec {}
