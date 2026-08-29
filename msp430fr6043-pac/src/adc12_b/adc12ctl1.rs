#[doc = "Register `ADC12CTL1` reader"]
pub type R = crate::R<Adc12ctl1Spec>;
#[doc = "Register `ADC12CTL1` writer"]
pub type W = crate::W<Adc12ctl1Spec>;
#[doc = "ADC busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12busy {
    #[doc = "0: No operation is active."]
    Adc12busy0 = 0,
    #[doc = "1: A sequence, sample, or conversion is active."]
    Adc12busy1 = 1,
}
impl From<Adc12busy> for bool {
    #[inline(always)]
    fn from(variant: Adc12busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12BUSY` reader - ADC busy"]
pub type Adc12busyR = crate::BitReader<Adc12busy>;
impl Adc12busyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12busy {
        match self.bits {
            false => Adc12busy::Adc12busy0,
            true => Adc12busy::Adc12busy1,
        }
    }
    #[doc = "No operation is active."]
    #[inline(always)]
    pub fn is_adc12busy_0(&self) -> bool {
        *self == Adc12busy::Adc12busy0
    }
    #[doc = "A sequence, sample, or conversion is active."]
    #[inline(always)]
    pub fn is_adc12busy_1(&self) -> bool {
        *self == Adc12busy::Adc12busy1
    }
}
#[doc = "conversion sequence mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc12conseq {
    #[doc = "0: Single-channel, single-conversion"]
    Adc12conseq0 = 0,
    #[doc = "1: Sequence-of-channels"]
    Adc12conseq1 = 1,
    #[doc = "2: Repeat-single-channel"]
    Adc12conseq2 = 2,
    #[doc = "3: Repeat-sequence-of-channels"]
    Adc12conseq3 = 3,
}
impl From<Adc12conseq> for u8 {
    #[inline(always)]
    fn from(variant: Adc12conseq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc12conseq {
    type Ux = u8;
}
impl crate::IsEnum for Adc12conseq {}
#[doc = "Field `ADC12CONSEQ` reader - conversion sequence mode select"]
pub type Adc12conseqR = crate::FieldReader<Adc12conseq>;
impl Adc12conseqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12conseq {
        match self.bits {
            0 => Adc12conseq::Adc12conseq0,
            1 => Adc12conseq::Adc12conseq1,
            2 => Adc12conseq::Adc12conseq2,
            3 => Adc12conseq::Adc12conseq3,
            _ => unreachable!(),
        }
    }
    #[doc = "Single-channel, single-conversion"]
    #[inline(always)]
    pub fn is_adc12conseq_0(&self) -> bool {
        *self == Adc12conseq::Adc12conseq0
    }
    #[doc = "Sequence-of-channels"]
    #[inline(always)]
    pub fn is_adc12conseq_1(&self) -> bool {
        *self == Adc12conseq::Adc12conseq1
    }
    #[doc = "Repeat-single-channel"]
    #[inline(always)]
    pub fn is_adc12conseq_2(&self) -> bool {
        *self == Adc12conseq::Adc12conseq2
    }
    #[doc = "Repeat-sequence-of-channels"]
    #[inline(always)]
    pub fn is_adc12conseq_3(&self) -> bool {
        *self == Adc12conseq::Adc12conseq3
    }
}
#[doc = "Field `ADC12CONSEQ` writer - conversion sequence mode select"]
pub type Adc12conseqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc12conseq, crate::Safe>;
impl<'a, REG> Adc12conseqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-channel, single-conversion"]
    #[inline(always)]
    pub fn adc12conseq_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12conseq::Adc12conseq0)
    }
    #[doc = "Sequence-of-channels"]
    #[inline(always)]
    pub fn adc12conseq_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12conseq::Adc12conseq1)
    }
    #[doc = "Repeat-single-channel"]
    #[inline(always)]
    pub fn adc12conseq_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12conseq::Adc12conseq2)
    }
    #[doc = "Repeat-sequence-of-channels"]
    #[inline(always)]
    pub fn adc12conseq_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12conseq::Adc12conseq3)
    }
}
#[doc = "clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc12ssel {
    #[doc = "0: ADC12OSC (MODOSC)"]
    Adc12ssel0 = 0,
    #[doc = "1: ACLK"]
    Adc12ssel1 = 1,
    #[doc = "2: MCLK"]
    Adc12ssel2 = 2,
    #[doc = "3: SMCLK"]
    Adc12ssel3 = 3,
}
impl From<Adc12ssel> for u8 {
    #[inline(always)]
    fn from(variant: Adc12ssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc12ssel {
    type Ux = u8;
}
impl crate::IsEnum for Adc12ssel {}
#[doc = "Field `ADC12SSEL` reader - clock source select"]
pub type Adc12sselR = crate::FieldReader<Adc12ssel>;
impl Adc12sselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12ssel {
        match self.bits {
            0 => Adc12ssel::Adc12ssel0,
            1 => Adc12ssel::Adc12ssel1,
            2 => Adc12ssel::Adc12ssel2,
            3 => Adc12ssel::Adc12ssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC12OSC (MODOSC)"]
    #[inline(always)]
    pub fn is_adc12ssel_0(&self) -> bool {
        *self == Adc12ssel::Adc12ssel0
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn is_adc12ssel_1(&self) -> bool {
        *self == Adc12ssel::Adc12ssel1
    }
    #[doc = "MCLK"]
    #[inline(always)]
    pub fn is_adc12ssel_2(&self) -> bool {
        *self == Adc12ssel::Adc12ssel2
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_adc12ssel_3(&self) -> bool {
        *self == Adc12ssel::Adc12ssel3
    }
}
#[doc = "Field `ADC12SSEL` writer - clock source select"]
pub type Adc12sselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc12ssel, crate::Safe>;
impl<'a, REG> Adc12sselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12OSC (MODOSC)"]
    #[inline(always)]
    pub fn adc12ssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ssel::Adc12ssel0)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn adc12ssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ssel::Adc12ssel1)
    }
    #[doc = "MCLK"]
    #[inline(always)]
    pub fn adc12ssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ssel::Adc12ssel2)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn adc12ssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ssel::Adc12ssel3)
    }
}
#[doc = "clock divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc12div {
    #[doc = "0: /1"]
    Adc12div0 = 0,
    #[doc = "1: /2"]
    Adc12div1 = 1,
    #[doc = "2: /3"]
    Adc12div2 = 2,
    #[doc = "3: /4"]
    Adc12div3 = 3,
    #[doc = "4: /5"]
    Adc12div4 = 4,
    #[doc = "5: /6"]
    Adc12div5 = 5,
    #[doc = "6: /7"]
    Adc12div6 = 6,
    #[doc = "7: /8"]
    Adc12div7 = 7,
}
impl From<Adc12div> for u8 {
    #[inline(always)]
    fn from(variant: Adc12div) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc12div {
    type Ux = u8;
}
impl crate::IsEnum for Adc12div {}
#[doc = "Field `ADC12DIV` reader - clock divider"]
pub type Adc12divR = crate::FieldReader<Adc12div>;
impl Adc12divR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12div {
        match self.bits {
            0 => Adc12div::Adc12div0,
            1 => Adc12div::Adc12div1,
            2 => Adc12div::Adc12div2,
            3 => Adc12div::Adc12div3,
            4 => Adc12div::Adc12div4,
            5 => Adc12div::Adc12div5,
            6 => Adc12div::Adc12div6,
            7 => Adc12div::Adc12div7,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_adc12div_0(&self) -> bool {
        *self == Adc12div::Adc12div0
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_adc12div_1(&self) -> bool {
        *self == Adc12div::Adc12div1
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn is_adc12div_2(&self) -> bool {
        *self == Adc12div::Adc12div2
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_adc12div_3(&self) -> bool {
        *self == Adc12div::Adc12div3
    }
    #[doc = "/5"]
    #[inline(always)]
    pub fn is_adc12div_4(&self) -> bool {
        *self == Adc12div::Adc12div4
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn is_adc12div_5(&self) -> bool {
        *self == Adc12div::Adc12div5
    }
    #[doc = "/7"]
    #[inline(always)]
    pub fn is_adc12div_6(&self) -> bool {
        *self == Adc12div::Adc12div6
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_adc12div_7(&self) -> bool {
        *self == Adc12div::Adc12div7
    }
}
#[doc = "Field `ADC12DIV` writer - clock divider"]
pub type Adc12divW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adc12div, crate::Safe>;
impl<'a, REG> Adc12divW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn adc12div_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div0)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn adc12div_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div1)
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn adc12div_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn adc12div_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div3)
    }
    #[doc = "/5"]
    #[inline(always)]
    pub fn adc12div_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div4)
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn adc12div_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div5)
    }
    #[doc = "/7"]
    #[inline(always)]
    pub fn adc12div_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div6)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn adc12div_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div7)
    }
}
#[doc = "invert signal sample-and-hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12issh {
    #[doc = "0: The sample-input signal is not inverted."]
    Adc12issh0 = 0,
    #[doc = "1: The sample-input signal is inverted."]
    Adc12issh1 = 1,
}
impl From<Adc12issh> for bool {
    #[inline(always)]
    fn from(variant: Adc12issh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12ISSH` reader - invert signal sample-and-hold"]
pub type Adc12isshR = crate::BitReader<Adc12issh>;
impl Adc12isshR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12issh {
        match self.bits {
            false => Adc12issh::Adc12issh0,
            true => Adc12issh::Adc12issh1,
        }
    }
    #[doc = "The sample-input signal is not inverted."]
    #[inline(always)]
    pub fn is_adc12issh_0(&self) -> bool {
        *self == Adc12issh::Adc12issh0
    }
    #[doc = "The sample-input signal is inverted."]
    #[inline(always)]
    pub fn is_adc12issh_1(&self) -> bool {
        *self == Adc12issh::Adc12issh1
    }
}
#[doc = "Field `ADC12ISSH` writer - invert signal sample-and-hold"]
pub type Adc12isshW<'a, REG> = crate::BitWriter<'a, REG, Adc12issh>;
impl<'a, REG> Adc12isshW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The sample-input signal is not inverted."]
    #[inline(always)]
    pub fn adc12issh_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12issh::Adc12issh0)
    }
    #[doc = "The sample-input signal is inverted."]
    #[inline(always)]
    pub fn adc12issh_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12issh::Adc12issh1)
    }
}
#[doc = "sample-and-hold pulse-mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12shp {
    #[doc = "0: SAMPCON signal is sourced from the sample-input signal."]
    Adc12shp0 = 0,
    #[doc = "1: SAMPCON signal is sourced from the sampling timer."]
    Adc12shp1 = 1,
}
impl From<Adc12shp> for bool {
    #[inline(always)]
    fn from(variant: Adc12shp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12SHP` reader - sample-and-hold pulse-mode select"]
pub type Adc12shpR = crate::BitReader<Adc12shp>;
impl Adc12shpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12shp {
        match self.bits {
            false => Adc12shp::Adc12shp0,
            true => Adc12shp::Adc12shp1,
        }
    }
    #[doc = "SAMPCON signal is sourced from the sample-input signal."]
    #[inline(always)]
    pub fn is_adc12shp_0(&self) -> bool {
        *self == Adc12shp::Adc12shp0
    }
    #[doc = "SAMPCON signal is sourced from the sampling timer."]
    #[inline(always)]
    pub fn is_adc12shp_1(&self) -> bool {
        *self == Adc12shp::Adc12shp1
    }
}
#[doc = "Field `ADC12SHP` writer - sample-and-hold pulse-mode select"]
pub type Adc12shpW<'a, REG> = crate::BitWriter<'a, REG, Adc12shp>;
impl<'a, REG> Adc12shpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAMPCON signal is sourced from the sample-input signal."]
    #[inline(always)]
    pub fn adc12shp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12shp::Adc12shp0)
    }
    #[doc = "SAMPCON signal is sourced from the sampling timer."]
    #[inline(always)]
    pub fn adc12shp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12shp::Adc12shp1)
    }
}
#[doc = "sample-and-hold source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc12shs {
    #[doc = "0: ADC12SC bit"]
    Adc12shs0 = 0,
    #[doc = "1: see the device-specific data sheet for source"]
    Adc12shs1 = 1,
    #[doc = "2: see the device-specific data sheet for source"]
    Adc12shs2 = 2,
    #[doc = "3: see the device-specific data sheet for source"]
    Adc12shs3 = 3,
    #[doc = "4: see the device-specific data sheet for source"]
    Adc12shs4 = 4,
    #[doc = "5: see the device-specific data sheet for source"]
    Adc12shs5 = 5,
    #[doc = "6: see the device-specific data sheet for source"]
    Adc12shs6 = 6,
    #[doc = "7: see the device-specific data sheet for source"]
    Adc12shs7 = 7,
}
impl From<Adc12shs> for u8 {
    #[inline(always)]
    fn from(variant: Adc12shs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc12shs {
    type Ux = u8;
}
impl crate::IsEnum for Adc12shs {}
#[doc = "Field `ADC12SHS` reader - sample-and-hold source select"]
pub type Adc12shsR = crate::FieldReader<Adc12shs>;
impl Adc12shsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12shs {
        match self.bits {
            0 => Adc12shs::Adc12shs0,
            1 => Adc12shs::Adc12shs1,
            2 => Adc12shs::Adc12shs2,
            3 => Adc12shs::Adc12shs3,
            4 => Adc12shs::Adc12shs4,
            5 => Adc12shs::Adc12shs5,
            6 => Adc12shs::Adc12shs6,
            7 => Adc12shs::Adc12shs7,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC12SC bit"]
    #[inline(always)]
    pub fn is_adc12shs_0(&self) -> bool {
        *self == Adc12shs::Adc12shs0
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn is_adc12shs_1(&self) -> bool {
        *self == Adc12shs::Adc12shs1
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn is_adc12shs_2(&self) -> bool {
        *self == Adc12shs::Adc12shs2
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn is_adc12shs_3(&self) -> bool {
        *self == Adc12shs::Adc12shs3
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn is_adc12shs_4(&self) -> bool {
        *self == Adc12shs::Adc12shs4
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn is_adc12shs_5(&self) -> bool {
        *self == Adc12shs::Adc12shs5
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn is_adc12shs_6(&self) -> bool {
        *self == Adc12shs::Adc12shs6
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn is_adc12shs_7(&self) -> bool {
        *self == Adc12shs::Adc12shs7
    }
}
#[doc = "Field `ADC12SHS` writer - sample-and-hold source select"]
pub type Adc12shsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adc12shs, crate::Safe>;
impl<'a, REG> Adc12shsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12SC bit"]
    #[inline(always)]
    pub fn adc12shs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12shs::Adc12shs0)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc12shs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12shs::Adc12shs1)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc12shs_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12shs::Adc12shs2)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc12shs_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12shs::Adc12shs3)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc12shs_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12shs::Adc12shs4)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc12shs_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12shs::Adc12shs5)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc12shs_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12shs::Adc12shs6)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc12shs_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12shs::Adc12shs7)
    }
}
#[doc = "predivider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc12pdiv {
    #[doc = "0: Predivide by 1"]
    _1 = 0,
    #[doc = "1: Predivide by 4"]
    _4 = 1,
    #[doc = "2: Predivide by 32"]
    _32 = 2,
    #[doc = "3: Predivide by 64"]
    _64 = 3,
}
impl From<Adc12pdiv> for u8 {
    #[inline(always)]
    fn from(variant: Adc12pdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc12pdiv {
    type Ux = u8;
}
impl crate::IsEnum for Adc12pdiv {}
#[doc = "Field `ADC12PDIV` reader - predivider"]
pub type Adc12pdivR = crate::FieldReader<Adc12pdiv>;
impl Adc12pdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12pdiv {
        match self.bits {
            0 => Adc12pdiv::_1,
            1 => Adc12pdiv::_4,
            2 => Adc12pdiv::_32,
            3 => Adc12pdiv::_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Predivide by 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adc12pdiv::_1
    }
    #[doc = "Predivide by 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Adc12pdiv::_4
    }
    #[doc = "Predivide by 32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Adc12pdiv::_32
    }
    #[doc = "Predivide by 64"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Adc12pdiv::_64
    }
}
#[doc = "Field `ADC12PDIV` writer - predivider"]
pub type Adc12pdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc12pdiv, crate::Safe>;
impl<'a, REG> Adc12pdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Predivide by 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12pdiv::_1)
    }
    #[doc = "Predivide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12pdiv::_4)
    }
    #[doc = "Predivide by 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12pdiv::_32)
    }
    #[doc = "Predivide by 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12pdiv::_64)
    }
}
impl R {
    #[doc = "Bit 0 - ADC busy"]
    #[inline(always)]
    pub fn adc12busy(&self) -> Adc12busyR {
        Adc12busyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - conversion sequence mode select"]
    #[inline(always)]
    pub fn adc12conseq(&self) -> Adc12conseqR {
        Adc12conseqR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - clock source select"]
    #[inline(always)]
    pub fn adc12ssel(&self) -> Adc12sselR {
        Adc12sselR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - clock divider"]
    #[inline(always)]
    pub fn adc12div(&self) -> Adc12divR {
        Adc12divR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - invert signal sample-and-hold"]
    #[inline(always)]
    pub fn adc12issh(&self) -> Adc12isshR {
        Adc12isshR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - sample-and-hold pulse-mode select"]
    #[inline(always)]
    pub fn adc12shp(&self) -> Adc12shpR {
        Adc12shpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - sample-and-hold source select"]
    #[inline(always)]
    pub fn adc12shs(&self) -> Adc12shsR {
        Adc12shsR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:14 - predivider"]
    #[inline(always)]
    pub fn adc12pdiv(&self) -> Adc12pdivR {
        Adc12pdivR::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - conversion sequence mode select"]
    #[inline(always)]
    pub fn adc12conseq(&mut self) -> Adc12conseqW<'_, Adc12ctl1Spec> {
        Adc12conseqW::new(self, 1)
    }
    #[doc = "Bits 3:4 - clock source select"]
    #[inline(always)]
    pub fn adc12ssel(&mut self) -> Adc12sselW<'_, Adc12ctl1Spec> {
        Adc12sselW::new(self, 3)
    }
    #[doc = "Bits 5:7 - clock divider"]
    #[inline(always)]
    pub fn adc12div(&mut self) -> Adc12divW<'_, Adc12ctl1Spec> {
        Adc12divW::new(self, 5)
    }
    #[doc = "Bit 8 - invert signal sample-and-hold"]
    #[inline(always)]
    pub fn adc12issh(&mut self) -> Adc12isshW<'_, Adc12ctl1Spec> {
        Adc12isshW::new(self, 8)
    }
    #[doc = "Bit 9 - sample-and-hold pulse-mode select"]
    #[inline(always)]
    pub fn adc12shp(&mut self) -> Adc12shpW<'_, Adc12ctl1Spec> {
        Adc12shpW::new(self, 9)
    }
    #[doc = "Bits 10:12 - sample-and-hold source select"]
    #[inline(always)]
    pub fn adc12shs(&mut self) -> Adc12shsW<'_, Adc12ctl1Spec> {
        Adc12shsW::new(self, 10)
    }
    #[doc = "Bits 13:14 - predivider"]
    #[inline(always)]
    pub fn adc12pdiv(&mut self) -> Adc12pdivW<'_, Adc12ctl1Spec> {
        Adc12pdivW::new(self, 13)
    }
}
#[doc = "ADC12_B Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12ctl1Spec;
impl crate::RegisterSpec for Adc12ctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12ctl1::R`](R) reader structure"]
impl crate::Readable for Adc12ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc12ctl1::W`](W) writer structure"]
impl crate::Writable for Adc12ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC12CTL1 to value 0"]
impl crate::Resettable for Adc12ctl1Spec {}
