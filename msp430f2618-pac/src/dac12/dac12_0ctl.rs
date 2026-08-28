#[doc = "Register `DAC12_0CTL` reader"]
pub type R = crate::R<Dac12_0ctlSpec>;
#[doc = "Register `DAC12_0CTL` writer"]
pub type W = crate::W<Dac12_0ctlSpec>;
#[doc = "Field `DAC12GRP` reader - DAC12 group"]
pub type Dac12grpR = crate::BitReader;
#[doc = "Field `DAC12GRP` writer - DAC12 group"]
pub type Dac12grpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC12ENC` reader - DAC12 enable conversion"]
pub type Dac12encR = crate::BitReader;
#[doc = "Field `DAC12ENC` writer - DAC12 enable conversion"]
pub type Dac12encW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC12IFG` reader - DAC12 interrupt flag"]
pub type Dac12ifgR = crate::BitReader;
#[doc = "Field `DAC12IFG` writer - DAC12 interrupt flag"]
pub type Dac12ifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC12IE` reader - DAC12 interrupt enable"]
pub type Dac12ieR = crate::BitReader;
#[doc = "Field `DAC12IE` writer - DAC12 interrupt enable"]
pub type Dac12ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC12DF` reader - DAC12 data format"]
pub type Dac12dfR = crate::BitReader;
#[doc = "Field `DAC12DF` writer - DAC12 data format"]
pub type Dac12dfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DAC12 amplifier bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dac12amp {
    #[doc = "0: DAC12 amplifier 0: off"]
    Dac12amp0 = 0,
    #[doc = "1: DAC12 amplifier 1: off"]
    Dac12amp1 = 1,
    #[doc = "2: DAC12 amplifier 2: low"]
    Dac12amp2 = 2,
    #[doc = "3: DAC12 amplifier 3: low"]
    Dac12amp3 = 3,
    #[doc = "4: DAC12 amplifier 4: low"]
    Dac12amp4 = 4,
    #[doc = "5: DAC12 amplifier 5: medium"]
    Dac12amp5 = 5,
    #[doc = "6: DAC12 amplifier 6: medium"]
    Dac12amp6 = 6,
    #[doc = "7: DAC12 amplifier 7: high"]
    Dac12amp7 = 7,
}
impl From<Dac12amp> for u8 {
    #[inline(always)]
    fn from(variant: Dac12amp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dac12amp {
    type Ux = u8;
}
impl crate::IsEnum for Dac12amp {}
#[doc = "Field `DAC12AMP` reader - DAC12 amplifier bit 0"]
pub type Dac12ampR = crate::FieldReader<Dac12amp>;
impl Dac12ampR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac12amp {
        match self.bits {
            0 => Dac12amp::Dac12amp0,
            1 => Dac12amp::Dac12amp1,
            2 => Dac12amp::Dac12amp2,
            3 => Dac12amp::Dac12amp3,
            4 => Dac12amp::Dac12amp4,
            5 => Dac12amp::Dac12amp5,
            6 => Dac12amp::Dac12amp6,
            7 => Dac12amp::Dac12amp7,
            _ => unreachable!(),
        }
    }
    #[doc = "DAC12 amplifier 0: off"]
    #[inline(always)]
    pub fn is_dac12amp_0(&self) -> bool {
        *self == Dac12amp::Dac12amp0
    }
    #[doc = "DAC12 amplifier 1: off"]
    #[inline(always)]
    pub fn is_dac12amp_1(&self) -> bool {
        *self == Dac12amp::Dac12amp1
    }
    #[doc = "DAC12 amplifier 2: low"]
    #[inline(always)]
    pub fn is_dac12amp_2(&self) -> bool {
        *self == Dac12amp::Dac12amp2
    }
    #[doc = "DAC12 amplifier 3: low"]
    #[inline(always)]
    pub fn is_dac12amp_3(&self) -> bool {
        *self == Dac12amp::Dac12amp3
    }
    #[doc = "DAC12 amplifier 4: low"]
    #[inline(always)]
    pub fn is_dac12amp_4(&self) -> bool {
        *self == Dac12amp::Dac12amp4
    }
    #[doc = "DAC12 amplifier 5: medium"]
    #[inline(always)]
    pub fn is_dac12amp_5(&self) -> bool {
        *self == Dac12amp::Dac12amp5
    }
    #[doc = "DAC12 amplifier 6: medium"]
    #[inline(always)]
    pub fn is_dac12amp_6(&self) -> bool {
        *self == Dac12amp::Dac12amp6
    }
    #[doc = "DAC12 amplifier 7: high"]
    #[inline(always)]
    pub fn is_dac12amp_7(&self) -> bool {
        *self == Dac12amp::Dac12amp7
    }
}
#[doc = "Field `DAC12AMP` writer - DAC12 amplifier bit 0"]
pub type Dac12ampW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dac12amp, crate::Safe>;
impl<'a, REG> Dac12ampW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DAC12 amplifier 0: off"]
    #[inline(always)]
    pub fn dac12amp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12amp::Dac12amp0)
    }
    #[doc = "DAC12 amplifier 1: off"]
    #[inline(always)]
    pub fn dac12amp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12amp::Dac12amp1)
    }
    #[doc = "DAC12 amplifier 2: low"]
    #[inline(always)]
    pub fn dac12amp_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12amp::Dac12amp2)
    }
    #[doc = "DAC12 amplifier 3: low"]
    #[inline(always)]
    pub fn dac12amp_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12amp::Dac12amp3)
    }
    #[doc = "DAC12 amplifier 4: low"]
    #[inline(always)]
    pub fn dac12amp_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12amp::Dac12amp4)
    }
    #[doc = "DAC12 amplifier 5: medium"]
    #[inline(always)]
    pub fn dac12amp_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12amp::Dac12amp5)
    }
    #[doc = "DAC12 amplifier 6: medium"]
    #[inline(always)]
    pub fn dac12amp_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12amp::Dac12amp6)
    }
    #[doc = "DAC12 amplifier 7: high"]
    #[inline(always)]
    pub fn dac12amp_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12amp::Dac12amp7)
    }
}
#[doc = "Field `DAC12IR` reader - DAC12 input reference and output range"]
pub type Dac12irR = crate::BitReader;
#[doc = "Field `DAC12IR` writer - DAC12 input reference and output range"]
pub type Dac12irW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC12CALON` reader - DAC12 calibration"]
pub type Dac12calonR = crate::BitReader;
#[doc = "Field `DAC12CALON` writer - DAC12 calibration"]
pub type Dac12calonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DAC12 load select bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dac12lsel {
    #[doc = "0: DAC12 load select 0: direct"]
    Dac12lsel0 = 0,
    #[doc = "1: DAC12 load select 1: latched with DAT"]
    Dac12lsel1 = 1,
    #[doc = "2: DAC12 load select 2: latched with pos. Timer_A3.OUT1"]
    Dac12lsel2 = 2,
    #[doc = "3: DAC12 load select 3: latched with pos. Timer_B7.OUT1"]
    Dac12lsel3 = 3,
}
impl From<Dac12lsel> for u8 {
    #[inline(always)]
    fn from(variant: Dac12lsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dac12lsel {
    type Ux = u8;
}
impl crate::IsEnum for Dac12lsel {}
#[doc = "Field `DAC12LSEL` reader - DAC12 load select bit 0"]
pub type Dac12lselR = crate::FieldReader<Dac12lsel>;
impl Dac12lselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac12lsel {
        match self.bits {
            0 => Dac12lsel::Dac12lsel0,
            1 => Dac12lsel::Dac12lsel1,
            2 => Dac12lsel::Dac12lsel2,
            3 => Dac12lsel::Dac12lsel3,
            _ => unreachable!(),
        }
    }
    #[doc = "DAC12 load select 0: direct"]
    #[inline(always)]
    pub fn is_dac12lsel_0(&self) -> bool {
        *self == Dac12lsel::Dac12lsel0
    }
    #[doc = "DAC12 load select 1: latched with DAT"]
    #[inline(always)]
    pub fn is_dac12lsel_1(&self) -> bool {
        *self == Dac12lsel::Dac12lsel1
    }
    #[doc = "DAC12 load select 2: latched with pos. Timer_A3.OUT1"]
    #[inline(always)]
    pub fn is_dac12lsel_2(&self) -> bool {
        *self == Dac12lsel::Dac12lsel2
    }
    #[doc = "DAC12 load select 3: latched with pos. Timer_B7.OUT1"]
    #[inline(always)]
    pub fn is_dac12lsel_3(&self) -> bool {
        *self == Dac12lsel::Dac12lsel3
    }
}
#[doc = "Field `DAC12LSEL` writer - DAC12 load select bit 0"]
pub type Dac12lselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dac12lsel, crate::Safe>;
impl<'a, REG> Dac12lselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DAC12 load select 0: direct"]
    #[inline(always)]
    pub fn dac12lsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12lsel::Dac12lsel0)
    }
    #[doc = "DAC12 load select 1: latched with DAT"]
    #[inline(always)]
    pub fn dac12lsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12lsel::Dac12lsel1)
    }
    #[doc = "DAC12 load select 2: latched with pos. Timer_A3.OUT1"]
    #[inline(always)]
    pub fn dac12lsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12lsel::Dac12lsel2)
    }
    #[doc = "DAC12 load select 3: latched with pos. Timer_B7.OUT1"]
    #[inline(always)]
    pub fn dac12lsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12lsel::Dac12lsel3)
    }
}
#[doc = "Field `DAC12RES` reader - DAC12 resolution"]
pub type Dac12resR = crate::BitReader;
#[doc = "Field `DAC12RES` writer - DAC12 resolution"]
pub type Dac12resW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DAC12 reference bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dac12sref {
    #[doc = "0: DAC12 reference 0: Vref+"]
    Dac12sref0 = 0,
    #[doc = "1: DAC12 reference 1: Vref+"]
    Dac12sref1 = 1,
    #[doc = "2: DAC12 reference 2: Veref+"]
    Dac12sref2 = 2,
    #[doc = "3: DAC12 reference 3: Veref+"]
    Dac12sref3 = 3,
}
impl From<Dac12sref> for u8 {
    #[inline(always)]
    fn from(variant: Dac12sref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dac12sref {
    type Ux = u8;
}
impl crate::IsEnum for Dac12sref {}
#[doc = "Field `DAC12SREF` reader - DAC12 reference bit 0"]
pub type Dac12srefR = crate::FieldReader<Dac12sref>;
impl Dac12srefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac12sref {
        match self.bits {
            0 => Dac12sref::Dac12sref0,
            1 => Dac12sref::Dac12sref1,
            2 => Dac12sref::Dac12sref2,
            3 => Dac12sref::Dac12sref3,
            _ => unreachable!(),
        }
    }
    #[doc = "DAC12 reference 0: Vref+"]
    #[inline(always)]
    pub fn is_dac12sref_0(&self) -> bool {
        *self == Dac12sref::Dac12sref0
    }
    #[doc = "DAC12 reference 1: Vref+"]
    #[inline(always)]
    pub fn is_dac12sref_1(&self) -> bool {
        *self == Dac12sref::Dac12sref1
    }
    #[doc = "DAC12 reference 2: Veref+"]
    #[inline(always)]
    pub fn is_dac12sref_2(&self) -> bool {
        *self == Dac12sref::Dac12sref2
    }
    #[doc = "DAC12 reference 3: Veref+"]
    #[inline(always)]
    pub fn is_dac12sref_3(&self) -> bool {
        *self == Dac12sref::Dac12sref3
    }
}
#[doc = "Field `DAC12SREF` writer - DAC12 reference bit 0"]
pub type Dac12srefW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dac12sref, crate::Safe>;
impl<'a, REG> Dac12srefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DAC12 reference 0: Vref+"]
    #[inline(always)]
    pub fn dac12sref_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12sref::Dac12sref0)
    }
    #[doc = "DAC12 reference 1: Vref+"]
    #[inline(always)]
    pub fn dac12sref_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12sref::Dac12sref1)
    }
    #[doc = "DAC12 reference 2: Veref+"]
    #[inline(always)]
    pub fn dac12sref_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12sref::Dac12sref2)
    }
    #[doc = "DAC12 reference 3: Veref+"]
    #[inline(always)]
    pub fn dac12sref_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dac12sref::Dac12sref3)
    }
}
#[doc = "Field `DAC12OPS` reader - DAC12 Operation Amp."]
pub type Dac12opsR = crate::BitReader;
#[doc = "Field `DAC12OPS` writer - DAC12 Operation Amp."]
pub type Dac12opsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DAC12 group"]
    #[inline(always)]
    pub fn dac12grp(&self) -> Dac12grpR {
        Dac12grpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC12 enable conversion"]
    #[inline(always)]
    pub fn dac12enc(&self) -> Dac12encR {
        Dac12encR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC12 interrupt flag"]
    #[inline(always)]
    pub fn dac12ifg(&self) -> Dac12ifgR {
        Dac12ifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DAC12 interrupt enable"]
    #[inline(always)]
    pub fn dac12ie(&self) -> Dac12ieR {
        Dac12ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DAC12 data format"]
    #[inline(always)]
    pub fn dac12df(&self) -> Dac12dfR {
        Dac12dfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - DAC12 amplifier bit 0"]
    #[inline(always)]
    pub fn dac12amp(&self) -> Dac12ampR {
        Dac12ampR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - DAC12 input reference and output range"]
    #[inline(always)]
    pub fn dac12ir(&self) -> Dac12irR {
        Dac12irR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAC12 calibration"]
    #[inline(always)]
    pub fn dac12calon(&self) -> Dac12calonR {
        Dac12calonR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - DAC12 load select bit 0"]
    #[inline(always)]
    pub fn dac12lsel(&self) -> Dac12lselR {
        Dac12lselR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - DAC12 resolution"]
    #[inline(always)]
    pub fn dac12res(&self) -> Dac12resR {
        Dac12resR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - DAC12 reference bit 0"]
    #[inline(always)]
    pub fn dac12sref(&self) -> Dac12srefR {
        Dac12srefR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - DAC12 Operation Amp."]
    #[inline(always)]
    pub fn dac12ops(&self) -> Dac12opsR {
        Dac12opsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC12 group"]
    #[inline(always)]
    pub fn dac12grp(&mut self) -> Dac12grpW<'_, Dac12_0ctlSpec> {
        Dac12grpW::new(self, 0)
    }
    #[doc = "Bit 1 - DAC12 enable conversion"]
    #[inline(always)]
    pub fn dac12enc(&mut self) -> Dac12encW<'_, Dac12_0ctlSpec> {
        Dac12encW::new(self, 1)
    }
    #[doc = "Bit 2 - DAC12 interrupt flag"]
    #[inline(always)]
    pub fn dac12ifg(&mut self) -> Dac12ifgW<'_, Dac12_0ctlSpec> {
        Dac12ifgW::new(self, 2)
    }
    #[doc = "Bit 3 - DAC12 interrupt enable"]
    #[inline(always)]
    pub fn dac12ie(&mut self) -> Dac12ieW<'_, Dac12_0ctlSpec> {
        Dac12ieW::new(self, 3)
    }
    #[doc = "Bit 4 - DAC12 data format"]
    #[inline(always)]
    pub fn dac12df(&mut self) -> Dac12dfW<'_, Dac12_0ctlSpec> {
        Dac12dfW::new(self, 4)
    }
    #[doc = "Bits 5:7 - DAC12 amplifier bit 0"]
    #[inline(always)]
    pub fn dac12amp(&mut self) -> Dac12ampW<'_, Dac12_0ctlSpec> {
        Dac12ampW::new(self, 5)
    }
    #[doc = "Bit 8 - DAC12 input reference and output range"]
    #[inline(always)]
    pub fn dac12ir(&mut self) -> Dac12irW<'_, Dac12_0ctlSpec> {
        Dac12irW::new(self, 8)
    }
    #[doc = "Bit 9 - DAC12 calibration"]
    #[inline(always)]
    pub fn dac12calon(&mut self) -> Dac12calonW<'_, Dac12_0ctlSpec> {
        Dac12calonW::new(self, 9)
    }
    #[doc = "Bits 10:11 - DAC12 load select bit 0"]
    #[inline(always)]
    pub fn dac12lsel(&mut self) -> Dac12lselW<'_, Dac12_0ctlSpec> {
        Dac12lselW::new(self, 10)
    }
    #[doc = "Bit 12 - DAC12 resolution"]
    #[inline(always)]
    pub fn dac12res(&mut self) -> Dac12resW<'_, Dac12_0ctlSpec> {
        Dac12resW::new(self, 12)
    }
    #[doc = "Bits 13:14 - DAC12 reference bit 0"]
    #[inline(always)]
    pub fn dac12sref(&mut self) -> Dac12srefW<'_, Dac12_0ctlSpec> {
        Dac12srefW::new(self, 13)
    }
    #[doc = "Bit 15 - DAC12 Operation Amp."]
    #[inline(always)]
    pub fn dac12ops(&mut self) -> Dac12opsW<'_, Dac12_0ctlSpec> {
        Dac12opsW::new(self, 15)
    }
}
#[doc = "DAC12_0 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dac12_0ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac12_0ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac12_0ctlSpec;
impl crate::RegisterSpec for Dac12_0ctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dac12_0ctl::R`](R) reader structure"]
impl crate::Readable for Dac12_0ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dac12_0ctl::W`](W) writer structure"]
impl crate::Writable for Dac12_0ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC12_0CTL to value 0"]
impl crate::Resettable for Dac12_0ctlSpec {}
