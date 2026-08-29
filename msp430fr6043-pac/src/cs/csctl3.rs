#[doc = "Register `CSCTL3` reader"]
pub type R = crate::R<Csctl3Spec>;
#[doc = "Register `CSCTL3` writer"]
pub type W = crate::W<Csctl3Spec>;
#[doc = "MCLK source divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divm {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /2"]
    _2 = 1,
    #[doc = "2: /4"]
    _4 = 2,
    #[doc = "3: /8"]
    _8 = 3,
    #[doc = "4: /16"]
    _16 = 4,
    #[doc = "5: /32"]
    _32 = 5,
}
impl From<Divm> for u8 {
    #[inline(always)]
    fn from(variant: Divm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divm {
    type Ux = u8;
}
impl crate::IsEnum for Divm {}
#[doc = "Field `DIVM` reader - MCLK source divider"]
pub type DivmR = crate::FieldReader<Divm>;
impl DivmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Divm> {
        match self.bits {
            0 => Some(Divm::_1),
            1 => Some(Divm::_2),
            2 => Some(Divm::_4),
            3 => Some(Divm::_8),
            4 => Some(Divm::_16),
            5 => Some(Divm::_32),
            _ => None,
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Divm::_1
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Divm::_2
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Divm::_4
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Divm::_8
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Divm::_16
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Divm::_32
    }
}
#[doc = "Field `DIVM` writer - MCLK source divider"]
pub type DivmW<'a, REG> = crate::FieldWriter<'a, REG, 3, Divm>;
impl<'a, REG> DivmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::_1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::_8)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::_16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::_32)
    }
}
#[doc = "SMCLK source divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divs {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /2"]
    _2 = 1,
    #[doc = "2: /4"]
    _4 = 2,
    #[doc = "3: /8"]
    _8 = 3,
    #[doc = "4: /16"]
    _16 = 4,
    #[doc = "5: /32"]
    _32 = 5,
}
impl From<Divs> for u8 {
    #[inline(always)]
    fn from(variant: Divs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divs {
    type Ux = u8;
}
impl crate::IsEnum for Divs {}
#[doc = "Field `DIVS` reader - SMCLK source divider"]
pub type DivsR = crate::FieldReader<Divs>;
impl DivsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Divs> {
        match self.bits {
            0 => Some(Divs::_1),
            1 => Some(Divs::_2),
            2 => Some(Divs::_4),
            3 => Some(Divs::_8),
            4 => Some(Divs::_16),
            5 => Some(Divs::_32),
            _ => None,
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Divs::_1
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Divs::_2
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Divs::_4
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Divs::_8
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Divs::_16
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Divs::_32
    }
}
#[doc = "Field `DIVS` writer - SMCLK source divider"]
pub type DivsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Divs>;
impl<'a, REG> DivsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::_1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::_8)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::_16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::_32)
    }
}
#[doc = "ACLK source divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Diva {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /2"]
    _2 = 1,
    #[doc = "2: /4"]
    _4 = 2,
    #[doc = "3: /8"]
    _8 = 3,
    #[doc = "4: /16"]
    _16 = 4,
    #[doc = "5: /32"]
    _32 = 5,
}
impl From<Diva> for u8 {
    #[inline(always)]
    fn from(variant: Diva) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Diva {
    type Ux = u8;
}
impl crate::IsEnum for Diva {}
#[doc = "Field `DIVA` reader - ACLK source divider"]
pub type DivaR = crate::FieldReader<Diva>;
impl DivaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Diva> {
        match self.bits {
            0 => Some(Diva::_1),
            1 => Some(Diva::_2),
            2 => Some(Diva::_4),
            3 => Some(Diva::_8),
            4 => Some(Diva::_16),
            5 => Some(Diva::_32),
            _ => None,
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Diva::_1
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Diva::_2
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Diva::_4
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Diva::_8
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Diva::_16
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Diva::_32
    }
}
#[doc = "Field `DIVA` writer - ACLK source divider"]
pub type DivaW<'a, REG> = crate::FieldWriter<'a, REG, 3, Diva>;
impl<'a, REG> DivaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_8)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_32)
    }
}
impl R {
    #[doc = "Bits 0:2 - MCLK source divider"]
    #[inline(always)]
    pub fn divm(&self) -> DivmR {
        DivmR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - SMCLK source divider"]
    #[inline(always)]
    pub fn divs(&self) -> DivsR {
        DivsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - ACLK source divider"]
    #[inline(always)]
    pub fn diva(&self) -> DivaR {
        DivaR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK source divider"]
    #[inline(always)]
    pub fn divm(&mut self) -> DivmW<'_, Csctl3Spec> {
        DivmW::new(self, 0)
    }
    #[doc = "Bits 4:6 - SMCLK source divider"]
    #[inline(always)]
    pub fn divs(&mut self) -> DivsW<'_, Csctl3Spec> {
        DivsW::new(self, 4)
    }
    #[doc = "Bits 8:10 - ACLK source divider"]
    #[inline(always)]
    pub fn diva(&mut self) -> DivaW<'_, Csctl3Spec> {
        DivaW::new(self, 8)
    }
}
#[doc = "Clock System Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl3Spec;
impl crate::RegisterSpec for Csctl3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl3::R`](R) reader structure"]
impl crate::Readable for Csctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl3::W`](W) writer structure"]
impl crate::Writable for Csctl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL3 to value 0"]
impl crate::Resettable for Csctl3Spec {}
