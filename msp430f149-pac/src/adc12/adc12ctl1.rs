#[doc = "Register `ADC12CTL1` reader"]
pub type R = crate::R<Adc12ctl1Spec>;
#[doc = "Register `ADC12CTL1` writer"]
pub type W = crate::W<Adc12ctl1Spec>;
#[doc = "Field `ADC12BUSY` reader - ADC12 Busy"]
pub type Adc12busyR = crate::BitReader;
#[doc = "Field `ADC12BUSY` writer - ADC12 Busy"]
pub type Adc12busyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC12 Conversion Sequence Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Conseq {
    #[doc = "0: ADC12 Conversion Sequence Select: 0"]
    Conseq0 = 0,
    #[doc = "1: ADC12 Conversion Sequence Select: 1"]
    Conseq1 = 1,
    #[doc = "2: ADC12 Conversion Sequence Select: 2"]
    Conseq2 = 2,
    #[doc = "3: ADC12 Conversion Sequence Select: 3"]
    Conseq3 = 3,
}
impl From<Conseq> for u8 {
    #[inline(always)]
    fn from(variant: Conseq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Conseq {
    type Ux = u8;
}
impl crate::IsEnum for Conseq {}
#[doc = "Field `CONSEQ` reader - ADC12 Conversion Sequence Select 0"]
pub type ConseqR = crate::FieldReader<Conseq>;
impl ConseqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Conseq {
        match self.bits {
            0 => Conseq::Conseq0,
            1 => Conseq::Conseq1,
            2 => Conseq::Conseq2,
            3 => Conseq::Conseq3,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC12 Conversion Sequence Select: 0"]
    #[inline(always)]
    pub fn is_conseq_0(&self) -> bool {
        *self == Conseq::Conseq0
    }
    #[doc = "ADC12 Conversion Sequence Select: 1"]
    #[inline(always)]
    pub fn is_conseq_1(&self) -> bool {
        *self == Conseq::Conseq1
    }
    #[doc = "ADC12 Conversion Sequence Select: 2"]
    #[inline(always)]
    pub fn is_conseq_2(&self) -> bool {
        *self == Conseq::Conseq2
    }
    #[doc = "ADC12 Conversion Sequence Select: 3"]
    #[inline(always)]
    pub fn is_conseq_3(&self) -> bool {
        *self == Conseq::Conseq3
    }
}
#[doc = "Field `CONSEQ` writer - ADC12 Conversion Sequence Select 0"]
pub type ConseqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Conseq, crate::Safe>;
impl<'a, REG> ConseqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12 Conversion Sequence Select: 0"]
    #[inline(always)]
    pub fn conseq_0(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Conseq0)
    }
    #[doc = "ADC12 Conversion Sequence Select: 1"]
    #[inline(always)]
    pub fn conseq_1(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Conseq1)
    }
    #[doc = "ADC12 Conversion Sequence Select: 2"]
    #[inline(always)]
    pub fn conseq_2(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Conseq2)
    }
    #[doc = "ADC12 Conversion Sequence Select: 3"]
    #[inline(always)]
    pub fn conseq_3(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Conseq3)
    }
}
#[doc = "ADC12 Clock Source Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc12ssel {
    #[doc = "0: ADC12 Clock Source Select: 0"]
    Adc12ssel0 = 0,
    #[doc = "1: ADC12 Clock Source Select: 1"]
    Adc12ssel1 = 1,
    #[doc = "2: ADC12 Clock Source Select: 2"]
    Adc12ssel2 = 2,
    #[doc = "3: ADC12 Clock Source Select: 3"]
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
#[doc = "Field `ADC12SSEL` reader - ADC12 Clock Source Select 0"]
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
    #[doc = "ADC12 Clock Source Select: 0"]
    #[inline(always)]
    pub fn is_adc12ssel_0(&self) -> bool {
        *self == Adc12ssel::Adc12ssel0
    }
    #[doc = "ADC12 Clock Source Select: 1"]
    #[inline(always)]
    pub fn is_adc12ssel_1(&self) -> bool {
        *self == Adc12ssel::Adc12ssel1
    }
    #[doc = "ADC12 Clock Source Select: 2"]
    #[inline(always)]
    pub fn is_adc12ssel_2(&self) -> bool {
        *self == Adc12ssel::Adc12ssel2
    }
    #[doc = "ADC12 Clock Source Select: 3"]
    #[inline(always)]
    pub fn is_adc12ssel_3(&self) -> bool {
        *self == Adc12ssel::Adc12ssel3
    }
}
#[doc = "Field `ADC12SSEL` writer - ADC12 Clock Source Select 0"]
pub type Adc12sselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc12ssel, crate::Safe>;
impl<'a, REG> Adc12sselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12 Clock Source Select: 0"]
    #[inline(always)]
    pub fn adc12ssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ssel::Adc12ssel0)
    }
    #[doc = "ADC12 Clock Source Select: 1"]
    #[inline(always)]
    pub fn adc12ssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ssel::Adc12ssel1)
    }
    #[doc = "ADC12 Clock Source Select: 2"]
    #[inline(always)]
    pub fn adc12ssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ssel::Adc12ssel2)
    }
    #[doc = "ADC12 Clock Source Select: 3"]
    #[inline(always)]
    pub fn adc12ssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12ssel::Adc12ssel3)
    }
}
#[doc = "ADC12 Clock Divider Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc12div {
    #[doc = "0: ADC12 Clock Divider Select: 0"]
    Adc12div0 = 0,
    #[doc = "1: ADC12 Clock Divider Select: 1"]
    Adc12div1 = 1,
    #[doc = "2: ADC12 Clock Divider Select: 2"]
    Adc12div2 = 2,
    #[doc = "3: ADC12 Clock Divider Select: 3"]
    Adc12div3 = 3,
    #[doc = "4: ADC12 Clock Divider Select: 4"]
    Adc12div4 = 4,
    #[doc = "5: ADC12 Clock Divider Select: 5"]
    Adc12div5 = 5,
    #[doc = "6: ADC12 Clock Divider Select: 6"]
    Adc12div6 = 6,
    #[doc = "7: ADC12 Clock Divider Select: 7"]
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
#[doc = "Field `ADC12DIV` reader - ADC12 Clock Divider Select 0"]
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
    #[doc = "ADC12 Clock Divider Select: 0"]
    #[inline(always)]
    pub fn is_adc12div_0(&self) -> bool {
        *self == Adc12div::Adc12div0
    }
    #[doc = "ADC12 Clock Divider Select: 1"]
    #[inline(always)]
    pub fn is_adc12div_1(&self) -> bool {
        *self == Adc12div::Adc12div1
    }
    #[doc = "ADC12 Clock Divider Select: 2"]
    #[inline(always)]
    pub fn is_adc12div_2(&self) -> bool {
        *self == Adc12div::Adc12div2
    }
    #[doc = "ADC12 Clock Divider Select: 3"]
    #[inline(always)]
    pub fn is_adc12div_3(&self) -> bool {
        *self == Adc12div::Adc12div3
    }
    #[doc = "ADC12 Clock Divider Select: 4"]
    #[inline(always)]
    pub fn is_adc12div_4(&self) -> bool {
        *self == Adc12div::Adc12div4
    }
    #[doc = "ADC12 Clock Divider Select: 5"]
    #[inline(always)]
    pub fn is_adc12div_5(&self) -> bool {
        *self == Adc12div::Adc12div5
    }
    #[doc = "ADC12 Clock Divider Select: 6"]
    #[inline(always)]
    pub fn is_adc12div_6(&self) -> bool {
        *self == Adc12div::Adc12div6
    }
    #[doc = "ADC12 Clock Divider Select: 7"]
    #[inline(always)]
    pub fn is_adc12div_7(&self) -> bool {
        *self == Adc12div::Adc12div7
    }
}
#[doc = "Field `ADC12DIV` writer - ADC12 Clock Divider Select 0"]
pub type Adc12divW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adc12div, crate::Safe>;
impl<'a, REG> Adc12divW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12 Clock Divider Select: 0"]
    #[inline(always)]
    pub fn adc12div_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div0)
    }
    #[doc = "ADC12 Clock Divider Select: 1"]
    #[inline(always)]
    pub fn adc12div_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div1)
    }
    #[doc = "ADC12 Clock Divider Select: 2"]
    #[inline(always)]
    pub fn adc12div_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div2)
    }
    #[doc = "ADC12 Clock Divider Select: 3"]
    #[inline(always)]
    pub fn adc12div_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div3)
    }
    #[doc = "ADC12 Clock Divider Select: 4"]
    #[inline(always)]
    pub fn adc12div_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div4)
    }
    #[doc = "ADC12 Clock Divider Select: 5"]
    #[inline(always)]
    pub fn adc12div_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div5)
    }
    #[doc = "ADC12 Clock Divider Select: 6"]
    #[inline(always)]
    pub fn adc12div_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div6)
    }
    #[doc = "ADC12 Clock Divider Select: 7"]
    #[inline(always)]
    pub fn adc12div_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12div::Adc12div7)
    }
}
#[doc = "Field `ISSH` reader - ADC12 Invert Sample Hold Signal"]
pub type IsshR = crate::BitReader;
#[doc = "Field `ISSH` writer - ADC12 Invert Sample Hold Signal"]
pub type IsshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHP` reader - ADC12 Sample/Hold Pulse Mode"]
pub type ShpR = crate::BitReader;
#[doc = "Field `SHP` writer - ADC12 Sample/Hold Pulse Mode"]
pub type ShpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC12 Sample/Hold Source 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Shs {
    #[doc = "0: ADC12 Sample/Hold Source: 0"]
    Shs0 = 0,
    #[doc = "1: ADC12 Sample/Hold Source: 1"]
    Shs1 = 1,
    #[doc = "2: ADC12 Sample/Hold Source: 2"]
    Shs2 = 2,
    #[doc = "3: ADC12 Sample/Hold Source: 3"]
    Shs3 = 3,
}
impl From<Shs> for u8 {
    #[inline(always)]
    fn from(variant: Shs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Shs {
    type Ux = u8;
}
impl crate::IsEnum for Shs {}
#[doc = "Field `SHS` reader - ADC12 Sample/Hold Source 0"]
pub type ShsR = crate::FieldReader<Shs>;
impl ShsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shs {
        match self.bits {
            0 => Shs::Shs0,
            1 => Shs::Shs1,
            2 => Shs::Shs2,
            3 => Shs::Shs3,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC12 Sample/Hold Source: 0"]
    #[inline(always)]
    pub fn is_shs_0(&self) -> bool {
        *self == Shs::Shs0
    }
    #[doc = "ADC12 Sample/Hold Source: 1"]
    #[inline(always)]
    pub fn is_shs_1(&self) -> bool {
        *self == Shs::Shs1
    }
    #[doc = "ADC12 Sample/Hold Source: 2"]
    #[inline(always)]
    pub fn is_shs_2(&self) -> bool {
        *self == Shs::Shs2
    }
    #[doc = "ADC12 Sample/Hold Source: 3"]
    #[inline(always)]
    pub fn is_shs_3(&self) -> bool {
        *self == Shs::Shs3
    }
}
#[doc = "Field `SHS` writer - ADC12 Sample/Hold Source 0"]
pub type ShsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Shs, crate::Safe>;
impl<'a, REG> ShsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12 Sample/Hold Source: 0"]
    #[inline(always)]
    pub fn shs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Shs::Shs0)
    }
    #[doc = "ADC12 Sample/Hold Source: 1"]
    #[inline(always)]
    pub fn shs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Shs::Shs1)
    }
    #[doc = "ADC12 Sample/Hold Source: 2"]
    #[inline(always)]
    pub fn shs_2(self) -> &'a mut crate::W<REG> {
        self.variant(Shs::Shs2)
    }
    #[doc = "ADC12 Sample/Hold Source: 3"]
    #[inline(always)]
    pub fn shs_3(self) -> &'a mut crate::W<REG> {
        self.variant(Shs::Shs3)
    }
}
#[doc = "ADC12 Conversion Start Address 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cstartadd {
    #[doc = "0: ADC12 Conversion Start Address: 0"]
    Cstartadd0 = 0,
    #[doc = "1: ADC12 Conversion Start Address: 1"]
    Cstartadd1 = 1,
    #[doc = "2: ADC12 Conversion Start Address: 2"]
    Cstartadd2 = 2,
    #[doc = "3: ADC12 Conversion Start Address: 3"]
    Cstartadd3 = 3,
    #[doc = "4: ADC12 Conversion Start Address: 4"]
    Cstartadd4 = 4,
    #[doc = "5: ADC12 Conversion Start Address: 5"]
    Cstartadd5 = 5,
    #[doc = "6: ADC12 Conversion Start Address: 6"]
    Cstartadd6 = 6,
    #[doc = "7: ADC12 Conversion Start Address: 7"]
    Cstartadd7 = 7,
    #[doc = "8: ADC12 Conversion Start Address: 8"]
    Cstartadd8 = 8,
    #[doc = "9: ADC12 Conversion Start Address: 9"]
    Cstartadd9 = 9,
    #[doc = "10: ADC12 Conversion Start Address: 10"]
    Cstartadd10 = 10,
    #[doc = "11: ADC12 Conversion Start Address: 11"]
    Cstartadd11 = 11,
    #[doc = "12: ADC12 Conversion Start Address: 12"]
    Cstartadd12 = 12,
    #[doc = "13: ADC12 Conversion Start Address: 13"]
    Cstartadd13 = 13,
    #[doc = "14: ADC12 Conversion Start Address: 14"]
    Cstartadd14 = 14,
    #[doc = "15: ADC12 Conversion Start Address: 15"]
    Cstartadd15 = 15,
}
impl From<Cstartadd> for u8 {
    #[inline(always)]
    fn from(variant: Cstartadd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cstartadd {
    type Ux = u8;
}
impl crate::IsEnum for Cstartadd {}
#[doc = "Field `CSTARTADD` reader - ADC12 Conversion Start Address 0"]
pub type CstartaddR = crate::FieldReader<Cstartadd>;
impl CstartaddR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstartadd {
        match self.bits {
            0 => Cstartadd::Cstartadd0,
            1 => Cstartadd::Cstartadd1,
            2 => Cstartadd::Cstartadd2,
            3 => Cstartadd::Cstartadd3,
            4 => Cstartadd::Cstartadd4,
            5 => Cstartadd::Cstartadd5,
            6 => Cstartadd::Cstartadd6,
            7 => Cstartadd::Cstartadd7,
            8 => Cstartadd::Cstartadd8,
            9 => Cstartadd::Cstartadd9,
            10 => Cstartadd::Cstartadd10,
            11 => Cstartadd::Cstartadd11,
            12 => Cstartadd::Cstartadd12,
            13 => Cstartadd::Cstartadd13,
            14 => Cstartadd::Cstartadd14,
            15 => Cstartadd::Cstartadd15,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC12 Conversion Start Address: 0"]
    #[inline(always)]
    pub fn is_cstartadd_0(&self) -> bool {
        *self == Cstartadd::Cstartadd0
    }
    #[doc = "ADC12 Conversion Start Address: 1"]
    #[inline(always)]
    pub fn is_cstartadd_1(&self) -> bool {
        *self == Cstartadd::Cstartadd1
    }
    #[doc = "ADC12 Conversion Start Address: 2"]
    #[inline(always)]
    pub fn is_cstartadd_2(&self) -> bool {
        *self == Cstartadd::Cstartadd2
    }
    #[doc = "ADC12 Conversion Start Address: 3"]
    #[inline(always)]
    pub fn is_cstartadd_3(&self) -> bool {
        *self == Cstartadd::Cstartadd3
    }
    #[doc = "ADC12 Conversion Start Address: 4"]
    #[inline(always)]
    pub fn is_cstartadd_4(&self) -> bool {
        *self == Cstartadd::Cstartadd4
    }
    #[doc = "ADC12 Conversion Start Address: 5"]
    #[inline(always)]
    pub fn is_cstartadd_5(&self) -> bool {
        *self == Cstartadd::Cstartadd5
    }
    #[doc = "ADC12 Conversion Start Address: 6"]
    #[inline(always)]
    pub fn is_cstartadd_6(&self) -> bool {
        *self == Cstartadd::Cstartadd6
    }
    #[doc = "ADC12 Conversion Start Address: 7"]
    #[inline(always)]
    pub fn is_cstartadd_7(&self) -> bool {
        *self == Cstartadd::Cstartadd7
    }
    #[doc = "ADC12 Conversion Start Address: 8"]
    #[inline(always)]
    pub fn is_cstartadd_8(&self) -> bool {
        *self == Cstartadd::Cstartadd8
    }
    #[doc = "ADC12 Conversion Start Address: 9"]
    #[inline(always)]
    pub fn is_cstartadd_9(&self) -> bool {
        *self == Cstartadd::Cstartadd9
    }
    #[doc = "ADC12 Conversion Start Address: 10"]
    #[inline(always)]
    pub fn is_cstartadd_10(&self) -> bool {
        *self == Cstartadd::Cstartadd10
    }
    #[doc = "ADC12 Conversion Start Address: 11"]
    #[inline(always)]
    pub fn is_cstartadd_11(&self) -> bool {
        *self == Cstartadd::Cstartadd11
    }
    #[doc = "ADC12 Conversion Start Address: 12"]
    #[inline(always)]
    pub fn is_cstartadd_12(&self) -> bool {
        *self == Cstartadd::Cstartadd12
    }
    #[doc = "ADC12 Conversion Start Address: 13"]
    #[inline(always)]
    pub fn is_cstartadd_13(&self) -> bool {
        *self == Cstartadd::Cstartadd13
    }
    #[doc = "ADC12 Conversion Start Address: 14"]
    #[inline(always)]
    pub fn is_cstartadd_14(&self) -> bool {
        *self == Cstartadd::Cstartadd14
    }
    #[doc = "ADC12 Conversion Start Address: 15"]
    #[inline(always)]
    pub fn is_cstartadd_15(&self) -> bool {
        *self == Cstartadd::Cstartadd15
    }
}
#[doc = "Field `CSTARTADD` writer - ADC12 Conversion Start Address 0"]
pub type CstartaddW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cstartadd, crate::Safe>;
impl<'a, REG> CstartaddW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12 Conversion Start Address: 0"]
    #[inline(always)]
    pub fn cstartadd_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd0)
    }
    #[doc = "ADC12 Conversion Start Address: 1"]
    #[inline(always)]
    pub fn cstartadd_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd1)
    }
    #[doc = "ADC12 Conversion Start Address: 2"]
    #[inline(always)]
    pub fn cstartadd_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd2)
    }
    #[doc = "ADC12 Conversion Start Address: 3"]
    #[inline(always)]
    pub fn cstartadd_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd3)
    }
    #[doc = "ADC12 Conversion Start Address: 4"]
    #[inline(always)]
    pub fn cstartadd_4(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd4)
    }
    #[doc = "ADC12 Conversion Start Address: 5"]
    #[inline(always)]
    pub fn cstartadd_5(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd5)
    }
    #[doc = "ADC12 Conversion Start Address: 6"]
    #[inline(always)]
    pub fn cstartadd_6(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd6)
    }
    #[doc = "ADC12 Conversion Start Address: 7"]
    #[inline(always)]
    pub fn cstartadd_7(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd7)
    }
    #[doc = "ADC12 Conversion Start Address: 8"]
    #[inline(always)]
    pub fn cstartadd_8(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd8)
    }
    #[doc = "ADC12 Conversion Start Address: 9"]
    #[inline(always)]
    pub fn cstartadd_9(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd9)
    }
    #[doc = "ADC12 Conversion Start Address: 10"]
    #[inline(always)]
    pub fn cstartadd_10(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd10)
    }
    #[doc = "ADC12 Conversion Start Address: 11"]
    #[inline(always)]
    pub fn cstartadd_11(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd11)
    }
    #[doc = "ADC12 Conversion Start Address: 12"]
    #[inline(always)]
    pub fn cstartadd_12(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd12)
    }
    #[doc = "ADC12 Conversion Start Address: 13"]
    #[inline(always)]
    pub fn cstartadd_13(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd13)
    }
    #[doc = "ADC12 Conversion Start Address: 14"]
    #[inline(always)]
    pub fn cstartadd_14(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd14)
    }
    #[doc = "ADC12 Conversion Start Address: 15"]
    #[inline(always)]
    pub fn cstartadd_15(self) -> &'a mut crate::W<REG> {
        self.variant(Cstartadd::Cstartadd15)
    }
}
impl R {
    #[doc = "Bit 0 - ADC12 Busy"]
    #[inline(always)]
    pub fn adc12busy(&self) -> Adc12busyR {
        Adc12busyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - ADC12 Conversion Sequence Select 0"]
    #[inline(always)]
    pub fn conseq(&self) -> ConseqR {
        ConseqR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - ADC12 Clock Source Select 0"]
    #[inline(always)]
    pub fn adc12ssel(&self) -> Adc12sselR {
        Adc12sselR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - ADC12 Clock Divider Select 0"]
    #[inline(always)]
    pub fn adc12div(&self) -> Adc12divR {
        Adc12divR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - ADC12 Invert Sample Hold Signal"]
    #[inline(always)]
    pub fn issh(&self) -> IsshR {
        IsshR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC12 Sample/Hold Pulse Mode"]
    #[inline(always)]
    pub fn shp(&self) -> ShpR {
        ShpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - ADC12 Sample/Hold Source 0"]
    #[inline(always)]
    pub fn shs(&self) -> ShsR {
        ShsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - ADC12 Conversion Start Address 0"]
    #[inline(always)]
    pub fn cstartadd(&self) -> CstartaddR {
        CstartaddR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Busy"]
    #[inline(always)]
    pub fn adc12busy(&mut self) -> Adc12busyW<'_, Adc12ctl1Spec> {
        Adc12busyW::new(self, 0)
    }
    #[doc = "Bits 1:2 - ADC12 Conversion Sequence Select 0"]
    #[inline(always)]
    pub fn conseq(&mut self) -> ConseqW<'_, Adc12ctl1Spec> {
        ConseqW::new(self, 1)
    }
    #[doc = "Bits 3:4 - ADC12 Clock Source Select 0"]
    #[inline(always)]
    pub fn adc12ssel(&mut self) -> Adc12sselW<'_, Adc12ctl1Spec> {
        Adc12sselW::new(self, 3)
    }
    #[doc = "Bits 5:7 - ADC12 Clock Divider Select 0"]
    #[inline(always)]
    pub fn adc12div(&mut self) -> Adc12divW<'_, Adc12ctl1Spec> {
        Adc12divW::new(self, 5)
    }
    #[doc = "Bit 8 - ADC12 Invert Sample Hold Signal"]
    #[inline(always)]
    pub fn issh(&mut self) -> IsshW<'_, Adc12ctl1Spec> {
        IsshW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC12 Sample/Hold Pulse Mode"]
    #[inline(always)]
    pub fn shp(&mut self) -> ShpW<'_, Adc12ctl1Spec> {
        ShpW::new(self, 9)
    }
    #[doc = "Bits 10:11 - ADC12 Sample/Hold Source 0"]
    #[inline(always)]
    pub fn shs(&mut self) -> ShsW<'_, Adc12ctl1Spec> {
        ShsW::new(self, 10)
    }
    #[doc = "Bits 12:15 - ADC12 Conversion Start Address 0"]
    #[inline(always)]
    pub fn cstartadd(&mut self) -> CstartaddW<'_, Adc12ctl1Spec> {
        CstartaddW::new(self, 12)
    }
}
#[doc = "ADC12 Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
