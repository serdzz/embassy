#[doc = "Register `ADC12IFGR2` reader"]
pub type R = crate::R<Adc12ifgr2Spec>;
#[doc = "Register `ADC12IFGR2` writer"]
pub type W = crate::W<Adc12ifgr2Spec>;
#[doc = "Interrupt flag for ADC12MEMx between ADC12HI and ADC12LO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12inifg {
    #[doc = "0: No interrupt pending"]
    Adc12inifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12inifg1 = 1,
}
impl From<Adc12inifg> for bool {
    #[inline(always)]
    fn from(variant: Adc12inifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12INIFG` reader - Interrupt flag for ADC12MEMx between ADC12HI and ADC12LO"]
pub type Adc12inifgR = crate::BitReader<Adc12inifg>;
impl Adc12inifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12inifg {
        match self.bits {
            false => Adc12inifg::Adc12inifg0,
            true => Adc12inifg::Adc12inifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12inifg_0(&self) -> bool {
        *self == Adc12inifg::Adc12inifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12inifg_1(&self) -> bool {
        *self == Adc12inifg::Adc12inifg1
    }
}
#[doc = "Field `ADC12INIFG` writer - Interrupt flag for ADC12MEMx between ADC12HI and ADC12LO"]
pub type Adc12inifgW<'a, REG> = crate::BitWriter<'a, REG, Adc12inifg>;
impl<'a, REG> Adc12inifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12inifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inifg::Adc12inifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12inifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inifg::Adc12inifg1)
    }
}
#[doc = "Interrupt flag for ADC12MEMx ADC12LO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12loifg {
    #[doc = "0: No interrupt pending"]
    Adc12loifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12loifg1 = 1,
}
impl From<Adc12loifg> for bool {
    #[inline(always)]
    fn from(variant: Adc12loifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12LOIFG` reader - Interrupt flag for ADC12MEMx ADC12LO"]
pub type Adc12loifgR = crate::BitReader<Adc12loifg>;
impl Adc12loifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12loifg {
        match self.bits {
            false => Adc12loifg::Adc12loifg0,
            true => Adc12loifg::Adc12loifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12loifg_0(&self) -> bool {
        *self == Adc12loifg::Adc12loifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12loifg_1(&self) -> bool {
        *self == Adc12loifg::Adc12loifg1
    }
}
#[doc = "Field `ADC12LOIFG` writer - Interrupt flag for ADC12MEMx ADC12LO"]
pub type Adc12loifgW<'a, REG> = crate::BitWriter<'a, REG, Adc12loifg>;
impl<'a, REG> Adc12loifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12loifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12loifg::Adc12loifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12loifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12loifg::Adc12loifg1)
    }
}
#[doc = "Interrupt flag for ADC12MEMx ADC12HI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12hiifg {
    #[doc = "0: No interrupt pending"]
    Adc12hiifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12hiifg1 = 1,
}
impl From<Adc12hiifg> for bool {
    #[inline(always)]
    fn from(variant: Adc12hiifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12HIIFG` reader - Interrupt flag for ADC12MEMx ADC12HI"]
pub type Adc12hiifgR = crate::BitReader<Adc12hiifg>;
impl Adc12hiifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12hiifg {
        match self.bits {
            false => Adc12hiifg::Adc12hiifg0,
            true => Adc12hiifg::Adc12hiifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12hiifg_0(&self) -> bool {
        *self == Adc12hiifg::Adc12hiifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12hiifg_1(&self) -> bool {
        *self == Adc12hiifg::Adc12hiifg1
    }
}
#[doc = "Field `ADC12HIIFG` writer - Interrupt flag for ADC12MEMx ADC12HI"]
pub type Adc12hiifgW<'a, REG> = crate::BitWriter<'a, REG, Adc12hiifg>;
impl<'a, REG> Adc12hiifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12hiifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12hiifg::Adc12hiifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12hiifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12hiifg::Adc12hiifg1)
    }
}
#[doc = "ADC12MEMx overflow-interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12ovifg {
    #[doc = "0: No interrupt pending"]
    Adc12ovifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12ovifg1 = 1,
}
impl From<Adc12ovifg> for bool {
    #[inline(always)]
    fn from(variant: Adc12ovifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12OVIFG` reader - ADC12MEMx overflow-interrupt flag."]
pub type Adc12ovifgR = crate::BitReader<Adc12ovifg>;
impl Adc12ovifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ovifg {
        match self.bits {
            false => Adc12ovifg::Adc12ovifg0,
            true => Adc12ovifg::Adc12ovifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ovifg_0(&self) -> bool {
        *self == Adc12ovifg::Adc12ovifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12ovifg_1(&self) -> bool {
        *self == Adc12ovifg::Adc12ovifg1
    }
}
#[doc = "Field `ADC12OVIFG` writer - ADC12MEMx overflow-interrupt flag."]
pub type Adc12ovifgW<'a, REG> = crate::BitWriter<'a, REG, Adc12ovifg>;
impl<'a, REG> Adc12ovifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ovifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ovifg::Adc12ovifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ovifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ovifg::Adc12ovifg1)
    }
}
#[doc = "conversion-time-overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12tovifg {
    #[doc = "0: No interrupt pending"]
    Adc12tovifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12tovifg1 = 1,
}
impl From<Adc12tovifg> for bool {
    #[inline(always)]
    fn from(variant: Adc12tovifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12TOVIFG` reader - conversion-time-overflow interrupt flag"]
pub type Adc12tovifgR = crate::BitReader<Adc12tovifg>;
impl Adc12tovifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12tovifg {
        match self.bits {
            false => Adc12tovifg::Adc12tovifg0,
            true => Adc12tovifg::Adc12tovifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12tovifg_0(&self) -> bool {
        *self == Adc12tovifg::Adc12tovifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12tovifg_1(&self) -> bool {
        *self == Adc12tovifg::Adc12tovifg1
    }
}
#[doc = "Field `ADC12TOVIFG` writer - conversion-time-overflow interrupt flag"]
pub type Adc12tovifgW<'a, REG> = crate::BitWriter<'a, REG, Adc12tovifg>;
impl<'a, REG> Adc12tovifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12tovifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12tovifg::Adc12tovifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12tovifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12tovifg::Adc12tovifg1)
    }
}
#[doc = "reference buffer ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12rdyifg {
    #[doc = "0: No interrupt pending"]
    Adc12rdyifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc12rdyifg1 = 1,
}
impl From<Adc12rdyifg> for bool {
    #[inline(always)]
    fn from(variant: Adc12rdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12RDYIFG` reader - reference buffer ready interrupt flag"]
pub type Adc12rdyifgR = crate::BitReader<Adc12rdyifg>;
impl Adc12rdyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12rdyifg {
        match self.bits {
            false => Adc12rdyifg::Adc12rdyifg0,
            true => Adc12rdyifg::Adc12rdyifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc12rdyifg_0(&self) -> bool {
        *self == Adc12rdyifg::Adc12rdyifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc12rdyifg_1(&self) -> bool {
        *self == Adc12rdyifg::Adc12rdyifg1
    }
}
#[doc = "Field `ADC12RDYIFG` writer - reference buffer ready interrupt flag"]
pub type Adc12rdyifgW<'a, REG> = crate::BitWriter<'a, REG, Adc12rdyifg>;
impl<'a, REG> Adc12rdyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12rdyifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12rdyifg::Adc12rdyifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12rdyifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12rdyifg::Adc12rdyifg1)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt flag for ADC12MEMx between ADC12HI and ADC12LO"]
    #[inline(always)]
    pub fn adc12inifg(&self) -> Adc12inifgR {
        Adc12inifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for ADC12MEMx ADC12LO"]
    #[inline(always)]
    pub fn adc12loifg(&self) -> Adc12loifgR {
        Adc12loifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for ADC12MEMx ADC12HI"]
    #[inline(always)]
    pub fn adc12hiifg(&self) -> Adc12hiifgR {
        Adc12hiifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC12MEMx overflow-interrupt flag."]
    #[inline(always)]
    pub fn adc12ovifg(&self) -> Adc12ovifgR {
        Adc12ovifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - conversion-time-overflow interrupt flag"]
    #[inline(always)]
    pub fn adc12tovifg(&self) -> Adc12tovifgR {
        Adc12tovifgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reference buffer ready interrupt flag"]
    #[inline(always)]
    pub fn adc12rdyifg(&self) -> Adc12rdyifgR {
        Adc12rdyifgR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt flag for ADC12MEMx between ADC12HI and ADC12LO"]
    #[inline(always)]
    pub fn adc12inifg(&mut self) -> Adc12inifgW<'_, Adc12ifgr2Spec> {
        Adc12inifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt flag for ADC12MEMx ADC12LO"]
    #[inline(always)]
    pub fn adc12loifg(&mut self) -> Adc12loifgW<'_, Adc12ifgr2Spec> {
        Adc12loifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt flag for ADC12MEMx ADC12HI"]
    #[inline(always)]
    pub fn adc12hiifg(&mut self) -> Adc12hiifgW<'_, Adc12ifgr2Spec> {
        Adc12hiifgW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC12MEMx overflow-interrupt flag."]
    #[inline(always)]
    pub fn adc12ovifg(&mut self) -> Adc12ovifgW<'_, Adc12ifgr2Spec> {
        Adc12ovifgW::new(self, 4)
    }
    #[doc = "Bit 5 - conversion-time-overflow interrupt flag"]
    #[inline(always)]
    pub fn adc12tovifg(&mut self) -> Adc12tovifgW<'_, Adc12ifgr2Spec> {
        Adc12tovifgW::new(self, 5)
    }
    #[doc = "Bit 6 - reference buffer ready interrupt flag"]
    #[inline(always)]
    pub fn adc12rdyifg(&mut self) -> Adc12rdyifgW<'_, Adc12ifgr2Spec> {
        Adc12rdyifgW::new(self, 6)
    }
}
#[doc = "ADC12_B Interrupt Flag 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ifgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ifgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12ifgr2Spec;
impl crate::RegisterSpec for Adc12ifgr2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12ifgr2::R`](R) reader structure"]
impl crate::Readable for Adc12ifgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`adc12ifgr2::W`](W) writer structure"]
impl crate::Writable for Adc12ifgr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC12IFGR2 to value 0"]
impl crate::Resettable for Adc12ifgr2Spec {}
