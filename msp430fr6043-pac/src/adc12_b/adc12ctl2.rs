#[doc = "Register `ADC12CTL2` reader"]
pub type R = crate::R<Adc12ctl2Spec>;
#[doc = "Register `ADC12CTL2` writer"]
pub type W = crate::W<Adc12ctl2Spec>;
#[doc = "low-power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12pwrmd {
    #[doc = "0: Regular power mode where sample rate is not restricted"]
    Adc12pwrmd0 = 0,
    #[doc = "1: Low power mode enable, ADC12CLK can not be greater than 1/4 the device-specific data sheet specified maximum for ADC12PWRMD = 0"]
    Adc12pwrmd1 = 1,
}
impl From<Adc12pwrmd> for bool {
    #[inline(always)]
    fn from(variant: Adc12pwrmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12PWRMD` reader - low-power mode"]
pub type Adc12pwrmdR = crate::BitReader<Adc12pwrmd>;
impl Adc12pwrmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12pwrmd {
        match self.bits {
            false => Adc12pwrmd::Adc12pwrmd0,
            true => Adc12pwrmd::Adc12pwrmd1,
        }
    }
    #[doc = "Regular power mode where sample rate is not restricted"]
    #[inline(always)]
    pub fn is_adc12pwrmd_0(&self) -> bool {
        *self == Adc12pwrmd::Adc12pwrmd0
    }
    #[doc = "Low power mode enable, ADC12CLK can not be greater than 1/4 the device-specific data sheet specified maximum for ADC12PWRMD = 0"]
    #[inline(always)]
    pub fn is_adc12pwrmd_1(&self) -> bool {
        *self == Adc12pwrmd::Adc12pwrmd1
    }
}
#[doc = "Field `ADC12PWRMD` writer - low-power mode"]
pub type Adc12pwrmdW<'a, REG> = crate::BitWriter<'a, REG, Adc12pwrmd>;
impl<'a, REG> Adc12pwrmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regular power mode where sample rate is not restricted"]
    #[inline(always)]
    pub fn adc12pwrmd_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12pwrmd::Adc12pwrmd0)
    }
    #[doc = "Low power mode enable, ADC12CLK can not be greater than 1/4 the device-specific data sheet specified maximum for ADC12PWRMD = 0"]
    #[inline(always)]
    pub fn adc12pwrmd_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12pwrmd::Adc12pwrmd1)
    }
}
#[doc = "data read-back format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12df {
    #[doc = "0: Binary unsigned. Theoretically for ADC12DIF = 0 and 12-bit mode the analog input voltage VREF results in 0000h, the analog input voltage + VREF results in 0FFFh."]
    Adc12df0 = 0,
    #[doc = "1: Signed binary (2s complement), left aligned. Theoretically, for ADC12DIF = 0 and 12-bit mode, the analog input voltage VREF results in 8000h, the analog input voltage + VREF results in 7FF0h."]
    Adc12df1 = 1,
}
impl From<Adc12df> for bool {
    #[inline(always)]
    fn from(variant: Adc12df) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12DF` reader - data read-back format"]
pub type Adc12dfR = crate::BitReader<Adc12df>;
impl Adc12dfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12df {
        match self.bits {
            false => Adc12df::Adc12df0,
            true => Adc12df::Adc12df1,
        }
    }
    #[doc = "Binary unsigned. Theoretically for ADC12DIF = 0 and 12-bit mode the analog input voltage VREF results in 0000h, the analog input voltage + VREF results in 0FFFh."]
    #[inline(always)]
    pub fn is_adc12df_0(&self) -> bool {
        *self == Adc12df::Adc12df0
    }
    #[doc = "Signed binary (2s complement), left aligned. Theoretically, for ADC12DIF = 0 and 12-bit mode, the analog input voltage VREF results in 8000h, the analog input voltage + VREF results in 7FF0h."]
    #[inline(always)]
    pub fn is_adc12df_1(&self) -> bool {
        *self == Adc12df::Adc12df1
    }
}
#[doc = "Field `ADC12DF` writer - data read-back format"]
pub type Adc12dfW<'a, REG> = crate::BitWriter<'a, REG, Adc12df>;
impl<'a, REG> Adc12dfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Binary unsigned. Theoretically for ADC12DIF = 0 and 12-bit mode the analog input voltage VREF results in 0000h, the analog input voltage + VREF results in 0FFFh."]
    #[inline(always)]
    pub fn adc12df_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12df::Adc12df0)
    }
    #[doc = "Signed binary (2s complement), left aligned. Theoretically, for ADC12DIF = 0 and 12-bit mode, the analog input voltage VREF results in 8000h, the analog input voltage + VREF results in 7FF0h."]
    #[inline(always)]
    pub fn adc12df_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12df::Adc12df1)
    }
}
#[doc = "resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc12res {
    #[doc = "0: 8 bit (10 clock cycle conversion time)"]
    _8bit = 0,
    #[doc = "1: 10 bit (12 clock cycle conversion time)"]
    _10bit = 1,
    #[doc = "2: 12 bit (14 clock cycle conversion time)"]
    _12bit = 2,
    #[doc = "3: Reserved"]
    Adc12res3 = 3,
}
impl From<Adc12res> for u8 {
    #[inline(always)]
    fn from(variant: Adc12res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc12res {
    type Ux = u8;
}
impl crate::IsEnum for Adc12res {}
#[doc = "Field `ADC12RES` reader - resolution"]
pub type Adc12resR = crate::FieldReader<Adc12res>;
impl Adc12resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12res {
        match self.bits {
            0 => Adc12res::_8bit,
            1 => Adc12res::_10bit,
            2 => Adc12res::_12bit,
            3 => Adc12res::Adc12res3,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bit (10 clock cycle conversion time)"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Adc12res::_8bit
    }
    #[doc = "10 bit (12 clock cycle conversion time)"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == Adc12res::_10bit
    }
    #[doc = "12 bit (14 clock cycle conversion time)"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == Adc12res::_12bit
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adc12res_3(&self) -> bool {
        *self == Adc12res::Adc12res3
    }
}
#[doc = "Field `ADC12RES` writer - resolution"]
pub type Adc12resW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc12res, crate::Safe>;
impl<'a, REG> Adc12resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bit (10 clock cycle conversion time)"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12res::_8bit)
    }
    #[doc = "10 bit (12 clock cycle conversion time)"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12res::_10bit)
    }
    #[doc = "12 bit (14 clock cycle conversion time)"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12res::_12bit)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12res_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12res::Adc12res3)
    }
}
impl R {
    #[doc = "Bit 0 - low-power mode"]
    #[inline(always)]
    pub fn adc12pwrmd(&self) -> Adc12pwrmdR {
        Adc12pwrmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - data read-back format"]
    #[inline(always)]
    pub fn adc12df(&self) -> Adc12dfR {
        Adc12dfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - resolution"]
    #[inline(always)]
    pub fn adc12res(&self) -> Adc12resR {
        Adc12resR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - low-power mode"]
    #[inline(always)]
    pub fn adc12pwrmd(&mut self) -> Adc12pwrmdW<'_, Adc12ctl2Spec> {
        Adc12pwrmdW::new(self, 0)
    }
    #[doc = "Bit 3 - data read-back format"]
    #[inline(always)]
    pub fn adc12df(&mut self) -> Adc12dfW<'_, Adc12ctl2Spec> {
        Adc12dfW::new(self, 3)
    }
    #[doc = "Bits 4:5 - resolution"]
    #[inline(always)]
    pub fn adc12res(&mut self) -> Adc12resW<'_, Adc12ctl2Spec> {
        Adc12resW::new(self, 4)
    }
}
#[doc = "ADC12_B Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12ctl2Spec;
impl crate::RegisterSpec for Adc12ctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12ctl2::R`](R) reader structure"]
impl crate::Readable for Adc12ctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`adc12ctl2::W`](W) writer structure"]
impl crate::Writable for Adc12ctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC12CTL2 to value 0"]
impl crate::Resettable for Adc12ctl2Spec {}
