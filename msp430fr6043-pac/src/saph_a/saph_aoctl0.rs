#[doc = "Register `SAPH_AOCTL0` reader"]
pub type R = crate::R<SaphAoctl0Spec>;
#[doc = "Register `SAPH_AOCTL0` writer"]
pub type W = crate::W<SaphAoctl0Spec>;
#[doc = "CH0_OUT Enable. When OSEL.PCH0SEL =0, this bit enables the output CH0 when set to 1. When OSEL.PCH0SEL != 0, this bit is invalid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0oe {
    #[doc = "0: Ch0 Output is HiZ"]
    Ch0oe0 = 0,
    #[doc = "1: CH0 Output is driving"]
    Ch0oe1 = 1,
}
impl From<Ch0oe> for bool {
    #[inline(always)]
    fn from(variant: Ch0oe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0OE` reader - CH0_OUT Enable. When OSEL.PCH0SEL =0, this bit enables the output CH0 when set to 1. When OSEL.PCH0SEL != 0, this bit is invalid."]
pub type Ch0oeR = crate::BitReader<Ch0oe>;
impl Ch0oeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0oe {
        match self.bits {
            false => Ch0oe::Ch0oe0,
            true => Ch0oe::Ch0oe1,
        }
    }
    #[doc = "Ch0 Output is HiZ"]
    #[inline(always)]
    pub fn is_ch0oe_0(&self) -> bool {
        *self == Ch0oe::Ch0oe0
    }
    #[doc = "CH0 Output is driving"]
    #[inline(always)]
    pub fn is_ch0oe_1(&self) -> bool {
        *self == Ch0oe::Ch0oe1
    }
}
#[doc = "Field `CH0OE` writer - CH0_OUT Enable. When OSEL.PCH0SEL =0, this bit enables the output CH0 when set to 1. When OSEL.PCH0SEL != 0, this bit is invalid."]
pub type Ch0oeW<'a, REG> = crate::BitWriter<'a, REG, Ch0oe>;
impl<'a, REG> Ch0oeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ch0 Output is HiZ"]
    #[inline(always)]
    pub fn ch0oe_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0oe::Ch0oe0)
    }
    #[doc = "CH0 Output is driving"]
    #[inline(always)]
    pub fn ch0oe_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0oe::Ch0oe1)
    }
}
#[doc = "CH1_OUT Enable. When OSEL.PCH1SEL =0, this bit enables the output CH1 when set to 1. When OSEL.PCH1SEL != 0, this bit is invalid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1oe {
    #[doc = "0: Ch1 Output is HiZ"]
    Ch1oe0 = 0,
    #[doc = "1: CH1 Output is driving"]
    Ch1oe1 = 1,
}
impl From<Ch1oe> for bool {
    #[inline(always)]
    fn from(variant: Ch1oe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1OE` reader - CH1_OUT Enable. When OSEL.PCH1SEL =0, this bit enables the output CH1 when set to 1. When OSEL.PCH1SEL != 0, this bit is invalid."]
pub type Ch1oeR = crate::BitReader<Ch1oe>;
impl Ch1oeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1oe {
        match self.bits {
            false => Ch1oe::Ch1oe0,
            true => Ch1oe::Ch1oe1,
        }
    }
    #[doc = "Ch1 Output is HiZ"]
    #[inline(always)]
    pub fn is_ch1oe_0(&self) -> bool {
        *self == Ch1oe::Ch1oe0
    }
    #[doc = "CH1 Output is driving"]
    #[inline(always)]
    pub fn is_ch1oe_1(&self) -> bool {
        *self == Ch1oe::Ch1oe1
    }
}
#[doc = "Field `CH1OE` writer - CH1_OUT Enable. When OSEL.PCH1SEL =0, this bit enables the output CH1 when set to 1. When OSEL.PCH1SEL != 0, this bit is invalid."]
pub type Ch1oeW<'a, REG> = crate::BitWriter<'a, REG, Ch1oe>;
impl<'a, REG> Ch1oeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ch1 Output is HiZ"]
    #[inline(always)]
    pub fn ch1oe_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1oe::Ch1oe0)
    }
    #[doc = "CH1 Output is driving"]
    #[inline(always)]
    pub fn ch1oe_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1oe::Ch1oe1)
    }
}
#[doc = "CH0_OUT Value. When OSEL.PCH0SEL =0 and OCTL0.CH0OE=1, this bit represents the logical value on the CH0 terminal. 0 = low 1 = high\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0out {
    #[doc = "0: Ch0 is set to low signal"]
    Ch0out0 = 0,
    #[doc = "1: Ch0 is set to high signal"]
    Ch0out1 = 1,
}
impl From<Ch0out> for bool {
    #[inline(always)]
    fn from(variant: Ch0out) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0OUT` reader - CH0_OUT Value. When OSEL.PCH0SEL =0 and OCTL0.CH0OE=1, this bit represents the logical value on the CH0 terminal. 0 = low 1 = high"]
pub type Ch0outR = crate::BitReader<Ch0out>;
impl Ch0outR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0out {
        match self.bits {
            false => Ch0out::Ch0out0,
            true => Ch0out::Ch0out1,
        }
    }
    #[doc = "Ch0 is set to low signal"]
    #[inline(always)]
    pub fn is_ch0out_0(&self) -> bool {
        *self == Ch0out::Ch0out0
    }
    #[doc = "Ch0 is set to high signal"]
    #[inline(always)]
    pub fn is_ch0out_1(&self) -> bool {
        *self == Ch0out::Ch0out1
    }
}
#[doc = "Field `CH0OUT` writer - CH0_OUT Value. When OSEL.PCH0SEL =0 and OCTL0.CH0OE=1, this bit represents the logical value on the CH0 terminal. 0 = low 1 = high"]
pub type Ch0outW<'a, REG> = crate::BitWriter<'a, REG, Ch0out>;
impl<'a, REG> Ch0outW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ch0 is set to low signal"]
    #[inline(always)]
    pub fn ch0out_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0out::Ch0out0)
    }
    #[doc = "Ch0 is set to high signal"]
    #[inline(always)]
    pub fn ch0out_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0out::Ch0out1)
    }
}
#[doc = "CH1_OUT Value. When OSEL.PCH1SEL =0 and OCTL0.CH1OE=1, this bit represents the logical value on the CH1 terminal. 0 = low 1 = high\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1out {
    #[doc = "0: Ch1 is set to low signal"]
    Ch1out0 = 0,
    #[doc = "1: Ch1 is set to high signal"]
    Ch1out1 = 1,
}
impl From<Ch1out> for bool {
    #[inline(always)]
    fn from(variant: Ch1out) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1OUT` reader - CH1_OUT Value. When OSEL.PCH1SEL =0 and OCTL0.CH1OE=1, this bit represents the logical value on the CH1 terminal. 0 = low 1 = high"]
pub type Ch1outR = crate::BitReader<Ch1out>;
impl Ch1outR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1out {
        match self.bits {
            false => Ch1out::Ch1out0,
            true => Ch1out::Ch1out1,
        }
    }
    #[doc = "Ch1 is set to low signal"]
    #[inline(always)]
    pub fn is_ch1out_0(&self) -> bool {
        *self == Ch1out::Ch1out0
    }
    #[doc = "Ch1 is set to high signal"]
    #[inline(always)]
    pub fn is_ch1out_1(&self) -> bool {
        *self == Ch1out::Ch1out1
    }
}
#[doc = "Field `CH1OUT` writer - CH1_OUT Value. When OSEL.PCH1SEL =0 and OCTL0.CH1OE=1, this bit represents the logical value on the CH1 terminal. 0 = low 1 = high"]
pub type Ch1outW<'a, REG> = crate::BitWriter<'a, REG, Ch1out>;
impl<'a, REG> Ch1outW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ch1 is set to low signal"]
    #[inline(always)]
    pub fn ch1out_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1out::Ch1out0)
    }
    #[doc = "Ch1 is set to high signal"]
    #[inline(always)]
    pub fn ch1out_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1out::Ch1out1)
    }
}
impl R {
    #[doc = "Bit 0 - CH0_OUT Enable. When OSEL.PCH0SEL =0, this bit enables the output CH0 when set to 1. When OSEL.PCH0SEL != 0, this bit is invalid."]
    #[inline(always)]
    pub fn ch0oe(&self) -> Ch0oeR {
        Ch0oeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1_OUT Enable. When OSEL.PCH1SEL =0, this bit enables the output CH1 when set to 1. When OSEL.PCH1SEL != 0, this bit is invalid."]
    #[inline(always)]
    pub fn ch1oe(&self) -> Ch1oeR {
        Ch1oeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - CH0_OUT Value. When OSEL.PCH0SEL =0 and OCTL0.CH0OE=1, this bit represents the logical value on the CH0 terminal. 0 = low 1 = high"]
    #[inline(always)]
    pub fn ch0out(&self) -> Ch0outR {
        Ch0outR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CH1_OUT Value. When OSEL.PCH1SEL =0 and OCTL0.CH1OE=1, this bit represents the logical value on the CH1 terminal. 0 = low 1 = high"]
    #[inline(always)]
    pub fn ch1out(&self) -> Ch1outR {
        Ch1outR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0_OUT Enable. When OSEL.PCH0SEL =0, this bit enables the output CH0 when set to 1. When OSEL.PCH0SEL != 0, this bit is invalid."]
    #[inline(always)]
    pub fn ch0oe(&mut self) -> Ch0oeW<'_, SaphAoctl0Spec> {
        Ch0oeW::new(self, 0)
    }
    #[doc = "Bit 1 - CH1_OUT Enable. When OSEL.PCH1SEL =0, this bit enables the output CH1 when set to 1. When OSEL.PCH1SEL != 0, this bit is invalid."]
    #[inline(always)]
    pub fn ch1oe(&mut self) -> Ch1oeW<'_, SaphAoctl0Spec> {
        Ch1oeW::new(self, 1)
    }
    #[doc = "Bit 8 - CH0_OUT Value. When OSEL.PCH0SEL =0 and OCTL0.CH0OE=1, this bit represents the logical value on the CH0 terminal. 0 = low 1 = high"]
    #[inline(always)]
    pub fn ch0out(&mut self) -> Ch0outW<'_, SaphAoctl0Spec> {
        Ch0outW::new(self, 8)
    }
    #[doc = "Bit 9 - CH1_OUT Value. When OSEL.PCH1SEL =0 and OCTL0.CH1OE=1, this bit represents the logical value on the CH1 terminal. 0 = low 1 = high"]
    #[inline(always)]
    pub fn ch1out(&mut self) -> Ch1outW<'_, SaphAoctl0Spec> {
        Ch1outW::new(self, 9)
    }
}
#[doc = "Physical Interface Output Control #0\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aoctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aoctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAoctl0Spec;
impl crate::RegisterSpec for SaphAoctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aoctl0::R`](R) reader structure"]
impl crate::Readable for SaphAoctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`saph_aoctl0::W`](W) writer structure"]
impl crate::Writable for SaphAoctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AOCTL0 to value 0"]
impl crate::Resettable for SaphAoctl0Spec {}
