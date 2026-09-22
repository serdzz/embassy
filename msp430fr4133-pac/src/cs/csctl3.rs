#[doc = "Register `CSCTL3` reader"]
pub type R = crate::R<Csctl3Spec>;
#[doc = "Register `CSCTL3` writer"]
pub type W = crate::W<Csctl3Spec>;
#[doc = "Reference Divider Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fllrefdiv {
    #[doc = "0: Reference Divider: f(LFCLK)/1"]
    Fllrefdiv0 = 0,
    #[doc = "1: Reference Divider: f(LFCLK)/2"]
    Fllrefdiv1 = 1,
    #[doc = "2: Reference Divider: f(LFCLK)/4"]
    Fllrefdiv2 = 2,
    #[doc = "3: Reference Divider: f(LFCLK)/8"]
    Fllrefdiv3 = 3,
    #[doc = "4: Reference Divider: f(LFCLK)/12"]
    Fllrefdiv4 = 4,
    #[doc = "5: Reference Divider: f(LFCLK)/16"]
    Fllrefdiv5 = 5,
    #[doc = "6: Reference Divider: f(LFCLK)/16"]
    Fllrefdiv6 = 6,
    #[doc = "7: Reference Divider: f(LFCLK)/16"]
    Fllrefdiv7 = 7,
}
impl From<Fllrefdiv> for u8 {
    #[inline(always)]
    fn from(variant: Fllrefdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fllrefdiv {
    type Ux = u8;
}
impl crate::IsEnum for Fllrefdiv {}
#[doc = "Field `FLLREFDIV` reader - Reference Divider Bit : 0"]
pub type FllrefdivR = crate::FieldReader<Fllrefdiv>;
impl FllrefdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fllrefdiv {
        match self.bits {
            0 => Fllrefdiv::Fllrefdiv0,
            1 => Fllrefdiv::Fllrefdiv1,
            2 => Fllrefdiv::Fllrefdiv2,
            3 => Fllrefdiv::Fllrefdiv3,
            4 => Fllrefdiv::Fllrefdiv4,
            5 => Fllrefdiv::Fllrefdiv5,
            6 => Fllrefdiv::Fllrefdiv6,
            7 => Fllrefdiv::Fllrefdiv7,
            _ => unreachable!(),
        }
    }
    #[doc = "Reference Divider: f(LFCLK)/1"]
    #[inline(always)]
    pub fn is_fllrefdiv_0(&self) -> bool {
        *self == Fllrefdiv::Fllrefdiv0
    }
    #[doc = "Reference Divider: f(LFCLK)/2"]
    #[inline(always)]
    pub fn is_fllrefdiv_1(&self) -> bool {
        *self == Fllrefdiv::Fllrefdiv1
    }
    #[doc = "Reference Divider: f(LFCLK)/4"]
    #[inline(always)]
    pub fn is_fllrefdiv_2(&self) -> bool {
        *self == Fllrefdiv::Fllrefdiv2
    }
    #[doc = "Reference Divider: f(LFCLK)/8"]
    #[inline(always)]
    pub fn is_fllrefdiv_3(&self) -> bool {
        *self == Fllrefdiv::Fllrefdiv3
    }
    #[doc = "Reference Divider: f(LFCLK)/12"]
    #[inline(always)]
    pub fn is_fllrefdiv_4(&self) -> bool {
        *self == Fllrefdiv::Fllrefdiv4
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn is_fllrefdiv_5(&self) -> bool {
        *self == Fllrefdiv::Fllrefdiv5
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn is_fllrefdiv_6(&self) -> bool {
        *self == Fllrefdiv::Fllrefdiv6
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn is_fllrefdiv_7(&self) -> bool {
        *self == Fllrefdiv::Fllrefdiv7
    }
}
#[doc = "Field `FLLREFDIV` writer - Reference Divider Bit : 0"]
pub type FllrefdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fllrefdiv, crate::Safe>;
impl<'a, REG> FllrefdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reference Divider: f(LFCLK)/1"]
    #[inline(always)]
    pub fn fllrefdiv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::Fllrefdiv0)
    }
    #[doc = "Reference Divider: f(LFCLK)/2"]
    #[inline(always)]
    pub fn fllrefdiv_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::Fllrefdiv1)
    }
    #[doc = "Reference Divider: f(LFCLK)/4"]
    #[inline(always)]
    pub fn fllrefdiv_2(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::Fllrefdiv2)
    }
    #[doc = "Reference Divider: f(LFCLK)/8"]
    #[inline(always)]
    pub fn fllrefdiv_3(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::Fllrefdiv3)
    }
    #[doc = "Reference Divider: f(LFCLK)/12"]
    #[inline(always)]
    pub fn fllrefdiv_4(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::Fllrefdiv4)
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn fllrefdiv_5(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::Fllrefdiv5)
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn fllrefdiv_6(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::Fllrefdiv6)
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn fllrefdiv_7(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::Fllrefdiv7)
    }
}
#[doc = "FLL Reference Clock Select Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selref {
    #[doc = "0: FLL Reference Clock Select 0"]
    Selref0 = 0,
    #[doc = "1: FLL Reference Clock Select 1"]
    Selref1 = 1,
    #[doc = "2: FLL Reference Clock Select 2"]
    Selref2 = 2,
    #[doc = "3: FLL Reference Clock Select 3"]
    Selref3 = 3,
}
impl From<Selref> for u8 {
    #[inline(always)]
    fn from(variant: Selref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selref {
    type Ux = u8;
}
impl crate::IsEnum for Selref {}
#[doc = "Field `SELREF` reader - FLL Reference Clock Select Bit : 0"]
pub type SelrefR = crate::FieldReader<Selref>;
impl SelrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selref {
        match self.bits {
            0 => Selref::Selref0,
            1 => Selref::Selref1,
            2 => Selref::Selref2,
            3 => Selref::Selref3,
            _ => unreachable!(),
        }
    }
    #[doc = "FLL Reference Clock Select 0"]
    #[inline(always)]
    pub fn is_selref_0(&self) -> bool {
        *self == Selref::Selref0
    }
    #[doc = "FLL Reference Clock Select 1"]
    #[inline(always)]
    pub fn is_selref_1(&self) -> bool {
        *self == Selref::Selref1
    }
    #[doc = "FLL Reference Clock Select 2"]
    #[inline(always)]
    pub fn is_selref_2(&self) -> bool {
        *self == Selref::Selref2
    }
    #[doc = "FLL Reference Clock Select 3"]
    #[inline(always)]
    pub fn is_selref_3(&self) -> bool {
        *self == Selref::Selref3
    }
}
#[doc = "Field `SELREF` writer - FLL Reference Clock Select Bit : 0"]
pub type SelrefW<'a, REG> = crate::FieldWriter<'a, REG, 2, Selref, crate::Safe>;
impl<'a, REG> SelrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FLL Reference Clock Select 0"]
    #[inline(always)]
    pub fn selref_0(self) -> &'a mut crate::W<REG> {
        self.variant(Selref::Selref0)
    }
    #[doc = "FLL Reference Clock Select 1"]
    #[inline(always)]
    pub fn selref_1(self) -> &'a mut crate::W<REG> {
        self.variant(Selref::Selref1)
    }
    #[doc = "FLL Reference Clock Select 2"]
    #[inline(always)]
    pub fn selref_2(self) -> &'a mut crate::W<REG> {
        self.variant(Selref::Selref2)
    }
    #[doc = "FLL Reference Clock Select 3"]
    #[inline(always)]
    pub fn selref_3(self) -> &'a mut crate::W<REG> {
        self.variant(Selref::Selref3)
    }
}
impl R {
    #[doc = "Bits 0:2 - Reference Divider Bit : 0"]
    #[inline(always)]
    pub fn fllrefdiv(&self) -> FllrefdivR {
        FllrefdivR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - FLL Reference Clock Select Bit : 0"]
    #[inline(always)]
    pub fn selref(&self) -> SelrefR {
        SelrefR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reference Divider Bit : 0"]
    #[inline(always)]
    pub fn fllrefdiv(&mut self) -> FllrefdivW<'_, Csctl3Spec> {
        FllrefdivW::new(self, 0)
    }
    #[doc = "Bits 4:5 - FLL Reference Clock Select Bit : 0"]
    #[inline(always)]
    pub fn selref(&mut self) -> SelrefW<'_, Csctl3Spec> {
        SelrefW::new(self, 4)
    }
}
#[doc = "CS Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
