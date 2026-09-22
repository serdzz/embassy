#[doc = "Register `ADCCTL2` reader"]
pub type R = crate::R<Adcctl2Spec>;
#[doc = "Register `ADCCTL2` writer"]
pub type W = crate::W<Adcctl2Spec>;
#[doc = "Field `ADCSR` reader - ADC Sampling Rate"]
pub type AdcsrR = crate::BitReader;
#[doc = "Field `ADCSR` writer - ADC Sampling Rate"]
pub type AdcsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDF` reader - ADC Data Format"]
pub type AdcdfR = crate::BitReader;
#[doc = "Field `ADCDF` writer - ADC Data Format"]
pub type AdcdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcres {
    #[doc = "0: 8 bit"]
    Adcres0 = 0,
    #[doc = "1: 10 bit"]
    Adcres1 = 1,
    #[doc = "2: Reserved"]
    Adcres2 = 2,
    #[doc = "3: Reserved"]
    Adcres3 = 3,
}
impl From<Adcres> for u8 {
    #[inline(always)]
    fn from(variant: Adcres) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcres {
    type Ux = u8;
}
impl crate::IsEnum for Adcres {}
#[doc = "Field `ADCRES` reader - ADC Resolution"]
pub type AdcresR = crate::FieldReader<Adcres>;
impl AdcresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcres {
        match self.bits {
            0 => Adcres::Adcres0,
            1 => Adcres::Adcres1,
            2 => Adcres::Adcres2,
            3 => Adcres::Adcres3,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn is_adcres_0(&self) -> bool {
        *self == Adcres::Adcres0
    }
    #[doc = "10 bit"]
    #[inline(always)]
    pub fn is_adcres_1(&self) -> bool {
        *self == Adcres::Adcres1
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adcres_2(&self) -> bool {
        *self == Adcres::Adcres2
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adcres_3(&self) -> bool {
        *self == Adcres::Adcres3
    }
}
#[doc = "Field `ADCRES` writer - ADC Resolution"]
pub type AdcresW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adcres, crate::Safe>;
impl<'a, REG> AdcresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn adcres_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcres::Adcres0)
    }
    #[doc = "10 bit"]
    #[inline(always)]
    pub fn adcres_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcres::Adcres1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adcres_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcres::Adcres2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adcres_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adcres::Adcres3)
    }
}
#[doc = "ADC predivider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcpdiv {
    #[doc = "0: ADC predivider /1"]
    Adcpdiv0 = 0,
    #[doc = "1: ADC predivider /2"]
    Adcpdiv1 = 1,
    #[doc = "2: ADC predivider /64"]
    Adcpdiv2 = 2,
    #[doc = "3: ADC predivider reserved"]
    Adcpdiv3 = 3,
}
impl From<Adcpdiv> for u8 {
    #[inline(always)]
    fn from(variant: Adcpdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcpdiv {
    type Ux = u8;
}
impl crate::IsEnum for Adcpdiv {}
#[doc = "Field `ADCPDIV` reader - ADC predivider Bit: 0"]
pub type AdcpdivR = crate::FieldReader<Adcpdiv>;
impl AdcpdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcpdiv {
        match self.bits {
            0 => Adcpdiv::Adcpdiv0,
            1 => Adcpdiv::Adcpdiv1,
            2 => Adcpdiv::Adcpdiv2,
            3 => Adcpdiv::Adcpdiv3,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC predivider /1"]
    #[inline(always)]
    pub fn is_adcpdiv_0(&self) -> bool {
        *self == Adcpdiv::Adcpdiv0
    }
    #[doc = "ADC predivider /2"]
    #[inline(always)]
    pub fn is_adcpdiv_1(&self) -> bool {
        *self == Adcpdiv::Adcpdiv1
    }
    #[doc = "ADC predivider /64"]
    #[inline(always)]
    pub fn is_adcpdiv_2(&self) -> bool {
        *self == Adcpdiv::Adcpdiv2
    }
    #[doc = "ADC predivider reserved"]
    #[inline(always)]
    pub fn is_adcpdiv_3(&self) -> bool {
        *self == Adcpdiv::Adcpdiv3
    }
}
#[doc = "Field `ADCPDIV` writer - ADC predivider Bit: 0"]
pub type AdcpdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adcpdiv, crate::Safe>;
impl<'a, REG> AdcpdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC predivider /1"]
    #[inline(always)]
    pub fn adcpdiv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpdiv::Adcpdiv0)
    }
    #[doc = "ADC predivider /2"]
    #[inline(always)]
    pub fn adcpdiv_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpdiv::Adcpdiv1)
    }
    #[doc = "ADC predivider /64"]
    #[inline(always)]
    pub fn adcpdiv_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpdiv::Adcpdiv2)
    }
    #[doc = "ADC predivider reserved"]
    #[inline(always)]
    pub fn adcpdiv_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpdiv::Adcpdiv3)
    }
}
impl R {
    #[doc = "Bit 2 - ADC Sampling Rate"]
    #[inline(always)]
    pub fn adcsr(&self) -> AdcsrR {
        AdcsrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Data Format"]
    #[inline(always)]
    pub fn adcdf(&self) -> AdcdfR {
        AdcdfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - ADC Resolution"]
    #[inline(always)]
    pub fn adcres(&self) -> AdcresR {
        AdcresR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ADC predivider Bit: 0"]
    #[inline(always)]
    pub fn adcpdiv(&self) -> AdcpdivR {
        AdcpdivR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - ADC Sampling Rate"]
    #[inline(always)]
    pub fn adcsr(&mut self) -> AdcsrW<'_, Adcctl2Spec> {
        AdcsrW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC Data Format"]
    #[inline(always)]
    pub fn adcdf(&mut self) -> AdcdfW<'_, Adcctl2Spec> {
        AdcdfW::new(self, 3)
    }
    #[doc = "Bits 4:5 - ADC Resolution"]
    #[inline(always)]
    pub fn adcres(&mut self) -> AdcresW<'_, Adcctl2Spec> {
        AdcresW::new(self, 4)
    }
    #[doc = "Bits 8:9 - ADC predivider Bit: 0"]
    #[inline(always)]
    pub fn adcpdiv(&mut self) -> AdcpdivW<'_, Adcctl2Spec> {
        AdcpdivW::new(self, 8)
    }
}
#[doc = "ADC Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adcctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcctl2Spec;
impl crate::RegisterSpec for Adcctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcctl2::R`](R) reader structure"]
impl crate::Readable for Adcctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`adcctl2::W`](W) writer structure"]
impl crate::Writable for Adcctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCCTL2 to value 0"]
impl crate::Resettable for Adcctl2Spec {}
