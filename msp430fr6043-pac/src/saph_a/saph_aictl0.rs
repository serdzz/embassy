#[doc = "Register `SAPH_AICTL0` reader"]
pub type R = crate::R<SaphAictl0Spec>;
#[doc = "Register `SAPH_AICTL0` writer"]
pub type W = crate::W<SaphAictl0Spec>;
#[doc = "Input Multiplexer Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Muxsel {
    #[doc = "0: Channel 0 is selected for input"]
    Ch0in = 0,
    #[doc = "1: Channel 1 is selected for input"]
    Ch1in = 1,
    #[doc = "2: reserved for future channels"]
    Muxsel2 = 2,
    #[doc = "3: reserved for future channels"]
    Muxsel3 = 3,
    #[doc = "4: reserved for future channels"]
    Muxsel4 = 4,
    #[doc = "5: reserved for future channels"]
    Muxsel5 = 5,
    #[doc = "6: reserved for future channels"]
    Muxsel6 = 6,
    #[doc = "7: reserved for future channels"]
    Muxsel7 = 7,
    #[doc = "8: no channel is selected"]
    Muxsel8 = 8,
    #[doc = "9: no channel is selected"]
    Muxsel9 = 9,
    #[doc = "10: no channel is selected"]
    Muxsel10 = 10,
    #[doc = "11: no channel is selected"]
    Muxsel11 = 11,
    #[doc = "12: no channel is selected"]
    Muxsel12 = 12,
    #[doc = "13: no channel is selected"]
    Muxsel13 = 13,
    #[doc = "14: no channel is selected"]
    Muxsel14 = 14,
    #[doc = "15: no channel is selected"]
    Muxsel15 = 15,
}
impl From<Muxsel> for u8 {
    #[inline(always)]
    fn from(variant: Muxsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Muxsel {
    type Ux = u8;
}
impl crate::IsEnum for Muxsel {}
#[doc = "Field `MUXSEL` reader - Input Multiplexer Channel Select"]
pub type MuxselR = crate::FieldReader<Muxsel>;
impl MuxselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Muxsel {
        match self.bits {
            0 => Muxsel::Ch0in,
            1 => Muxsel::Ch1in,
            2 => Muxsel::Muxsel2,
            3 => Muxsel::Muxsel3,
            4 => Muxsel::Muxsel4,
            5 => Muxsel::Muxsel5,
            6 => Muxsel::Muxsel6,
            7 => Muxsel::Muxsel7,
            8 => Muxsel::Muxsel8,
            9 => Muxsel::Muxsel9,
            10 => Muxsel::Muxsel10,
            11 => Muxsel::Muxsel11,
            12 => Muxsel::Muxsel12,
            13 => Muxsel::Muxsel13,
            14 => Muxsel::Muxsel14,
            15 => Muxsel::Muxsel15,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 0 is selected for input"]
    #[inline(always)]
    pub fn is_ch0in(&self) -> bool {
        *self == Muxsel::Ch0in
    }
    #[doc = "Channel 1 is selected for input"]
    #[inline(always)]
    pub fn is_ch1in(&self) -> bool {
        *self == Muxsel::Ch1in
    }
    #[doc = "reserved for future channels"]
    #[inline(always)]
    pub fn is_muxsel_2(&self) -> bool {
        *self == Muxsel::Muxsel2
    }
    #[doc = "reserved for future channels"]
    #[inline(always)]
    pub fn is_muxsel_3(&self) -> bool {
        *self == Muxsel::Muxsel3
    }
    #[doc = "reserved for future channels"]
    #[inline(always)]
    pub fn is_muxsel_4(&self) -> bool {
        *self == Muxsel::Muxsel4
    }
    #[doc = "reserved for future channels"]
    #[inline(always)]
    pub fn is_muxsel_5(&self) -> bool {
        *self == Muxsel::Muxsel5
    }
    #[doc = "reserved for future channels"]
    #[inline(always)]
    pub fn is_muxsel_6(&self) -> bool {
        *self == Muxsel::Muxsel6
    }
    #[doc = "reserved for future channels"]
    #[inline(always)]
    pub fn is_muxsel_7(&self) -> bool {
        *self == Muxsel::Muxsel7
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn is_muxsel_8(&self) -> bool {
        *self == Muxsel::Muxsel8
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn is_muxsel_9(&self) -> bool {
        *self == Muxsel::Muxsel9
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn is_muxsel_10(&self) -> bool {
        *self == Muxsel::Muxsel10
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn is_muxsel_11(&self) -> bool {
        *self == Muxsel::Muxsel11
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn is_muxsel_12(&self) -> bool {
        *self == Muxsel::Muxsel12
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn is_muxsel_13(&self) -> bool {
        *self == Muxsel::Muxsel13
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn is_muxsel_14(&self) -> bool {
        *self == Muxsel::Muxsel14
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn is_muxsel_15(&self) -> bool {
        *self == Muxsel::Muxsel15
    }
}
#[doc = "Field `MUXSEL` writer - Input Multiplexer Channel Select"]
pub type MuxselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Muxsel, crate::Safe>;
impl<'a, REG> MuxselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 is selected for input"]
    #[inline(always)]
    pub fn ch0in(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Ch0in)
    }
    #[doc = "Channel 1 is selected for input"]
    #[inline(always)]
    pub fn ch1in(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Ch1in)
    }
    #[doc = "reserved for future channels"]
    #[inline(always)]
    pub fn muxsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Muxsel2)
    }
    #[doc = "reserved for future channels"]
    #[inline(always)]
    pub fn muxsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Muxsel3)
    }
    #[doc = "reserved for future channels"]
    #[inline(always)]
    pub fn muxsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Muxsel4)
    }
    #[doc = "reserved for future channels"]
    #[inline(always)]
    pub fn muxsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Muxsel5)
    }
    #[doc = "reserved for future channels"]
    #[inline(always)]
    pub fn muxsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Muxsel6)
    }
    #[doc = "reserved for future channels"]
    #[inline(always)]
    pub fn muxsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Muxsel7)
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn muxsel_8(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Muxsel8)
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn muxsel_9(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Muxsel9)
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn muxsel_10(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Muxsel10)
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn muxsel_11(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Muxsel11)
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn muxsel_12(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Muxsel12)
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn muxsel_13(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Muxsel13)
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn muxsel_14(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Muxsel14)
    }
    #[doc = "no channel is selected"]
    #[inline(always)]
    pub fn muxsel_15(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::Muxsel15)
    }
}
#[doc = "Input Multiplexer Control source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Muxctl {
    #[doc = "0: The input multiplexer is controlled by ICTL0.MUXSEL (register mode)"]
    Muxctl0 = 0,
    #[doc = "1: The input multiplexer is controlled by ASQ (auto mode)"]
    Muxctl1 = 1,
}
impl From<Muxctl> for bool {
    #[inline(always)]
    fn from(variant: Muxctl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUXCTL` reader - Input Multiplexer Control source"]
pub type MuxctlR = crate::BitReader<Muxctl>;
impl MuxctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Muxctl {
        match self.bits {
            false => Muxctl::Muxctl0,
            true => Muxctl::Muxctl1,
        }
    }
    #[doc = "The input multiplexer is controlled by ICTL0.MUXSEL (register mode)"]
    #[inline(always)]
    pub fn is_muxctl_0(&self) -> bool {
        *self == Muxctl::Muxctl0
    }
    #[doc = "The input multiplexer is controlled by ASQ (auto mode)"]
    #[inline(always)]
    pub fn is_muxctl_1(&self) -> bool {
        *self == Muxctl::Muxctl1
    }
}
#[doc = "Field `MUXCTL` writer - Input Multiplexer Control source"]
pub type MuxctlW<'a, REG> = crate::BitWriter<'a, REG, Muxctl>;
impl<'a, REG> MuxctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input multiplexer is controlled by ICTL0.MUXSEL (register mode)"]
    #[inline(always)]
    pub fn muxctl_0(self) -> &'a mut crate::W<REG> {
        self.variant(Muxctl::Muxctl0)
    }
    #[doc = "The input multiplexer is controlled by ASQ (auto mode)"]
    #[inline(always)]
    pub fn muxctl_1(self) -> &'a mut crate::W<REG> {
        self.variant(Muxctl::Muxctl1)
    }
}
#[doc = "PGA dummy load enable on the deselected multiplexer inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dumen {
    #[doc = "0: PGA dummy input load is Hi-Z."]
    Dumen0 = 0,
    #[doc = "1: PGA dummy input load matches the PGA input impedance."]
    Dumen1 = 1,
}
impl From<Dumen> for bool {
    #[inline(always)]
    fn from(variant: Dumen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUMEN` reader - PGA dummy load enable on the deselected multiplexer inputs."]
pub type DumenR = crate::BitReader<Dumen>;
impl DumenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dumen {
        match self.bits {
            false => Dumen::Dumen0,
            true => Dumen::Dumen1,
        }
    }
    #[doc = "PGA dummy input load is Hi-Z."]
    #[inline(always)]
    pub fn is_dumen_0(&self) -> bool {
        *self == Dumen::Dumen0
    }
    #[doc = "PGA dummy input load matches the PGA input impedance."]
    #[inline(always)]
    pub fn is_dumen_1(&self) -> bool {
        *self == Dumen::Dumen1
    }
}
#[doc = "Field `DUMEN` writer - PGA dummy load enable on the deselected multiplexer inputs."]
pub type DumenW<'a, REG> = crate::BitWriter<'a, REG, Dumen>;
impl<'a, REG> DumenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PGA dummy input load is Hi-Z."]
    #[inline(always)]
    pub fn dumen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dumen::Dumen0)
    }
    #[doc = "PGA dummy input load matches the PGA input impedance."]
    #[inline(always)]
    pub fn dumen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dumen::Dumen1)
    }
}
#[doc = "XPB0 Pin Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xpb0fen {
    #[doc = "0: XPB0 pin function disabled"]
    Xpb0fen0 = 0,
    #[doc = "1: XPB0 pin function enabled"]
    Xpb0fen1 = 1,
}
impl From<Xpb0fen> for bool {
    #[inline(always)]
    fn from(variant: Xpb0fen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XPB0FEN` reader - XPB0 Pin Function Enable"]
pub type Xpb0fenR = crate::BitReader<Xpb0fen>;
impl Xpb0fenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xpb0fen {
        match self.bits {
            false => Xpb0fen::Xpb0fen0,
            true => Xpb0fen::Xpb0fen1,
        }
    }
    #[doc = "XPB0 pin function disabled"]
    #[inline(always)]
    pub fn is_xpb0fen_0(&self) -> bool {
        *self == Xpb0fen::Xpb0fen0
    }
    #[doc = "XPB0 pin function enabled"]
    #[inline(always)]
    pub fn is_xpb0fen_1(&self) -> bool {
        *self == Xpb0fen::Xpb0fen1
    }
}
#[doc = "Field `XPB0FEN` writer - XPB0 Pin Function Enable"]
pub type Xpb0fenW<'a, REG> = crate::BitWriter<'a, REG, Xpb0fen>;
impl<'a, REG> Xpb0fenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "XPB0 pin function disabled"]
    #[inline(always)]
    pub fn xpb0fen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xpb0fen::Xpb0fen0)
    }
    #[doc = "XPB0 pin function enabled"]
    #[inline(always)]
    pub fn xpb0fen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xpb0fen::Xpb0fen1)
    }
}
#[doc = "XPB1 Pin Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xpb1fen {
    #[doc = "0: XPB1 pin function disabled"]
    Xpb1fen0 = 0,
    #[doc = "1: XPB1 pin function enabled"]
    Xpb1fen1 = 1,
}
impl From<Xpb1fen> for bool {
    #[inline(always)]
    fn from(variant: Xpb1fen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XPB1FEN` reader - XPB1 Pin Function Enable"]
pub type Xpb1fenR = crate::BitReader<Xpb1fen>;
impl Xpb1fenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xpb1fen {
        match self.bits {
            false => Xpb1fen::Xpb1fen0,
            true => Xpb1fen::Xpb1fen1,
        }
    }
    #[doc = "XPB1 pin function disabled"]
    #[inline(always)]
    pub fn is_xpb1fen_0(&self) -> bool {
        *self == Xpb1fen::Xpb1fen0
    }
    #[doc = "XPB1 pin function enabled"]
    #[inline(always)]
    pub fn is_xpb1fen_1(&self) -> bool {
        *self == Xpb1fen::Xpb1fen1
    }
}
#[doc = "Field `XPB1FEN` writer - XPB1 Pin Function Enable"]
pub type Xpb1fenW<'a, REG> = crate::BitWriter<'a, REG, Xpb1fen>;
impl<'a, REG> Xpb1fenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "XPB1 pin function disabled"]
    #[inline(always)]
    pub fn xpb1fen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xpb1fen::Xpb1fen0)
    }
    #[doc = "XPB1 pin function enabled"]
    #[inline(always)]
    pub fn xpb1fen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xpb1fen::Xpb1fen1)
    }
}
#[doc = "External PGA Bias Switch 0 control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xpbsw0 {
    #[doc = "0: External bias switch is open (no bias driven)"]
    Xpbsw0_0 = 0,
    #[doc = "1: External bias switch is closed (bias is driven)"]
    Xpbsw0_1 = 1,
}
impl From<Xpbsw0> for bool {
    #[inline(always)]
    fn from(variant: Xpbsw0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XPBSW0` reader - External PGA Bias Switch 0 control"]
pub type Xpbsw0R = crate::BitReader<Xpbsw0>;
impl Xpbsw0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xpbsw0 {
        match self.bits {
            false => Xpbsw0::Xpbsw0_0,
            true => Xpbsw0::Xpbsw0_1,
        }
    }
    #[doc = "External bias switch is open (no bias driven)"]
    #[inline(always)]
    pub fn is_xpbsw0_0(&self) -> bool {
        *self == Xpbsw0::Xpbsw0_0
    }
    #[doc = "External bias switch is closed (bias is driven)"]
    #[inline(always)]
    pub fn is_xpbsw0_1(&self) -> bool {
        *self == Xpbsw0::Xpbsw0_1
    }
}
#[doc = "Field `XPBSW0` writer - External PGA Bias Switch 0 control"]
pub type Xpbsw0W<'a, REG> = crate::BitWriter<'a, REG, Xpbsw0>;
impl<'a, REG> Xpbsw0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External bias switch is open (no bias driven)"]
    #[inline(always)]
    pub fn xpbsw0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xpbsw0::Xpbsw0_0)
    }
    #[doc = "External bias switch is closed (bias is driven)"]
    #[inline(always)]
    pub fn xpbsw0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xpbsw0::Xpbsw0_1)
    }
}
#[doc = "External PGA Bias Switch 1 control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xpbsw1 {
    #[doc = "0: External bias switch is open (no bias driven)"]
    Xpbsw1_0 = 0,
    #[doc = "1: External bias switch is closed (bias is driven)"]
    Xpbsw1_1 = 1,
}
impl From<Xpbsw1> for bool {
    #[inline(always)]
    fn from(variant: Xpbsw1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XPBSW1` reader - External PGA Bias Switch 1 control"]
pub type Xpbsw1R = crate::BitReader<Xpbsw1>;
impl Xpbsw1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xpbsw1 {
        match self.bits {
            false => Xpbsw1::Xpbsw1_0,
            true => Xpbsw1::Xpbsw1_1,
        }
    }
    #[doc = "External bias switch is open (no bias driven)"]
    #[inline(always)]
    pub fn is_xpbsw1_0(&self) -> bool {
        *self == Xpbsw1::Xpbsw1_0
    }
    #[doc = "External bias switch is closed (bias is driven)"]
    #[inline(always)]
    pub fn is_xpbsw1_1(&self) -> bool {
        *self == Xpbsw1::Xpbsw1_1
    }
}
#[doc = "Field `XPBSW1` writer - External PGA Bias Switch 1 control"]
pub type Xpbsw1W<'a, REG> = crate::BitWriter<'a, REG, Xpbsw1>;
impl<'a, REG> Xpbsw1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External bias switch is open (no bias driven)"]
    #[inline(always)]
    pub fn xpbsw1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xpbsw1::Xpbsw1_0)
    }
    #[doc = "External bias switch is closed (bias is driven)"]
    #[inline(always)]
    pub fn xpbsw1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xpbsw1::Xpbsw1_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Input Multiplexer Channel Select"]
    #[inline(always)]
    pub fn muxsel(&self) -> MuxselR {
        MuxselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Input Multiplexer Control source"]
    #[inline(always)]
    pub fn muxctl(&self) -> MuxctlR {
        MuxctlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - PGA dummy load enable on the deselected multiplexer inputs."]
    #[inline(always)]
    pub fn dumen(&self) -> DumenR {
        DumenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - XPB0 Pin Function Enable"]
    #[inline(always)]
    pub fn xpb0fen(&self) -> Xpb0fenR {
        Xpb0fenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - XPB1 Pin Function Enable"]
    #[inline(always)]
    pub fn xpb1fen(&self) -> Xpb1fenR {
        Xpb1fenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - External PGA Bias Switch 0 control"]
    #[inline(always)]
    pub fn xpbsw0(&self) -> Xpbsw0R {
        Xpbsw0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External PGA Bias Switch 1 control"]
    #[inline(always)]
    pub fn xpbsw1(&self) -> Xpbsw1R {
        Xpbsw1R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input Multiplexer Channel Select"]
    #[inline(always)]
    pub fn muxsel(&mut self) -> MuxselW<'_, SaphAictl0Spec> {
        MuxselW::new(self, 0)
    }
    #[doc = "Bit 4 - Input Multiplexer Control source"]
    #[inline(always)]
    pub fn muxctl(&mut self) -> MuxctlW<'_, SaphAictl0Spec> {
        MuxctlW::new(self, 4)
    }
    #[doc = "Bit 7 - PGA dummy load enable on the deselected multiplexer inputs."]
    #[inline(always)]
    pub fn dumen(&mut self) -> DumenW<'_, SaphAictl0Spec> {
        DumenW::new(self, 7)
    }
    #[doc = "Bit 8 - XPB0 Pin Function Enable"]
    #[inline(always)]
    pub fn xpb0fen(&mut self) -> Xpb0fenW<'_, SaphAictl0Spec> {
        Xpb0fenW::new(self, 8)
    }
    #[doc = "Bit 9 - XPB1 Pin Function Enable"]
    #[inline(always)]
    pub fn xpb1fen(&mut self) -> Xpb1fenW<'_, SaphAictl0Spec> {
        Xpb1fenW::new(self, 9)
    }
    #[doc = "Bit 12 - External PGA Bias Switch 0 control"]
    #[inline(always)]
    pub fn xpbsw0(&mut self) -> Xpbsw0W<'_, SaphAictl0Spec> {
        Xpbsw0W::new(self, 12)
    }
    #[doc = "Bit 13 - External PGA Bias Switch 1 control"]
    #[inline(always)]
    pub fn xpbsw1(&mut self) -> Xpbsw1W<'_, SaphAictl0Spec> {
        Xpbsw1W::new(self, 13)
    }
}
#[doc = "Physical Interface Input Control #0\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aictl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aictl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAictl0Spec;
impl crate::RegisterSpec for SaphAictl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aictl0::R`](R) reader structure"]
impl crate::Readable for SaphAictl0Spec {}
#[doc = "`write(|w| ..)` method takes [`saph_aictl0::W`](W) writer structure"]
impl crate::Writable for SaphAictl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AICTL0 to value 0"]
impl crate::Resettable for SaphAictl0Spec {}
