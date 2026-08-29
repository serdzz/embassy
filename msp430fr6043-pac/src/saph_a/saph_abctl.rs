#[doc = "Register `SAPH_ABCTL` reader"]
pub type R = crate::R<SaphAbctlSpec>;
#[doc = "Register `SAPH_ABCTL` writer"]
pub type W = crate::W<SaphAbctlSpec>;
#[doc = "Tx bias and Rx bias switches control source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asqbsc {
    #[doc = "0: Bias switches are controlled by BCTL.CH0EBSW, BCTL.CH1EBSW, BCTL.PGABSW bits (register mode)."]
    Asqbsc0 = 0,
    #[doc = "1: Bias switches are controlled by ASQ (auto mode)"]
    Asqbsc1 = 1,
}
impl From<Asqbsc> for bool {
    #[inline(always)]
    fn from(variant: Asqbsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASQBSC` reader - Tx bias and Rx bias switches control source select"]
pub type AsqbscR = crate::BitReader<Asqbsc>;
impl AsqbscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asqbsc {
        match self.bits {
            false => Asqbsc::Asqbsc0,
            true => Asqbsc::Asqbsc1,
        }
    }
    #[doc = "Bias switches are controlled by BCTL.CH0EBSW, BCTL.CH1EBSW, BCTL.PGABSW bits (register mode)."]
    #[inline(always)]
    pub fn is_asqbsc_0(&self) -> bool {
        *self == Asqbsc::Asqbsc0
    }
    #[doc = "Bias switches are controlled by ASQ (auto mode)"]
    #[inline(always)]
    pub fn is_asqbsc_1(&self) -> bool {
        *self == Asqbsc::Asqbsc1
    }
}
#[doc = "Field `ASQBSC` writer - Tx bias and Rx bias switches control source select"]
pub type AsqbscW<'a, REG> = crate::BitWriter<'a, REG, Asqbsc>;
impl<'a, REG> AsqbscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bias switches are controlled by BCTL.CH0EBSW, BCTL.CH1EBSW, BCTL.PGABSW bits (register mode)."]
    #[inline(always)]
    pub fn asqbsc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Asqbsc::Asqbsc0)
    }
    #[doc = "Bias switches are controlled by ASQ (auto mode)"]
    #[inline(always)]
    pub fn asqbsc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Asqbsc::Asqbsc1)
    }
}
#[doc = "Rx bias (PGA bias) switch control. Note that the channel to apply the Rx bias is determined by ICTL0.MUXSEL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pgabsw {
    #[doc = "0: Rx bias switch is open."]
    Pgabsw0 = 0,
    #[doc = "1: Rx bias switch is closed (enabled)."]
    Pgabsw1 = 1,
}
impl From<Pgabsw> for bool {
    #[inline(always)]
    fn from(variant: Pgabsw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGABSW` reader - Rx bias (PGA bias) switch control. Note that the channel to apply the Rx bias is determined by ICTL0.MUXSEL."]
pub type PgabswR = crate::BitReader<Pgabsw>;
impl PgabswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pgabsw {
        match self.bits {
            false => Pgabsw::Pgabsw0,
            true => Pgabsw::Pgabsw1,
        }
    }
    #[doc = "Rx bias switch is open."]
    #[inline(always)]
    pub fn is_pgabsw_0(&self) -> bool {
        *self == Pgabsw::Pgabsw0
    }
    #[doc = "Rx bias switch is closed (enabled)."]
    #[inline(always)]
    pub fn is_pgabsw_1(&self) -> bool {
        *self == Pgabsw::Pgabsw1
    }
}
#[doc = "Field `PGABSW` writer - Rx bias (PGA bias) switch control. Note that the channel to apply the Rx bias is determined by ICTL0.MUXSEL."]
pub type PgabswW<'a, REG> = crate::BitWriter<'a, REG, Pgabsw>;
impl<'a, REG> PgabswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx bias switch is open."]
    #[inline(always)]
    pub fn pgabsw_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pgabsw::Pgabsw0)
    }
    #[doc = "Rx bias switch is closed (enabled)."]
    #[inline(always)]
    pub fn pgabsw_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pgabsw::Pgabsw1)
    }
}
#[doc = "Line input leakage compensation (LILC) enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lilc {
    #[doc = "0: LICL is disabled."]
    Lilc0 = 0,
    #[doc = "1: LICL is enabled."]
    Lilc1 = 1,
}
impl From<Lilc> for bool {
    #[inline(always)]
    fn from(variant: Lilc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LILC` reader - Line input leakage compensation (LILC) enable."]
pub type LilcR = crate::BitReader<Lilc>;
impl LilcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lilc {
        match self.bits {
            false => Lilc::Lilc0,
            true => Lilc::Lilc1,
        }
    }
    #[doc = "LICL is disabled."]
    #[inline(always)]
    pub fn is_lilc_0(&self) -> bool {
        *self == Lilc::Lilc0
    }
    #[doc = "LICL is enabled."]
    #[inline(always)]
    pub fn is_lilc_1(&self) -> bool {
        *self == Lilc::Lilc1
    }
}
#[doc = "Field `LILC` writer - Line input leakage compensation (LILC) enable."]
pub type LilcW<'a, REG> = crate::BitWriter<'a, REG, Lilc>;
impl<'a, REG> LilcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LICL is disabled."]
    #[inline(always)]
    pub fn lilc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lilc::Lilc0)
    }
    #[doc = "LICL is enabled."]
    #[inline(always)]
    pub fn lilc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lilc::Lilc1)
    }
}
#[doc = "Enable the power supply (Charge Pump) for the the input multiplexer during data acquisition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpda {
    #[doc = "0: Turn off the charge pump during data acquisition."]
    Cpda0 = 0,
    #[doc = "1: Keep turning on the charge pump during data acquisition."]
    Cpda1 = 1,
}
impl From<Cpda> for bool {
    #[inline(always)]
    fn from(variant: Cpda) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPDA` reader - Enable the power supply (Charge Pump) for the the input multiplexer during data acquisition."]
pub type CpdaR = crate::BitReader<Cpda>;
impl CpdaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpda {
        match self.bits {
            false => Cpda::Cpda0,
            true => Cpda::Cpda1,
        }
    }
    #[doc = "Turn off the charge pump during data acquisition."]
    #[inline(always)]
    pub fn is_cpda_0(&self) -> bool {
        *self == Cpda::Cpda0
    }
    #[doc = "Keep turning on the charge pump during data acquisition."]
    #[inline(always)]
    pub fn is_cpda_1(&self) -> bool {
        *self == Cpda::Cpda1
    }
}
#[doc = "Field `CPDA` writer - Enable the power supply (Charge Pump) for the the input multiplexer during data acquisition."]
pub type CpdaW<'a, REG> = crate::BitWriter<'a, REG, Cpda>;
impl<'a, REG> CpdaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Turn off the charge pump during data acquisition."]
    #[inline(always)]
    pub fn cpda_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpda::Cpda0)
    }
    #[doc = "Keep turning on the charge pump during data acquisition."]
    #[inline(always)]
    pub fn cpda_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpda::Cpda1)
    }
}
#[doc = "Excitation bias (Rx bias) Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Excbias {
    #[doc = "0: 0.2V nominal"]
    Excbias0 = 0,
    #[doc = "1: 0.3V nominal"]
    Excbias1 = 1,
    #[doc = "2: 0.4V nominal"]
    Excbias2 = 2,
    #[doc = "3: 0.6V nominal"]
    Excbias3 = 3,
}
impl From<Excbias> for u8 {
    #[inline(always)]
    fn from(variant: Excbias) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Excbias {
    type Ux = u8;
}
impl crate::IsEnum for Excbias {}
#[doc = "Field `EXCBIAS` reader - Excitation bias (Rx bias) Voltage Select"]
pub type ExcbiasR = crate::FieldReader<Excbias>;
impl ExcbiasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Excbias {
        match self.bits {
            0 => Excbias::Excbias0,
            1 => Excbias::Excbias1,
            2 => Excbias::Excbias2,
            3 => Excbias::Excbias3,
            _ => unreachable!(),
        }
    }
    #[doc = "0.2V nominal"]
    #[inline(always)]
    pub fn is_excbias_0(&self) -> bool {
        *self == Excbias::Excbias0
    }
    #[doc = "0.3V nominal"]
    #[inline(always)]
    pub fn is_excbias_1(&self) -> bool {
        *self == Excbias::Excbias1
    }
    #[doc = "0.4V nominal"]
    #[inline(always)]
    pub fn is_excbias_2(&self) -> bool {
        *self == Excbias::Excbias2
    }
    #[doc = "0.6V nominal"]
    #[inline(always)]
    pub fn is_excbias_3(&self) -> bool {
        *self == Excbias::Excbias3
    }
}
#[doc = "Field `EXCBIAS` writer - Excitation bias (Rx bias) Voltage Select"]
pub type ExcbiasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Excbias, crate::Safe>;
impl<'a, REG> ExcbiasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.2V nominal"]
    #[inline(always)]
    pub fn excbias_0(self) -> &'a mut crate::W<REG> {
        self.variant(Excbias::Excbias0)
    }
    #[doc = "0.3V nominal"]
    #[inline(always)]
    pub fn excbias_1(self) -> &'a mut crate::W<REG> {
        self.variant(Excbias::Excbias1)
    }
    #[doc = "0.4V nominal"]
    #[inline(always)]
    pub fn excbias_2(self) -> &'a mut crate::W<REG> {
        self.variant(Excbias::Excbias2)
    }
    #[doc = "0.6V nominal"]
    #[inline(always)]
    pub fn excbias_3(self) -> &'a mut crate::W<REG> {
        self.variant(Excbias::Excbias3)
    }
}
#[doc = "PGA bias (Rx bias) Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pgabias {
    #[doc = "0: 0.75V nominal"]
    Pgabias0 = 0,
    #[doc = "1: 0.8V nominal"]
    Pgabias1 = 1,
    #[doc = "2: 0.9V nominal"]
    Pgabias2 = 2,
    #[doc = "3: 0.95V nominal"]
    Pgabias3 = 3,
}
impl From<Pgabias> for u8 {
    #[inline(always)]
    fn from(variant: Pgabias) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pgabias {
    type Ux = u8;
}
impl crate::IsEnum for Pgabias {}
#[doc = "Field `PGABIAS` reader - PGA bias (Rx bias) Voltage Select"]
pub type PgabiasR = crate::FieldReader<Pgabias>;
impl PgabiasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pgabias {
        match self.bits {
            0 => Pgabias::Pgabias0,
            1 => Pgabias::Pgabias1,
            2 => Pgabias::Pgabias2,
            3 => Pgabias::Pgabias3,
            _ => unreachable!(),
        }
    }
    #[doc = "0.75V nominal"]
    #[inline(always)]
    pub fn is_pgabias_0(&self) -> bool {
        *self == Pgabias::Pgabias0
    }
    #[doc = "0.8V nominal"]
    #[inline(always)]
    pub fn is_pgabias_1(&self) -> bool {
        *self == Pgabias::Pgabias1
    }
    #[doc = "0.9V nominal"]
    #[inline(always)]
    pub fn is_pgabias_2(&self) -> bool {
        *self == Pgabias::Pgabias2
    }
    #[doc = "0.95V nominal"]
    #[inline(always)]
    pub fn is_pgabias_3(&self) -> bool {
        *self == Pgabias::Pgabias3
    }
}
#[doc = "Field `PGABIAS` writer - PGA bias (Rx bias) Voltage Select"]
pub type PgabiasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pgabias, crate::Safe>;
impl<'a, REG> PgabiasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.75V nominal"]
    #[inline(always)]
    pub fn pgabias_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pgabias::Pgabias0)
    }
    #[doc = "0.8V nominal"]
    #[inline(always)]
    pub fn pgabias_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pgabias::Pgabias1)
    }
    #[doc = "0.9V nominal"]
    #[inline(always)]
    pub fn pgabias_2(self) -> &'a mut crate::W<REG> {
        self.variant(Pgabias::Pgabias2)
    }
    #[doc = "0.95V nominal"]
    #[inline(always)]
    pub fn pgabias_3(self) -> &'a mut crate::W<REG> {
        self.variant(Pgabias::Pgabias3)
    }
}
#[doc = "Channel 0 Tx bias Switch Control. Note that the Tx bias voltage is determined by BCTL.EXCBIAS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0ebsw {
    #[doc = "0: Tx bias switch to CH0 is open."]
    Ch0ebsw0 = 0,
    #[doc = "1: Tx bias switch to CH0 is closed (enabled)."]
    Ch0ebsw1 = 1,
}
impl From<Ch0ebsw> for bool {
    #[inline(always)]
    fn from(variant: Ch0ebsw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0EBSW` reader - Channel 0 Tx bias Switch Control. Note that the Tx bias voltage is determined by BCTL.EXCBIAS."]
pub type Ch0ebswR = crate::BitReader<Ch0ebsw>;
impl Ch0ebswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0ebsw {
        match self.bits {
            false => Ch0ebsw::Ch0ebsw0,
            true => Ch0ebsw::Ch0ebsw1,
        }
    }
    #[doc = "Tx bias switch to CH0 is open."]
    #[inline(always)]
    pub fn is_ch0ebsw_0(&self) -> bool {
        *self == Ch0ebsw::Ch0ebsw0
    }
    #[doc = "Tx bias switch to CH0 is closed (enabled)."]
    #[inline(always)]
    pub fn is_ch0ebsw_1(&self) -> bool {
        *self == Ch0ebsw::Ch0ebsw1
    }
}
#[doc = "Field `CH0EBSW` writer - Channel 0 Tx bias Switch Control. Note that the Tx bias voltage is determined by BCTL.EXCBIAS."]
pub type Ch0ebswW<'a, REG> = crate::BitWriter<'a, REG, Ch0ebsw>;
impl<'a, REG> Ch0ebswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx bias switch to CH0 is open."]
    #[inline(always)]
    pub fn ch0ebsw_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0ebsw::Ch0ebsw0)
    }
    #[doc = "Tx bias switch to CH0 is closed (enabled)."]
    #[inline(always)]
    pub fn ch0ebsw_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0ebsw::Ch0ebsw1)
    }
}
#[doc = "Channel 1 Tx bias Switch Control. Note that the Tx bias voltage is determined by BCTL.EXCBIAS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1ebsw {
    #[doc = "0: Tx bias switch to CH1 is open."]
    Ch1ebsw0 = 0,
    #[doc = "1: Tx bias switch to CH1 is closed (enabled)."]
    Ch1ebsw1 = 1,
}
impl From<Ch1ebsw> for bool {
    #[inline(always)]
    fn from(variant: Ch1ebsw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1EBSW` reader - Channel 1 Tx bias Switch Control. Note that the Tx bias voltage is determined by BCTL.EXCBIAS."]
pub type Ch1ebswR = crate::BitReader<Ch1ebsw>;
impl Ch1ebswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1ebsw {
        match self.bits {
            false => Ch1ebsw::Ch1ebsw0,
            true => Ch1ebsw::Ch1ebsw1,
        }
    }
    #[doc = "Tx bias switch to CH1 is open."]
    #[inline(always)]
    pub fn is_ch1ebsw_0(&self) -> bool {
        *self == Ch1ebsw::Ch1ebsw0
    }
    #[doc = "Tx bias switch to CH1 is closed (enabled)."]
    #[inline(always)]
    pub fn is_ch1ebsw_1(&self) -> bool {
        *self == Ch1ebsw::Ch1ebsw1
    }
}
#[doc = "Field `CH1EBSW` writer - Channel 1 Tx bias Switch Control. Note that the Tx bias voltage is determined by BCTL.EXCBIAS."]
pub type Ch1ebswW<'a, REG> = crate::BitWriter<'a, REG, Ch1ebsw>;
impl<'a, REG> Ch1ebswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx bias switch to CH1 is open."]
    #[inline(always)]
    pub fn ch1ebsw_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1ebsw::Ch1ebsw0)
    }
    #[doc = "Tx bias switch to CH1 is closed (enabled)."]
    #[inline(always)]
    pub fn ch1ebsw_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1ebsw::Ch1ebsw1)
    }
}
impl R {
    #[doc = "Bit 0 - Tx bias and Rx bias switches control source select"]
    #[inline(always)]
    pub fn asqbsc(&self) -> AsqbscR {
        AsqbscR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx bias (PGA bias) switch control. Note that the channel to apply the Rx bias is determined by ICTL0.MUXSEL."]
    #[inline(always)]
    pub fn pgabsw(&self) -> PgabswR {
        PgabswR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Line input leakage compensation (LILC) enable."]
    #[inline(always)]
    pub fn lilc(&self) -> LilcR {
        LilcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable the power supply (Charge Pump) for the the input multiplexer during data acquisition."]
    #[inline(always)]
    pub fn cpda(&self) -> CpdaR {
        CpdaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Excitation bias (Rx bias) Voltage Select"]
    #[inline(always)]
    pub fn excbias(&self) -> ExcbiasR {
        ExcbiasR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PGA bias (Rx bias) Voltage Select"]
    #[inline(always)]
    pub fn pgabias(&self) -> PgabiasR {
        PgabiasR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Channel 0 Tx bias Switch Control. Note that the Tx bias voltage is determined by BCTL.EXCBIAS."]
    #[inline(always)]
    pub fn ch0ebsw(&self) -> Ch0ebswR {
        Ch0ebswR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Tx bias Switch Control. Note that the Tx bias voltage is determined by BCTL.EXCBIAS."]
    #[inline(always)]
    pub fn ch1ebsw(&self) -> Ch1ebswR {
        Ch1ebswR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tx bias and Rx bias switches control source select"]
    #[inline(always)]
    pub fn asqbsc(&mut self) -> AsqbscW<'_, SaphAbctlSpec> {
        AsqbscW::new(self, 0)
    }
    #[doc = "Bit 1 - Rx bias (PGA bias) switch control. Note that the channel to apply the Rx bias is determined by ICTL0.MUXSEL."]
    #[inline(always)]
    pub fn pgabsw(&mut self) -> PgabswW<'_, SaphAbctlSpec> {
        PgabswW::new(self, 1)
    }
    #[doc = "Bit 2 - Line input leakage compensation (LILC) enable."]
    #[inline(always)]
    pub fn lilc(&mut self) -> LilcW<'_, SaphAbctlSpec> {
        LilcW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable the power supply (Charge Pump) for the the input multiplexer during data acquisition."]
    #[inline(always)]
    pub fn cpda(&mut self) -> CpdaW<'_, SaphAbctlSpec> {
        CpdaW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Excitation bias (Rx bias) Voltage Select"]
    #[inline(always)]
    pub fn excbias(&mut self) -> ExcbiasW<'_, SaphAbctlSpec> {
        ExcbiasW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PGA bias (Rx bias) Voltage Select"]
    #[inline(always)]
    pub fn pgabias(&mut self) -> PgabiasW<'_, SaphAbctlSpec> {
        PgabiasW::new(self, 6)
    }
    #[doc = "Bit 8 - Channel 0 Tx bias Switch Control. Note that the Tx bias voltage is determined by BCTL.EXCBIAS."]
    #[inline(always)]
    pub fn ch0ebsw(&mut self) -> Ch0ebswW<'_, SaphAbctlSpec> {
        Ch0ebswW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 Tx bias Switch Control. Note that the Tx bias voltage is determined by BCTL.EXCBIAS."]
    #[inline(always)]
    pub fn ch1ebsw(&mut self) -> Ch1ebswW<'_, SaphAbctlSpec> {
        Ch1ebswW::new(self, 9)
    }
}
#[doc = "Bias Control\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_abctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_abctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAbctlSpec;
impl crate::RegisterSpec for SaphAbctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_abctl::R`](R) reader structure"]
impl crate::Readable for SaphAbctlSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_abctl::W`](W) writer structure"]
impl crate::Writable for SaphAbctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_ABCTL to value 0"]
impl crate::Resettable for SaphAbctlSpec {}
