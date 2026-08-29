#[doc = "Register `LCDCCTL1` reader"]
pub type R = crate::R<Lcdcctl1Spec>;
#[doc = "Register `LCDCCTL1` writer"]
pub type W = crate::W<Lcdcctl1Spec>;
#[doc = "LCD frame interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdfrmifg {
    #[doc = "0: No interrupt pending"]
    Lcdfrmifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Lcdfrmifg1 = 1,
}
impl From<Lcdfrmifg> for bool {
    #[inline(always)]
    fn from(variant: Lcdfrmifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDFRMIFG` reader - LCD frame interrupt flag"]
pub type LcdfrmifgR = crate::BitReader<Lcdfrmifg>;
impl LcdfrmifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdfrmifg {
        match self.bits {
            false => Lcdfrmifg::Lcdfrmifg0,
            true => Lcdfrmifg::Lcdfrmifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_lcdfrmifg_0(&self) -> bool {
        *self == Lcdfrmifg::Lcdfrmifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_lcdfrmifg_1(&self) -> bool {
        *self == Lcdfrmifg::Lcdfrmifg1
    }
}
#[doc = "Field `LCDFRMIFG` writer - LCD frame interrupt flag"]
pub type LcdfrmifgW<'a, REG> = crate::BitWriter<'a, REG, Lcdfrmifg>;
impl<'a, REG> LcdfrmifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn lcdfrmifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdfrmifg::Lcdfrmifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn lcdfrmifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdfrmifg::Lcdfrmifg1)
    }
}
#[doc = "LCD blinking interrupt flag, segments switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdblkoffifg {
    #[doc = "0: No interrupt pending"]
    Lcdblkoffifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Lcdblkoffifg1 = 1,
}
impl From<Lcdblkoffifg> for bool {
    #[inline(always)]
    fn from(variant: Lcdblkoffifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDBLKOFFIFG` reader - LCD blinking interrupt flag, segments switched off"]
pub type LcdblkoffifgR = crate::BitReader<Lcdblkoffifg>;
impl LcdblkoffifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdblkoffifg {
        match self.bits {
            false => Lcdblkoffifg::Lcdblkoffifg0,
            true => Lcdblkoffifg::Lcdblkoffifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_lcdblkoffifg_0(&self) -> bool {
        *self == Lcdblkoffifg::Lcdblkoffifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_lcdblkoffifg_1(&self) -> bool {
        *self == Lcdblkoffifg::Lcdblkoffifg1
    }
}
#[doc = "Field `LCDBLKOFFIFG` writer - LCD blinking interrupt flag, segments switched off"]
pub type LcdblkoffifgW<'a, REG> = crate::BitWriter<'a, REG, Lcdblkoffifg>;
impl<'a, REG> LcdblkoffifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn lcdblkoffifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkoffifg::Lcdblkoffifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn lcdblkoffifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkoffifg::Lcdblkoffifg1)
    }
}
#[doc = "LCD blinking interrupt flag, segments switched on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdblkonifg {
    #[doc = "0: No interrupt pending"]
    Lcdblkonifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Lcdblkonifg1 = 1,
}
impl From<Lcdblkonifg> for bool {
    #[inline(always)]
    fn from(variant: Lcdblkonifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDBLKONIFG` reader - LCD blinking interrupt flag, segments switched on"]
pub type LcdblkonifgR = crate::BitReader<Lcdblkonifg>;
impl LcdblkonifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdblkonifg {
        match self.bits {
            false => Lcdblkonifg::Lcdblkonifg0,
            true => Lcdblkonifg::Lcdblkonifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_lcdblkonifg_0(&self) -> bool {
        *self == Lcdblkonifg::Lcdblkonifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_lcdblkonifg_1(&self) -> bool {
        *self == Lcdblkonifg::Lcdblkonifg1
    }
}
#[doc = "Field `LCDBLKONIFG` writer - LCD blinking interrupt flag, segments switched on"]
pub type LcdblkonifgW<'a, REG> = crate::BitWriter<'a, REG, Lcdblkonifg>;
impl<'a, REG> LcdblkonifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn lcdblkonifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkonifg::Lcdblkonifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn lcdblkonifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkonifg::Lcdblkonifg1)
    }
}
#[doc = "No capacitance connected interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdnocapifg {
    #[doc = "0: No interrupt pending"]
    Lcdnocapifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Lcdnocapifg1 = 1,
}
impl From<Lcdnocapifg> for bool {
    #[inline(always)]
    fn from(variant: Lcdnocapifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDNOCAPIFG` reader - No capacitance connected interrupt flag"]
pub type LcdnocapifgR = crate::BitReader<Lcdnocapifg>;
impl LcdnocapifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdnocapifg {
        match self.bits {
            false => Lcdnocapifg::Lcdnocapifg0,
            true => Lcdnocapifg::Lcdnocapifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_lcdnocapifg_0(&self) -> bool {
        *self == Lcdnocapifg::Lcdnocapifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_lcdnocapifg_1(&self) -> bool {
        *self == Lcdnocapifg::Lcdnocapifg1
    }
}
#[doc = "Field `LCDNOCAPIFG` writer - No capacitance connected interrupt flag"]
pub type LcdnocapifgW<'a, REG> = crate::BitWriter<'a, REG, Lcdnocapifg>;
impl<'a, REG> LcdnocapifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn lcdnocapifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdnocapifg::Lcdnocapifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn lcdnocapifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdnocapifg::Lcdnocapifg1)
    }
}
#[doc = "LCD frame interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdfrmie {
    #[doc = "0: Interrupt disabled"]
    Lcdfrmie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Lcdfrmie1 = 1,
}
impl From<Lcdfrmie> for bool {
    #[inline(always)]
    fn from(variant: Lcdfrmie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDFRMIE` reader - LCD frame interrupt enable"]
pub type LcdfrmieR = crate::BitReader<Lcdfrmie>;
impl LcdfrmieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdfrmie {
        match self.bits {
            false => Lcdfrmie::Lcdfrmie0,
            true => Lcdfrmie::Lcdfrmie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_lcdfrmie_0(&self) -> bool {
        *self == Lcdfrmie::Lcdfrmie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_lcdfrmie_1(&self) -> bool {
        *self == Lcdfrmie::Lcdfrmie1
    }
}
#[doc = "Field `LCDFRMIE` writer - LCD frame interrupt enable"]
pub type LcdfrmieW<'a, REG> = crate::BitWriter<'a, REG, Lcdfrmie>;
impl<'a, REG> LcdfrmieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn lcdfrmie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdfrmie::Lcdfrmie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn lcdfrmie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdfrmie::Lcdfrmie1)
    }
}
#[doc = "LCD blinking interrupt enable, segments switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdblkoffie {
    #[doc = "0: Interrupt disabled"]
    Lcdblkoffie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Lcdblkoffie1 = 1,
}
impl From<Lcdblkoffie> for bool {
    #[inline(always)]
    fn from(variant: Lcdblkoffie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDBLKOFFIE` reader - LCD blinking interrupt enable, segments switched off"]
pub type LcdblkoffieR = crate::BitReader<Lcdblkoffie>;
impl LcdblkoffieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdblkoffie {
        match self.bits {
            false => Lcdblkoffie::Lcdblkoffie0,
            true => Lcdblkoffie::Lcdblkoffie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_lcdblkoffie_0(&self) -> bool {
        *self == Lcdblkoffie::Lcdblkoffie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_lcdblkoffie_1(&self) -> bool {
        *self == Lcdblkoffie::Lcdblkoffie1
    }
}
#[doc = "Field `LCDBLKOFFIE` writer - LCD blinking interrupt enable, segments switched off"]
pub type LcdblkoffieW<'a, REG> = crate::BitWriter<'a, REG, Lcdblkoffie>;
impl<'a, REG> LcdblkoffieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn lcdblkoffie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkoffie::Lcdblkoffie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn lcdblkoffie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkoffie::Lcdblkoffie1)
    }
}
#[doc = "LCD blinking interrupt enable, segments switched on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdblkonie {
    #[doc = "0: Interrupt disabled"]
    Lcdblkonie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Lcdblkonie1 = 1,
}
impl From<Lcdblkonie> for bool {
    #[inline(always)]
    fn from(variant: Lcdblkonie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDBLKONIE` reader - LCD blinking interrupt enable, segments switched on"]
pub type LcdblkonieR = crate::BitReader<Lcdblkonie>;
impl LcdblkonieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdblkonie {
        match self.bits {
            false => Lcdblkonie::Lcdblkonie0,
            true => Lcdblkonie::Lcdblkonie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_lcdblkonie_0(&self) -> bool {
        *self == Lcdblkonie::Lcdblkonie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_lcdblkonie_1(&self) -> bool {
        *self == Lcdblkonie::Lcdblkonie1
    }
}
#[doc = "Field `LCDBLKONIE` writer - LCD blinking interrupt enable, segments switched on"]
pub type LcdblkonieW<'a, REG> = crate::BitWriter<'a, REG, Lcdblkonie>;
impl<'a, REG> LcdblkonieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn lcdblkonie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkonie::Lcdblkonie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn lcdblkonie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkonie::Lcdblkonie1)
    }
}
#[doc = "No capacitance connected interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdnocapie {
    #[doc = "0: Interrupt disabled"]
    Lcdnocapie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Lcdnocapie1 = 1,
}
impl From<Lcdnocapie> for bool {
    #[inline(always)]
    fn from(variant: Lcdnocapie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDNOCAPIE` reader - No capacitance connected interrupt enable"]
pub type LcdnocapieR = crate::BitReader<Lcdnocapie>;
impl LcdnocapieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdnocapie {
        match self.bits {
            false => Lcdnocapie::Lcdnocapie0,
            true => Lcdnocapie::Lcdnocapie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_lcdnocapie_0(&self) -> bool {
        *self == Lcdnocapie::Lcdnocapie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_lcdnocapie_1(&self) -> bool {
        *self == Lcdnocapie::Lcdnocapie1
    }
}
#[doc = "Field `LCDNOCAPIE` writer - No capacitance connected interrupt enable"]
pub type LcdnocapieW<'a, REG> = crate::BitWriter<'a, REG, Lcdnocapie>;
impl<'a, REG> LcdnocapieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn lcdnocapie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdnocapie::Lcdnocapie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn lcdnocapie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdnocapie::Lcdnocapie1)
    }
}
impl R {
    #[doc = "Bit 0 - LCD frame interrupt flag"]
    #[inline(always)]
    pub fn lcdfrmifg(&self) -> LcdfrmifgR {
        LcdfrmifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD blinking interrupt flag, segments switched off"]
    #[inline(always)]
    pub fn lcdblkoffifg(&self) -> LcdblkoffifgR {
        LcdblkoffifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD blinking interrupt flag, segments switched on"]
    #[inline(always)]
    pub fn lcdblkonifg(&self) -> LcdblkonifgR {
        LcdblkonifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No capacitance connected interrupt flag"]
    #[inline(always)]
    pub fn lcdnocapifg(&self) -> LcdnocapifgR {
        LcdnocapifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD frame interrupt enable"]
    #[inline(always)]
    pub fn lcdfrmie(&self) -> LcdfrmieR {
        LcdfrmieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD blinking interrupt enable, segments switched off"]
    #[inline(always)]
    pub fn lcdblkoffie(&self) -> LcdblkoffieR {
        LcdblkoffieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD blinking interrupt enable, segments switched on"]
    #[inline(always)]
    pub fn lcdblkonie(&self) -> LcdblkonieR {
        LcdblkonieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - No capacitance connected interrupt enable"]
    #[inline(always)]
    pub fn lcdnocapie(&self) -> LcdnocapieR {
        LcdnocapieR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD frame interrupt flag"]
    #[inline(always)]
    pub fn lcdfrmifg(&mut self) -> LcdfrmifgW<'_, Lcdcctl1Spec> {
        LcdfrmifgW::new(self, 0)
    }
    #[doc = "Bit 1 - LCD blinking interrupt flag, segments switched off"]
    #[inline(always)]
    pub fn lcdblkoffifg(&mut self) -> LcdblkoffifgW<'_, Lcdcctl1Spec> {
        LcdblkoffifgW::new(self, 1)
    }
    #[doc = "Bit 2 - LCD blinking interrupt flag, segments switched on"]
    #[inline(always)]
    pub fn lcdblkonifg(&mut self) -> LcdblkonifgW<'_, Lcdcctl1Spec> {
        LcdblkonifgW::new(self, 2)
    }
    #[doc = "Bit 3 - No capacitance connected interrupt flag"]
    #[inline(always)]
    pub fn lcdnocapifg(&mut self) -> LcdnocapifgW<'_, Lcdcctl1Spec> {
        LcdnocapifgW::new(self, 3)
    }
    #[doc = "Bit 8 - LCD frame interrupt enable"]
    #[inline(always)]
    pub fn lcdfrmie(&mut self) -> LcdfrmieW<'_, Lcdcctl1Spec> {
        LcdfrmieW::new(self, 8)
    }
    #[doc = "Bit 9 - LCD blinking interrupt enable, segments switched off"]
    #[inline(always)]
    pub fn lcdblkoffie(&mut self) -> LcdblkoffieW<'_, Lcdcctl1Spec> {
        LcdblkoffieW::new(self, 9)
    }
    #[doc = "Bit 10 - LCD blinking interrupt enable, segments switched on"]
    #[inline(always)]
    pub fn lcdblkonie(&mut self) -> LcdblkonieW<'_, Lcdcctl1Spec> {
        LcdblkonieW::new(self, 10)
    }
    #[doc = "Bit 11 - No capacitance connected interrupt enable"]
    #[inline(always)]
    pub fn lcdnocapie(&mut self) -> LcdnocapieW<'_, Lcdcctl1Spec> {
        LcdnocapieW::new(self, 11)
    }
}
#[doc = "LCD_C control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdcctl1Spec;
impl crate::RegisterSpec for Lcdcctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdcctl1::R`](R) reader structure"]
impl crate::Readable for Lcdcctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdcctl1::W`](W) writer structure"]
impl crate::Writable for Lcdcctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDCCTL1 to value 0"]
impl crate::Resettable for Lcdcctl1Spec {}
