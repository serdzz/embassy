#[doc = "Register `SAPH_AASCTL1` reader"]
pub type R = crate::R<SaphAasctl1Spec>;
#[doc = "Register `SAPH_AASCTL1` writer"]
pub type W = crate::W<SaphAasctl1Spec>;
#[doc = "Channel toggle enable at each PNGDN interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chtog {
    #[doc = "0: Channel toggle is disabled."]
    Chtog0 = 0,
    #[doc = "1: Channel toggle is enabled at each PNGDN interrupt."]
    Chtog1 = 1,
}
impl From<Chtog> for bool {
    #[inline(always)]
    fn from(variant: Chtog) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHTOG` reader - Channel toggle enable at each PNGDN interrupt."]
pub type ChtogR = crate::BitReader<Chtog>;
impl ChtogR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chtog {
        match self.bits {
            false => Chtog::Chtog0,
            true => Chtog::Chtog1,
        }
    }
    #[doc = "Channel toggle is disabled."]
    #[inline(always)]
    pub fn is_chtog_0(&self) -> bool {
        *self == Chtog::Chtog0
    }
    #[doc = "Channel toggle is enabled at each PNGDN interrupt."]
    #[inline(always)]
    pub fn is_chtog_1(&self) -> bool {
        *self == Chtog::Chtog1
    }
}
#[doc = "Field `CHTOG` writer - Channel toggle enable at each PNGDN interrupt."]
pub type ChtogW<'a, REG> = crate::BitWriter<'a, REG, Chtog>;
impl<'a, REG> ChtogW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel toggle is disabled."]
    #[inline(always)]
    pub fn chtog_0(self) -> &'a mut crate::W<REG> {
        self.variant(Chtog::Chtog0)
    }
    #[doc = "Channel toggle is enabled at each PNGDN interrupt."]
    #[inline(always)]
    pub fn chtog_1(self) -> &'a mut crate::W<REG> {
        self.variant(Chtog::Chtog1)
    }
}
#[doc = "Field `CHACT` reader - Read Only bit. This bit indicates the currently selected Tx channel."]
pub type ChactR = crate::BitReader;
#[doc = "Early Receive Bias Control. The Rx bias is applied at the TIMEMARK C, but when this bit is set to '1', the Rx bias is applied at the TIMEMARK A.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Earlyrb {
    #[doc = "0: Rx bias is applied to the Rx channel by the TIMEMARK C"]
    Earlyrb0 = 0,
    #[doc = "1: Rx bias is applied to the Rx channel by the TIMEMARK A"]
    Earlyrb1 = 1,
}
impl From<Earlyrb> for bool {
    #[inline(always)]
    fn from(variant: Earlyrb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EARLYRB` reader - Early Receive Bias Control. The Rx bias is applied at the TIMEMARK C, but when this bit is set to '1', the Rx bias is applied at the TIMEMARK A."]
pub type EarlyrbR = crate::BitReader<Earlyrb>;
impl EarlyrbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Earlyrb {
        match self.bits {
            false => Earlyrb::Earlyrb0,
            true => Earlyrb::Earlyrb1,
        }
    }
    #[doc = "Rx bias is applied to the Rx channel by the TIMEMARK C"]
    #[inline(always)]
    pub fn is_earlyrb_0(&self) -> bool {
        *self == Earlyrb::Earlyrb0
    }
    #[doc = "Rx bias is applied to the Rx channel by the TIMEMARK A"]
    #[inline(always)]
    pub fn is_earlyrb_1(&self) -> bool {
        *self == Earlyrb::Earlyrb1
    }
}
#[doc = "Field `EARLYRB` writer - Early Receive Bias Control. The Rx bias is applied at the TIMEMARK C, but when this bit is set to '1', the Rx bias is applied at the TIMEMARK A."]
pub type EarlyrbW<'a, REG> = crate::BitWriter<'a, REG, Earlyrb>;
impl<'a, REG> EarlyrbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx bias is applied to the Rx channel by the TIMEMARK C"]
    #[inline(always)]
    pub fn earlyrb_0(self) -> &'a mut crate::W<REG> {
        self.variant(Earlyrb::Earlyrb0)
    }
    #[doc = "Rx bias is applied to the Rx channel by the TIMEMARK A"]
    #[inline(always)]
    pub fn earlyrb_1(self) -> &'a mut crate::W<REG> {
        self.variant(Earlyrb::Earlyrb1)
    }
}
#[doc = "Enable the OFF request when ASQ completes all of the measurement sequences.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esoff {
    #[doc = "0: OFF request is disabled. The ASQ does not send a request about USS power mode to the PSQ."]
    Esoff0 = 0,
    #[doc = "1: OFF request is generated after sequence"]
    Esoff1 = 1,
}
impl From<Esoff> for bool {
    #[inline(always)]
    fn from(variant: Esoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESOFF` reader - Enable the OFF request when ASQ completes all of the measurement sequences."]
pub type EsoffR = crate::BitReader<Esoff>;
impl EsoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Esoff {
        match self.bits {
            false => Esoff::Esoff0,
            true => Esoff::Esoff1,
        }
    }
    #[doc = "OFF request is disabled. The ASQ does not send a request about USS power mode to the PSQ."]
    #[inline(always)]
    pub fn is_esoff_0(&self) -> bool {
        *self == Esoff::Esoff0
    }
    #[doc = "OFF request is generated after sequence"]
    #[inline(always)]
    pub fn is_esoff_1(&self) -> bool {
        *self == Esoff::Esoff1
    }
}
#[doc = "Field `ESOFF` writer - Enable the OFF request when ASQ completes all of the measurement sequences."]
pub type EsoffW<'a, REG> = crate::BitWriter<'a, REG, Esoff>;
impl<'a, REG> EsoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OFF request is disabled. The ASQ does not send a request about USS power mode to the PSQ."]
    #[inline(always)]
    pub fn esoff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Esoff::Esoff0)
    }
    #[doc = "OFF request is generated after sequence"]
    #[inline(always)]
    pub fn esoff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Esoff::Esoff1)
    }
}
#[doc = "ASQ can send a request sigal to the PSQ (Power Sequencer) of the USS module when the OFF request is received (See ASCTL1.ESOFF and the UUPS module).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stdby {
    #[doc = "0: The ASQ sends a power down request to the PSQ (Power Sequencer) when the OFF request is received."]
    Pwroff = 0,
    #[doc = "1: The ASQ sends a standby request to the PSQ (Power Sequencer) when the OFF request is received."]
    Stdby = 1,
}
impl From<Stdby> for bool {
    #[inline(always)]
    fn from(variant: Stdby) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STDBY` reader - ASQ can send a request sigal to the PSQ (Power Sequencer) of the USS module when the OFF request is received (See ASCTL1.ESOFF and the UUPS module)."]
pub type StdbyR = crate::BitReader<Stdby>;
impl StdbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stdby {
        match self.bits {
            false => Stdby::Pwroff,
            true => Stdby::Stdby,
        }
    }
    #[doc = "The ASQ sends a power down request to the PSQ (Power Sequencer) when the OFF request is received."]
    #[inline(always)]
    pub fn is_pwroff(&self) -> bool {
        *self == Stdby::Pwroff
    }
    #[doc = "The ASQ sends a standby request to the PSQ (Power Sequencer) when the OFF request is received."]
    #[inline(always)]
    pub fn is_stdby(&self) -> bool {
        *self == Stdby::Stdby
    }
}
#[doc = "Field `STDBY` writer - ASQ can send a request sigal to the PSQ (Power Sequencer) of the USS module when the OFF request is received (See ASCTL1.ESOFF and the UUPS module)."]
pub type StdbyW<'a, REG> = crate::BitWriter<'a, REG, Stdby>;
impl<'a, REG> StdbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The ASQ sends a power down request to the PSQ (Power Sequencer) when the OFF request is received."]
    #[inline(always)]
    pub fn pwroff(self) -> &'a mut crate::W<REG> {
        self.variant(Stdby::Pwroff)
    }
    #[doc = "The ASQ sends a standby request to the PSQ (Power Sequencer) when the OFF request is received."]
    #[inline(always)]
    pub fn stdby(self) -> &'a mut crate::W<REG> {
        self.variant(Stdby::Stdby)
    }
}
#[doc = "In general, if CH0 is selected for Tx, CH1 is selected for Rx. However, when this bit is set to '1', the Tx channel and Rx channel are identical. This bit can be used for debugging purpose.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chown {
    #[doc = "0: Tx channel and Rx channel are not the same (This is the typical configuration)."]
    Chown0 = 0,
    #[doc = "1: Tx channel and Rx channel are identical."]
    Chown1 = 1,
}
impl From<Chown> for bool {
    #[inline(always)]
    fn from(variant: Chown) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHOWN` reader - In general, if CH0 is selected for Tx, CH1 is selected for Rx. However, when this bit is set to '1', the Tx channel and Rx channel are identical. This bit can be used for debugging purpose."]
pub type ChownR = crate::BitReader<Chown>;
impl ChownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chown {
        match self.bits {
            false => Chown::Chown0,
            true => Chown::Chown1,
        }
    }
    #[doc = "Tx channel and Rx channel are not the same (This is the typical configuration)."]
    #[inline(always)]
    pub fn is_chown_0(&self) -> bool {
        *self == Chown::Chown0
    }
    #[doc = "Tx channel and Rx channel are identical."]
    #[inline(always)]
    pub fn is_chown_1(&self) -> bool {
        *self == Chown::Chown1
    }
}
#[doc = "Field `CHOWN` writer - In general, if CH0 is selected for Tx, CH1 is selected for Rx. However, when this bit is set to '1', the Tx channel and Rx channel are identical. This bit can be used for debugging purpose."]
pub type ChownW<'a, REG> = crate::BitWriter<'a, REG, Chown>;
impl<'a, REG> ChownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx channel and Rx channel are not the same (This is the typical configuration)."]
    #[inline(always)]
    pub fn chown_0(self) -> &'a mut crate::W<REG> {
        self.variant(Chown::Chown0)
    }
    #[doc = "Tx channel and Rx channel are identical."]
    #[inline(always)]
    pub fn chown_1(self) -> &'a mut crate::W<REG> {
        self.variant(Chown::Chown1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel toggle enable at each PNGDN interrupt."]
    #[inline(always)]
    pub fn chtog(&self) -> ChtogR {
        ChtogR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Read Only bit. This bit indicates the currently selected Tx channel."]
    #[inline(always)]
    pub fn chact(&self) -> ChactR {
        ChactR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Early Receive Bias Control. The Rx bias is applied at the TIMEMARK C, but when this bit is set to '1', the Rx bias is applied at the TIMEMARK A."]
    #[inline(always)]
    pub fn earlyrb(&self) -> EarlyrbR {
        EarlyrbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable the OFF request when ASQ completes all of the measurement sequences."]
    #[inline(always)]
    pub fn esoff(&self) -> EsoffR {
        EsoffR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - ASQ can send a request sigal to the PSQ (Power Sequencer) of the USS module when the OFF request is received (See ASCTL1.ESOFF and the UUPS module)."]
    #[inline(always)]
    pub fn stdby(&self) -> StdbyR {
        StdbyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - In general, if CH0 is selected for Tx, CH1 is selected for Rx. However, when this bit is set to '1', the Tx channel and Rx channel are identical. This bit can be used for debugging purpose."]
    #[inline(always)]
    pub fn chown(&self) -> ChownR {
        ChownR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel toggle enable at each PNGDN interrupt."]
    #[inline(always)]
    pub fn chtog(&mut self) -> ChtogW<'_, SaphAasctl1Spec> {
        ChtogW::new(self, 0)
    }
    #[doc = "Bit 7 - Early Receive Bias Control. The Rx bias is applied at the TIMEMARK C, but when this bit is set to '1', the Rx bias is applied at the TIMEMARK A."]
    #[inline(always)]
    pub fn earlyrb(&mut self) -> EarlyrbW<'_, SaphAasctl1Spec> {
        EarlyrbW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable the OFF request when ASQ completes all of the measurement sequences."]
    #[inline(always)]
    pub fn esoff(&mut self) -> EsoffW<'_, SaphAasctl1Spec> {
        EsoffW::new(self, 8)
    }
    #[doc = "Bit 10 - ASQ can send a request sigal to the PSQ (Power Sequencer) of the USS module when the OFF request is received (See ASCTL1.ESOFF and the UUPS module)."]
    #[inline(always)]
    pub fn stdby(&mut self) -> StdbyW<'_, SaphAasctl1Spec> {
        StdbyW::new(self, 10)
    }
    #[doc = "Bit 11 - In general, if CH0 is selected for Tx, CH1 is selected for Rx. However, when this bit is set to '1', the Tx channel and Rx channel are identical. This bit can be used for debugging purpose."]
    #[inline(always)]
    pub fn chown(&mut self) -> ChownW<'_, SaphAasctl1Spec> {
        ChownW::new(self, 11)
    }
}
#[doc = "A-SEQ control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aasctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aasctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAasctl1Spec;
impl crate::RegisterSpec for SaphAasctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aasctl1::R`](R) reader structure"]
impl crate::Readable for SaphAasctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`saph_aasctl1::W`](W) writer structure"]
impl crate::Writable for SaphAasctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AASCTL1 to value 0"]
impl crate::Resettable for SaphAasctl1Spec {}
