#[doc = "Register `CSCTL4` reader"]
pub type R = crate::R<Csctl4Spec>;
#[doc = "Register `CSCTL4` writer"]
pub type W = crate::W<Csctl4Spec>;
#[doc = "LFXT off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfxtoff {
    #[doc = "0: LFXT is on if LFXT is selected via the port selection and LFXT is not in bypass mode of operation"]
    Lfxtoff0 = 0,
    #[doc = "1: LFXT is off if it is not used as a source for ACLK, MCLK, or SMCLK"]
    Lfxtoff1 = 1,
}
impl From<Lfxtoff> for bool {
    #[inline(always)]
    fn from(variant: Lfxtoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXTOFF` reader - LFXT off"]
pub type LfxtoffR = crate::BitReader<Lfxtoff>;
impl LfxtoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfxtoff {
        match self.bits {
            false => Lfxtoff::Lfxtoff0,
            true => Lfxtoff::Lfxtoff1,
        }
    }
    #[doc = "LFXT is on if LFXT is selected via the port selection and LFXT is not in bypass mode of operation"]
    #[inline(always)]
    pub fn is_lfxtoff_0(&self) -> bool {
        *self == Lfxtoff::Lfxtoff0
    }
    #[doc = "LFXT is off if it is not used as a source for ACLK, MCLK, or SMCLK"]
    #[inline(always)]
    pub fn is_lfxtoff_1(&self) -> bool {
        *self == Lfxtoff::Lfxtoff1
    }
}
#[doc = "Field `LFXTOFF` writer - LFXT off"]
pub type LfxtoffW<'a, REG> = crate::BitWriter<'a, REG, Lfxtoff>;
impl<'a, REG> LfxtoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LFXT is on if LFXT is selected via the port selection and LFXT is not in bypass mode of operation"]
    #[inline(always)]
    pub fn lfxtoff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtoff::Lfxtoff0)
    }
    #[doc = "LFXT is off if it is not used as a source for ACLK, MCLK, or SMCLK"]
    #[inline(always)]
    pub fn lfxtoff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtoff::Lfxtoff1)
    }
}
#[doc = "SMCLK off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smclkoff {
    #[doc = "0: SMCLK on"]
    Smclkoff0 = 0,
    #[doc = "1: SMCLK off"]
    Smclkoff1 = 1,
}
impl From<Smclkoff> for bool {
    #[inline(always)]
    fn from(variant: Smclkoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMCLKOFF` reader - SMCLK off"]
pub type SmclkoffR = crate::BitReader<Smclkoff>;
impl SmclkoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smclkoff {
        match self.bits {
            false => Smclkoff::Smclkoff0,
            true => Smclkoff::Smclkoff1,
        }
    }
    #[doc = "SMCLK on"]
    #[inline(always)]
    pub fn is_smclkoff_0(&self) -> bool {
        *self == Smclkoff::Smclkoff0
    }
    #[doc = "SMCLK off"]
    #[inline(always)]
    pub fn is_smclkoff_1(&self) -> bool {
        *self == Smclkoff::Smclkoff1
    }
}
#[doc = "Field `SMCLKOFF` writer - SMCLK off"]
pub type SmclkoffW<'a, REG> = crate::BitWriter<'a, REG, Smclkoff>;
impl<'a, REG> SmclkoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMCLK on"]
    #[inline(always)]
    pub fn smclkoff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Smclkoff::Smclkoff0)
    }
    #[doc = "SMCLK off"]
    #[inline(always)]
    pub fn smclkoff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Smclkoff::Smclkoff1)
    }
}
#[doc = "VLO off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vlooff {
    #[doc = "0: VLO is on"]
    Vlooff0 = 0,
    #[doc = "1: VLO is off if it is not used as a source for ACLK, MCLK, or SMCLK or if not used as a source for the RTC in LPM3.5"]
    Vlooff1 = 1,
}
impl From<Vlooff> for bool {
    #[inline(always)]
    fn from(variant: Vlooff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLOOFF` reader - VLO off"]
pub type VlooffR = crate::BitReader<Vlooff>;
impl VlooffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vlooff {
        match self.bits {
            false => Vlooff::Vlooff0,
            true => Vlooff::Vlooff1,
        }
    }
    #[doc = "VLO is on"]
    #[inline(always)]
    pub fn is_vlooff_0(&self) -> bool {
        *self == Vlooff::Vlooff0
    }
    #[doc = "VLO is off if it is not used as a source for ACLK, MCLK, or SMCLK or if not used as a source for the RTC in LPM3.5"]
    #[inline(always)]
    pub fn is_vlooff_1(&self) -> bool {
        *self == Vlooff::Vlooff1
    }
}
#[doc = "Field `VLOOFF` writer - VLO off"]
pub type VlooffW<'a, REG> = crate::BitWriter<'a, REG, Vlooff>;
impl<'a, REG> VlooffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VLO is on"]
    #[inline(always)]
    pub fn vlooff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vlooff::Vlooff0)
    }
    #[doc = "VLO is off if it is not used as a source for ACLK, MCLK, or SMCLK or if not used as a source for the RTC in LPM3.5"]
    #[inline(always)]
    pub fn vlooff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vlooff::Vlooff1)
    }
}
#[doc = "LFXT bypass select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfxtbypass {
    #[doc = "0: LFXT sourced from external crystal"]
    Lfxtbypass0 = 0,
    #[doc = "1: LFXT sourced from external clock signal"]
    Lfxtbypass1 = 1,
}
impl From<Lfxtbypass> for bool {
    #[inline(always)]
    fn from(variant: Lfxtbypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXTBYPASS` reader - LFXT bypass select"]
pub type LfxtbypassR = crate::BitReader<Lfxtbypass>;
impl LfxtbypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfxtbypass {
        match self.bits {
            false => Lfxtbypass::Lfxtbypass0,
            true => Lfxtbypass::Lfxtbypass1,
        }
    }
    #[doc = "LFXT sourced from external crystal"]
    #[inline(always)]
    pub fn is_lfxtbypass_0(&self) -> bool {
        *self == Lfxtbypass::Lfxtbypass0
    }
    #[doc = "LFXT sourced from external clock signal"]
    #[inline(always)]
    pub fn is_lfxtbypass_1(&self) -> bool {
        *self == Lfxtbypass::Lfxtbypass1
    }
}
#[doc = "Field `LFXTBYPASS` writer - LFXT bypass select"]
pub type LfxtbypassW<'a, REG> = crate::BitWriter<'a, REG, Lfxtbypass>;
impl<'a, REG> LfxtbypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LFXT sourced from external crystal"]
    #[inline(always)]
    pub fn lfxtbypass_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtbypass::Lfxtbypass0)
    }
    #[doc = "LFXT sourced from external clock signal"]
    #[inline(always)]
    pub fn lfxtbypass_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtbypass::Lfxtbypass1)
    }
}
#[doc = "LFXT oscillator current\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfxtdrive {
    #[doc = "0: Lowest drive strength and current consumption LFXT oscillator"]
    Lfxtdrive0 = 0,
    #[doc = "1: Increased drive strength LFXT oscillator"]
    Lfxtdrive1 = 1,
    #[doc = "2: Increased drive strength LFXT oscillator"]
    Lfxtdrive2 = 2,
    #[doc = "3: Maximum drive strength and maximum current consumption LFXT oscillator"]
    Lfxtdrive3 = 3,
}
impl From<Lfxtdrive> for u8 {
    #[inline(always)]
    fn from(variant: Lfxtdrive) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfxtdrive {
    type Ux = u8;
}
impl crate::IsEnum for Lfxtdrive {}
#[doc = "Field `LFXTDRIVE` reader - LFXT oscillator current"]
pub type LfxtdriveR = crate::FieldReader<Lfxtdrive>;
impl LfxtdriveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfxtdrive {
        match self.bits {
            0 => Lfxtdrive::Lfxtdrive0,
            1 => Lfxtdrive::Lfxtdrive1,
            2 => Lfxtdrive::Lfxtdrive2,
            3 => Lfxtdrive::Lfxtdrive3,
            _ => unreachable!(),
        }
    }
    #[doc = "Lowest drive strength and current consumption LFXT oscillator"]
    #[inline(always)]
    pub fn is_lfxtdrive_0(&self) -> bool {
        *self == Lfxtdrive::Lfxtdrive0
    }
    #[doc = "Increased drive strength LFXT oscillator"]
    #[inline(always)]
    pub fn is_lfxtdrive_1(&self) -> bool {
        *self == Lfxtdrive::Lfxtdrive1
    }
    #[doc = "Increased drive strength LFXT oscillator"]
    #[inline(always)]
    pub fn is_lfxtdrive_2(&self) -> bool {
        *self == Lfxtdrive::Lfxtdrive2
    }
    #[doc = "Maximum drive strength and maximum current consumption LFXT oscillator"]
    #[inline(always)]
    pub fn is_lfxtdrive_3(&self) -> bool {
        *self == Lfxtdrive::Lfxtdrive3
    }
}
#[doc = "Field `LFXTDRIVE` writer - LFXT oscillator current"]
pub type LfxtdriveW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lfxtdrive, crate::Safe>;
impl<'a, REG> LfxtdriveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lowest drive strength and current consumption LFXT oscillator"]
    #[inline(always)]
    pub fn lfxtdrive_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtdrive::Lfxtdrive0)
    }
    #[doc = "Increased drive strength LFXT oscillator"]
    #[inline(always)]
    pub fn lfxtdrive_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtdrive::Lfxtdrive1)
    }
    #[doc = "Increased drive strength LFXT oscillator"]
    #[inline(always)]
    pub fn lfxtdrive_2(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtdrive::Lfxtdrive2)
    }
    #[doc = "Maximum drive strength and maximum current consumption LFXT oscillator"]
    #[inline(always)]
    pub fn lfxtdrive_3(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtdrive::Lfxtdrive3)
    }
}
#[doc = "Turns off the HFXT oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfxtoff {
    #[doc = "0: HFXT is on if HFXT is selected via the port selection and HFXT is not in bypass mode of operation"]
    Hfxtoff0 = 0,
    #[doc = "1: HFXT is off if it is not used as a source for ACLK, MCLK, or SMCLK"]
    Hfxtoff1 = 1,
}
impl From<Hfxtoff> for bool {
    #[inline(always)]
    fn from(variant: Hfxtoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFXTOFF` reader - Turns off the HFXT oscillator"]
pub type HfxtoffR = crate::BitReader<Hfxtoff>;
impl HfxtoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfxtoff {
        match self.bits {
            false => Hfxtoff::Hfxtoff0,
            true => Hfxtoff::Hfxtoff1,
        }
    }
    #[doc = "HFXT is on if HFXT is selected via the port selection and HFXT is not in bypass mode of operation"]
    #[inline(always)]
    pub fn is_hfxtoff_0(&self) -> bool {
        *self == Hfxtoff::Hfxtoff0
    }
    #[doc = "HFXT is off if it is not used as a source for ACLK, MCLK, or SMCLK"]
    #[inline(always)]
    pub fn is_hfxtoff_1(&self) -> bool {
        *self == Hfxtoff::Hfxtoff1
    }
}
#[doc = "Field `HFXTOFF` writer - Turns off the HFXT oscillator"]
pub type HfxtoffW<'a, REG> = crate::BitWriter<'a, REG, Hfxtoff>;
impl<'a, REG> HfxtoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HFXT is on if HFXT is selected via the port selection and HFXT is not in bypass mode of operation"]
    #[inline(always)]
    pub fn hfxtoff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtoff::Hfxtoff0)
    }
    #[doc = "HFXT is off if it is not used as a source for ACLK, MCLK, or SMCLK"]
    #[inline(always)]
    pub fn hfxtoff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtoff::Hfxtoff1)
    }
}
#[doc = "HFXT frequency selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hffreq {
    #[doc = "0: 0 to 4 MHz"]
    Hffreq0 = 0,
    #[doc = "1: Greater than 4 MHz to 8 MHz"]
    Hffreq1 = 1,
    #[doc = "2: Greater than 8 MHz to 16 MHz"]
    Hffreq2 = 2,
    #[doc = "3: Greater than 16 MHz to 24 MHz"]
    Hffreq3 = 3,
}
impl From<Hffreq> for u8 {
    #[inline(always)]
    fn from(variant: Hffreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hffreq {
    type Ux = u8;
}
impl crate::IsEnum for Hffreq {}
#[doc = "Field `HFFREQ` reader - HFXT frequency selection"]
pub type HffreqR = crate::FieldReader<Hffreq>;
impl HffreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hffreq {
        match self.bits {
            0 => Hffreq::Hffreq0,
            1 => Hffreq::Hffreq1,
            2 => Hffreq::Hffreq2,
            3 => Hffreq::Hffreq3,
            _ => unreachable!(),
        }
    }
    #[doc = "0 to 4 MHz"]
    #[inline(always)]
    pub fn is_hffreq_0(&self) -> bool {
        *self == Hffreq::Hffreq0
    }
    #[doc = "Greater than 4 MHz to 8 MHz"]
    #[inline(always)]
    pub fn is_hffreq_1(&self) -> bool {
        *self == Hffreq::Hffreq1
    }
    #[doc = "Greater than 8 MHz to 16 MHz"]
    #[inline(always)]
    pub fn is_hffreq_2(&self) -> bool {
        *self == Hffreq::Hffreq2
    }
    #[doc = "Greater than 16 MHz to 24 MHz"]
    #[inline(always)]
    pub fn is_hffreq_3(&self) -> bool {
        *self == Hffreq::Hffreq3
    }
}
#[doc = "Field `HFFREQ` writer - HFXT frequency selection"]
pub type HffreqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hffreq, crate::Safe>;
impl<'a, REG> HffreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 to 4 MHz"]
    #[inline(always)]
    pub fn hffreq_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hffreq::Hffreq0)
    }
    #[doc = "Greater than 4 MHz to 8 MHz"]
    #[inline(always)]
    pub fn hffreq_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hffreq::Hffreq1)
    }
    #[doc = "Greater than 8 MHz to 16 MHz"]
    #[inline(always)]
    pub fn hffreq_2(self) -> &'a mut crate::W<REG> {
        self.variant(Hffreq::Hffreq2)
    }
    #[doc = "Greater than 16 MHz to 24 MHz"]
    #[inline(always)]
    pub fn hffreq_3(self) -> &'a mut crate::W<REG> {
        self.variant(Hffreq::Hffreq3)
    }
}
#[doc = "HFXT bypass select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfxtbypass {
    #[doc = "0: HFXT sourced from external crystal"]
    Hfxtbypass0 = 0,
    #[doc = "1: HFXT sourced from external clock signal"]
    Hfxtbypass1 = 1,
}
impl From<Hfxtbypass> for bool {
    #[inline(always)]
    fn from(variant: Hfxtbypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFXTBYPASS` reader - HFXT bypass select"]
pub type HfxtbypassR = crate::BitReader<Hfxtbypass>;
impl HfxtbypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfxtbypass {
        match self.bits {
            false => Hfxtbypass::Hfxtbypass0,
            true => Hfxtbypass::Hfxtbypass1,
        }
    }
    #[doc = "HFXT sourced from external crystal"]
    #[inline(always)]
    pub fn is_hfxtbypass_0(&self) -> bool {
        *self == Hfxtbypass::Hfxtbypass0
    }
    #[doc = "HFXT sourced from external clock signal"]
    #[inline(always)]
    pub fn is_hfxtbypass_1(&self) -> bool {
        *self == Hfxtbypass::Hfxtbypass1
    }
}
#[doc = "Field `HFXTBYPASS` writer - HFXT bypass select"]
pub type HfxtbypassW<'a, REG> = crate::BitWriter<'a, REG, Hfxtbypass>;
impl<'a, REG> HfxtbypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HFXT sourced from external crystal"]
    #[inline(always)]
    pub fn hfxtbypass_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtbypass::Hfxtbypass0)
    }
    #[doc = "HFXT sourced from external clock signal"]
    #[inline(always)]
    pub fn hfxtbypass_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtbypass::Hfxtbypass1)
    }
}
#[doc = "HFXT oscillator current\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfxtdrive {
    #[doc = "0: Lowest current consumption"]
    Hfxtdrive0 = 0,
    #[doc = "1: Increased drive strength HFXT oscillator"]
    Hfxtdrive1 = 1,
    #[doc = "2: Increased drive strength HFXT oscillator"]
    Hfxtdrive2 = 2,
    #[doc = "3: Maximum drive strength HFXT oscillator"]
    Hfxtdrive3 = 3,
}
impl From<Hfxtdrive> for u8 {
    #[inline(always)]
    fn from(variant: Hfxtdrive) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfxtdrive {
    type Ux = u8;
}
impl crate::IsEnum for Hfxtdrive {}
#[doc = "Field `HFXTDRIVE` reader - HFXT oscillator current"]
pub type HfxtdriveR = crate::FieldReader<Hfxtdrive>;
impl HfxtdriveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfxtdrive {
        match self.bits {
            0 => Hfxtdrive::Hfxtdrive0,
            1 => Hfxtdrive::Hfxtdrive1,
            2 => Hfxtdrive::Hfxtdrive2,
            3 => Hfxtdrive::Hfxtdrive3,
            _ => unreachable!(),
        }
    }
    #[doc = "Lowest current consumption"]
    #[inline(always)]
    pub fn is_hfxtdrive_0(&self) -> bool {
        *self == Hfxtdrive::Hfxtdrive0
    }
    #[doc = "Increased drive strength HFXT oscillator"]
    #[inline(always)]
    pub fn is_hfxtdrive_1(&self) -> bool {
        *self == Hfxtdrive::Hfxtdrive1
    }
    #[doc = "Increased drive strength HFXT oscillator"]
    #[inline(always)]
    pub fn is_hfxtdrive_2(&self) -> bool {
        *self == Hfxtdrive::Hfxtdrive2
    }
    #[doc = "Maximum drive strength HFXT oscillator"]
    #[inline(always)]
    pub fn is_hfxtdrive_3(&self) -> bool {
        *self == Hfxtdrive::Hfxtdrive3
    }
}
#[doc = "Field `HFXTDRIVE` writer - HFXT oscillator current"]
pub type HfxtdriveW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hfxtdrive, crate::Safe>;
impl<'a, REG> HfxtdriveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lowest current consumption"]
    #[inline(always)]
    pub fn hfxtdrive_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtdrive::Hfxtdrive0)
    }
    #[doc = "Increased drive strength HFXT oscillator"]
    #[inline(always)]
    pub fn hfxtdrive_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtdrive::Hfxtdrive1)
    }
    #[doc = "Increased drive strength HFXT oscillator"]
    #[inline(always)]
    pub fn hfxtdrive_2(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtdrive::Hfxtdrive2)
    }
    #[doc = "Maximum drive strength HFXT oscillator"]
    #[inline(always)]
    pub fn hfxtdrive_3(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtdrive::Hfxtdrive3)
    }
}
impl R {
    #[doc = "Bit 0 - LFXT off"]
    #[inline(always)]
    pub fn lfxtoff(&self) -> LfxtoffR {
        LfxtoffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMCLK off"]
    #[inline(always)]
    pub fn smclkoff(&self) -> SmclkoffR {
        SmclkoffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - VLO off"]
    #[inline(always)]
    pub fn vlooff(&self) -> VlooffR {
        VlooffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LFXT bypass select"]
    #[inline(always)]
    pub fn lfxtbypass(&self) -> LfxtbypassR {
        LfxtbypassR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - LFXT oscillator current"]
    #[inline(always)]
    pub fn lfxtdrive(&self) -> LfxtdriveR {
        LfxtdriveR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Turns off the HFXT oscillator"]
    #[inline(always)]
    pub fn hfxtoff(&self) -> HfxtoffR {
        HfxtoffR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - HFXT frequency selection"]
    #[inline(always)]
    pub fn hffreq(&self) -> HffreqR {
        HffreqR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - HFXT bypass select"]
    #[inline(always)]
    pub fn hfxtbypass(&self) -> HfxtbypassR {
        HfxtbypassR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - HFXT oscillator current"]
    #[inline(always)]
    pub fn hfxtdrive(&self) -> HfxtdriveR {
        HfxtdriveR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LFXT off"]
    #[inline(always)]
    pub fn lfxtoff(&mut self) -> LfxtoffW<'_, Csctl4Spec> {
        LfxtoffW::new(self, 0)
    }
    #[doc = "Bit 1 - SMCLK off"]
    #[inline(always)]
    pub fn smclkoff(&mut self) -> SmclkoffW<'_, Csctl4Spec> {
        SmclkoffW::new(self, 1)
    }
    #[doc = "Bit 3 - VLO off"]
    #[inline(always)]
    pub fn vlooff(&mut self) -> VlooffW<'_, Csctl4Spec> {
        VlooffW::new(self, 3)
    }
    #[doc = "Bit 4 - LFXT bypass select"]
    #[inline(always)]
    pub fn lfxtbypass(&mut self) -> LfxtbypassW<'_, Csctl4Spec> {
        LfxtbypassW::new(self, 4)
    }
    #[doc = "Bits 6:7 - LFXT oscillator current"]
    #[inline(always)]
    pub fn lfxtdrive(&mut self) -> LfxtdriveW<'_, Csctl4Spec> {
        LfxtdriveW::new(self, 6)
    }
    #[doc = "Bit 8 - Turns off the HFXT oscillator"]
    #[inline(always)]
    pub fn hfxtoff(&mut self) -> HfxtoffW<'_, Csctl4Spec> {
        HfxtoffW::new(self, 8)
    }
    #[doc = "Bits 10:11 - HFXT frequency selection"]
    #[inline(always)]
    pub fn hffreq(&mut self) -> HffreqW<'_, Csctl4Spec> {
        HffreqW::new(self, 10)
    }
    #[doc = "Bit 12 - HFXT bypass select"]
    #[inline(always)]
    pub fn hfxtbypass(&mut self) -> HfxtbypassW<'_, Csctl4Spec> {
        HfxtbypassW::new(self, 12)
    }
    #[doc = "Bits 14:15 - HFXT oscillator current"]
    #[inline(always)]
    pub fn hfxtdrive(&mut self) -> HfxtdriveW<'_, Csctl4Spec> {
        HfxtdriveW::new(self, 14)
    }
}
#[doc = "Clock System Control 4\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl4Spec;
impl crate::RegisterSpec for Csctl4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl4::R`](R) reader structure"]
impl crate::Readable for Csctl4Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl4::W`](W) writer structure"]
impl crate::Writable for Csctl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL4 to value 0"]
impl crate::Resettable for Csctl4Spec {}
