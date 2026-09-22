#[doc = "Register `ADCCTL1` reader"]
pub type R = crate::R<Adcctl1Spec>;
#[doc = "Register `ADCCTL1` writer"]
pub type W = crate::W<Adcctl1Spec>;
#[doc = "Field `ADCBUSY` reader - ADC Busy"]
pub type AdcbusyR = crate::BitReader;
#[doc = "Field `ADCBUSY` writer - ADC Busy"]
pub type AdcbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC Conversion Sequence Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcconseq {
    #[doc = "0: ADC Conversion Sequence Select: 0"]
    Adcconseq0 = 0,
    #[doc = "1: ADC Conversion Sequence Select: 1"]
    Adcconseq1 = 1,
    #[doc = "2: ADC Conversion Sequence Select: 2"]
    Adcconseq2 = 2,
    #[doc = "3: ADC Conversion Sequence Select: 3"]
    Adcconseq3 = 3,
}
impl From<Adcconseq> for u8 {
    #[inline(always)]
    fn from(variant: Adcconseq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcconseq {
    type Ux = u8;
}
impl crate::IsEnum for Adcconseq {}
#[doc = "Field `ADCCONSEQ` reader - ADC Conversion Sequence Select 0"]
pub type AdcconseqR = crate::FieldReader<Adcconseq>;
impl AdcconseqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcconseq {
        match self.bits {
            0 => Adcconseq::Adcconseq0,
            1 => Adcconseq::Adcconseq1,
            2 => Adcconseq::Adcconseq2,
            3 => Adcconseq::Adcconseq3,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC Conversion Sequence Select: 0"]
    #[inline(always)]
    pub fn is_adcconseq_0(&self) -> bool {
        *self == Adcconseq::Adcconseq0
    }
    #[doc = "ADC Conversion Sequence Select: 1"]
    #[inline(always)]
    pub fn is_adcconseq_1(&self) -> bool {
        *self == Adcconseq::Adcconseq1
    }
    #[doc = "ADC Conversion Sequence Select: 2"]
    #[inline(always)]
    pub fn is_adcconseq_2(&self) -> bool {
        *self == Adcconseq::Adcconseq2
    }
    #[doc = "ADC Conversion Sequence Select: 3"]
    #[inline(always)]
    pub fn is_adcconseq_3(&self) -> bool {
        *self == Adcconseq::Adcconseq3
    }
}
#[doc = "Field `ADCCONSEQ` writer - ADC Conversion Sequence Select 0"]
pub type AdcconseqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adcconseq, crate::Safe>;
impl<'a, REG> AdcconseqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC Conversion Sequence Select: 0"]
    #[inline(always)]
    pub fn adcconseq_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcconseq::Adcconseq0)
    }
    #[doc = "ADC Conversion Sequence Select: 1"]
    #[inline(always)]
    pub fn adcconseq_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcconseq::Adcconseq1)
    }
    #[doc = "ADC Conversion Sequence Select: 2"]
    #[inline(always)]
    pub fn adcconseq_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcconseq::Adcconseq2)
    }
    #[doc = "ADC Conversion Sequence Select: 3"]
    #[inline(always)]
    pub fn adcconseq_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adcconseq::Adcconseq3)
    }
}
#[doc = "ADC Clock Source Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcssel {
    #[doc = "0: ADC Clock Source Select: 0"]
    Adcssel0 = 0,
    #[doc = "1: ADC Clock Source Select: 1"]
    Adcssel1 = 1,
    #[doc = "2: ADC Clock Source Select: 2"]
    Adcssel2 = 2,
    #[doc = "3: ADC Clock Source Select: 3"]
    Adcssel3 = 3,
}
impl From<Adcssel> for u8 {
    #[inline(always)]
    fn from(variant: Adcssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcssel {
    type Ux = u8;
}
impl crate::IsEnum for Adcssel {}
#[doc = "Field `ADCSSEL` reader - ADC Clock Source Select 0"]
pub type AdcsselR = crate::FieldReader<Adcssel>;
impl AdcsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcssel {
        match self.bits {
            0 => Adcssel::Adcssel0,
            1 => Adcssel::Adcssel1,
            2 => Adcssel::Adcssel2,
            3 => Adcssel::Adcssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC Clock Source Select: 0"]
    #[inline(always)]
    pub fn is_adcssel_0(&self) -> bool {
        *self == Adcssel::Adcssel0
    }
    #[doc = "ADC Clock Source Select: 1"]
    #[inline(always)]
    pub fn is_adcssel_1(&self) -> bool {
        *self == Adcssel::Adcssel1
    }
    #[doc = "ADC Clock Source Select: 2"]
    #[inline(always)]
    pub fn is_adcssel_2(&self) -> bool {
        *self == Adcssel::Adcssel2
    }
    #[doc = "ADC Clock Source Select: 3"]
    #[inline(always)]
    pub fn is_adcssel_3(&self) -> bool {
        *self == Adcssel::Adcssel3
    }
}
#[doc = "Field `ADCSSEL` writer - ADC Clock Source Select 0"]
pub type AdcsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adcssel, crate::Safe>;
impl<'a, REG> AdcsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC Clock Source Select: 0"]
    #[inline(always)]
    pub fn adcssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcssel::Adcssel0)
    }
    #[doc = "ADC Clock Source Select: 1"]
    #[inline(always)]
    pub fn adcssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcssel::Adcssel1)
    }
    #[doc = "ADC Clock Source Select: 2"]
    #[inline(always)]
    pub fn adcssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcssel::Adcssel2)
    }
    #[doc = "ADC Clock Source Select: 3"]
    #[inline(always)]
    pub fn adcssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adcssel::Adcssel3)
    }
}
#[doc = "ADC Clock Divider Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcdiv {
    #[doc = "0: ADC Clock Divider Select: 0"]
    Adcdiv0 = 0,
    #[doc = "1: ADC Clock Divider Select: 1"]
    Adcdiv1 = 1,
    #[doc = "2: ADC Clock Divider Select: 2"]
    Adcdiv2 = 2,
    #[doc = "3: ADC Clock Divider Select: 3"]
    Adcdiv3 = 3,
    #[doc = "4: ADC Clock Divider Select: 4"]
    Adcdiv4 = 4,
    #[doc = "5: ADC Clock Divider Select: 5"]
    Adcdiv5 = 5,
    #[doc = "6: ADC Clock Divider Select: 6"]
    Adcdiv6 = 6,
    #[doc = "7: ADC Clock Divider Select: 7"]
    Adcdiv7 = 7,
}
impl From<Adcdiv> for u8 {
    #[inline(always)]
    fn from(variant: Adcdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcdiv {
    type Ux = u8;
}
impl crate::IsEnum for Adcdiv {}
#[doc = "Field `ADCDIV` reader - ADC Clock Divider Select 0"]
pub type AdcdivR = crate::FieldReader<Adcdiv>;
impl AdcdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcdiv {
        match self.bits {
            0 => Adcdiv::Adcdiv0,
            1 => Adcdiv::Adcdiv1,
            2 => Adcdiv::Adcdiv2,
            3 => Adcdiv::Adcdiv3,
            4 => Adcdiv::Adcdiv4,
            5 => Adcdiv::Adcdiv5,
            6 => Adcdiv::Adcdiv6,
            7 => Adcdiv::Adcdiv7,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC Clock Divider Select: 0"]
    #[inline(always)]
    pub fn is_adcdiv_0(&self) -> bool {
        *self == Adcdiv::Adcdiv0
    }
    #[doc = "ADC Clock Divider Select: 1"]
    #[inline(always)]
    pub fn is_adcdiv_1(&self) -> bool {
        *self == Adcdiv::Adcdiv1
    }
    #[doc = "ADC Clock Divider Select: 2"]
    #[inline(always)]
    pub fn is_adcdiv_2(&self) -> bool {
        *self == Adcdiv::Adcdiv2
    }
    #[doc = "ADC Clock Divider Select: 3"]
    #[inline(always)]
    pub fn is_adcdiv_3(&self) -> bool {
        *self == Adcdiv::Adcdiv3
    }
    #[doc = "ADC Clock Divider Select: 4"]
    #[inline(always)]
    pub fn is_adcdiv_4(&self) -> bool {
        *self == Adcdiv::Adcdiv4
    }
    #[doc = "ADC Clock Divider Select: 5"]
    #[inline(always)]
    pub fn is_adcdiv_5(&self) -> bool {
        *self == Adcdiv::Adcdiv5
    }
    #[doc = "ADC Clock Divider Select: 6"]
    #[inline(always)]
    pub fn is_adcdiv_6(&self) -> bool {
        *self == Adcdiv::Adcdiv6
    }
    #[doc = "ADC Clock Divider Select: 7"]
    #[inline(always)]
    pub fn is_adcdiv_7(&self) -> bool {
        *self == Adcdiv::Adcdiv7
    }
}
#[doc = "Field `ADCDIV` writer - ADC Clock Divider Select 0"]
pub type AdcdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adcdiv, crate::Safe>;
impl<'a, REG> AdcdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC Clock Divider Select: 0"]
    #[inline(always)]
    pub fn adcdiv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::Adcdiv0)
    }
    #[doc = "ADC Clock Divider Select: 1"]
    #[inline(always)]
    pub fn adcdiv_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::Adcdiv1)
    }
    #[doc = "ADC Clock Divider Select: 2"]
    #[inline(always)]
    pub fn adcdiv_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::Adcdiv2)
    }
    #[doc = "ADC Clock Divider Select: 3"]
    #[inline(always)]
    pub fn adcdiv_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::Adcdiv3)
    }
    #[doc = "ADC Clock Divider Select: 4"]
    #[inline(always)]
    pub fn adcdiv_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::Adcdiv4)
    }
    #[doc = "ADC Clock Divider Select: 5"]
    #[inline(always)]
    pub fn adcdiv_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::Adcdiv5)
    }
    #[doc = "ADC Clock Divider Select: 6"]
    #[inline(always)]
    pub fn adcdiv_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::Adcdiv6)
    }
    #[doc = "ADC Clock Divider Select: 7"]
    #[inline(always)]
    pub fn adcdiv_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::Adcdiv7)
    }
}
#[doc = "Field `ADCISSH` reader - ADC Invert Sample Hold Signal"]
pub type AdcisshR = crate::BitReader;
#[doc = "Field `ADCISSH` writer - ADC Invert Sample Hold Signal"]
pub type AdcisshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCSHP` reader - ADC Sample/Hold Pulse Mode"]
pub type AdcshpR = crate::BitReader;
#[doc = "Field `ADCSHP` writer - ADC Sample/Hold Pulse Mode"]
pub type AdcshpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC Sample/Hold Source 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcshs {
    #[doc = "0: ADC Sample/Hold Source: 0"]
    Adcshs0 = 0,
    #[doc = "1: ADC Sample/Hold Source: 1"]
    Adcshs1 = 1,
    #[doc = "2: ADC Sample/Hold Source: 2"]
    Adcshs2 = 2,
    #[doc = "3: ADC Sample/Hold Source: 3"]
    Adcshs3 = 3,
}
impl From<Adcshs> for u8 {
    #[inline(always)]
    fn from(variant: Adcshs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcshs {
    type Ux = u8;
}
impl crate::IsEnum for Adcshs {}
#[doc = "Field `ADCSHS` reader - ADC Sample/Hold Source 0"]
pub type AdcshsR = crate::FieldReader<Adcshs>;
impl AdcshsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcshs {
        match self.bits {
            0 => Adcshs::Adcshs0,
            1 => Adcshs::Adcshs1,
            2 => Adcshs::Adcshs2,
            3 => Adcshs::Adcshs3,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC Sample/Hold Source: 0"]
    #[inline(always)]
    pub fn is_adcshs_0(&self) -> bool {
        *self == Adcshs::Adcshs0
    }
    #[doc = "ADC Sample/Hold Source: 1"]
    #[inline(always)]
    pub fn is_adcshs_1(&self) -> bool {
        *self == Adcshs::Adcshs1
    }
    #[doc = "ADC Sample/Hold Source: 2"]
    #[inline(always)]
    pub fn is_adcshs_2(&self) -> bool {
        *self == Adcshs::Adcshs2
    }
    #[doc = "ADC Sample/Hold Source: 3"]
    #[inline(always)]
    pub fn is_adcshs_3(&self) -> bool {
        *self == Adcshs::Adcshs3
    }
}
#[doc = "Field `ADCSHS` writer - ADC Sample/Hold Source 0"]
pub type AdcshsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adcshs, crate::Safe>;
impl<'a, REG> AdcshsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC Sample/Hold Source: 0"]
    #[inline(always)]
    pub fn adcshs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcshs::Adcshs0)
    }
    #[doc = "ADC Sample/Hold Source: 1"]
    #[inline(always)]
    pub fn adcshs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcshs::Adcshs1)
    }
    #[doc = "ADC Sample/Hold Source: 2"]
    #[inline(always)]
    pub fn adcshs_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcshs::Adcshs2)
    }
    #[doc = "ADC Sample/Hold Source: 3"]
    #[inline(always)]
    pub fn adcshs_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adcshs::Adcshs3)
    }
}
impl R {
    #[doc = "Bit 0 - ADC Busy"]
    #[inline(always)]
    pub fn adcbusy(&self) -> AdcbusyR {
        AdcbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - ADC Conversion Sequence Select 0"]
    #[inline(always)]
    pub fn adcconseq(&self) -> AdcconseqR {
        AdcconseqR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - ADC Clock Source Select 0"]
    #[inline(always)]
    pub fn adcssel(&self) -> AdcsselR {
        AdcsselR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - ADC Clock Divider Select 0"]
    #[inline(always)]
    pub fn adcdiv(&self) -> AdcdivR {
        AdcdivR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - ADC Invert Sample Hold Signal"]
    #[inline(always)]
    pub fn adcissh(&self) -> AdcisshR {
        AdcisshR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC Sample/Hold Pulse Mode"]
    #[inline(always)]
    pub fn adcshp(&self) -> AdcshpR {
        AdcshpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - ADC Sample/Hold Source 0"]
    #[inline(always)]
    pub fn adcshs(&self) -> AdcshsR {
        AdcshsR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Busy"]
    #[inline(always)]
    pub fn adcbusy(&mut self) -> AdcbusyW<'_, Adcctl1Spec> {
        AdcbusyW::new(self, 0)
    }
    #[doc = "Bits 1:2 - ADC Conversion Sequence Select 0"]
    #[inline(always)]
    pub fn adcconseq(&mut self) -> AdcconseqW<'_, Adcctl1Spec> {
        AdcconseqW::new(self, 1)
    }
    #[doc = "Bits 3:4 - ADC Clock Source Select 0"]
    #[inline(always)]
    pub fn adcssel(&mut self) -> AdcsselW<'_, Adcctl1Spec> {
        AdcsselW::new(self, 3)
    }
    #[doc = "Bits 5:7 - ADC Clock Divider Select 0"]
    #[inline(always)]
    pub fn adcdiv(&mut self) -> AdcdivW<'_, Adcctl1Spec> {
        AdcdivW::new(self, 5)
    }
    #[doc = "Bit 8 - ADC Invert Sample Hold Signal"]
    #[inline(always)]
    pub fn adcissh(&mut self) -> AdcisshW<'_, Adcctl1Spec> {
        AdcisshW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC Sample/Hold Pulse Mode"]
    #[inline(always)]
    pub fn adcshp(&mut self) -> AdcshpW<'_, Adcctl1Spec> {
        AdcshpW::new(self, 9)
    }
    #[doc = "Bits 10:11 - ADC Sample/Hold Source 0"]
    #[inline(always)]
    pub fn adcshs(&mut self) -> AdcshsW<'_, Adcctl1Spec> {
        AdcshsW::new(self, 10)
    }
}
#[doc = "ADC Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcctl1Spec;
impl crate::RegisterSpec for Adcctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcctl1::R`](R) reader structure"]
impl crate::Readable for Adcctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcctl1::W`](W) writer structure"]
impl crate::Writable for Adcctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCCTL1 to value 0"]
impl crate::Resettable for Adcctl1Spec {}
