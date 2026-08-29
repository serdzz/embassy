#[doc = "Register `UUPSCTL` reader"]
pub type R = crate::R<UupsctlSpec>;
#[doc = "Register `UUPSCTL` writer"]
pub type W = crate::W<UupsctlSpec>;
#[doc = "USS LDO is ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ldordy {
    #[doc = "0: USS LDO is powered down or in transition mode"]
    Ldordy0 = 0,
    #[doc = "1: USS LDO is powered on"]
    Ldordy1 = 1,
}
impl From<Ldordy> for bool {
    #[inline(always)]
    fn from(variant: Ldordy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDORDY` reader - USS LDO is ready"]
pub type LdordyR = crate::BitReader<Ldordy>;
impl LdordyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldordy {
        match self.bits {
            false => Ldordy::Ldordy0,
            true => Ldordy::Ldordy1,
        }
    }
    #[doc = "USS LDO is powered down or in transition mode"]
    #[inline(always)]
    pub fn is_ldordy_0(&self) -> bool {
        *self == Ldordy::Ldordy0
    }
    #[doc = "USS LDO is powered on"]
    #[inline(always)]
    pub fn is_ldordy_1(&self) -> bool {
        *self == Ldordy::Ldordy1
    }
}
#[doc = "USS Power Up trigger source select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Upstate {
    #[doc = "0: USS is in OFF mode"]
    Upstate0 = 0,
    #[doc = "1: USS is in STANDBY mode"]
    Upstate1 = 1,
    #[doc = "2: USS power mode is in transition."]
    Upstate2 = 2,
    #[doc = "3: USS is in READY mode"]
    Upstate3 = 3,
}
impl From<Upstate> for u8 {
    #[inline(always)]
    fn from(variant: Upstate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Upstate {
    type Ux = u8;
}
impl crate::IsEnum for Upstate {}
#[doc = "Field `UPSTATE` reader - USS Power Up trigger source select."]
pub type UpstateR = crate::FieldReader<Upstate>;
impl UpstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Upstate {
        match self.bits {
            0 => Upstate::Upstate0,
            1 => Upstate::Upstate1,
            2 => Upstate::Upstate2,
            3 => Upstate::Upstate3,
            _ => unreachable!(),
        }
    }
    #[doc = "USS is in OFF mode"]
    #[inline(always)]
    pub fn is_upstate_0(&self) -> bool {
        *self == Upstate::Upstate0
    }
    #[doc = "USS is in STANDBY mode"]
    #[inline(always)]
    pub fn is_upstate_1(&self) -> bool {
        *self == Upstate::Upstate1
    }
    #[doc = "USS power mode is in transition."]
    #[inline(always)]
    pub fn is_upstate_2(&self) -> bool {
        *self == Upstate::Upstate2
    }
    #[doc = "USS is in READY mode"]
    #[inline(always)]
    pub fn is_upstate_3(&self) -> bool {
        *self == Upstate::Upstate3
    }
}
#[doc = "USS Busy bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UssBusy {
    #[doc = "0: The USS module is not busy."]
    UssBusy0 = 0,
    #[doc = "1: The USS module is busy."]
    UssBusy1 = 1,
}
impl From<UssBusy> for bool {
    #[inline(always)]
    fn from(variant: UssBusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USS_BUSY` reader - USS Busy bit."]
pub type UssBusyR = crate::BitReader<UssBusy>;
impl UssBusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UssBusy {
        match self.bits {
            false => UssBusy::UssBusy0,
            true => UssBusy::UssBusy1,
        }
    }
    #[doc = "The USS module is not busy."]
    #[inline(always)]
    pub fn is_uss_busy_0(&self) -> bool {
        *self == UssBusy::UssBusy0
    }
    #[doc = "The USS module is busy."]
    #[inline(always)]
    pub fn is_uss_busy_1(&self) -> bool {
        *self == UssBusy::UssBusy1
    }
}
#[doc = "Software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ussswrst {
    #[doc = "0: Disabled. USS (and sub modules) reset released for operation"]
    Ussswrst0 = 0,
    #[doc = "1: Enabled. USS (and sub modules) logic held in reset state"]
    Ussswrst1 = 1,
}
impl From<Ussswrst> for bool {
    #[inline(always)]
    fn from(variant: Ussswrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USSSWRST` reader - Software reset"]
pub type UssswrstR = crate::BitReader<Ussswrst>;
impl UssswrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ussswrst {
        match self.bits {
            false => Ussswrst::Ussswrst0,
            true => Ussswrst::Ussswrst1,
        }
    }
    #[doc = "Disabled. USS (and sub modules) reset released for operation"]
    #[inline(always)]
    pub fn is_ussswrst_0(&self) -> bool {
        *self == Ussswrst::Ussswrst0
    }
    #[doc = "Enabled. USS (and sub modules) logic held in reset state"]
    #[inline(always)]
    pub fn is_ussswrst_1(&self) -> bool {
        *self == Ussswrst::Ussswrst1
    }
}
#[doc = "Field `USSSWRST` writer - Software reset"]
pub type UssswrstW<'a, REG> = crate::BitWriter<'a, REG, Ussswrst>;
impl<'a, REG> UssswrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. USS (and sub modules) reset released for operation"]
    #[inline(always)]
    pub fn ussswrst_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ussswrst::Ussswrst0)
    }
    #[doc = "Enabled. USS (and sub modules) logic held in reset state"]
    #[inline(always)]
    pub fn ussswrst_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ussswrst::Ussswrst1)
    }
}
#[doc = "Turn on USS Power and PLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usspwrup {
    #[doc = "0: No action"]
    Usspwrup0 = 0,
    #[doc = "1: Power up the USS module and generate the PSQ_START to the ASQ if CTL.ASQEN = 1. Note: This bit becomes invalid in debug mode."]
    Usspwrup1 = 1,
}
impl From<Usspwrup> for bool {
    #[inline(always)]
    fn from(variant: Usspwrup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USSPWRUP` reader - Turn on USS Power and PLL"]
pub type UsspwrupR = crate::BitReader<Usspwrup>;
impl UsspwrupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usspwrup {
        match self.bits {
            false => Usspwrup::Usspwrup0,
            true => Usspwrup::Usspwrup1,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_usspwrup_0(&self) -> bool {
        *self == Usspwrup::Usspwrup0
    }
    #[doc = "Power up the USS module and generate the PSQ_START to the ASQ if CTL.ASQEN = 1. Note: This bit becomes invalid in debug mode."]
    #[inline(always)]
    pub fn is_usspwrup_1(&self) -> bool {
        *self == Usspwrup::Usspwrup1
    }
}
#[doc = "Field `USSPWRUP` writer - Turn on USS Power and PLL"]
pub type UsspwrupW<'a, REG> = crate::BitWriter<'a, REG, Usspwrup>;
impl<'a, REG> UsspwrupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn usspwrup_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usspwrup::Usspwrup0)
    }
    #[doc = "Power up the USS module and generate the PSQ_START to the ASQ if CTL.ASQEN = 1. Note: This bit becomes invalid in debug mode."]
    #[inline(always)]
    pub fn usspwrup_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usspwrup::Usspwrup1)
    }
}
#[doc = "USS Power Up trigger source select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usspwrupsel {
    #[doc = "0: CTL.USSPWRUP bit"]
    Usspwrupsel0 = 0,
    #[doc = "1: Ext. trigger (see the device specific datasheet)"]
    Usspwrupsel1 = 1,
    #[doc = "2: Ext. trigger (see the device-specific data sheet)"]
    Usspwrupsel2 = 2,
    #[doc = "3: Ext. trigger (see the device-specific data sheet)"]
    Usspwrupsel3 = 3,
}
impl From<Usspwrupsel> for u8 {
    #[inline(always)]
    fn from(variant: Usspwrupsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usspwrupsel {
    type Ux = u8;
}
impl crate::IsEnum for Usspwrupsel {}
#[doc = "Field `USSPWRUPSEL` reader - USS Power Up trigger source select."]
pub type UsspwrupselR = crate::FieldReader<Usspwrupsel>;
impl UsspwrupselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usspwrupsel {
        match self.bits {
            0 => Usspwrupsel::Usspwrupsel0,
            1 => Usspwrupsel::Usspwrupsel1,
            2 => Usspwrupsel::Usspwrupsel2,
            3 => Usspwrupsel::Usspwrupsel3,
            _ => unreachable!(),
        }
    }
    #[doc = "CTL.USSPWRUP bit"]
    #[inline(always)]
    pub fn is_usspwrupsel_0(&self) -> bool {
        *self == Usspwrupsel::Usspwrupsel0
    }
    #[doc = "Ext. trigger (see the device specific datasheet)"]
    #[inline(always)]
    pub fn is_usspwrupsel_1(&self) -> bool {
        *self == Usspwrupsel::Usspwrupsel1
    }
    #[doc = "Ext. trigger (see the device-specific data sheet)"]
    #[inline(always)]
    pub fn is_usspwrupsel_2(&self) -> bool {
        *self == Usspwrupsel::Usspwrupsel2
    }
    #[doc = "Ext. trigger (see the device-specific data sheet)"]
    #[inline(always)]
    pub fn is_usspwrupsel_3(&self) -> bool {
        *self == Usspwrupsel::Usspwrupsel3
    }
}
#[doc = "Field `USSPWRUPSEL` writer - USS Power Up trigger source select."]
pub type UsspwrupselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usspwrupsel, crate::Safe>;
impl<'a, REG> UsspwrupselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CTL.USSPWRUP bit"]
    #[inline(always)]
    pub fn usspwrupsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usspwrupsel::Usspwrupsel0)
    }
    #[doc = "Ext. trigger (see the device specific datasheet)"]
    #[inline(always)]
    pub fn usspwrupsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usspwrupsel::Usspwrupsel1)
    }
    #[doc = "Ext. trigger (see the device-specific data sheet)"]
    #[inline(always)]
    pub fn usspwrupsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Usspwrupsel::Usspwrupsel2)
    }
    #[doc = "Ext. trigger (see the device-specific data sheet)"]
    #[inline(always)]
    pub fn usspwrupsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Usspwrupsel::Usspwrupsel3)
    }
}
#[doc = "Power Ready Output Event Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asqen {
    #[doc = "0: Do not generate the PSQ_START signal event to ASQ."]
    Asqen0 = 0,
    #[doc = "1: Generate the PSQ_START signal event to the ASQ."]
    Asqen1 = 1,
}
impl From<Asqen> for bool {
    #[inline(always)]
    fn from(variant: Asqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASQEN` reader - Power Ready Output Event Select"]
pub type AsqenR = crate::BitReader<Asqen>;
impl AsqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asqen {
        match self.bits {
            false => Asqen::Asqen0,
            true => Asqen::Asqen1,
        }
    }
    #[doc = "Do not generate the PSQ_START signal event to ASQ."]
    #[inline(always)]
    pub fn is_asqen_0(&self) -> bool {
        *self == Asqen::Asqen0
    }
    #[doc = "Generate the PSQ_START signal event to the ASQ."]
    #[inline(always)]
    pub fn is_asqen_1(&self) -> bool {
        *self == Asqen::Asqen1
    }
}
#[doc = "Field `ASQEN` writer - Power Ready Output Event Select"]
pub type AsqenW<'a, REG> = crate::BitWriter<'a, REG, Asqen>;
impl<'a, REG> AsqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not generate the PSQ_START signal event to ASQ."]
    #[inline(always)]
    pub fn asqen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Asqen::Asqen0)
    }
    #[doc = "Generate the PSQ_START signal event to the ASQ."]
    #[inline(always)]
    pub fn asqen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Asqen::Asqen1)
    }
}
#[doc = "Reserved_2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lbhdel {
    #[doc = "0: no additional delay"]
    Lbhdel0 = 0,
    #[doc = "1: additional hold off delay of ~100us (512 REFCLKs)"]
    Lbhdel1 = 1,
    #[doc = "2: additional hold off delay of ~200us (1024 REFCLKs)"]
    Lbhdel2 = 2,
    #[doc = "3: additional hold off delay of ~300us (1536 REFCLKs)"]
    Lbhdel3 = 3,
}
impl From<Lbhdel> for u8 {
    #[inline(always)]
    fn from(variant: Lbhdel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lbhdel {
    type Ux = u8;
}
impl crate::IsEnum for Lbhdel {}
#[doc = "Field `LBHDEL` reader - Reserved_2"]
pub type LbhdelR = crate::FieldReader<Lbhdel>;
impl LbhdelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbhdel {
        match self.bits {
            0 => Lbhdel::Lbhdel0,
            1 => Lbhdel::Lbhdel1,
            2 => Lbhdel::Lbhdel2,
            3 => Lbhdel::Lbhdel3,
            _ => unreachable!(),
        }
    }
    #[doc = "no additional delay"]
    #[inline(always)]
    pub fn is_lbhdel_0(&self) -> bool {
        *self == Lbhdel::Lbhdel0
    }
    #[doc = "additional hold off delay of ~100us (512 REFCLKs)"]
    #[inline(always)]
    pub fn is_lbhdel_1(&self) -> bool {
        *self == Lbhdel::Lbhdel1
    }
    #[doc = "additional hold off delay of ~200us (1024 REFCLKs)"]
    #[inline(always)]
    pub fn is_lbhdel_2(&self) -> bool {
        *self == Lbhdel::Lbhdel2
    }
    #[doc = "additional hold off delay of ~300us (1536 REFCLKs)"]
    #[inline(always)]
    pub fn is_lbhdel_3(&self) -> bool {
        *self == Lbhdel::Lbhdel3
    }
}
#[doc = "Field `LBHDEL` writer - Reserved_2"]
pub type LbhdelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lbhdel, crate::Safe>;
impl<'a, REG> LbhdelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no additional delay"]
    #[inline(always)]
    pub fn lbhdel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lbhdel::Lbhdel0)
    }
    #[doc = "additional hold off delay of ~100us (512 REFCLKs)"]
    #[inline(always)]
    pub fn lbhdel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lbhdel::Lbhdel1)
    }
    #[doc = "additional hold off delay of ~200us (1024 REFCLKs)"]
    #[inline(always)]
    pub fn lbhdel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Lbhdel::Lbhdel2)
    }
    #[doc = "additional hold off delay of ~300us (1536 REFCLKs)"]
    #[inline(always)]
    pub fn lbhdel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Lbhdel::Lbhdel3)
    }
}
#[doc = "USS Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usspwrdn {
    #[doc = "0: No action"]
    Usspwrdn0 = 0,
    #[doc = "1: Stop the current measurement and power off the USS module."]
    Usspwrdn1 = 1,
}
impl From<Usspwrdn> for bool {
    #[inline(always)]
    fn from(variant: Usspwrdn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USSPWRDN` reader - USS Power Down"]
pub type UsspwrdnR = crate::BitReader<Usspwrdn>;
impl UsspwrdnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usspwrdn {
        match self.bits {
            false => Usspwrdn::Usspwrdn0,
            true => Usspwrdn::Usspwrdn1,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_usspwrdn_0(&self) -> bool {
        *self == Usspwrdn::Usspwrdn0
    }
    #[doc = "Stop the current measurement and power off the USS module."]
    #[inline(always)]
    pub fn is_usspwrdn_1(&self) -> bool {
        *self == Usspwrdn::Usspwrdn1
    }
}
#[doc = "Field `USSPWRDN` writer - USS Power Down"]
pub type UsspwrdnW<'a, REG> = crate::BitWriter<'a, REG, Usspwrdn>;
impl<'a, REG> UsspwrdnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn usspwrdn_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usspwrdn::Usspwrdn0)
    }
    #[doc = "Stop the current measurement and power off the USS module."]
    #[inline(always)]
    pub fn usspwrdn_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usspwrdn::Usspwrdn1)
    }
}
#[doc = "USS Measurement Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ussstop {
    #[doc = "0: No action"]
    Ussstop0 = 0,
    #[doc = "1: Stop the current measurement."]
    Ussstop1 = 1,
}
impl From<Ussstop> for bool {
    #[inline(always)]
    fn from(variant: Ussstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USSSTOP` reader - USS Measurement Stop"]
pub type UssstopR = crate::BitReader<Ussstop>;
impl UssstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ussstop {
        match self.bits {
            false => Ussstop::Ussstop0,
            true => Ussstop::Ussstop1,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_ussstop_0(&self) -> bool {
        *self == Ussstop::Ussstop0
    }
    #[doc = "Stop the current measurement."]
    #[inline(always)]
    pub fn is_ussstop_1(&self) -> bool {
        *self == Ussstop::Ussstop1
    }
}
#[doc = "Field `USSSTOP` writer - USS Measurement Stop"]
pub type UssstopW<'a, REG> = crate::BitWriter<'a, REG, Ussstop>;
impl<'a, REG> UssstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn ussstop_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ussstop::Ussstop0)
    }
    #[doc = "Stop the current measurement."]
    #[inline(always)]
    pub fn ussstop_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ussstop::Ussstop1)
    }
}
impl R {
    #[doc = "Bit 0 - USS LDO is ready"]
    #[inline(always)]
    pub fn ldordy(&self) -> LdordyR {
        LdordyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - USS Power Up trigger source select."]
    #[inline(always)]
    pub fn upstate(&self) -> UpstateR {
        UpstateR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - USS Busy bit."]
    #[inline(always)]
    pub fn uss_busy(&self) -> UssBusyR {
        UssBusyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Software reset"]
    #[inline(always)]
    pub fn ussswrst(&self) -> UssswrstR {
        UssswrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Turn on USS Power and PLL"]
    #[inline(always)]
    pub fn usspwrup(&self) -> UsspwrupR {
        UsspwrupR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - USS Power Up trigger source select."]
    #[inline(always)]
    pub fn usspwrupsel(&self) -> UsspwrupselR {
        UsspwrupselR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Power Ready Output Event Select"]
    #[inline(always)]
    pub fn asqen(&self) -> AsqenR {
        AsqenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Reserved_2"]
    #[inline(always)]
    pub fn lbhdel(&self) -> LbhdelR {
        LbhdelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - USS Power Down"]
    #[inline(always)]
    pub fn usspwrdn(&self) -> UsspwrdnR {
        UsspwrdnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - USS Measurement Stop"]
    #[inline(always)]
    pub fn ussstop(&self) -> UssstopR {
        UssstopR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Software reset"]
    #[inline(always)]
    pub fn ussswrst(&mut self) -> UssswrstW<'_, UupsctlSpec> {
        UssswrstW::new(self, 7)
    }
    #[doc = "Bit 8 - Turn on USS Power and PLL"]
    #[inline(always)]
    pub fn usspwrup(&mut self) -> UsspwrupW<'_, UupsctlSpec> {
        UsspwrupW::new(self, 8)
    }
    #[doc = "Bits 9:10 - USS Power Up trigger source select."]
    #[inline(always)]
    pub fn usspwrupsel(&mut self) -> UsspwrupselW<'_, UupsctlSpec> {
        UsspwrupselW::new(self, 9)
    }
    #[doc = "Bit 11 - Power Ready Output Event Select"]
    #[inline(always)]
    pub fn asqen(&mut self) -> AsqenW<'_, UupsctlSpec> {
        AsqenW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Reserved_2"]
    #[inline(always)]
    pub fn lbhdel(&mut self) -> LbhdelW<'_, UupsctlSpec> {
        LbhdelW::new(self, 12)
    }
    #[doc = "Bit 14 - USS Power Down"]
    #[inline(always)]
    pub fn usspwrdn(&mut self) -> UsspwrdnW<'_, UupsctlSpec> {
        UsspwrdnW::new(self, 14)
    }
    #[doc = "Bit 15 - USS Measurement Stop"]
    #[inline(always)]
    pub fn ussstop(&mut self) -> UssstopW<'_, UupsctlSpec> {
        UssstopW::new(self, 15)
    }
}
#[doc = "UUPS Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UupsctlSpec;
impl crate::RegisterSpec for UupsctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uupsctl::R`](R) reader structure"]
impl crate::Readable for UupsctlSpec {}
#[doc = "`write(|w| ..)` method takes [`uupsctl::W`](W) writer structure"]
impl crate::Writable for UupsctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UUPSCTL to value 0"]
impl crate::Resettable for UupsctlSpec {}
