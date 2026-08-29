#[doc = "Register `SAPH_APGC` reader"]
pub type R = crate::R<SaphApgcSpec>;
#[doc = "Register `SAPH_APGC` writer"]
pub type W = crate::W<SaphApgcSpec>;
#[doc = "Field `EPULS` reader - Excitation Pulse Count. This bit field defines the number of excitation pulses. Minimum value is zero."]
pub type EpulsR = crate::FieldReader;
#[doc = "Field `EPULS` writer - Excitation Pulse Count. This bit field defines the number of excitation pulses. Minimum value is zero."]
pub type EpulsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SPULS` reader - Stop Pulse Count; This bit field defines the number of stop pulses. Minimum value is zero. Stop pulses have the inverted polarity of excitation pulses."]
pub type SpulsR = crate::FieldReader;
#[doc = "Field `SPULS` writer - Stop Pulse Count; This bit field defines the number of stop pulses. Minimum value is zero. Stop pulses have the inverted polarity of excitation pulses."]
pub type SpulsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Pulse Polarity. This bit defines the polarity of the first excitation pulse.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ppol {
    #[doc = "0: The excitation begins with logical high phase. The stop begines with logical low phase."]
    Ppol0 = 0,
    #[doc = "1: The excitation begins with logical low phase. The stop begines with logical high phase."]
    Ppol1 = 1,
}
impl From<Ppol> for bool {
    #[inline(always)]
    fn from(variant: Ppol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPOL` reader - Pulse Polarity. This bit defines the polarity of the first excitation pulse."]
pub type PpolR = crate::BitReader<Ppol>;
impl PpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ppol {
        match self.bits {
            false => Ppol::Ppol0,
            true => Ppol::Ppol1,
        }
    }
    #[doc = "The excitation begins with logical high phase. The stop begines with logical low phase."]
    #[inline(always)]
    pub fn is_ppol_0(&self) -> bool {
        *self == Ppol::Ppol0
    }
    #[doc = "The excitation begins with logical low phase. The stop begines with logical high phase."]
    #[inline(always)]
    pub fn is_ppol_1(&self) -> bool {
        *self == Ppol::Ppol1
    }
}
#[doc = "Field `PPOL` writer - Pulse Polarity. This bit defines the polarity of the first excitation pulse."]
pub type PpolW<'a, REG> = crate::BitWriter<'a, REG, Ppol>;
impl<'a, REG> PpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The excitation begins with logical high phase. The stop begines with logical low phase."]
    #[inline(always)]
    pub fn ppol_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ppol::Ppol0)
    }
    #[doc = "The excitation begins with logical low phase. The stop begines with logical high phase."]
    #[inline(always)]
    pub fn ppol_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ppol::Ppol1)
    }
}
#[doc = "PPG ouptut level during inactive. This bit affects the status of PPG output before and after excitations. Note that this bit is only valid when PGC.PHIZ =0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plev {
    #[doc = "0: PPG output is low during inactive"]
    Plev0 = 0,
    #[doc = "1: PPG output is high during inactive"]
    Plev1 = 1,
}
impl From<Plev> for bool {
    #[inline(always)]
    fn from(variant: Plev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLEV` reader - PPG ouptut level during inactive. This bit affects the status of PPG output before and after excitations. Note that this bit is only valid when PGC.PHIZ =0."]
pub type PlevR = crate::BitReader<Plev>;
impl PlevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Plev {
        match self.bits {
            false => Plev::Plev0,
            true => Plev::Plev1,
        }
    }
    #[doc = "PPG output is low during inactive"]
    #[inline(always)]
    pub fn is_plev_0(&self) -> bool {
        *self == Plev::Plev0
    }
    #[doc = "PPG output is high during inactive"]
    #[inline(always)]
    pub fn is_plev_1(&self) -> bool {
        *self == Plev::Plev1
    }
}
#[doc = "Field `PLEV` writer - PPG ouptut level during inactive. This bit affects the status of PPG output before and after excitations. Note that this bit is only valid when PGC.PHIZ =0."]
pub type PlevW<'a, REG> = crate::BitWriter<'a, REG, Plev>;
impl<'a, REG> PlevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PPG output is low during inactive"]
    #[inline(always)]
    pub fn plev_0(self) -> &'a mut crate::W<REG> {
        self.variant(Plev::Plev0)
    }
    #[doc = "PPG output is high during inactive"]
    #[inline(always)]
    pub fn plev_1(self) -> &'a mut crate::W<REG> {
        self.variant(Plev::Plev1)
    }
}
#[doc = "Hi-Z enable to PPG output during inactive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phiz {
    #[doc = "0: PPG output during inactive is determined by PGC.PLEV bit."]
    Phiz0 = 0,
    #[doc = "1: PPG output is in Hi-Z during inactive regardless of PGC.PLEV bit."]
    Phiz1 = 1,
}
impl From<Phiz> for bool {
    #[inline(always)]
    fn from(variant: Phiz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHIZ` reader - Hi-Z enable to PPG output during inactive."]
pub type PhizR = crate::BitReader<Phiz>;
impl PhizR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Phiz {
        match self.bits {
            false => Phiz::Phiz0,
            true => Phiz::Phiz1,
        }
    }
    #[doc = "PPG output during inactive is determined by PGC.PLEV bit."]
    #[inline(always)]
    pub fn is_phiz_0(&self) -> bool {
        *self == Phiz::Phiz0
    }
    #[doc = "PPG output is in Hi-Z during inactive regardless of PGC.PLEV bit."]
    #[inline(always)]
    pub fn is_phiz_1(&self) -> bool {
        *self == Phiz::Phiz1
    }
}
#[doc = "Field `PHIZ` writer - Hi-Z enable to PPG output during inactive."]
pub type PhizW<'a, REG> = crate::BitWriter<'a, REG, Phiz>;
impl<'a, REG> PhizW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PPG output during inactive is determined by PGC.PLEV bit."]
    #[inline(always)]
    pub fn phiz_0(self) -> &'a mut crate::W<REG> {
        self.variant(Phiz::Phiz0)
    }
    #[doc = "PPG output is in Hi-Z during inactive regardless of PGC.PLEV bit."]
    #[inline(always)]
    pub fn phiz_1(self) -> &'a mut crate::W<REG> {
        self.variant(Phiz::Phiz1)
    }
}
impl R {
    #[doc = "Bits 0:6 - Excitation Pulse Count. This bit field defines the number of excitation pulses. Minimum value is zero."]
    #[inline(always)]
    pub fn epuls(&self) -> EpulsR {
        EpulsR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:11 - Stop Pulse Count; This bit field defines the number of stop pulses. Minimum value is zero. Stop pulses have the inverted polarity of excitation pulses."]
    #[inline(always)]
    pub fn spuls(&self) -> SpulsR {
        SpulsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Pulse Polarity. This bit defines the polarity of the first excitation pulse."]
    #[inline(always)]
    pub fn ppol(&self) -> PpolR {
        PpolR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PPG ouptut level during inactive. This bit affects the status of PPG output before and after excitations. Note that this bit is only valid when PGC.PHIZ =0."]
    #[inline(always)]
    pub fn plev(&self) -> PlevR {
        PlevR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Hi-Z enable to PPG output during inactive."]
    #[inline(always)]
    pub fn phiz(&self) -> PhizR {
        PhizR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Excitation Pulse Count. This bit field defines the number of excitation pulses. Minimum value is zero."]
    #[inline(always)]
    pub fn epuls(&mut self) -> EpulsW<'_, SaphApgcSpec> {
        EpulsW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Stop Pulse Count; This bit field defines the number of stop pulses. Minimum value is zero. Stop pulses have the inverted polarity of excitation pulses."]
    #[inline(always)]
    pub fn spuls(&mut self) -> SpulsW<'_, SaphApgcSpec> {
        SpulsW::new(self, 8)
    }
    #[doc = "Bit 13 - Pulse Polarity. This bit defines the polarity of the first excitation pulse."]
    #[inline(always)]
    pub fn ppol(&mut self) -> PpolW<'_, SaphApgcSpec> {
        PpolW::new(self, 13)
    }
    #[doc = "Bit 14 - PPG ouptut level during inactive. This bit affects the status of PPG output before and after excitations. Note that this bit is only valid when PGC.PHIZ =0."]
    #[inline(always)]
    pub fn plev(&mut self) -> PlevW<'_, SaphApgcSpec> {
        PlevW::new(self, 14)
    }
    #[doc = "Bit 15 - Hi-Z enable to PPG output during inactive."]
    #[inline(always)]
    pub fn phiz(&mut self) -> PhizW<'_, SaphApgcSpec> {
        PhizW::new(self, 15)
    }
}
#[doc = "PPG Count\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_apgc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_apgc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphApgcSpec;
impl crate::RegisterSpec for SaphApgcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_apgc::R`](R) reader structure"]
impl crate::Readable for SaphApgcSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_apgc::W`](W) writer structure"]
impl crate::Writable for SaphApgcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_APGC to value 0"]
impl crate::Resettable for SaphApgcSpec {}
