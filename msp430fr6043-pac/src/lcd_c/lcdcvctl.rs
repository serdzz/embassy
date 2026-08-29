#[doc = "Register `LCDCVCTL` reader"]
pub type R = crate::R<LcdcvctlSpec>;
#[doc = "Register `LCDCVCTL` writer"]
pub type W = crate::W<LcdcvctlSpec>;
#[doc = "Bias select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcd2b {
    #[doc = "0: 1/3 bias"]
    Lcd2b0 = 0,
    #[doc = "1: 1/2 bias"]
    Lcd2b1 = 1,
}
impl From<Lcd2b> for bool {
    #[inline(always)]
    fn from(variant: Lcd2b) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCD2B` reader - Bias select"]
pub type Lcd2bR = crate::BitReader<Lcd2b>;
impl Lcd2bR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcd2b {
        match self.bits {
            false => Lcd2b::Lcd2b0,
            true => Lcd2b::Lcd2b1,
        }
    }
    #[doc = "1/3 bias"]
    #[inline(always)]
    pub fn is_lcd2b_0(&self) -> bool {
        *self == Lcd2b::Lcd2b0
    }
    #[doc = "1/2 bias"]
    #[inline(always)]
    pub fn is_lcd2b_1(&self) -> bool {
        *self == Lcd2b::Lcd2b1
    }
}
#[doc = "Field `LCD2B` writer - Bias select"]
pub type Lcd2bW<'a, REG> = crate::BitWriter<'a, REG, Lcd2b>;
impl<'a, REG> Lcd2bW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1/3 bias"]
    #[inline(always)]
    pub fn lcd2b_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcd2b::Lcd2b0)
    }
    #[doc = "1/2 bias"]
    #[inline(always)]
    pub fn lcd2b_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcd2b::Lcd2b1)
    }
}
#[doc = "Charge pump reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vlcdref {
    #[doc = "0: Internal reference voltage"]
    Vlcdref0 = 0,
    #[doc = "1: External reference voltage"]
    Vlcdref1 = 1,
    #[doc = "2: Internal reference voltage switched to external pin LCDREF/R13"]
    Vlcdref2 = 2,
    #[doc = "3: Reserved (defaults to external reference voltage)"]
    Vlcdref3 = 3,
}
impl From<Vlcdref> for u8 {
    #[inline(always)]
    fn from(variant: Vlcdref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vlcdref {
    type Ux = u8;
}
impl crate::IsEnum for Vlcdref {}
#[doc = "Field `VLCDREF` reader - Charge pump reference select"]
pub type VlcdrefR = crate::FieldReader<Vlcdref>;
impl VlcdrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vlcdref {
        match self.bits {
            0 => Vlcdref::Vlcdref0,
            1 => Vlcdref::Vlcdref1,
            2 => Vlcdref::Vlcdref2,
            3 => Vlcdref::Vlcdref3,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal reference voltage"]
    #[inline(always)]
    pub fn is_vlcdref_0(&self) -> bool {
        *self == Vlcdref::Vlcdref0
    }
    #[doc = "External reference voltage"]
    #[inline(always)]
    pub fn is_vlcdref_1(&self) -> bool {
        *self == Vlcdref::Vlcdref1
    }
    #[doc = "Internal reference voltage switched to external pin LCDREF/R13"]
    #[inline(always)]
    pub fn is_vlcdref_2(&self) -> bool {
        *self == Vlcdref::Vlcdref2
    }
    #[doc = "Reserved (defaults to external reference voltage)"]
    #[inline(always)]
    pub fn is_vlcdref_3(&self) -> bool {
        *self == Vlcdref::Vlcdref3
    }
}
#[doc = "Field `VLCDREF` writer - Charge pump reference select"]
pub type VlcdrefW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vlcdref, crate::Safe>;
impl<'a, REG> VlcdrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal reference voltage"]
    #[inline(always)]
    pub fn vlcdref_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdref::Vlcdref0)
    }
    #[doc = "External reference voltage"]
    #[inline(always)]
    pub fn vlcdref_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdref::Vlcdref1)
    }
    #[doc = "Internal reference voltage switched to external pin LCDREF/R13"]
    #[inline(always)]
    pub fn vlcdref_2(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdref::Vlcdref2)
    }
    #[doc = "Reserved (defaults to external reference voltage)"]
    #[inline(always)]
    pub fn vlcdref_3(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdref::Vlcdref3)
    }
}
#[doc = "Charge pump enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcpen {
    #[doc = "0: Charge pump disabled"]
    Lcdcpen0 = 0,
    #[doc = "1: Charge pump enabled when VLCD is generated internally (VLCDEXT = 0) and VLCD 0 or VLCDREF 0"]
    Lcdcpen1 = 1,
}
impl From<Lcdcpen> for bool {
    #[inline(always)]
    fn from(variant: Lcdcpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCPEN` reader - Charge pump enable"]
pub type LcdcpenR = crate::BitReader<Lcdcpen>;
impl LcdcpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcpen {
        match self.bits {
            false => Lcdcpen::Lcdcpen0,
            true => Lcdcpen::Lcdcpen1,
        }
    }
    #[doc = "Charge pump disabled"]
    #[inline(always)]
    pub fn is_lcdcpen_0(&self) -> bool {
        *self == Lcdcpen::Lcdcpen0
    }
    #[doc = "Charge pump enabled when VLCD is generated internally (VLCDEXT = 0) and VLCD 0 or VLCDREF 0"]
    #[inline(always)]
    pub fn is_lcdcpen_1(&self) -> bool {
        *self == Lcdcpen::Lcdcpen1
    }
}
#[doc = "Field `LCDCPEN` writer - Charge pump enable"]
pub type LcdcpenW<'a, REG> = crate::BitWriter<'a, REG, Lcdcpen>;
impl<'a, REG> LcdcpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Charge pump disabled"]
    #[inline(always)]
    pub fn lcdcpen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpen::Lcdcpen0)
    }
    #[doc = "Charge pump enabled when VLCD is generated internally (VLCDEXT = 0) and VLCD 0 or VLCDREF 0"]
    #[inline(always)]
    pub fn lcdcpen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpen::Lcdcpen1)
    }
}
#[doc = "VLCD source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vlcdext {
    #[doc = "0: VLCD is generated internally"]
    Vlcdext0 = 0,
    #[doc = "1: VLCD is sourced externally"]
    Vlcdext1 = 1,
}
impl From<Vlcdext> for bool {
    #[inline(always)]
    fn from(variant: Vlcdext) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLCDEXT` reader - VLCD source select"]
pub type VlcdextR = crate::BitReader<Vlcdext>;
impl VlcdextR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vlcdext {
        match self.bits {
            false => Vlcdext::Vlcdext0,
            true => Vlcdext::Vlcdext1,
        }
    }
    #[doc = "VLCD is generated internally"]
    #[inline(always)]
    pub fn is_vlcdext_0(&self) -> bool {
        *self == Vlcdext::Vlcdext0
    }
    #[doc = "VLCD is sourced externally"]
    #[inline(always)]
    pub fn is_vlcdext_1(&self) -> bool {
        *self == Vlcdext::Vlcdext1
    }
}
#[doc = "Field `VLCDEXT` writer - VLCD source select"]
pub type VlcdextW<'a, REG> = crate::BitWriter<'a, REG, Vlcdext>;
impl<'a, REG> VlcdextW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VLCD is generated internally"]
    #[inline(always)]
    pub fn vlcdext_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdext::Vlcdext0)
    }
    #[doc = "VLCD is sourced externally"]
    #[inline(always)]
    pub fn vlcdext_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcdext::Vlcdext1)
    }
}
#[doc = "V2 to V4 voltage select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdextbias {
    #[doc = "0: V2 to V4 are generated internally"]
    Lcdextbias0 = 0,
    #[doc = "1: V2 to V4 are sourced externally and the internal bias generator is switched off"]
    Lcdextbias1 = 1,
}
impl From<Lcdextbias> for bool {
    #[inline(always)]
    fn from(variant: Lcdextbias) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDEXTBIAS` reader - V2 to V4 voltage select"]
pub type LcdextbiasR = crate::BitReader<Lcdextbias>;
impl LcdextbiasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdextbias {
        match self.bits {
            false => Lcdextbias::Lcdextbias0,
            true => Lcdextbias::Lcdextbias1,
        }
    }
    #[doc = "V2 to V4 are generated internally"]
    #[inline(always)]
    pub fn is_lcdextbias_0(&self) -> bool {
        *self == Lcdextbias::Lcdextbias0
    }
    #[doc = "V2 to V4 are sourced externally and the internal bias generator is switched off"]
    #[inline(always)]
    pub fn is_lcdextbias_1(&self) -> bool {
        *self == Lcdextbias::Lcdextbias1
    }
}
#[doc = "Field `LCDEXTBIAS` writer - V2 to V4 voltage select"]
pub type LcdextbiasW<'a, REG> = crate::BitWriter<'a, REG, Lcdextbias>;
impl<'a, REG> LcdextbiasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "V2 to V4 are generated internally"]
    #[inline(always)]
    pub fn lcdextbias_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdextbias::Lcdextbias0)
    }
    #[doc = "V2 to V4 are sourced externally and the internal bias generator is switched off"]
    #[inline(always)]
    pub fn lcdextbias_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdextbias::Lcdextbias1)
    }
}
#[doc = "V5 voltage select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum R03ext {
    #[doc = "0: V5 is VSS"]
    Vss = 0,
    #[doc = "1: V5 is sourced from the R03 pin"]
    R03 = 1,
}
impl From<R03ext> for bool {
    #[inline(always)]
    fn from(variant: R03ext) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R03EXT` reader - V5 voltage select"]
pub type R03extR = crate::BitReader<R03ext>;
impl R03extR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> R03ext {
        match self.bits {
            false => R03ext::Vss,
            true => R03ext::R03,
        }
    }
    #[doc = "V5 is VSS"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == R03ext::Vss
    }
    #[doc = "V5 is sourced from the R03 pin"]
    #[inline(always)]
    pub fn is_r03(&self) -> bool {
        *self == R03ext::R03
    }
}
#[doc = "Field `R03EXT` writer - V5 voltage select"]
pub type R03extW<'a, REG> = crate::BitWriter<'a, REG, R03ext>;
impl<'a, REG> R03extW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "V5 is VSS"]
    #[inline(always)]
    pub fn vss(self) -> &'a mut crate::W<REG> {
        self.variant(R03ext::Vss)
    }
    #[doc = "V5 is sourced from the R03 pin"]
    #[inline(always)]
    pub fn r03(self) -> &'a mut crate::W<REG> {
        self.variant(R03ext::R03)
    }
}
#[doc = "V2 to V4 voltage on external Rx3 pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdrext {
    #[doc = "0: Internally generated V2 to V4 are not switched to pins (LCDEXTBIAS = 0)"]
    Lcdrext0 = 0,
    #[doc = "1: Internally generated V2 to V4 are switched to pins (LCDEXTBIAS = 0)"]
    Lcdrext1 = 1,
}
impl From<Lcdrext> for bool {
    #[inline(always)]
    fn from(variant: Lcdrext) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDREXT` reader - V2 to V4 voltage on external Rx3 pins"]
pub type LcdrextR = crate::BitReader<Lcdrext>;
impl LcdrextR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdrext {
        match self.bits {
            false => Lcdrext::Lcdrext0,
            true => Lcdrext::Lcdrext1,
        }
    }
    #[doc = "Internally generated V2 to V4 are not switched to pins (LCDEXTBIAS = 0)"]
    #[inline(always)]
    pub fn is_lcdrext_0(&self) -> bool {
        *self == Lcdrext::Lcdrext0
    }
    #[doc = "Internally generated V2 to V4 are switched to pins (LCDEXTBIAS = 0)"]
    #[inline(always)]
    pub fn is_lcdrext_1(&self) -> bool {
        *self == Lcdrext::Lcdrext1
    }
}
#[doc = "Field `LCDREXT` writer - V2 to V4 voltage on external Rx3 pins"]
pub type LcdrextW<'a, REG> = crate::BitWriter<'a, REG, Lcdrext>;
impl<'a, REG> LcdrextW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internally generated V2 to V4 are not switched to pins (LCDEXTBIAS = 0)"]
    #[inline(always)]
    pub fn lcdrext_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdrext::Lcdrext0)
    }
    #[doc = "Internally generated V2 to V4 are switched to pins (LCDEXTBIAS = 0)"]
    #[inline(always)]
    pub fn lcdrext_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdrext::Lcdrext1)
    }
}
#[doc = "Charge pump voltage select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vlcd {
    #[doc = "0: Charge pump disabled"]
    Disabled = 0,
    #[doc = "1: If VLCDREF = 00 or 10: VLCD = 2.60 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF"]
    _2_60 = 1,
    #[doc = "2: If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    _2_66 = 2,
    #[doc = "3: If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    _2_72 = 3,
    #[doc = "4: If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    _2_78 = 4,
    #[doc = "5: If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    _2_84 = 5,
    #[doc = "6: If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    _2_90 = 6,
    #[doc = "7: If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    _2_96 = 7,
    #[doc = "8: If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    _3_02 = 8,
    #[doc = "9: If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    _3_08 = 9,
    #[doc = "10: If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    _3_14 = 10,
    #[doc = "11: If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    _3_20 = 11,
    #[doc = "12: If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    _3_26 = 12,
    #[doc = "13: If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    _3_32 = 13,
    #[doc = "14: If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    _3_38 = 14,
    #[doc = "15: If VLCDREF = 00 or 10: VLCD = 2.60 V + (15 1) * 0.06 V = 3.44 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (15 1) * 0.05 * VREF = 2.87 * VREF"]
    _3_44 = 15,
}
impl From<Vlcd> for u8 {
    #[inline(always)]
    fn from(variant: Vlcd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vlcd {
    type Ux = u8;
}
impl crate::IsEnum for Vlcd {}
#[doc = "Field `VLCD` reader - Charge pump voltage select"]
pub type VlcdR = crate::FieldReader<Vlcd>;
impl VlcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vlcd {
        match self.bits {
            0 => Vlcd::Disabled,
            1 => Vlcd::_2_60,
            2 => Vlcd::_2_66,
            3 => Vlcd::_2_72,
            4 => Vlcd::_2_78,
            5 => Vlcd::_2_84,
            6 => Vlcd::_2_90,
            7 => Vlcd::_2_96,
            8 => Vlcd::_3_02,
            9 => Vlcd::_3_08,
            10 => Vlcd::_3_14,
            11 => Vlcd::_3_20,
            12 => Vlcd::_3_26,
            13 => Vlcd::_3_32,
            14 => Vlcd::_3_38,
            15 => Vlcd::_3_44,
            _ => unreachable!(),
        }
    }
    #[doc = "Charge pump disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Vlcd::Disabled
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF"]
    #[inline(always)]
    pub fn is_2_60(&self) -> bool {
        *self == Vlcd::_2_60
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn is_2_66(&self) -> bool {
        *self == Vlcd::_2_66
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn is_2_72(&self) -> bool {
        *self == Vlcd::_2_72
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn is_2_78(&self) -> bool {
        *self == Vlcd::_2_78
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn is_2_84(&self) -> bool {
        *self == Vlcd::_2_84
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn is_2_90(&self) -> bool {
        *self == Vlcd::_2_90
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn is_2_96(&self) -> bool {
        *self == Vlcd::_2_96
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn is_3_02(&self) -> bool {
        *self == Vlcd::_3_02
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn is_3_08(&self) -> bool {
        *self == Vlcd::_3_08
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn is_3_14(&self) -> bool {
        *self == Vlcd::_3_14
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn is_3_20(&self) -> bool {
        *self == Vlcd::_3_20
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn is_3_26(&self) -> bool {
        *self == Vlcd::_3_26
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn is_3_32(&self) -> bool {
        *self == Vlcd::_3_32
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn is_3_38(&self) -> bool {
        *self == Vlcd::_3_38
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (15 1) * 0.06 V = 3.44 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (15 1) * 0.05 * VREF = 2.87 * VREF"]
    #[inline(always)]
    pub fn is_3_44(&self) -> bool {
        *self == Vlcd::_3_44
    }
}
#[doc = "Field `VLCD` writer - Charge pump voltage select"]
pub type VlcdW<'a, REG> = crate::FieldWriter<'a, REG, 4, Vlcd, crate::Safe>;
impl<'a, REG> VlcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Charge pump disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Disabled)
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF"]
    #[inline(always)]
    pub fn _2_60(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::_2_60)
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn _2_66(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::_2_66)
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn _2_72(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::_2_72)
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn _2_78(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::_2_78)
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn _2_84(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::_2_84)
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn _2_90(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::_2_90)
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn _2_96(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::_2_96)
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn _3_02(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::_3_02)
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn _3_08(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::_3_08)
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn _3_14(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::_3_14)
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn _3_20(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::_3_20)
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn _3_26(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::_3_26)
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn _3_32(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::_3_32)
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD 1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD 1) * 0.05 * VREF"]
    #[inline(always)]
    pub fn _3_38(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::_3_38)
    }
    #[doc = "If VLCDREF = 00 or 10: VLCD = 2.60 V + (15 1) * 0.06 V = 3.44 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (15 1) * 0.05 * VREF = 2.87 * VREF"]
    #[inline(always)]
    pub fn _3_44(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::_3_44)
    }
}
impl R {
    #[doc = "Bit 0 - Bias select"]
    #[inline(always)]
    pub fn lcd2b(&self) -> Lcd2bR {
        Lcd2bR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Charge pump reference select"]
    #[inline(always)]
    pub fn vlcdref(&self) -> VlcdrefR {
        VlcdrefR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Charge pump enable"]
    #[inline(always)]
    pub fn lcdcpen(&self) -> LcdcpenR {
        LcdcpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VLCD source select"]
    #[inline(always)]
    pub fn vlcdext(&self) -> VlcdextR {
        VlcdextR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - V2 to V4 voltage select"]
    #[inline(always)]
    pub fn lcdextbias(&self) -> LcdextbiasR {
        LcdextbiasR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - V5 voltage select"]
    #[inline(always)]
    pub fn r03ext(&self) -> R03extR {
        R03extR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - V2 to V4 voltage on external Rx3 pins"]
    #[inline(always)]
    pub fn lcdrext(&self) -> LcdrextR {
        LcdrextR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 9:12 - Charge pump voltage select"]
    #[inline(always)]
    pub fn vlcd(&self) -> VlcdR {
        VlcdR::new(((self.bits >> 9) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bias select"]
    #[inline(always)]
    pub fn lcd2b(&mut self) -> Lcd2bW<'_, LcdcvctlSpec> {
        Lcd2bW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Charge pump reference select"]
    #[inline(always)]
    pub fn vlcdref(&mut self) -> VlcdrefW<'_, LcdcvctlSpec> {
        VlcdrefW::new(self, 1)
    }
    #[doc = "Bit 3 - Charge pump enable"]
    #[inline(always)]
    pub fn lcdcpen(&mut self) -> LcdcpenW<'_, LcdcvctlSpec> {
        LcdcpenW::new(self, 3)
    }
    #[doc = "Bit 4 - VLCD source select"]
    #[inline(always)]
    pub fn vlcdext(&mut self) -> VlcdextW<'_, LcdcvctlSpec> {
        VlcdextW::new(self, 4)
    }
    #[doc = "Bit 5 - V2 to V4 voltage select"]
    #[inline(always)]
    pub fn lcdextbias(&mut self) -> LcdextbiasW<'_, LcdcvctlSpec> {
        LcdextbiasW::new(self, 5)
    }
    #[doc = "Bit 6 - V5 voltage select"]
    #[inline(always)]
    pub fn r03ext(&mut self) -> R03extW<'_, LcdcvctlSpec> {
        R03extW::new(self, 6)
    }
    #[doc = "Bit 7 - V2 to V4 voltage on external Rx3 pins"]
    #[inline(always)]
    pub fn lcdrext(&mut self) -> LcdrextW<'_, LcdcvctlSpec> {
        LcdrextW::new(self, 7)
    }
    #[doc = "Bits 9:12 - Charge pump voltage select"]
    #[inline(always)]
    pub fn vlcd(&mut self) -> VlcdW<'_, LcdcvctlSpec> {
        VlcdW::new(self, 9)
    }
}
#[doc = "LCD_C Voltage Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcvctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcvctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcvctlSpec;
impl crate::RegisterSpec for LcdcvctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdcvctl::R`](R) reader structure"]
impl crate::Readable for LcdcvctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdcvctl::W`](W) writer structure"]
impl crate::Writable for LcdcvctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDCVCTL to value 0"]
impl crate::Resettable for LcdcvctlSpec {}
