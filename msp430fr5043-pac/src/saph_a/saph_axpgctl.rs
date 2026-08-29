#[doc = "Register `SAPH_AXPGCTL` reader"]
pub type R = crate::R<SaphAxpgctlSpec>;
#[doc = "Register `SAPH_AXPGCTL` writer"]
pub type W = crate::W<SaphAxpgctlSpec>;
#[doc = "Field `XPULS` reader - XPULS Extra Pulse Count; This bit field defines the count of extra excitation pulses excited. If set to zero no extra excitation pulses are generated. Directly after the start the regular excitation phase is entered. Any other number will generate up the number of excitation pulses set by this field ."]
pub type XpulsR = crate::FieldReader;
#[doc = "Field `XPULS` writer - XPULS Extra Pulse Count; This bit field defines the count of extra excitation pulses excited. If set to zero no extra excitation pulses are generated. Directly after the start the regular excitation phase is entered. Any other number will generate up the number of excitation pulses set by this field ."]
pub type XpulsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Phase Status of PPG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xstat {
    #[doc = "0: PPG in Pause Phase"]
    Xstat0 = 0,
    #[doc = "1: PPG in Stop Phase"]
    Xstat1 = 1,
    #[doc = "2: PPG in Regular Excitation phase"]
    Xstat2 = 2,
    #[doc = "3: PPG in Extra Excitation Phase"]
    Xstat3 = 3,
}
impl From<Xstat> for u8 {
    #[inline(always)]
    fn from(variant: Xstat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xstat {
    type Ux = u8;
}
impl crate::IsEnum for Xstat {}
#[doc = "Field `XSTAT` reader - Phase Status of PPG"]
pub type XstatR = crate::FieldReader<Xstat>;
impl XstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xstat {
        match self.bits {
            0 => Xstat::Xstat0,
            1 => Xstat::Xstat1,
            2 => Xstat::Xstat2,
            3 => Xstat::Xstat3,
            _ => unreachable!(),
        }
    }
    #[doc = "PPG in Pause Phase"]
    #[inline(always)]
    pub fn is_xstat_0(&self) -> bool {
        *self == Xstat::Xstat0
    }
    #[doc = "PPG in Stop Phase"]
    #[inline(always)]
    pub fn is_xstat_1(&self) -> bool {
        *self == Xstat::Xstat1
    }
    #[doc = "PPG in Regular Excitation phase"]
    #[inline(always)]
    pub fn is_xstat_2(&self) -> bool {
        *self == Xstat::Xstat2
    }
    #[doc = "PPG in Extra Excitation Phase"]
    #[inline(always)]
    pub fn is_xstat_3(&self) -> bool {
        *self == Xstat::Xstat3
    }
}
#[doc = "Extended Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xmod {
    #[doc = "0: Single tone generation (Pause-E-S-Pause)"]
    Xmod0 = 0,
    #[doc = "1: reserved"]
    Xmod1 = 1,
    #[doc = "2: Dual Tone Generation (Pause-X-E-S-Pause)"]
    Xmod2 = 2,
    #[doc = "3: Dual Tone Loop (X-E-X-E triggers events)"]
    Xmod3 = 3,
}
impl From<Xmod> for u8 {
    #[inline(always)]
    fn from(variant: Xmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xmod {
    type Ux = u8;
}
impl crate::IsEnum for Xmod {}
#[doc = "Field `XMOD` reader - Extended Mode Select"]
pub type XmodR = crate::FieldReader<Xmod>;
impl XmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xmod {
        match self.bits {
            0 => Xmod::Xmod0,
            1 => Xmod::Xmod1,
            2 => Xmod::Xmod2,
            3 => Xmod::Xmod3,
            _ => unreachable!(),
        }
    }
    #[doc = "Single tone generation (Pause-E-S-Pause)"]
    #[inline(always)]
    pub fn is_xmod_0(&self) -> bool {
        *self == Xmod::Xmod0
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_xmod_1(&self) -> bool {
        *self == Xmod::Xmod1
    }
    #[doc = "Dual Tone Generation (Pause-X-E-S-Pause)"]
    #[inline(always)]
    pub fn is_xmod_2(&self) -> bool {
        *self == Xmod::Xmod2
    }
    #[doc = "Dual Tone Loop (X-E-X-E triggers events)"]
    #[inline(always)]
    pub fn is_xmod_3(&self) -> bool {
        *self == Xmod::Xmod3
    }
}
#[doc = "Field `XMOD` writer - Extended Mode Select"]
pub type XmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Xmod, crate::Safe>;
impl<'a, REG> XmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single tone generation (Pause-E-S-Pause)"]
    #[inline(always)]
    pub fn xmod_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xmod::Xmod0)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn xmod_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xmod::Xmod1)
    }
    #[doc = "Dual Tone Generation (Pause-X-E-S-Pause)"]
    #[inline(always)]
    pub fn xmod_2(self) -> &'a mut crate::W<REG> {
        self.variant(Xmod::Xmod2)
    }
    #[doc = "Dual Tone Loop (X-E-X-E triggers events)"]
    #[inline(always)]
    pub fn xmod_3(self) -> &'a mut crate::W<REG> {
        self.variant(Xmod::Xmod3)
    }
}
#[doc = "Event type. Selects the type of the event generated in XMOD=3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ety {
    #[doc = "0: Timer count event"]
    Ety0 = 0,
    #[doc = "1: DMA trigger event"]
    Ety1 = 1,
}
impl From<Ety> for bool {
    #[inline(always)]
    fn from(variant: Ety) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETY` reader - Event type. Selects the type of the event generated in XMOD=3"]
pub type EtyR = crate::BitReader<Ety>;
impl EtyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ety {
        match self.bits {
            false => Ety::Ety0,
            true => Ety::Ety1,
        }
    }
    #[doc = "Timer count event"]
    #[inline(always)]
    pub fn is_ety_0(&self) -> bool {
        *self == Ety::Ety0
    }
    #[doc = "DMA trigger event"]
    #[inline(always)]
    pub fn is_ety_1(&self) -> bool {
        *self == Ety::Ety1
    }
}
#[doc = "Field `ETY` writer - Event type. Selects the type of the event generated in XMOD=3"]
pub type EtyW<'a, REG> = crate::BitWriter<'a, REG, Ety>;
impl<'a, REG> EtyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer count event"]
    #[inline(always)]
    pub fn ety_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ety::Ety0)
    }
    #[doc = "DMA trigger event"]
    #[inline(always)]
    pub fn ety_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ety::Ety1)
    }
}
impl R {
    #[doc = "Bits 0:6 - XPULS Extra Pulse Count; This bit field defines the count of extra excitation pulses excited. If set to zero no extra excitation pulses are generated. Directly after the start the regular excitation phase is entered. Any other number will generate up the number of excitation pulses set by this field ."]
    #[inline(always)]
    pub fn xpuls(&self) -> XpulsR {
        XpulsR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Phase Status of PPG"]
    #[inline(always)]
    pub fn xstat(&self) -> XstatR {
        XstatR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Extended Mode Select"]
    #[inline(always)]
    pub fn xmod(&self) -> XmodR {
        XmodR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Event type. Selects the type of the event generated in XMOD=3"]
    #[inline(always)]
    pub fn ety(&self) -> EtyR {
        EtyR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - XPULS Extra Pulse Count; This bit field defines the count of extra excitation pulses excited. If set to zero no extra excitation pulses are generated. Directly after the start the regular excitation phase is entered. Any other number will generate up the number of excitation pulses set by this field ."]
    #[inline(always)]
    pub fn xpuls(&mut self) -> XpulsW<'_, SaphAxpgctlSpec> {
        XpulsW::new(self, 0)
    }
    #[doc = "Bits 12:13 - Extended Mode Select"]
    #[inline(always)]
    pub fn xmod(&mut self) -> XmodW<'_, SaphAxpgctlSpec> {
        XmodW::new(self, 12)
    }
    #[doc = "Bit 14 - Event type. Selects the type of the event generated in XMOD=3"]
    #[inline(always)]
    pub fn ety(&mut self) -> EtyW<'_, SaphAxpgctlSpec> {
        EtyW::new(self, 14)
    }
}
#[doc = "Extended Pulse Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_axpgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_axpgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAxpgctlSpec;
impl crate::RegisterSpec for SaphAxpgctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_axpgctl::R`](R) reader structure"]
impl crate::Readable for SaphAxpgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_axpgctl::W`](W) writer structure"]
impl crate::Writable for SaphAxpgctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AXPGCTL to value 0"]
impl crate::Resettable for SaphAxpgctlSpec {}
