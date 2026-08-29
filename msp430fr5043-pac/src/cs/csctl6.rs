#[doc = "Register `CSCTL6` reader"]
pub type R = crate::R<Csctl6Spec>;
#[doc = "Register `CSCTL6` writer"]
pub type W = crate::W<Csctl6Spec>;
#[doc = "ACLK clock request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aclkreqen {
    #[doc = "0: ACLK conditional requests are disabled"]
    Disable = 0,
    #[doc = "1: ACLK conditional requests are enabled"]
    Enable = 1,
}
impl From<Aclkreqen> for bool {
    #[inline(always)]
    fn from(variant: Aclkreqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACLKREQEN` reader - ACLK clock request enable"]
pub type AclkreqenR = crate::BitReader<Aclkreqen>;
impl AclkreqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aclkreqen {
        match self.bits {
            false => Aclkreqen::Disable,
            true => Aclkreqen::Enable,
        }
    }
    #[doc = "ACLK conditional requests are disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Aclkreqen::Disable
    }
    #[doc = "ACLK conditional requests are enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Aclkreqen::Enable
    }
}
#[doc = "Field `ACLKREQEN` writer - ACLK clock request enable"]
pub type AclkreqenW<'a, REG> = crate::BitWriter<'a, REG, Aclkreqen>;
impl<'a, REG> AclkreqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ACLK conditional requests are disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Aclkreqen::Disable)
    }
    #[doc = "ACLK conditional requests are enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Aclkreqen::Enable)
    }
}
#[doc = "MCLK clock request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mclkreqen {
    #[doc = "0: MCLK conditional requests are disabled"]
    Disable = 0,
    #[doc = "1: MCLK conditional requests are enabled"]
    Enable = 1,
}
impl From<Mclkreqen> for bool {
    #[inline(always)]
    fn from(variant: Mclkreqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLKREQEN` reader - MCLK clock request enable"]
pub type MclkreqenR = crate::BitReader<Mclkreqen>;
impl MclkreqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mclkreqen {
        match self.bits {
            false => Mclkreqen::Disable,
            true => Mclkreqen::Enable,
        }
    }
    #[doc = "MCLK conditional requests are disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mclkreqen::Disable
    }
    #[doc = "MCLK conditional requests are enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mclkreqen::Enable
    }
}
#[doc = "Field `MCLKREQEN` writer - MCLK clock request enable"]
pub type MclkreqenW<'a, REG> = crate::BitWriter<'a, REG, Mclkreqen>;
impl<'a, REG> MclkreqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCLK conditional requests are disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkreqen::Disable)
    }
    #[doc = "MCLK conditional requests are enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkreqen::Enable)
    }
}
#[doc = "SMCLK clock request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smclkreqen {
    #[doc = "0: SMCLK conditional requests are disabled"]
    Disable = 0,
    #[doc = "1: SMCLK conditional requests are enabled"]
    Enable = 1,
}
impl From<Smclkreqen> for bool {
    #[inline(always)]
    fn from(variant: Smclkreqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMCLKREQEN` reader - SMCLK clock request enable"]
pub type SmclkreqenR = crate::BitReader<Smclkreqen>;
impl SmclkreqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smclkreqen {
        match self.bits {
            false => Smclkreqen::Disable,
            true => Smclkreqen::Enable,
        }
    }
    #[doc = "SMCLK conditional requests are disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Smclkreqen::Disable
    }
    #[doc = "SMCLK conditional requests are enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Smclkreqen::Enable
    }
}
#[doc = "Field `SMCLKREQEN` writer - SMCLK clock request enable"]
pub type SmclkreqenW<'a, REG> = crate::BitWriter<'a, REG, Smclkreqen>;
impl<'a, REG> SmclkreqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMCLK conditional requests are disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Smclkreqen::Disable)
    }
    #[doc = "SMCLK conditional requests are enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Smclkreqen::Enable)
    }
}
#[doc = "MODCLK clock request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modclkreqen {
    #[doc = "0: MODCLK conditional requests are disabled"]
    Disable = 0,
    #[doc = "1: MODCLK conditional requests are enabled"]
    Enable = 1,
}
impl From<Modclkreqen> for bool {
    #[inline(always)]
    fn from(variant: Modclkreqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODCLKREQEN` reader - MODCLK clock request enable"]
pub type ModclkreqenR = crate::BitReader<Modclkreqen>;
impl ModclkreqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modclkreqen {
        match self.bits {
            false => Modclkreqen::Disable,
            true => Modclkreqen::Enable,
        }
    }
    #[doc = "MODCLK conditional requests are disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Modclkreqen::Disable
    }
    #[doc = "MODCLK conditional requests are enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Modclkreqen::Enable
    }
}
#[doc = "Field `MODCLKREQEN` writer - MODCLK clock request enable"]
pub type ModclkreqenW<'a, REG> = crate::BitWriter<'a, REG, Modclkreqen>;
impl<'a, REG> ModclkreqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MODCLK conditional requests are disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Modclkreqen::Disable)
    }
    #[doc = "MODCLK conditional requests are enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Modclkreqen::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - ACLK clock request enable"]
    #[inline(always)]
    pub fn aclkreqen(&self) -> AclkreqenR {
        AclkreqenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCLK clock request enable"]
    #[inline(always)]
    pub fn mclkreqen(&self) -> MclkreqenR {
        MclkreqenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMCLK clock request enable"]
    #[inline(always)]
    pub fn smclkreqen(&self) -> SmclkreqenR {
        SmclkreqenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MODCLK clock request enable"]
    #[inline(always)]
    pub fn modclkreqen(&self) -> ModclkreqenR {
        ModclkreqenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACLK clock request enable"]
    #[inline(always)]
    pub fn aclkreqen(&mut self) -> AclkreqenW<'_, Csctl6Spec> {
        AclkreqenW::new(self, 0)
    }
    #[doc = "Bit 1 - MCLK clock request enable"]
    #[inline(always)]
    pub fn mclkreqen(&mut self) -> MclkreqenW<'_, Csctl6Spec> {
        MclkreqenW::new(self, 1)
    }
    #[doc = "Bit 2 - SMCLK clock request enable"]
    #[inline(always)]
    pub fn smclkreqen(&mut self) -> SmclkreqenW<'_, Csctl6Spec> {
        SmclkreqenW::new(self, 2)
    }
    #[doc = "Bit 3 - MODCLK clock request enable"]
    #[inline(always)]
    pub fn modclkreqen(&mut self) -> ModclkreqenW<'_, Csctl6Spec> {
        ModclkreqenW::new(self, 3)
    }
}
#[doc = "Clock System Control 6\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl6Spec;
impl crate::RegisterSpec for Csctl6Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl6::R`](R) reader structure"]
impl crate::Readable for Csctl6Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl6::W`](W) writer structure"]
impl crate::Writable for Csctl6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL6 to value 0"]
impl crate::Resettable for Csctl6Spec {}
