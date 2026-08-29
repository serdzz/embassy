#[doc = "Register `CECTL1` reader"]
pub type R = crate::R<Cectl1Spec>;
#[doc = "Register `CECTL1` writer"]
pub type W = crate::W<Cectl1Spec>;
#[doc = "Field `CEOUT` reader - Comparator output value"]
pub type CeoutR = crate::BitReader;
#[doc = "Field `CEOUT` writer - Comparator output value"]
pub type CeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Comparator output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceoutpol {
    #[doc = "0: Noninverted"]
    Ceoutpol0 = 0,
    #[doc = "1: Inverted"]
    Ceoutpol1 = 1,
}
impl From<Ceoutpol> for bool {
    #[inline(always)]
    fn from(variant: Ceoutpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEOUTPOL` reader - Comparator output polarity"]
pub type CeoutpolR = crate::BitReader<Ceoutpol>;
impl CeoutpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceoutpol {
        match self.bits {
            false => Ceoutpol::Ceoutpol0,
            true => Ceoutpol::Ceoutpol1,
        }
    }
    #[doc = "Noninverted"]
    #[inline(always)]
    pub fn is_ceoutpol_0(&self) -> bool {
        *self == Ceoutpol::Ceoutpol0
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn is_ceoutpol_1(&self) -> bool {
        *self == Ceoutpol::Ceoutpol1
    }
}
#[doc = "Field `CEOUTPOL` writer - Comparator output polarity"]
pub type CeoutpolW<'a, REG> = crate::BitWriter<'a, REG, Ceoutpol>;
impl<'a, REG> CeoutpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Noninverted"]
    #[inline(always)]
    pub fn ceoutpol_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceoutpol::Ceoutpol0)
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn ceoutpol_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceoutpol::Ceoutpol1)
    }
}
#[doc = "Comparator output filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cef {
    #[doc = "0: Comparator output is not filtered"]
    Cef0 = 0,
    #[doc = "1: Comparator output is filtered"]
    Cef1 = 1,
}
impl From<Cef> for bool {
    #[inline(always)]
    fn from(variant: Cef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEF` reader - Comparator output filter"]
pub type CefR = crate::BitReader<Cef>;
impl CefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cef {
        match self.bits {
            false => Cef::Cef0,
            true => Cef::Cef1,
        }
    }
    #[doc = "Comparator output is not filtered"]
    #[inline(always)]
    pub fn is_cef_0(&self) -> bool {
        *self == Cef::Cef0
    }
    #[doc = "Comparator output is filtered"]
    #[inline(always)]
    pub fn is_cef_1(&self) -> bool {
        *self == Cef::Cef1
    }
}
#[doc = "Field `CEF` writer - Comparator output filter"]
pub type CefW<'a, REG> = crate::BitWriter<'a, REG, Cef>;
impl<'a, REG> CefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator output is not filtered"]
    #[inline(always)]
    pub fn cef_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cef::Cef0)
    }
    #[doc = "Comparator output is filtered"]
    #[inline(always)]
    pub fn cef_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cef::Cef1)
    }
}
#[doc = "Interrupt edge select for CEIIFG and CEIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceies {
    #[doc = "0: Rising edge for CEIFG, falling edge for CEIIFG"]
    Ceies0 = 0,
    #[doc = "1: Falling edge for CEIFG, rising edge for CEIIFG"]
    Ceies1 = 1,
}
impl From<Ceies> for bool {
    #[inline(always)]
    fn from(variant: Ceies) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIES` reader - Interrupt edge select for CEIIFG and CEIFG"]
pub type CeiesR = crate::BitReader<Ceies>;
impl CeiesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceies {
        match self.bits {
            false => Ceies::Ceies0,
            true => Ceies::Ceies1,
        }
    }
    #[doc = "Rising edge for CEIFG, falling edge for CEIIFG"]
    #[inline(always)]
    pub fn is_ceies_0(&self) -> bool {
        *self == Ceies::Ceies0
    }
    #[doc = "Falling edge for CEIFG, rising edge for CEIIFG"]
    #[inline(always)]
    pub fn is_ceies_1(&self) -> bool {
        *self == Ceies::Ceies1
    }
}
#[doc = "Field `CEIES` writer - Interrupt edge select for CEIIFG and CEIFG"]
pub type CeiesW<'a, REG> = crate::BitWriter<'a, REG, Ceies>;
impl<'a, REG> CeiesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge for CEIFG, falling edge for CEIIFG"]
    #[inline(always)]
    pub fn ceies_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceies::Ceies0)
    }
    #[doc = "Falling edge for CEIFG, rising edge for CEIIFG"]
    #[inline(always)]
    pub fn ceies_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceies::Ceies1)
    }
}
#[doc = "Input short\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceshort {
    #[doc = "0: Inputs not shorted"]
    Ceshort0 = 0,
    #[doc = "1: Inputs shorted"]
    Ceshort1 = 1,
}
impl From<Ceshort> for bool {
    #[inline(always)]
    fn from(variant: Ceshort) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CESHORT` reader - Input short"]
pub type CeshortR = crate::BitReader<Ceshort>;
impl CeshortR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceshort {
        match self.bits {
            false => Ceshort::Ceshort0,
            true => Ceshort::Ceshort1,
        }
    }
    #[doc = "Inputs not shorted"]
    #[inline(always)]
    pub fn is_ceshort_0(&self) -> bool {
        *self == Ceshort::Ceshort0
    }
    #[doc = "Inputs shorted"]
    #[inline(always)]
    pub fn is_ceshort_1(&self) -> bool {
        *self == Ceshort::Ceshort1
    }
}
#[doc = "Field `CESHORT` writer - Input short"]
pub type CeshortW<'a, REG> = crate::BitWriter<'a, REG, Ceshort>;
impl<'a, REG> CeshortW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inputs not shorted"]
    #[inline(always)]
    pub fn ceshort_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceshort::Ceshort0)
    }
    #[doc = "Inputs shorted"]
    #[inline(always)]
    pub fn ceshort_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceshort::Ceshort1)
    }
}
#[doc = "Field `CEEX` reader - Exchange"]
pub type CeexR = crate::BitReader;
#[doc = "Field `CEEX` writer - Exchange"]
pub type CeexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Filter delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cefdly {
    #[doc = "0: Typical filter delay of TBD (450) ns"]
    Cefdly0 = 0,
    #[doc = "1: Typical filter delay of TBD (900) ns"]
    Cefdly1 = 1,
    #[doc = "2: Typical filter delay of TBD (1800) ns"]
    Cefdly2 = 2,
    #[doc = "3: Typical filter delay of TBD (3600) ns"]
    Cefdly3 = 3,
}
impl From<Cefdly> for u8 {
    #[inline(always)]
    fn from(variant: Cefdly) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cefdly {
    type Ux = u8;
}
impl crate::IsEnum for Cefdly {}
#[doc = "Field `CEFDLY` reader - Filter delay"]
pub type CefdlyR = crate::FieldReader<Cefdly>;
impl CefdlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cefdly {
        match self.bits {
            0 => Cefdly::Cefdly0,
            1 => Cefdly::Cefdly1,
            2 => Cefdly::Cefdly2,
            3 => Cefdly::Cefdly3,
            _ => unreachable!(),
        }
    }
    #[doc = "Typical filter delay of TBD (450) ns"]
    #[inline(always)]
    pub fn is_cefdly_0(&self) -> bool {
        *self == Cefdly::Cefdly0
    }
    #[doc = "Typical filter delay of TBD (900) ns"]
    #[inline(always)]
    pub fn is_cefdly_1(&self) -> bool {
        *self == Cefdly::Cefdly1
    }
    #[doc = "Typical filter delay of TBD (1800) ns"]
    #[inline(always)]
    pub fn is_cefdly_2(&self) -> bool {
        *self == Cefdly::Cefdly2
    }
    #[doc = "Typical filter delay of TBD (3600) ns"]
    #[inline(always)]
    pub fn is_cefdly_3(&self) -> bool {
        *self == Cefdly::Cefdly3
    }
}
#[doc = "Field `CEFDLY` writer - Filter delay"]
pub type CefdlyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cefdly, crate::Safe>;
impl<'a, REG> CefdlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Typical filter delay of TBD (450) ns"]
    #[inline(always)]
    pub fn cefdly_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cefdly::Cefdly0)
    }
    #[doc = "Typical filter delay of TBD (900) ns"]
    #[inline(always)]
    pub fn cefdly_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cefdly::Cefdly1)
    }
    #[doc = "Typical filter delay of TBD (1800) ns"]
    #[inline(always)]
    pub fn cefdly_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cefdly::Cefdly2)
    }
    #[doc = "Typical filter delay of TBD (3600) ns"]
    #[inline(always)]
    pub fn cefdly_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cefdly::Cefdly3)
    }
}
#[doc = "Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cepwrmd {
    #[doc = "0: High-speed mode"]
    Cepwrmd0 = 0,
    #[doc = "1: Normal mode"]
    Cepwrmd1 = 1,
    #[doc = "2: Ultra-low power mode"]
    Cepwrmd2 = 2,
}
impl From<Cepwrmd> for u8 {
    #[inline(always)]
    fn from(variant: Cepwrmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cepwrmd {
    type Ux = u8;
}
impl crate::IsEnum for Cepwrmd {}
#[doc = "Field `CEPWRMD` reader - Power Mode"]
pub type CepwrmdR = crate::FieldReader<Cepwrmd>;
impl CepwrmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cepwrmd> {
        match self.bits {
            0 => Some(Cepwrmd::Cepwrmd0),
            1 => Some(Cepwrmd::Cepwrmd1),
            2 => Some(Cepwrmd::Cepwrmd2),
            _ => None,
        }
    }
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn is_cepwrmd_0(&self) -> bool {
        *self == Cepwrmd::Cepwrmd0
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_cepwrmd_1(&self) -> bool {
        *self == Cepwrmd::Cepwrmd1
    }
    #[doc = "Ultra-low power mode"]
    #[inline(always)]
    pub fn is_cepwrmd_2(&self) -> bool {
        *self == Cepwrmd::Cepwrmd2
    }
}
#[doc = "Field `CEPWRMD` writer - Power Mode"]
pub type CepwrmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cepwrmd>;
impl<'a, REG> CepwrmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn cepwrmd_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cepwrmd::Cepwrmd0)
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn cepwrmd_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cepwrmd::Cepwrmd1)
    }
    #[doc = "Ultra-low power mode"]
    #[inline(always)]
    pub fn cepwrmd_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cepwrmd::Cepwrmd2)
    }
}
#[doc = "Comparator On\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceon {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<Ceon> for bool {
    #[inline(always)]
    fn from(variant: Ceon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEON` reader - Comparator On"]
pub type CeonR = crate::BitReader<Ceon>;
impl CeonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceon {
        match self.bits {
            false => Ceon::Off,
            true => Ceon::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ceon::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Ceon::On
    }
}
#[doc = "Field `CEON` writer - Comparator On"]
pub type CeonW<'a, REG> = crate::BitWriter<'a, REG, Ceon>;
impl<'a, REG> CeonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Ceon::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Ceon::On)
    }
}
#[doc = "This bit is valid of CEMRVS is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cemrvl {
    #[doc = "0: VREF0 is selected if CERS = 00, 01, or 10"]
    Vref0 = 0,
    #[doc = "1: VREF1 is selected if CERS = 00, 01, or 10"]
    Vref1 = 1,
}
impl From<Cemrvl> for bool {
    #[inline(always)]
    fn from(variant: Cemrvl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEMRVL` reader - This bit is valid of CEMRVS is set to 1"]
pub type CemrvlR = crate::BitReader<Cemrvl>;
impl CemrvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cemrvl {
        match self.bits {
            false => Cemrvl::Vref0,
            true => Cemrvl::Vref1,
        }
    }
    #[doc = "VREF0 is selected if CERS = 00, 01, or 10"]
    #[inline(always)]
    pub fn is_vref0(&self) -> bool {
        *self == Cemrvl::Vref0
    }
    #[doc = "VREF1 is selected if CERS = 00, 01, or 10"]
    #[inline(always)]
    pub fn is_vref1(&self) -> bool {
        *self == Cemrvl::Vref1
    }
}
#[doc = "Field `CEMRVL` writer - This bit is valid of CEMRVS is set to 1"]
pub type CemrvlW<'a, REG> = crate::BitWriter<'a, REG, Cemrvl>;
impl<'a, REG> CemrvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VREF0 is selected if CERS = 00, 01, or 10"]
    #[inline(always)]
    pub fn vref0(self) -> &'a mut crate::W<REG> {
        self.variant(Cemrvl::Vref0)
    }
    #[doc = "VREF1 is selected if CERS = 00, 01, or 10"]
    #[inline(always)]
    pub fn vref1(self) -> &'a mut crate::W<REG> {
        self.variant(Cemrvl::Vref1)
    }
}
#[doc = "This bit defines if the comparator output selects between VREF0 or VREF1 if CERS = 00, 01, or 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cemrvs {
    #[doc = "0: Comparator output state selects between VREF0 or VREF1"]
    Cemrvs0 = 0,
    #[doc = "1: CEMRVL selects between VREF0 or VREF1"]
    Cemrvs1 = 1,
}
impl From<Cemrvs> for bool {
    #[inline(always)]
    fn from(variant: Cemrvs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEMRVS` reader - This bit defines if the comparator output selects between VREF0 or VREF1 if CERS = 00, 01, or 10."]
pub type CemrvsR = crate::BitReader<Cemrvs>;
impl CemrvsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cemrvs {
        match self.bits {
            false => Cemrvs::Cemrvs0,
            true => Cemrvs::Cemrvs1,
        }
    }
    #[doc = "Comparator output state selects between VREF0 or VREF1"]
    #[inline(always)]
    pub fn is_cemrvs_0(&self) -> bool {
        *self == Cemrvs::Cemrvs0
    }
    #[doc = "CEMRVL selects between VREF0 or VREF1"]
    #[inline(always)]
    pub fn is_cemrvs_1(&self) -> bool {
        *self == Cemrvs::Cemrvs1
    }
}
#[doc = "Field `CEMRVS` writer - This bit defines if the comparator output selects between VREF0 or VREF1 if CERS = 00, 01, or 10."]
pub type CemrvsW<'a, REG> = crate::BitWriter<'a, REG, Cemrvs>;
impl<'a, REG> CemrvsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator output state selects between VREF0 or VREF1"]
    #[inline(always)]
    pub fn cemrvs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cemrvs::Cemrvs0)
    }
    #[doc = "CEMRVL selects between VREF0 or VREF1"]
    #[inline(always)]
    pub fn cemrvs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cemrvs::Cemrvs1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator output value"]
    #[inline(always)]
    pub fn ceout(&self) -> CeoutR {
        CeoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator output polarity"]
    #[inline(always)]
    pub fn ceoutpol(&self) -> CeoutpolR {
        CeoutpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator output filter"]
    #[inline(always)]
    pub fn cef(&self) -> CefR {
        CefR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt edge select for CEIIFG and CEIFG"]
    #[inline(always)]
    pub fn ceies(&self) -> CeiesR {
        CeiesR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input short"]
    #[inline(always)]
    pub fn ceshort(&self) -> CeshortR {
        CeshortR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Exchange"]
    #[inline(always)]
    pub fn ceex(&self) -> CeexR {
        CeexR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Filter delay"]
    #[inline(always)]
    pub fn cefdly(&self) -> CefdlyR {
        CefdlyR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Power Mode"]
    #[inline(always)]
    pub fn cepwrmd(&self) -> CepwrmdR {
        CepwrmdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Comparator On"]
    #[inline(always)]
    pub fn ceon(&self) -> CeonR {
        CeonR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit is valid of CEMRVS is set to 1"]
    #[inline(always)]
    pub fn cemrvl(&self) -> CemrvlR {
        CemrvlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit defines if the comparator output selects between VREF0 or VREF1 if CERS = 00, 01, or 10."]
    #[inline(always)]
    pub fn cemrvs(&self) -> CemrvsR {
        CemrvsR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator output value"]
    #[inline(always)]
    pub fn ceout(&mut self) -> CeoutW<'_, Cectl1Spec> {
        CeoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator output polarity"]
    #[inline(always)]
    pub fn ceoutpol(&mut self) -> CeoutpolW<'_, Cectl1Spec> {
        CeoutpolW::new(self, 1)
    }
    #[doc = "Bit 2 - Comparator output filter"]
    #[inline(always)]
    pub fn cef(&mut self) -> CefW<'_, Cectl1Spec> {
        CefW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt edge select for CEIIFG and CEIFG"]
    #[inline(always)]
    pub fn ceies(&mut self) -> CeiesW<'_, Cectl1Spec> {
        CeiesW::new(self, 3)
    }
    #[doc = "Bit 4 - Input short"]
    #[inline(always)]
    pub fn ceshort(&mut self) -> CeshortW<'_, Cectl1Spec> {
        CeshortW::new(self, 4)
    }
    #[doc = "Bit 5 - Exchange"]
    #[inline(always)]
    pub fn ceex(&mut self) -> CeexW<'_, Cectl1Spec> {
        CeexW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Filter delay"]
    #[inline(always)]
    pub fn cefdly(&mut self) -> CefdlyW<'_, Cectl1Spec> {
        CefdlyW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Power Mode"]
    #[inline(always)]
    pub fn cepwrmd(&mut self) -> CepwrmdW<'_, Cectl1Spec> {
        CepwrmdW::new(self, 8)
    }
    #[doc = "Bit 10 - Comparator On"]
    #[inline(always)]
    pub fn ceon(&mut self) -> CeonW<'_, Cectl1Spec> {
        CeonW::new(self, 10)
    }
    #[doc = "Bit 11 - This bit is valid of CEMRVS is set to 1"]
    #[inline(always)]
    pub fn cemrvl(&mut self) -> CemrvlW<'_, Cectl1Spec> {
        CemrvlW::new(self, 11)
    }
    #[doc = "Bit 12 - This bit defines if the comparator output selects between VREF0 or VREF1 if CERS = 00, 01, or 10."]
    #[inline(always)]
    pub fn cemrvs(&mut self) -> CemrvsW<'_, Cectl1Spec> {
        CemrvsW::new(self, 12)
    }
}
#[doc = "Comparator Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cectl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cectl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cectl1Spec;
impl crate::RegisterSpec for Cectl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cectl1::R`](R) reader structure"]
impl crate::Readable for Cectl1Spec {}
#[doc = "`write(|w| ..)` method takes [`cectl1::W`](W) writer structure"]
impl crate::Writable for Cectl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CECTL1 to value 0"]
impl crate::Resettable for Cectl1Spec {}
