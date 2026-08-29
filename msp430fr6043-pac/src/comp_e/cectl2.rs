#[doc = "Register `CECTL2` reader"]
pub type R = crate::R<Cectl2Spec>;
#[doc = "Register `CECTL2` writer"]
pub type W = crate::W<Cectl2Spec>;
#[doc = "Field `CEREF0` reader - Reference resistor tap 0"]
pub type Ceref0R = crate::FieldReader;
#[doc = "Field `CEREF0` writer - Reference resistor tap 0"]
pub type Ceref0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cersel {
    #[doc = "0: When CEEX = 0, VREF is applied to the V+ terminal; When CEEX = 1, VREF is applied to the V- terminal"]
    Cersel0 = 0,
    #[doc = "1: When CEEX = 0, VREF is applied to the V- terminal; When CEEX = 1, VREF is applied to the V+ terminal"]
    Cersel1 = 1,
}
impl From<Cersel> for bool {
    #[inline(always)]
    fn from(variant: Cersel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERSEL` reader - Reference select"]
pub type CerselR = crate::BitReader<Cersel>;
impl CerselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cersel {
        match self.bits {
            false => Cersel::Cersel0,
            true => Cersel::Cersel1,
        }
    }
    #[doc = "When CEEX = 0, VREF is applied to the V+ terminal; When CEEX = 1, VREF is applied to the V- terminal"]
    #[inline(always)]
    pub fn is_cersel_0(&self) -> bool {
        *self == Cersel::Cersel0
    }
    #[doc = "When CEEX = 0, VREF is applied to the V- terminal; When CEEX = 1, VREF is applied to the V+ terminal"]
    #[inline(always)]
    pub fn is_cersel_1(&self) -> bool {
        *self == Cersel::Cersel1
    }
}
#[doc = "Field `CERSEL` writer - Reference select"]
pub type CerselW<'a, REG> = crate::BitWriter<'a, REG, Cersel>;
impl<'a, REG> CerselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When CEEX = 0, VREF is applied to the V+ terminal; When CEEX = 1, VREF is applied to the V- terminal"]
    #[inline(always)]
    pub fn cersel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cersel::Cersel0)
    }
    #[doc = "When CEEX = 0, VREF is applied to the V- terminal; When CEEX = 1, VREF is applied to the V+ terminal"]
    #[inline(always)]
    pub fn cersel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cersel::Cersel1)
    }
}
#[doc = "Reference source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cers {
    #[doc = "0: No current is drawn by the reference circuitry"]
    Cers0 = 0,
    #[doc = "1: VCC applied to the resistor ladder"]
    Cers1 = 1,
    #[doc = "2: Shared reference voltage applied to the resistor ladder"]
    Cers2 = 2,
    #[doc = "3: Shared reference voltage supplied to V(CREF). Resistor ladder is off"]
    Cers3 = 3,
}
impl From<Cers> for u8 {
    #[inline(always)]
    fn from(variant: Cers) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cers {
    type Ux = u8;
}
impl crate::IsEnum for Cers {}
#[doc = "Field `CERS` reader - Reference source"]
pub type CersR = crate::FieldReader<Cers>;
impl CersR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cers {
        match self.bits {
            0 => Cers::Cers0,
            1 => Cers::Cers1,
            2 => Cers::Cers2,
            3 => Cers::Cers3,
            _ => unreachable!(),
        }
    }
    #[doc = "No current is drawn by the reference circuitry"]
    #[inline(always)]
    pub fn is_cers_0(&self) -> bool {
        *self == Cers::Cers0
    }
    #[doc = "VCC applied to the resistor ladder"]
    #[inline(always)]
    pub fn is_cers_1(&self) -> bool {
        *self == Cers::Cers1
    }
    #[doc = "Shared reference voltage applied to the resistor ladder"]
    #[inline(always)]
    pub fn is_cers_2(&self) -> bool {
        *self == Cers::Cers2
    }
    #[doc = "Shared reference voltage supplied to V(CREF). Resistor ladder is off"]
    #[inline(always)]
    pub fn is_cers_3(&self) -> bool {
        *self == Cers::Cers3
    }
}
#[doc = "Field `CERS` writer - Reference source"]
pub type CersW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cers, crate::Safe>;
impl<'a, REG> CersW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No current is drawn by the reference circuitry"]
    #[inline(always)]
    pub fn cers_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cers::Cers0)
    }
    #[doc = "VCC applied to the resistor ladder"]
    #[inline(always)]
    pub fn cers_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cers::Cers1)
    }
    #[doc = "Shared reference voltage applied to the resistor ladder"]
    #[inline(always)]
    pub fn cers_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cers::Cers2)
    }
    #[doc = "Shared reference voltage supplied to V(CREF). Resistor ladder is off"]
    #[inline(always)]
    pub fn cers_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cers::Cers3)
    }
}
#[doc = "Field `CEREF1` reader - Reference resistor tap 1"]
pub type Ceref1R = crate::FieldReader;
#[doc = "Field `CEREF1` writer - Reference resistor tap 1"]
pub type Ceref1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Reference voltage level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cerefl {
    #[doc = "0: Reference amplifier is disabled. No reference voltage is requested"]
    Off = 0,
    #[doc = "1: 1.2 V is selected as shared reference voltage input"]
    _1p2v = 1,
    #[doc = "2: 2.0 V is selected as shared reference voltage input"]
    _2p0v = 2,
    #[doc = "3: 2.5 V is selected as shared reference voltage input"]
    _2p5v = 3,
}
impl From<Cerefl> for u8 {
    #[inline(always)]
    fn from(variant: Cerefl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cerefl {
    type Ux = u8;
}
impl crate::IsEnum for Cerefl {}
#[doc = "Field `CEREFL` reader - Reference voltage level"]
pub type CereflR = crate::FieldReader<Cerefl>;
impl CereflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cerefl {
        match self.bits {
            0 => Cerefl::Off,
            1 => Cerefl::_1p2v,
            2 => Cerefl::_2p0v,
            3 => Cerefl::_2p5v,
            _ => unreachable!(),
        }
    }
    #[doc = "Reference amplifier is disabled. No reference voltage is requested"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Cerefl::Off
    }
    #[doc = "1.2 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn is_1p2v(&self) -> bool {
        *self == Cerefl::_1p2v
    }
    #[doc = "2.0 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn is_2p0v(&self) -> bool {
        *self == Cerefl::_2p0v
    }
    #[doc = "2.5 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn is_2p5v(&self) -> bool {
        *self == Cerefl::_2p5v
    }
}
#[doc = "Field `CEREFL` writer - Reference voltage level"]
pub type CereflW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cerefl, crate::Safe>;
impl<'a, REG> CereflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reference amplifier is disabled. No reference voltage is requested"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Cerefl::Off)
    }
    #[doc = "1.2 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn _1p2v(self) -> &'a mut crate::W<REG> {
        self.variant(Cerefl::_1p2v)
    }
    #[doc = "2.0 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn _2p0v(self) -> &'a mut crate::W<REG> {
        self.variant(Cerefl::_2p0v)
    }
    #[doc = "2.5 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn _2p5v(self) -> &'a mut crate::W<REG> {
        self.variant(Cerefl::_2p5v)
    }
}
#[doc = "Reference accuracy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cerefacc {
    #[doc = "0: Static mode"]
    Static = 0,
    #[doc = "1: Clocked (low power, low accuracy) mode"]
    Clocked = 1,
}
impl From<Cerefacc> for bool {
    #[inline(always)]
    fn from(variant: Cerefacc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEREFACC` reader - Reference accuracy"]
pub type CerefaccR = crate::BitReader<Cerefacc>;
impl CerefaccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cerefacc {
        match self.bits {
            false => Cerefacc::Static,
            true => Cerefacc::Clocked,
        }
    }
    #[doc = "Static mode"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == Cerefacc::Static
    }
    #[doc = "Clocked (low power, low accuracy) mode"]
    #[inline(always)]
    pub fn is_clocked(&self) -> bool {
        *self == Cerefacc::Clocked
    }
}
#[doc = "Field `CEREFACC` writer - Reference accuracy"]
pub type CerefaccW<'a, REG> = crate::BitWriter<'a, REG, Cerefacc>;
impl<'a, REG> CerefaccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Static mode"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut crate::W<REG> {
        self.variant(Cerefacc::Static)
    }
    #[doc = "Clocked (low power, low accuracy) mode"]
    #[inline(always)]
    pub fn clocked(self) -> &'a mut crate::W<REG> {
        self.variant(Cerefacc::Clocked)
    }
}
impl R {
    #[doc = "Bits 0:4 - Reference resistor tap 0"]
    #[inline(always)]
    pub fn ceref0(&self) -> Ceref0R {
        Ceref0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Reference select"]
    #[inline(always)]
    pub fn cersel(&self) -> CerselR {
        CerselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reference source"]
    #[inline(always)]
    pub fn cers(&self) -> CersR {
        CersR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Reference resistor tap 1"]
    #[inline(always)]
    pub fn ceref1(&self) -> Ceref1R {
        Ceref1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Reference voltage level"]
    #[inline(always)]
    pub fn cerefl(&self) -> CereflR {
        CereflR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Reference accuracy"]
    #[inline(always)]
    pub fn cerefacc(&self) -> CerefaccR {
        CerefaccR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Reference resistor tap 0"]
    #[inline(always)]
    pub fn ceref0(&mut self) -> Ceref0W<'_, Cectl2Spec> {
        Ceref0W::new(self, 0)
    }
    #[doc = "Bit 5 - Reference select"]
    #[inline(always)]
    pub fn cersel(&mut self) -> CerselW<'_, Cectl2Spec> {
        CerselW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Reference source"]
    #[inline(always)]
    pub fn cers(&mut self) -> CersW<'_, Cectl2Spec> {
        CersW::new(self, 6)
    }
    #[doc = "Bits 8:12 - Reference resistor tap 1"]
    #[inline(always)]
    pub fn ceref1(&mut self) -> Ceref1W<'_, Cectl2Spec> {
        Ceref1W::new(self, 8)
    }
    #[doc = "Bits 13:14 - Reference voltage level"]
    #[inline(always)]
    pub fn cerefl(&mut self) -> CereflW<'_, Cectl2Spec> {
        CereflW::new(self, 13)
    }
    #[doc = "Bit 15 - Reference accuracy"]
    #[inline(always)]
    pub fn cerefacc(&mut self) -> CerefaccW<'_, Cectl2Spec> {
        CerefaccW::new(self, 15)
    }
}
#[doc = "Comparator Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cectl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cectl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cectl2Spec;
impl crate::RegisterSpec for Cectl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cectl2::R`](R) reader structure"]
impl crate::Readable for Cectl2Spec {}
#[doc = "`write(|w| ..)` method takes [`cectl2::W`](W) writer structure"]
impl crate::Writable for Cectl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CECTL2 to value 0"]
impl crate::Resettable for Cectl2Spec {}
