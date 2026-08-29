#[doc = "Register `SAPH_APGCTL` reader"]
pub type R = crate::R<SaphApgctlSpec>;
#[doc = "Register `SAPH_APGCTL` writer"]
pub type W = crate::W<SaphApgctlSpec>;
#[doc = "PPG output channel select source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pgsel {
    #[doc = "0: PPG output channel is selected by PGCTL.PPGCHSEL bit (register mode)."]
    Pgsel0 = 0,
    #[doc = "1: PPG output channel is selected by ASQ (auto mode)."]
    Pgsel1 = 1,
}
impl From<Pgsel> for bool {
    #[inline(always)]
    fn from(variant: Pgsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGSEL` reader - PPG output channel select source."]
pub type PgselR = crate::BitReader<Pgsel>;
impl PgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pgsel {
        match self.bits {
            false => Pgsel::Pgsel0,
            true => Pgsel::Pgsel1,
        }
    }
    #[doc = "PPG output channel is selected by PGCTL.PPGCHSEL bit (register mode)."]
    #[inline(always)]
    pub fn is_pgsel_0(&self) -> bool {
        *self == Pgsel::Pgsel0
    }
    #[doc = "PPG output channel is selected by ASQ (auto mode)."]
    #[inline(always)]
    pub fn is_pgsel_1(&self) -> bool {
        *self == Pgsel::Pgsel1
    }
}
#[doc = "Field `PGSEL` writer - PPG output channel select source."]
pub type PgselW<'a, REG> = crate::BitWriter<'a, REG, Pgsel>;
impl<'a, REG> PgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PPG output channel is selected by PGCTL.PPGCHSEL bit (register mode)."]
    #[inline(always)]
    pub fn pgsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pgsel::Pgsel0)
    }
    #[doc = "PPG output channel is selected by ASQ (auto mode)."]
    #[inline(always)]
    pub fn pgsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pgsel::Pgsel1)
    }
}
#[doc = "PPG output channel select when PGCTL.PGSEL = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ppgchsel {
    #[doc = "0: CH0 is selected"]
    Ppgchsel0 = 0,
    #[doc = "1: CH1 is selected"]
    Ppgchsel1 = 1,
}
impl From<Ppgchsel> for bool {
    #[inline(always)]
    fn from(variant: Ppgchsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPGCHSEL` reader - PPG output channel select when PGCTL.PGSEL = 0."]
pub type PpgchselR = crate::BitReader<Ppgchsel>;
impl PpgchselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ppgchsel {
        match self.bits {
            false => Ppgchsel::Ppgchsel0,
            true => Ppgchsel::Ppgchsel1,
        }
    }
    #[doc = "CH0 is selected"]
    #[inline(always)]
    pub fn is_ppgchsel_0(&self) -> bool {
        *self == Ppgchsel::Ppgchsel0
    }
    #[doc = "CH1 is selected"]
    #[inline(always)]
    pub fn is_ppgchsel_1(&self) -> bool {
        *self == Ppgchsel::Ppgchsel1
    }
}
#[doc = "Field `PPGCHSEL` writer - PPG output channel select when PGCTL.PGSEL = 0."]
pub type PpgchselW<'a, REG> = crate::BitWriter<'a, REG, Ppgchsel>;
impl<'a, REG> PpgchselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CH0 is selected"]
    #[inline(always)]
    pub fn ppgchsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ppgchsel::Ppgchsel0)
    }
    #[doc = "CH1 is selected"]
    #[inline(always)]
    pub fn ppgchsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ppgchsel::Ppgchsel1)
    }
}
#[doc = "PPG Trigger source select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trsel {
    #[doc = "0: Writing 1 to PPGTRIG.PPGTRIG to trigger the PPG (start pulse generation)."]
    Trsel0 = 0,
    #[doc = "1: PPG trigger is controlled by the ASQ."]
    Trsel1 = 1,
    #[doc = "2: Ext. Signal (See device specific datasheet)"]
    Trsel2 = 2,
    #[doc = "3: Ext. Signal (See device specific datasheet)"]
    Trsel3 = 3,
}
impl From<Trsel> for u8 {
    #[inline(always)]
    fn from(variant: Trsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trsel {
    type Ux = u8;
}
impl crate::IsEnum for Trsel {}
#[doc = "Field `TRSEL` reader - PPG Trigger source select."]
pub type TrselR = crate::FieldReader<Trsel>;
impl TrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trsel {
        match self.bits {
            0 => Trsel::Trsel0,
            1 => Trsel::Trsel1,
            2 => Trsel::Trsel2,
            3 => Trsel::Trsel3,
            _ => unreachable!(),
        }
    }
    #[doc = "Writing 1 to PPGTRIG.PPGTRIG to trigger the PPG (start pulse generation)."]
    #[inline(always)]
    pub fn is_trsel_0(&self) -> bool {
        *self == Trsel::Trsel0
    }
    #[doc = "PPG trigger is controlled by the ASQ."]
    #[inline(always)]
    pub fn is_trsel_1(&self) -> bool {
        *self == Trsel::Trsel1
    }
    #[doc = "Ext. Signal (See device specific datasheet)"]
    #[inline(always)]
    pub fn is_trsel_2(&self) -> bool {
        *self == Trsel::Trsel2
    }
    #[doc = "Ext. Signal (See device specific datasheet)"]
    #[inline(always)]
    pub fn is_trsel_3(&self) -> bool {
        *self == Trsel::Trsel3
    }
}
#[doc = "Field `TRSEL` writer - PPG Trigger source select."]
pub type TrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trsel, crate::Safe>;
impl<'a, REG> TrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing 1 to PPGTRIG.PPGTRIG to trigger the PPG (start pulse generation)."]
    #[inline(always)]
    pub fn trsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trsel::Trsel0)
    }
    #[doc = "PPG trigger is controlled by the ASQ."]
    #[inline(always)]
    pub fn trsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trsel::Trsel1)
    }
    #[doc = "Ext. Signal (See device specific datasheet)"]
    #[inline(always)]
    pub fn trsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Trsel::Trsel2)
    }
    #[doc = "Ext. Signal (See device specific datasheet)"]
    #[inline(always)]
    pub fn trsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Trsel::Trsel3)
    }
}
#[doc = "PPGEN PPG trigger enable. This bit can be used to indicates that the configuration of the pulse generator is complete. The PPG can only be started when this bit is set to 1 regardless of its trigger source. It is recommended to keep this bit zero while updating PPG registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ppgen {
    #[doc = "0: PPG trigger is disabled."]
    Ppgen0 = 0,
    #[doc = "1: PPG trigger is enabled."]
    Ppgen1 = 1,
}
impl From<Ppgen> for bool {
    #[inline(always)]
    fn from(variant: Ppgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPGEN` reader - PPGEN PPG trigger enable. This bit can be used to indicates that the configuration of the pulse generator is complete. The PPG can only be started when this bit is set to 1 regardless of its trigger source. It is recommended to keep this bit zero while updating PPG registers."]
pub type PpgenR = crate::BitReader<Ppgen>;
impl PpgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ppgen {
        match self.bits {
            false => Ppgen::Ppgen0,
            true => Ppgen::Ppgen1,
        }
    }
    #[doc = "PPG trigger is disabled."]
    #[inline(always)]
    pub fn is_ppgen_0(&self) -> bool {
        *self == Ppgen::Ppgen0
    }
    #[doc = "PPG trigger is enabled."]
    #[inline(always)]
    pub fn is_ppgen_1(&self) -> bool {
        *self == Ppgen::Ppgen1
    }
}
#[doc = "Field `PPGEN` writer - PPGEN PPG trigger enable. This bit can be used to indicates that the configuration of the pulse generator is complete. The PPG can only be started when this bit is set to 1 regardless of its trigger source. It is recommended to keep this bit zero while updating PPG registers."]
pub type PpgenW<'a, REG> = crate::BitWriter<'a, REG, Ppgen>;
impl<'a, REG> PpgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PPG trigger is disabled."]
    #[inline(always)]
    pub fn ppgen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ppgen::Ppgen0)
    }
    #[doc = "PPG trigger is enabled."]
    #[inline(always)]
    pub fn ppgen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ppgen::Ppgen1)
    }
}
#[doc = "PPG pre scaler enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscen {
    #[doc = "0: Prescaler is disabled. PPG clock = PLL output clock."]
    Disable = 0,
    #[doc = "1: Prescaler by four is enabled. PPG clock = 1/4 of the PLL output clock."]
    Enable = 1,
}
impl From<Pscen> for bool {
    #[inline(always)]
    fn from(variant: Pscen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSCEN` reader - PPG pre scaler enable."]
pub type PscenR = crate::BitReader<Pscen>;
impl PscenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pscen {
        match self.bits {
            false => Pscen::Disable,
            true => Pscen::Enable,
        }
    }
    #[doc = "Prescaler is disabled. PPG clock = PLL output clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pscen::Disable
    }
    #[doc = "Prescaler by four is enabled. PPG clock = 1/4 of the PLL output clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pscen::Enable
    }
}
#[doc = "Field `PSCEN` writer - PPG pre scaler enable."]
pub type PscenW<'a, REG> = crate::BitWriter<'a, REG, Pscen>;
impl<'a, REG> PscenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prescaler is disabled. PPG clock = PLL output clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pscen::Disable)
    }
    #[doc = "Prescaler by four is enabled. PPG clock = 1/4 of the PLL output clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pscen::Enable)
    }
}
#[doc = "Tone generation enable. The frequency of the test tone is determined by LPER and HPER. The test tone persists while PGCTL.TONE = 1 & PGCTL.STOP=0. Either writing 0 to PGCTL.TONE or writing 1 to PGCTL.STOP stops tone generation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tone {
    #[doc = "0: Test tone generation is disabled. Note: This bit is automatically cleared when writing '1' to PGCTL.STOP, and it stops test tone generation immediately."]
    Disable = 0,
    #[doc = "1: Test tone generation is enabled."]
    Enable = 1,
}
impl From<Tone> for bool {
    #[inline(always)]
    fn from(variant: Tone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TONE` reader - Tone generation enable. The frequency of the test tone is determined by LPER and HPER. The test tone persists while PGCTL.TONE = 1 & PGCTL.STOP=0. Either writing 0 to PGCTL.TONE or writing 1 to PGCTL.STOP stops tone generation."]
pub type ToneR = crate::BitReader<Tone>;
impl ToneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tone {
        match self.bits {
            false => Tone::Disable,
            true => Tone::Enable,
        }
    }
    #[doc = "Test tone generation is disabled. Note: This bit is automatically cleared when writing '1' to PGCTL.STOP, and it stops test tone generation immediately."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tone::Disable
    }
    #[doc = "Test tone generation is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tone::Enable
    }
}
#[doc = "Field `TONE` writer - Tone generation enable. The frequency of the test tone is determined by LPER and HPER. The test tone persists while PGCTL.TONE = 1 & PGCTL.STOP=0. Either writing 0 to PGCTL.TONE or writing 1 to PGCTL.STOP stops tone generation."]
pub type ToneW<'a, REG> = crate::BitWriter<'a, REG, Tone>;
impl<'a, REG> ToneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Test tone generation is disabled. Note: This bit is automatically cleared when writing '1' to PGCTL.STOP, and it stops test tone generation immediately."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tone::Disable)
    }
    #[doc = "Test tone generation is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tone::Enable)
    }
}
#[doc = "Field `PPGSTOP` reader - Writing one to this bit stopps the PPG to generate pulses. This bit is cleared automatically."]
pub type PpgstopR = crate::BitReader;
#[doc = "Field `PPGSTOP` writer - Writing one to this bit stopps the PPG to generate pulses. This bit is cleared automatically."]
pub type PpgstopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PPG output channel select source."]
    #[inline(always)]
    pub fn pgsel(&self) -> PgselR {
        PgselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PPG output channel select when PGCTL.PGSEL = 0."]
    #[inline(always)]
    pub fn ppgchsel(&self) -> PpgchselR {
        PpgchselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - PPG Trigger source select."]
    #[inline(always)]
    pub fn trsel(&self) -> TrselR {
        TrselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 9 - PPGEN PPG trigger enable. This bit can be used to indicates that the configuration of the pulse generator is complete. The PPG can only be started when this bit is set to 1 regardless of its trigger source. It is recommended to keep this bit zero while updating PPG registers."]
    #[inline(always)]
    pub fn ppgen(&self) -> PpgenR {
        PpgenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - PPG pre scaler enable."]
    #[inline(always)]
    pub fn pscen(&self) -> PscenR {
        PscenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tone generation enable. The frequency of the test tone is determined by LPER and HPER. The test tone persists while PGCTL.TONE = 1 & PGCTL.STOP=0. Either writing 0 to PGCTL.TONE or writing 1 to PGCTL.STOP stops tone generation."]
    #[inline(always)]
    pub fn tone(&self) -> ToneR {
        ToneR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Writing one to this bit stopps the PPG to generate pulses. This bit is cleared automatically."]
    #[inline(always)]
    pub fn ppgstop(&self) -> PpgstopR {
        PpgstopR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PPG output channel select source."]
    #[inline(always)]
    pub fn pgsel(&mut self) -> PgselW<'_, SaphApgctlSpec> {
        PgselW::new(self, 0)
    }
    #[doc = "Bit 1 - PPG output channel select when PGCTL.PGSEL = 0."]
    #[inline(always)]
    pub fn ppgchsel(&mut self) -> PpgchselW<'_, SaphApgctlSpec> {
        PpgchselW::new(self, 1)
    }
    #[doc = "Bits 4:5 - PPG Trigger source select."]
    #[inline(always)]
    pub fn trsel(&mut self) -> TrselW<'_, SaphApgctlSpec> {
        TrselW::new(self, 4)
    }
    #[doc = "Bit 9 - PPGEN PPG trigger enable. This bit can be used to indicates that the configuration of the pulse generator is complete. The PPG can only be started when this bit is set to 1 regardless of its trigger source. It is recommended to keep this bit zero while updating PPG registers."]
    #[inline(always)]
    pub fn ppgen(&mut self) -> PpgenW<'_, SaphApgctlSpec> {
        PpgenW::new(self, 9)
    }
    #[doc = "Bit 13 - PPG pre scaler enable."]
    #[inline(always)]
    pub fn pscen(&mut self) -> PscenW<'_, SaphApgctlSpec> {
        PscenW::new(self, 13)
    }
    #[doc = "Bit 14 - Tone generation enable. The frequency of the test tone is determined by LPER and HPER. The test tone persists while PGCTL.TONE = 1 & PGCTL.STOP=0. Either writing 0 to PGCTL.TONE or writing 1 to PGCTL.STOP stops tone generation."]
    #[inline(always)]
    pub fn tone(&mut self) -> ToneW<'_, SaphApgctlSpec> {
        ToneW::new(self, 14)
    }
    #[doc = "Bit 15 - Writing one to this bit stopps the PPG to generate pulses. This bit is cleared automatically."]
    #[inline(always)]
    pub fn ppgstop(&mut self) -> PpgstopW<'_, SaphApgctlSpec> {
        PpgstopW::new(self, 15)
    }
}
#[doc = "PPG Control\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_apgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_apgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphApgctlSpec;
impl crate::RegisterSpec for SaphApgctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_apgctl::R`](R) reader structure"]
impl crate::Readable for SaphApgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_apgctl::W`](W) writer structure"]
impl crate::Writable for SaphApgctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_APGCTL to value 0"]
impl crate::Resettable for SaphApgctlSpec {}
