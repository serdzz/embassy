#[doc = "Register `LCDCPCTL3` reader"]
pub type R = crate::R<Lcdcpctl3Spec>;
#[doc = "Register `LCDCPCTL3` writer"]
pub type W = crate::W<Lcdcpctl3Spec>;
#[doc = "LCD segment line 48 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds48 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds48_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds48_1 = 1,
}
impl From<Lcds48> for bool {
    #[inline(always)]
    fn from(variant: Lcds48) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS48` reader - LCD segment line 48 enable"]
pub type Lcds48R = crate::BitReader<Lcds48>;
impl Lcds48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds48 {
        match self.bits {
            false => Lcds48::Lcds48_0,
            true => Lcds48::Lcds48_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds48_0(&self) -> bool {
        *self == Lcds48::Lcds48_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds48_1(&self) -> bool {
        *self == Lcds48::Lcds48_1
    }
}
#[doc = "Field `LCDS48` writer - LCD segment line 48 enable"]
pub type Lcds48W<'a, REG> = crate::BitWriter<'a, REG, Lcds48>;
impl<'a, REG> Lcds48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds48_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds48::Lcds48_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds48_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds48::Lcds48_1)
    }
}
#[doc = "LCD segment line 49 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds49 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds49_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds49_1 = 1,
}
impl From<Lcds49> for bool {
    #[inline(always)]
    fn from(variant: Lcds49) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS49` reader - LCD segment line 49 enable"]
pub type Lcds49R = crate::BitReader<Lcds49>;
impl Lcds49R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds49 {
        match self.bits {
            false => Lcds49::Lcds49_0,
            true => Lcds49::Lcds49_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds49_0(&self) -> bool {
        *self == Lcds49::Lcds49_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds49_1(&self) -> bool {
        *self == Lcds49::Lcds49_1
    }
}
#[doc = "Field `LCDS49` writer - LCD segment line 49 enable"]
pub type Lcds49W<'a, REG> = crate::BitWriter<'a, REG, Lcds49>;
impl<'a, REG> Lcds49W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds49_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds49::Lcds49_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds49_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds49::Lcds49_1)
    }
}
#[doc = "LCD segment line 50 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds50 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds50_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds50_1 = 1,
}
impl From<Lcds50> for bool {
    #[inline(always)]
    fn from(variant: Lcds50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS50` reader - LCD segment line 50 enable"]
pub type Lcds50R = crate::BitReader<Lcds50>;
impl Lcds50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds50 {
        match self.bits {
            false => Lcds50::Lcds50_0,
            true => Lcds50::Lcds50_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds50_0(&self) -> bool {
        *self == Lcds50::Lcds50_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds50_1(&self) -> bool {
        *self == Lcds50::Lcds50_1
    }
}
#[doc = "Field `LCDS50` writer - LCD segment line 50 enable"]
pub type Lcds50W<'a, REG> = crate::BitWriter<'a, REG, Lcds50>;
impl<'a, REG> Lcds50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds50_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds50::Lcds50_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds50_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds50::Lcds50_1)
    }
}
#[doc = "LCD segment line 51 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds51 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds51_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds51_1 = 1,
}
impl From<Lcds51> for bool {
    #[inline(always)]
    fn from(variant: Lcds51) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS51` reader - LCD segment line 51 enable"]
pub type Lcds51R = crate::BitReader<Lcds51>;
impl Lcds51R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds51 {
        match self.bits {
            false => Lcds51::Lcds51_0,
            true => Lcds51::Lcds51_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds51_0(&self) -> bool {
        *self == Lcds51::Lcds51_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds51_1(&self) -> bool {
        *self == Lcds51::Lcds51_1
    }
}
#[doc = "Field `LCDS51` writer - LCD segment line 51 enable"]
pub type Lcds51W<'a, REG> = crate::BitWriter<'a, REG, Lcds51>;
impl<'a, REG> Lcds51W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds51_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds51::Lcds51_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds51_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds51::Lcds51_1)
    }
}
#[doc = "LCD segment line 52 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds52 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds52_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds52_1 = 1,
}
impl From<Lcds52> for bool {
    #[inline(always)]
    fn from(variant: Lcds52) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS52` reader - LCD segment line 52 enable"]
pub type Lcds52R = crate::BitReader<Lcds52>;
impl Lcds52R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds52 {
        match self.bits {
            false => Lcds52::Lcds52_0,
            true => Lcds52::Lcds52_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds52_0(&self) -> bool {
        *self == Lcds52::Lcds52_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds52_1(&self) -> bool {
        *self == Lcds52::Lcds52_1
    }
}
#[doc = "Field `LCDS52` writer - LCD segment line 52 enable"]
pub type Lcds52W<'a, REG> = crate::BitWriter<'a, REG, Lcds52>;
impl<'a, REG> Lcds52W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds52_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds52::Lcds52_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds52_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds52::Lcds52_1)
    }
}
#[doc = "LCD segment line 53 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcds53 {
    #[doc = "0: Multiplexed pins are port functions"]
    Lcds53_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    Lcds53_1 = 1,
}
impl From<Lcds53> for bool {
    #[inline(always)]
    fn from(variant: Lcds53) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDS53` reader - LCD segment line 53 enable"]
pub type Lcds53R = crate::BitReader<Lcds53>;
impl Lcds53R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcds53 {
        match self.bits {
            false => Lcds53::Lcds53_0,
            true => Lcds53::Lcds53_1,
        }
    }
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn is_lcds53_0(&self) -> bool {
        *self == Lcds53::Lcds53_0
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn is_lcds53_1(&self) -> bool {
        *self == Lcds53::Lcds53_1
    }
}
#[doc = "Field `LCDS53` writer - LCD segment line 53 enable"]
pub type Lcds53W<'a, REG> = crate::BitWriter<'a, REG, Lcds53>;
impl<'a, REG> Lcds53W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds53_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds53::Lcds53_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds53_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcds53::Lcds53_1)
    }
}
impl R {
    #[doc = "Bit 0 - LCD segment line 48 enable"]
    #[inline(always)]
    pub fn lcds48(&self) -> Lcds48R {
        Lcds48R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD segment line 49 enable"]
    #[inline(always)]
    pub fn lcds49(&self) -> Lcds49R {
        Lcds49R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD segment line 50 enable"]
    #[inline(always)]
    pub fn lcds50(&self) -> Lcds50R {
        Lcds50R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD segment line 51 enable"]
    #[inline(always)]
    pub fn lcds51(&self) -> Lcds51R {
        Lcds51R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD segment line 52 enable"]
    #[inline(always)]
    pub fn lcds52(&self) -> Lcds52R {
        Lcds52R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD segment line 53 enable"]
    #[inline(always)]
    pub fn lcds53(&self) -> Lcds53R {
        Lcds53R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD segment line 48 enable"]
    #[inline(always)]
    pub fn lcds48(&mut self) -> Lcds48W<'_, Lcdcpctl3Spec> {
        Lcds48W::new(self, 0)
    }
    #[doc = "Bit 1 - LCD segment line 49 enable"]
    #[inline(always)]
    pub fn lcds49(&mut self) -> Lcds49W<'_, Lcdcpctl3Spec> {
        Lcds49W::new(self, 1)
    }
    #[doc = "Bit 2 - LCD segment line 50 enable"]
    #[inline(always)]
    pub fn lcds50(&mut self) -> Lcds50W<'_, Lcdcpctl3Spec> {
        Lcds50W::new(self, 2)
    }
    #[doc = "Bit 3 - LCD segment line 51 enable"]
    #[inline(always)]
    pub fn lcds51(&mut self) -> Lcds51W<'_, Lcdcpctl3Spec> {
        Lcds51W::new(self, 3)
    }
    #[doc = "Bit 4 - LCD segment line 52 enable"]
    #[inline(always)]
    pub fn lcds52(&mut self) -> Lcds52W<'_, Lcdcpctl3Spec> {
        Lcds52W::new(self, 4)
    }
    #[doc = "Bit 5 - LCD segment line 53 enable"]
    #[inline(always)]
    pub fn lcds53(&mut self) -> Lcds53W<'_, Lcdcpctl3Spec> {
        Lcds53W::new(self, 5)
    }
}
#[doc = "LCD_C port control 3 (384 segments)\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcpctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcpctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdcpctl3Spec;
impl crate::RegisterSpec for Lcdcpctl3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdcpctl3::R`](R) reader structure"]
impl crate::Readable for Lcdcpctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdcpctl3::W`](W) writer structure"]
impl crate::Writable for Lcdcpctl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDCPCTL3 to value 0"]
impl crate::Resettable for Lcdcpctl3Spec {}
