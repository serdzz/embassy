#[doc = "Register `CACTL1` reader"]
pub type R = crate::R<Cactl1Spec>;
#[doc = "Register `CACTL1` writer"]
pub type W = crate::W<Cactl1Spec>;
#[doc = "Field `CAIFG` reader - Comp. A Interrupt Flag"]
pub type CaifgR = crate::BitReader;
#[doc = "Field `CAIFG` writer - Comp. A Interrupt Flag"]
pub type CaifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAIE` reader - Comp. A Interrupt Enable"]
pub type CaieR = crate::BitReader;
#[doc = "Field `CAIE` writer - Comp. A Interrupt Enable"]
pub type CaieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAIES` reader - Comp. A Int. Edge Select: 0:rising / 1:falling"]
pub type CaiesR = crate::BitReader;
#[doc = "Field `CAIES` writer - Comp. A Int. Edge Select: 0:rising / 1:falling"]
pub type CaiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAON` reader - Comp. A enable"]
pub type CaonR = crate::BitReader;
#[doc = "Field `CAON` writer - Comp. A enable"]
pub type CaonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Comp. A Internal Reference Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Caref {
    #[doc = "0: Comp. A Int. Ref. Select 0 : Off"]
    Caref0 = 0,
    #[doc = "1: Comp. A Int. Ref. Select 1 : 0.25*Vcc"]
    Caref1 = 1,
    #[doc = "2: Comp. A Int. Ref. Select 2 : 0.5*Vcc"]
    Caref2 = 2,
    #[doc = "3: Comp. A Int. Ref. Select 3 : Vt"]
    Caref3 = 3,
}
impl From<Caref> for u8 {
    #[inline(always)]
    fn from(variant: Caref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Caref {
    type Ux = u8;
}
impl crate::IsEnum for Caref {}
#[doc = "Field `CAREF` reader - Comp. A Internal Reference Select 0"]
pub type CarefR = crate::FieldReader<Caref>;
impl CarefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Caref {
        match self.bits {
            0 => Caref::Caref0,
            1 => Caref::Caref1,
            2 => Caref::Caref2,
            3 => Caref::Caref3,
            _ => unreachable!(),
        }
    }
    #[doc = "Comp. A Int. Ref. Select 0 : Off"]
    #[inline(always)]
    pub fn is_caref_0(&self) -> bool {
        *self == Caref::Caref0
    }
    #[doc = "Comp. A Int. Ref. Select 1 : 0.25*Vcc"]
    #[inline(always)]
    pub fn is_caref_1(&self) -> bool {
        *self == Caref::Caref1
    }
    #[doc = "Comp. A Int. Ref. Select 2 : 0.5*Vcc"]
    #[inline(always)]
    pub fn is_caref_2(&self) -> bool {
        *self == Caref::Caref2
    }
    #[doc = "Comp. A Int. Ref. Select 3 : Vt"]
    #[inline(always)]
    pub fn is_caref_3(&self) -> bool {
        *self == Caref::Caref3
    }
}
#[doc = "Field `CAREF` writer - Comp. A Internal Reference Select 0"]
pub type CarefW<'a, REG> = crate::FieldWriter<'a, REG, 2, Caref, crate::Safe>;
impl<'a, REG> CarefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comp. A Int. Ref. Select 0 : Off"]
    #[inline(always)]
    pub fn caref_0(self) -> &'a mut crate::W<REG> {
        self.variant(Caref::Caref0)
    }
    #[doc = "Comp. A Int. Ref. Select 1 : 0.25*Vcc"]
    #[inline(always)]
    pub fn caref_1(self) -> &'a mut crate::W<REG> {
        self.variant(Caref::Caref1)
    }
    #[doc = "Comp. A Int. Ref. Select 2 : 0.5*Vcc"]
    #[inline(always)]
    pub fn caref_2(self) -> &'a mut crate::W<REG> {
        self.variant(Caref::Caref2)
    }
    #[doc = "Comp. A Int. Ref. Select 3 : Vt"]
    #[inline(always)]
    pub fn caref_3(self) -> &'a mut crate::W<REG> {
        self.variant(Caref::Caref3)
    }
}
#[doc = "Field `CARSEL` reader - Comp. A Internal Reference Enable"]
pub type CarselR = crate::BitReader;
#[doc = "Field `CARSEL` writer - Comp. A Internal Reference Enable"]
pub type CarselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAEX` reader - Comp. A Exchange Inputs"]
pub type CaexR = crate::BitReader;
#[doc = "Field `CAEX` writer - Comp. A Exchange Inputs"]
pub type CaexW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comp. A Interrupt Flag"]
    #[inline(always)]
    pub fn caifg(&self) -> CaifgR {
        CaifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comp. A Interrupt Enable"]
    #[inline(always)]
    pub fn caie(&self) -> CaieR {
        CaieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comp. A Int. Edge Select: 0:rising / 1:falling"]
    #[inline(always)]
    pub fn caies(&self) -> CaiesR {
        CaiesR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comp. A enable"]
    #[inline(always)]
    pub fn caon(&self) -> CaonR {
        CaonR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Comp. A Internal Reference Select 0"]
    #[inline(always)]
    pub fn caref(&self) -> CarefR {
        CarefR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Comp. A Internal Reference Enable"]
    #[inline(always)]
    pub fn carsel(&self) -> CarselR {
        CarselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comp. A Exchange Inputs"]
    #[inline(always)]
    pub fn caex(&self) -> CaexR {
        CaexR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. A Interrupt Flag"]
    #[inline(always)]
    pub fn caifg(&mut self) -> CaifgW<'_, Cactl1Spec> {
        CaifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Comp. A Interrupt Enable"]
    #[inline(always)]
    pub fn caie(&mut self) -> CaieW<'_, Cactl1Spec> {
        CaieW::new(self, 1)
    }
    #[doc = "Bit 2 - Comp. A Int. Edge Select: 0:rising / 1:falling"]
    #[inline(always)]
    pub fn caies(&mut self) -> CaiesW<'_, Cactl1Spec> {
        CaiesW::new(self, 2)
    }
    #[doc = "Bit 3 - Comp. A enable"]
    #[inline(always)]
    pub fn caon(&mut self) -> CaonW<'_, Cactl1Spec> {
        CaonW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Comp. A Internal Reference Select 0"]
    #[inline(always)]
    pub fn caref(&mut self) -> CarefW<'_, Cactl1Spec> {
        CarefW::new(self, 4)
    }
    #[doc = "Bit 6 - Comp. A Internal Reference Enable"]
    #[inline(always)]
    pub fn carsel(&mut self) -> CarselW<'_, Cactl1Spec> {
        CarselW::new(self, 6)
    }
    #[doc = "Bit 7 - Comp. A Exchange Inputs"]
    #[inline(always)]
    pub fn caex(&mut self) -> CaexW<'_, Cactl1Spec> {
        CaexW::new(self, 7)
    }
}
#[doc = "Comparator A Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cactl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cactl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cactl1Spec;
impl crate::RegisterSpec for Cactl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cactl1::R`](R) reader structure"]
impl crate::Readable for Cactl1Spec {}
#[doc = "`write(|w| ..)` method takes [`cactl1::W`](W) writer structure"]
impl crate::Writable for Cactl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACTL1 to value 0"]
impl crate::Resettable for Cactl1Spec {}
