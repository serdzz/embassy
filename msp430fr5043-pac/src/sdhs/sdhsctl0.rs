#[doc = "Register `SDHSCTL0` reader"]
pub type R = crate::R<Sdhsctl0Spec>;
#[doc = "Register `SDHSCTL0` writer"]
pub type W = crate::W<Sdhsctl0Spec>;
#[doc = "SDHS Auto Sample Start Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autossdis {
    #[doc = "0: Auto Sample start enabled. SDHS is powered up when the SHDS_PWR_UP applied, then data conversion is automatically started once the SDHS is fully powered up."]
    Autossdis0 = 0,
    #[doc = "1: Auto Sample start disabled. (This configuration must be used when the ASQ controls the measurement sequences) - SHDS_PWR_UP signal to turns on the SDHS - CONVERSION_START signal to start data convesion"]
    Autossdis1 = 1,
}
impl From<Autossdis> for bool {
    #[inline(always)]
    fn from(variant: Autossdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOSSDIS` reader - SDHS Auto Sample Start Disable"]
pub type AutossdisR = crate::BitReader<Autossdis>;
impl AutossdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autossdis {
        match self.bits {
            false => Autossdis::Autossdis0,
            true => Autossdis::Autossdis1,
        }
    }
    #[doc = "Auto Sample start enabled. SDHS is powered up when the SHDS_PWR_UP applied, then data conversion is automatically started once the SDHS is fully powered up."]
    #[inline(always)]
    pub fn is_autossdis_0(&self) -> bool {
        *self == Autossdis::Autossdis0
    }
    #[doc = "Auto Sample start disabled. (This configuration must be used when the ASQ controls the measurement sequences) - SHDS_PWR_UP signal to turns on the SDHS - CONVERSION_START signal to start data convesion"]
    #[inline(always)]
    pub fn is_autossdis_1(&self) -> bool {
        *self == Autossdis::Autossdis1
    }
}
#[doc = "Field `AUTOSSDIS` writer - SDHS Auto Sample Start Disable"]
pub type AutossdisW<'a, REG> = crate::BitWriter<'a, REG, Autossdis>;
impl<'a, REG> AutossdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto Sample start enabled. SDHS is powered up when the SHDS_PWR_UP applied, then data conversion is automatically started once the SDHS is fully powered up."]
    #[inline(always)]
    pub fn autossdis_0(self) -> &'a mut crate::W<REG> {
        self.variant(Autossdis::Autossdis0)
    }
    #[doc = "Auto Sample start disabled. (This configuration must be used when the ASQ controls the measurement sequences) - SHDS_PWR_UP signal to turns on the SDHS - CONVERSION_START signal to start data convesion"]
    #[inline(always)]
    pub fn autossdis_1(self) -> &'a mut crate::W<REG> {
        self.variant(Autossdis::Autossdis1)
    }
}
#[doc = "DTRDY Interrupt delay select. This regiser can be used to discard up to 7 samples after conversion start. Note that the skipped samples will be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Intdly {
    #[doc = "0: No dealy"]
    Intdly0 = 0,
    #[doc = "1: 1 sample delay, 2nd sample is the first interrupt"]
    Intdly1 = 1,
    #[doc = "2: 2 samples delay, 3rd sample is the first interrupt"]
    Intdly2 = 2,
    #[doc = "3: 3 samples delay, 4rd sample is the first interrupt"]
    Intdly3 = 3,
    #[doc = "4: 4 samples delay, 5th sample is the first interrupt"]
    Intdly4 = 4,
    #[doc = "5: 5 samples delay, 6th sample is the first interrupt"]
    Intdly5 = 5,
    #[doc = "6: 6 samples delay, 7th sample is the first interrupt"]
    Intdly6 = 6,
    #[doc = "7: 7 samples delay, 8th sample is the first interrupt"]
    Intdly7 = 7,
}
impl From<Intdly> for u8 {
    #[inline(always)]
    fn from(variant: Intdly) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Intdly {
    type Ux = u8;
}
impl crate::IsEnum for Intdly {}
#[doc = "Field `INTDLY` reader - DTRDY Interrupt delay select. This regiser can be used to discard up to 7 samples after conversion start. Note that the skipped samples will be lost."]
pub type IntdlyR = crate::FieldReader<Intdly>;
impl IntdlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intdly {
        match self.bits {
            0 => Intdly::Intdly0,
            1 => Intdly::Intdly1,
            2 => Intdly::Intdly2,
            3 => Intdly::Intdly3,
            4 => Intdly::Intdly4,
            5 => Intdly::Intdly5,
            6 => Intdly::Intdly6,
            7 => Intdly::Intdly7,
            _ => unreachable!(),
        }
    }
    #[doc = "No dealy"]
    #[inline(always)]
    pub fn is_intdly_0(&self) -> bool {
        *self == Intdly::Intdly0
    }
    #[doc = "1 sample delay, 2nd sample is the first interrupt"]
    #[inline(always)]
    pub fn is_intdly_1(&self) -> bool {
        *self == Intdly::Intdly1
    }
    #[doc = "2 samples delay, 3rd sample is the first interrupt"]
    #[inline(always)]
    pub fn is_intdly_2(&self) -> bool {
        *self == Intdly::Intdly2
    }
    #[doc = "3 samples delay, 4rd sample is the first interrupt"]
    #[inline(always)]
    pub fn is_intdly_3(&self) -> bool {
        *self == Intdly::Intdly3
    }
    #[doc = "4 samples delay, 5th sample is the first interrupt"]
    #[inline(always)]
    pub fn is_intdly_4(&self) -> bool {
        *self == Intdly::Intdly4
    }
    #[doc = "5 samples delay, 6th sample is the first interrupt"]
    #[inline(always)]
    pub fn is_intdly_5(&self) -> bool {
        *self == Intdly::Intdly5
    }
    #[doc = "6 samples delay, 7th sample is the first interrupt"]
    #[inline(always)]
    pub fn is_intdly_6(&self) -> bool {
        *self == Intdly::Intdly6
    }
    #[doc = "7 samples delay, 8th sample is the first interrupt"]
    #[inline(always)]
    pub fn is_intdly_7(&self) -> bool {
        *self == Intdly::Intdly7
    }
}
#[doc = "Field `INTDLY` writer - DTRDY Interrupt delay select. This regiser can be used to discard up to 7 samples after conversion start. Note that the skipped samples will be lost."]
pub type IntdlyW<'a, REG> = crate::FieldWriter<'a, REG, 3, Intdly, crate::Safe>;
impl<'a, REG> IntdlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No dealy"]
    #[inline(always)]
    pub fn intdly_0(self) -> &'a mut crate::W<REG> {
        self.variant(Intdly::Intdly0)
    }
    #[doc = "1 sample delay, 2nd sample is the first interrupt"]
    #[inline(always)]
    pub fn intdly_1(self) -> &'a mut crate::W<REG> {
        self.variant(Intdly::Intdly1)
    }
    #[doc = "2 samples delay, 3rd sample is the first interrupt"]
    #[inline(always)]
    pub fn intdly_2(self) -> &'a mut crate::W<REG> {
        self.variant(Intdly::Intdly2)
    }
    #[doc = "3 samples delay, 4rd sample is the first interrupt"]
    #[inline(always)]
    pub fn intdly_3(self) -> &'a mut crate::W<REG> {
        self.variant(Intdly::Intdly3)
    }
    #[doc = "4 samples delay, 5th sample is the first interrupt"]
    #[inline(always)]
    pub fn intdly_4(self) -> &'a mut crate::W<REG> {
        self.variant(Intdly::Intdly4)
    }
    #[doc = "5 samples delay, 6th sample is the first interrupt"]
    #[inline(always)]
    pub fn intdly_5(self) -> &'a mut crate::W<REG> {
        self.variant(Intdly::Intdly5)
    }
    #[doc = "6 samples delay, 7th sample is the first interrupt"]
    #[inline(always)]
    pub fn intdly_6(self) -> &'a mut crate::W<REG> {
        self.variant(Intdly::Intdly6)
    }
    #[doc = "7 samples delay, 8th sample is the first interrupt"]
    #[inline(always)]
    pub fn intdly_7(self) -> &'a mut crate::W<REG> {
        self.variant(Intdly::Intdly7)
    }
}
#[doc = "Data alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dalgn {
    #[doc = "0: Right-aligned."]
    Dalgn0 = 0,
    #[doc = "1: Left-aligned."]
    Dalgn1 = 1,
}
impl From<Dalgn> for bool {
    #[inline(always)]
    fn from(variant: Dalgn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DALGN` reader - Data alignment"]
pub type DalgnR = crate::BitReader<Dalgn>;
impl DalgnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dalgn {
        match self.bits {
            false => Dalgn::Dalgn0,
            true => Dalgn::Dalgn1,
        }
    }
    #[doc = "Right-aligned."]
    #[inline(always)]
    pub fn is_dalgn_0(&self) -> bool {
        *self == Dalgn::Dalgn0
    }
    #[doc = "Left-aligned."]
    #[inline(always)]
    pub fn is_dalgn_1(&self) -> bool {
        *self == Dalgn::Dalgn1
    }
}
#[doc = "Field `DALGN` writer - Data alignment"]
pub type DalgnW<'a, REG> = crate::BitWriter<'a, REG, Dalgn>;
impl<'a, REG> DalgnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Right-aligned."]
    #[inline(always)]
    pub fn dalgn_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dalgn::Dalgn0)
    }
    #[doc = "Left-aligned."]
    #[inline(always)]
    pub fn dalgn_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dalgn::Dalgn1)
    }
}
#[doc = "Data format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dfmsel {
    #[doc = "0: 2's complement"]
    Dfmsel0 = 0,
    #[doc = "1: Offset binary"]
    Dfmsel1 = 1,
    #[doc = "2: Reserved (defaults to 0, 2s complement)"]
    Dfmsel2 = 2,
    #[doc = "3: Reserved (defaults to 0, 2s complement)"]
    Dfmsel3 = 3,
}
impl From<Dfmsel> for u8 {
    #[inline(always)]
    fn from(variant: Dfmsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dfmsel {
    type Ux = u8;
}
impl crate::IsEnum for Dfmsel {}
#[doc = "Field `DFMSEL` reader - Data format"]
pub type DfmselR = crate::FieldReader<Dfmsel>;
impl DfmselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfmsel {
        match self.bits {
            0 => Dfmsel::Dfmsel0,
            1 => Dfmsel::Dfmsel1,
            2 => Dfmsel::Dfmsel2,
            3 => Dfmsel::Dfmsel3,
            _ => unreachable!(),
        }
    }
    #[doc = "2's complement"]
    #[inline(always)]
    pub fn is_dfmsel_0(&self) -> bool {
        *self == Dfmsel::Dfmsel0
    }
    #[doc = "Offset binary"]
    #[inline(always)]
    pub fn is_dfmsel_1(&self) -> bool {
        *self == Dfmsel::Dfmsel1
    }
    #[doc = "Reserved (defaults to 0, 2s complement)"]
    #[inline(always)]
    pub fn is_dfmsel_2(&self) -> bool {
        *self == Dfmsel::Dfmsel2
    }
    #[doc = "Reserved (defaults to 0, 2s complement)"]
    #[inline(always)]
    pub fn is_dfmsel_3(&self) -> bool {
        *self == Dfmsel::Dfmsel3
    }
}
#[doc = "Field `DFMSEL` writer - Data format"]
pub type DfmselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dfmsel, crate::Safe>;
impl<'a, REG> DfmselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2's complement"]
    #[inline(always)]
    pub fn dfmsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dfmsel::Dfmsel0)
    }
    #[doc = "Offset binary"]
    #[inline(always)]
    pub fn dfmsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dfmsel::Dfmsel1)
    }
    #[doc = "Reserved (defaults to 0, 2s complement)"]
    #[inline(always)]
    pub fn dfmsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dfmsel::Dfmsel2)
    }
    #[doc = "Reserved (defaults to 0, 2s complement)"]
    #[inline(always)]
    pub fn dfmsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dfmsel::Dfmsel3)
    }
}
#[doc = "Output Bit Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Obr {
    #[doc = "0: 12-bit"]
    Obr0 = 0,
    #[doc = "1: 13-bit"]
    Obr1 = 1,
    #[doc = "2: 14-bit"]
    Obr2 = 2,
    #[doc = "3: Reserved (default: 12-bit)"]
    Obr3 = 3,
}
impl From<Obr> for u8 {
    #[inline(always)]
    fn from(variant: Obr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Obr {
    type Ux = u8;
}
impl crate::IsEnum for Obr {}
#[doc = "Field `OBR` reader - Output Bit Resolution"]
pub type ObrR = crate::FieldReader<Obr>;
impl ObrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Obr {
        match self.bits {
            0 => Obr::Obr0,
            1 => Obr::Obr1,
            2 => Obr::Obr2,
            3 => Obr::Obr3,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn is_obr_0(&self) -> bool {
        *self == Obr::Obr0
    }
    #[doc = "13-bit"]
    #[inline(always)]
    pub fn is_obr_1(&self) -> bool {
        *self == Obr::Obr1
    }
    #[doc = "14-bit"]
    #[inline(always)]
    pub fn is_obr_2(&self) -> bool {
        *self == Obr::Obr2
    }
    #[doc = "Reserved (default: 12-bit)"]
    #[inline(always)]
    pub fn is_obr_3(&self) -> bool {
        *self == Obr::Obr3
    }
}
#[doc = "Field `OBR` writer - Output Bit Resolution"]
pub type ObrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Obr, crate::Safe>;
impl<'a, REG> ObrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn obr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Obr::Obr0)
    }
    #[doc = "13-bit"]
    #[inline(always)]
    pub fn obr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Obr::Obr1)
    }
    #[doc = "14-bit"]
    #[inline(always)]
    pub fn obr_2(self) -> &'a mut crate::W<REG> {
        self.variant(Obr::Obr2)
    }
    #[doc = "Reserved (default: 12-bit)"]
    #[inline(always)]
    pub fn obr_3(self) -> &'a mut crate::W<REG> {
        self.variant(Obr::Obr3)
    }
}
#[doc = "MSB Shift\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Shift {
    #[doc = "0: No Shift, MSB."]
    Shift0 = 0,
    #[doc = "1: MSB - 1 (Shift left by 1 from filter out). If OBR = 2, then this configuration is invalid. No shift is performed."]
    Shift1 = 1,
    #[doc = "2: MSB -2 (Shift left by 2 from filter out). If OBR = 1, then this configuration is invalid. No shift is performed."]
    Shift2 = 2,
    #[doc = "3: Reserved (No shift)"]
    Shift3 = 3,
}
impl From<Shift> for u8 {
    #[inline(always)]
    fn from(variant: Shift) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Shift {
    type Ux = u8;
}
impl crate::IsEnum for Shift {}
#[doc = "Field `SHIFT` reader - MSB Shift"]
pub type ShiftR = crate::FieldReader<Shift>;
impl ShiftR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shift {
        match self.bits {
            0 => Shift::Shift0,
            1 => Shift::Shift1,
            2 => Shift::Shift2,
            3 => Shift::Shift3,
            _ => unreachable!(),
        }
    }
    #[doc = "No Shift, MSB."]
    #[inline(always)]
    pub fn is_shift_0(&self) -> bool {
        *self == Shift::Shift0
    }
    #[doc = "MSB - 1 (Shift left by 1 from filter out). If OBR = 2, then this configuration is invalid. No shift is performed."]
    #[inline(always)]
    pub fn is_shift_1(&self) -> bool {
        *self == Shift::Shift1
    }
    #[doc = "MSB -2 (Shift left by 2 from filter out). If OBR = 1, then this configuration is invalid. No shift is performed."]
    #[inline(always)]
    pub fn is_shift_2(&self) -> bool {
        *self == Shift::Shift2
    }
    #[doc = "Reserved (No shift)"]
    #[inline(always)]
    pub fn is_shift_3(&self) -> bool {
        *self == Shift::Shift3
    }
}
#[doc = "Field `SHIFT` writer - MSB Shift"]
pub type ShiftW<'a, REG> = crate::FieldWriter<'a, REG, 2, Shift, crate::Safe>;
impl<'a, REG> ShiftW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Shift, MSB."]
    #[inline(always)]
    pub fn shift_0(self) -> &'a mut crate::W<REG> {
        self.variant(Shift::Shift0)
    }
    #[doc = "MSB - 1 (Shift left by 1 from filter out). If OBR = 2, then this configuration is invalid. No shift is performed."]
    #[inline(always)]
    pub fn shift_1(self) -> &'a mut crate::W<REG> {
        self.variant(Shift::Shift1)
    }
    #[doc = "MSB -2 (Shift left by 2 from filter out). If OBR = 1, then this configuration is invalid. No shift is performed."]
    #[inline(always)]
    pub fn shift_2(self) -> &'a mut crate::W<REG> {
        self.variant(Shift::Shift2)
    }
    #[doc = "Reserved (No shift)"]
    #[inline(always)]
    pub fn shift_3(self) -> &'a mut crate::W<REG> {
        self.variant(Shift::Shift3)
    }
}
#[doc = "SDHS trigger source select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgsrc {
    #[doc = "0: Register control mode: - CTL4.SDHSON is the source of the SHDS_PWR_UP/DOWN signal - CTL5.SSTART is the source of the CONVERSION_START/STOP signal"]
    Trgsrc0 = 0,
    #[doc = "1: ASQ control mode: The SDHS is controlled by the ASQ. - ASQ_ACQARM signal from the ASQ is the source of the SHDS_PWR_UP/DOWN signal - ASQ_ACQTRIG signal from the ASQ is the source of the CONVERSION_START/STOP signal"]
    Trgsrc1 = 1,
}
impl From<Trgsrc> for bool {
    #[inline(always)]
    fn from(variant: Trgsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGSRC` reader - SDHS trigger source select."]
pub type TrgsrcR = crate::BitReader<Trgsrc>;
impl TrgsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc {
        match self.bits {
            false => Trgsrc::Trgsrc0,
            true => Trgsrc::Trgsrc1,
        }
    }
    #[doc = "Register control mode: - CTL4.SDHSON is the source of the SHDS_PWR_UP/DOWN signal - CTL5.SSTART is the source of the CONVERSION_START/STOP signal"]
    #[inline(always)]
    pub fn is_trgsrc_0(&self) -> bool {
        *self == Trgsrc::Trgsrc0
    }
    #[doc = "ASQ control mode: The SDHS is controlled by the ASQ. - ASQ_ACQARM signal from the ASQ is the source of the SHDS_PWR_UP/DOWN signal - ASQ_ACQTRIG signal from the ASQ is the source of the CONVERSION_START/STOP signal"]
    #[inline(always)]
    pub fn is_trgsrc_1(&self) -> bool {
        *self == Trgsrc::Trgsrc1
    }
}
#[doc = "Field `TRGSRC` writer - SDHS trigger source select."]
pub type TrgsrcW<'a, REG> = crate::BitWriter<'a, REG, Trgsrc>;
impl<'a, REG> TrgsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Register control mode: - CTL4.SDHSON is the source of the SHDS_PWR_UP/DOWN signal - CTL5.SSTART is the source of the CONVERSION_START/STOP signal"]
    #[inline(always)]
    pub fn trgsrc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc::Trgsrc0)
    }
    #[doc = "ASQ control mode: The SDHS is controlled by the ASQ. - ASQ_ACQARM signal from the ASQ is the source of the SHDS_PWR_UP/DOWN signal - ASQ_ACQTRIG signal from the ASQ is the source of the CONVERSION_START/STOP signal"]
    #[inline(always)]
    pub fn trgsrc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc::Trgsrc1)
    }
}
impl R {
    #[doc = "Bit 0 - SDHS Auto Sample Start Disable"]
    #[inline(always)]
    pub fn autossdis(&self) -> AutossdisR {
        AutossdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - DTRDY Interrupt delay select. This regiser can be used to discard up to 7 samples after conversion start. Note that the skipped samples will be lost."]
    #[inline(always)]
    pub fn intdly(&self) -> IntdlyR {
        IntdlyR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 7 - Data alignment"]
    #[inline(always)]
    pub fn dalgn(&self) -> DalgnR {
        DalgnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Data format"]
    #[inline(always)]
    pub fn dfmsel(&self) -> DfmselR {
        DfmselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Output Bit Resolution"]
    #[inline(always)]
    pub fn obr(&self) -> ObrR {
        ObrR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - MSB Shift"]
    #[inline(always)]
    pub fn shift(&self) -> ShiftR {
        ShiftR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - SDHS trigger source select."]
    #[inline(always)]
    pub fn trgsrc(&self) -> TrgsrcR {
        TrgsrcR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDHS Auto Sample Start Disable"]
    #[inline(always)]
    pub fn autossdis(&mut self) -> AutossdisW<'_, Sdhsctl0Spec> {
        AutossdisW::new(self, 0)
    }
    #[doc = "Bits 1:3 - DTRDY Interrupt delay select. This regiser can be used to discard up to 7 samples after conversion start. Note that the skipped samples will be lost."]
    #[inline(always)]
    pub fn intdly(&mut self) -> IntdlyW<'_, Sdhsctl0Spec> {
        IntdlyW::new(self, 1)
    }
    #[doc = "Bit 7 - Data alignment"]
    #[inline(always)]
    pub fn dalgn(&mut self) -> DalgnW<'_, Sdhsctl0Spec> {
        DalgnW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Data format"]
    #[inline(always)]
    pub fn dfmsel(&mut self) -> DfmselW<'_, Sdhsctl0Spec> {
        DfmselW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Output Bit Resolution"]
    #[inline(always)]
    pub fn obr(&mut self) -> ObrW<'_, Sdhsctl0Spec> {
        ObrW::new(self, 10)
    }
    #[doc = "Bits 12:13 - MSB Shift"]
    #[inline(always)]
    pub fn shift(&mut self) -> ShiftW<'_, Sdhsctl0Spec> {
        ShiftW::new(self, 12)
    }
    #[doc = "Bit 15 - SDHS trigger source select."]
    #[inline(always)]
    pub fn trgsrc(&mut self) -> TrgsrcW<'_, Sdhsctl0Spec> {
        TrgsrcW::new(self, 15)
    }
}
#[doc = "SDHS Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdhsctl0Spec;
impl crate::RegisterSpec for Sdhsctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsctl0::R`](R) reader structure"]
impl crate::Readable for Sdhsctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`sdhsctl0::W`](W) writer structure"]
impl crate::Writable for Sdhsctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSCTL0 to value 0"]
impl crate::Resettable for Sdhsctl0Spec {}
