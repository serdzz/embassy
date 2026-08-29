#[doc = "Register `LCDCMEMCTL` reader"]
pub type R = crate::R<LcdcmemctlSpec>;
#[doc = "Register `LCDCMEMCTL` writer"]
pub type W = crate::W<LcdcmemctlSpec>;
#[doc = "Select LCD memory registers for display\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcddisp {
    #[doc = "0: Display content of LCD memory registers LCDM"]
    Lcddisp0 = 0,
    #[doc = "1: Display content of LCD blinking memory registers LCDBM"]
    Lcddisp1 = 1,
}
impl From<Lcddisp> for bool {
    #[inline(always)]
    fn from(variant: Lcddisp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDDISP` reader - Select LCD memory registers for display"]
pub type LcddispR = crate::BitReader<Lcddisp>;
impl LcddispR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcddisp {
        match self.bits {
            false => Lcddisp::Lcddisp0,
            true => Lcddisp::Lcddisp1,
        }
    }
    #[doc = "Display content of LCD memory registers LCDM"]
    #[inline(always)]
    pub fn is_lcddisp_0(&self) -> bool {
        *self == Lcddisp::Lcddisp0
    }
    #[doc = "Display content of LCD blinking memory registers LCDBM"]
    #[inline(always)]
    pub fn is_lcddisp_1(&self) -> bool {
        *self == Lcddisp::Lcddisp1
    }
}
#[doc = "Field `LCDDISP` writer - Select LCD memory registers for display"]
pub type LcddispW<'a, REG> = crate::BitWriter<'a, REG, Lcddisp>;
impl<'a, REG> LcddispW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Display content of LCD memory registers LCDM"]
    #[inline(always)]
    pub fn lcddisp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddisp::Lcddisp0)
    }
    #[doc = "Display content of LCD blinking memory registers LCDBM"]
    #[inline(always)]
    pub fn lcddisp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddisp::Lcddisp1)
    }
}
#[doc = "Clear LCD memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdclrm {
    #[doc = "0: Contents of LCD memory registers LCDMx remain unchanged"]
    Lcdclrm0 = 0,
    #[doc = "1: Clear content of all LCD memory registers LCDM"]
    Lcdclrm1 = 1,
}
impl From<Lcdclrm> for bool {
    #[inline(always)]
    fn from(variant: Lcdclrm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCLRM` reader - Clear LCD memory"]
pub type LcdclrmR = crate::BitReader<Lcdclrm>;
impl LcdclrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdclrm {
        match self.bits {
            false => Lcdclrm::Lcdclrm0,
            true => Lcdclrm::Lcdclrm1,
        }
    }
    #[doc = "Contents of LCD memory registers LCDMx remain unchanged"]
    #[inline(always)]
    pub fn is_lcdclrm_0(&self) -> bool {
        *self == Lcdclrm::Lcdclrm0
    }
    #[doc = "Clear content of all LCD memory registers LCDM"]
    #[inline(always)]
    pub fn is_lcdclrm_1(&self) -> bool {
        *self == Lcdclrm::Lcdclrm1
    }
}
#[doc = "Field `LCDCLRM` writer - Clear LCD memory"]
pub type LcdclrmW<'a, REG> = crate::BitWriter<'a, REG, Lcdclrm>;
impl<'a, REG> LcdclrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Contents of LCD memory registers LCDMx remain unchanged"]
    #[inline(always)]
    pub fn lcdclrm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdclrm::Lcdclrm0)
    }
    #[doc = "Clear content of all LCD memory registers LCDM"]
    #[inline(always)]
    pub fn lcdclrm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdclrm::Lcdclrm1)
    }
}
#[doc = "Clear LCD blinking memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdclrbm {
    #[doc = "0: Contents of blinking memory registers LCDBM remain unchanged"]
    Lcdclrbm0 = 0,
    #[doc = "1: Clear content of all blinking memory registers LCDBM"]
    Lcdclrbm1 = 1,
}
impl From<Lcdclrbm> for bool {
    #[inline(always)]
    fn from(variant: Lcdclrbm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCLRBM` reader - Clear LCD blinking memory"]
pub type LcdclrbmR = crate::BitReader<Lcdclrbm>;
impl LcdclrbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdclrbm {
        match self.bits {
            false => Lcdclrbm::Lcdclrbm0,
            true => Lcdclrbm::Lcdclrbm1,
        }
    }
    #[doc = "Contents of blinking memory registers LCDBM remain unchanged"]
    #[inline(always)]
    pub fn is_lcdclrbm_0(&self) -> bool {
        *self == Lcdclrbm::Lcdclrbm0
    }
    #[doc = "Clear content of all blinking memory registers LCDBM"]
    #[inline(always)]
    pub fn is_lcdclrbm_1(&self) -> bool {
        *self == Lcdclrbm::Lcdclrbm1
    }
}
#[doc = "Field `LCDCLRBM` writer - Clear LCD blinking memory"]
pub type LcdclrbmW<'a, REG> = crate::BitWriter<'a, REG, Lcdclrbm>;
impl<'a, REG> LcdclrbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Contents of blinking memory registers LCDBM remain unchanged"]
    #[inline(always)]
    pub fn lcdclrbm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdclrbm::Lcdclrbm0)
    }
    #[doc = "Clear content of all blinking memory registers LCDBM"]
    #[inline(always)]
    pub fn lcdclrbm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdclrbm::Lcdclrbm1)
    }
}
impl R {
    #[doc = "Bit 0 - Select LCD memory registers for display"]
    #[inline(always)]
    pub fn lcddisp(&self) -> LcddispR {
        LcddispR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear LCD memory"]
    #[inline(always)]
    pub fn lcdclrm(&self) -> LcdclrmR {
        LcdclrmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear LCD blinking memory"]
    #[inline(always)]
    pub fn lcdclrbm(&self) -> LcdclrbmR {
        LcdclrbmR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select LCD memory registers for display"]
    #[inline(always)]
    pub fn lcddisp(&mut self) -> LcddispW<'_, LcdcmemctlSpec> {
        LcddispW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear LCD memory"]
    #[inline(always)]
    pub fn lcdclrm(&mut self) -> LcdclrmW<'_, LcdcmemctlSpec> {
        LcdclrmW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear LCD blinking memory"]
    #[inline(always)]
    pub fn lcdclrbm(&mut self) -> LcdclrbmW<'_, LcdcmemctlSpec> {
        LcdclrbmW::new(self, 2)
    }
}
#[doc = "LCD_C memory control\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcmemctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcmemctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcmemctlSpec;
impl crate::RegisterSpec for LcdcmemctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdcmemctl::R`](R) reader structure"]
impl crate::Readable for LcdcmemctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdcmemctl::W`](W) writer structure"]
impl crate::Writable for LcdcmemctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDCMEMCTL to value 0"]
impl crate::Resettable for LcdcmemctlSpec {}
