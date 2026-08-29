#[doc = "Register `SAPH_AOSEL` reader"]
pub type R = crate::R<SaphAoselSpec>;
#[doc = "Register `SAPH_AOSEL` writer"]
pub type W = crate::W<SaphAoselSpec>;
#[doc = "Output functional select for CH0_OUT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pch0sel {
    #[doc = "0: CH0_OUT is used as a GPO pin. It is controlled by OCTL0.CH0OUT and OCTL0.CH0OE."]
    Gpio = 0,
    #[doc = "1: CH0_OUT is driven by the PPG."]
    Ppgse = 1,
    #[doc = "2: CH0_OUT is driven by the PPG as differential output along with CH1_OUT. (CH0_OUT and CH1_OUT are alwasy opposite polarity)"]
    Pch0sel2 = 2,
    #[doc = "3: CH0_OUT is used as a GPO pin. It is controlled by OCTL0.CH0OUT and OCTL0.CH0OE."]
    Pch0sel3 = 3,
}
impl From<Pch0sel> for u8 {
    #[inline(always)]
    fn from(variant: Pch0sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pch0sel {
    type Ux = u8;
}
impl crate::IsEnum for Pch0sel {}
#[doc = "Field `PCH0SEL` reader - Output functional select for CH0_OUT."]
pub type Pch0selR = crate::FieldReader<Pch0sel>;
impl Pch0selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pch0sel {
        match self.bits {
            0 => Pch0sel::Gpio,
            1 => Pch0sel::Ppgse,
            2 => Pch0sel::Pch0sel2,
            3 => Pch0sel::Pch0sel3,
            _ => unreachable!(),
        }
    }
    #[doc = "CH0_OUT is used as a GPO pin. It is controlled by OCTL0.CH0OUT and OCTL0.CH0OE."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Pch0sel::Gpio
    }
    #[doc = "CH0_OUT is driven by the PPG."]
    #[inline(always)]
    pub fn is_ppgse(&self) -> bool {
        *self == Pch0sel::Ppgse
    }
    #[doc = "CH0_OUT is driven by the PPG as differential output along with CH1_OUT. (CH0_OUT and CH1_OUT are alwasy opposite polarity)"]
    #[inline(always)]
    pub fn is_pch0sel_2(&self) -> bool {
        *self == Pch0sel::Pch0sel2
    }
    #[doc = "CH0_OUT is used as a GPO pin. It is controlled by OCTL0.CH0OUT and OCTL0.CH0OE."]
    #[inline(always)]
    pub fn is_pch0sel_3(&self) -> bool {
        *self == Pch0sel::Pch0sel3
    }
}
#[doc = "Field `PCH0SEL` writer - Output functional select for CH0_OUT."]
pub type Pch0selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pch0sel, crate::Safe>;
impl<'a, REG> Pch0selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH0_OUT is used as a GPO pin. It is controlled by OCTL0.CH0OUT and OCTL0.CH0OE."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Pch0sel::Gpio)
    }
    #[doc = "CH0_OUT is driven by the PPG."]
    #[inline(always)]
    pub fn ppgse(self) -> &'a mut crate::W<REG> {
        self.variant(Pch0sel::Ppgse)
    }
    #[doc = "CH0_OUT is driven by the PPG as differential output along with CH1_OUT. (CH0_OUT and CH1_OUT are alwasy opposite polarity)"]
    #[inline(always)]
    pub fn pch0sel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Pch0sel::Pch0sel2)
    }
    #[doc = "CH0_OUT is used as a GPO pin. It is controlled by OCTL0.CH0OUT and OCTL0.CH0OE."]
    #[inline(always)]
    pub fn pch0sel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Pch0sel::Pch0sel3)
    }
}
#[doc = "Output functional select for CH0_OUT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pch1sel {
    #[doc = "0: CH1_OUT is used as a GPO pin. It is controlled by OCTL0.CH1OUT and OCTL0.CH1OE."]
    Gpio = 0,
    #[doc = "1: CH1_OUT is driven by the PPG."]
    Ppgse = 1,
    #[doc = "2: CH1_OUT is driven by the PPG as differential output along with CH0_OUT. (CH0_OUT and CH1_OUT are alwasy opposite polarity)"]
    Pch1sel2 = 2,
    #[doc = "3: CH1_OUT is used as a GPO pin. It is controlled by OCTL0.CH1OUT and OCTL0.CH1OE."]
    Pch1sel3 = 3,
}
impl From<Pch1sel> for u8 {
    #[inline(always)]
    fn from(variant: Pch1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pch1sel {
    type Ux = u8;
}
impl crate::IsEnum for Pch1sel {}
#[doc = "Field `PCH1SEL` reader - Output functional select for CH0_OUT."]
pub type Pch1selR = crate::FieldReader<Pch1sel>;
impl Pch1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pch1sel {
        match self.bits {
            0 => Pch1sel::Gpio,
            1 => Pch1sel::Ppgse,
            2 => Pch1sel::Pch1sel2,
            3 => Pch1sel::Pch1sel3,
            _ => unreachable!(),
        }
    }
    #[doc = "CH1_OUT is used as a GPO pin. It is controlled by OCTL0.CH1OUT and OCTL0.CH1OE."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Pch1sel::Gpio
    }
    #[doc = "CH1_OUT is driven by the PPG."]
    #[inline(always)]
    pub fn is_ppgse(&self) -> bool {
        *self == Pch1sel::Ppgse
    }
    #[doc = "CH1_OUT is driven by the PPG as differential output along with CH0_OUT. (CH0_OUT and CH1_OUT are alwasy opposite polarity)"]
    #[inline(always)]
    pub fn is_pch1sel_2(&self) -> bool {
        *self == Pch1sel::Pch1sel2
    }
    #[doc = "CH1_OUT is used as a GPO pin. It is controlled by OCTL0.CH1OUT and OCTL0.CH1OE."]
    #[inline(always)]
    pub fn is_pch1sel_3(&self) -> bool {
        *self == Pch1sel::Pch1sel3
    }
}
#[doc = "Field `PCH1SEL` writer - Output functional select for CH0_OUT."]
pub type Pch1selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pch1sel, crate::Safe>;
impl<'a, REG> Pch1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH1_OUT is used as a GPO pin. It is controlled by OCTL0.CH1OUT and OCTL0.CH1OE."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Pch1sel::Gpio)
    }
    #[doc = "CH1_OUT is driven by the PPG."]
    #[inline(always)]
    pub fn ppgse(self) -> &'a mut crate::W<REG> {
        self.variant(Pch1sel::Ppgse)
    }
    #[doc = "CH1_OUT is driven by the PPG as differential output along with CH0_OUT. (CH0_OUT and CH1_OUT are alwasy opposite polarity)"]
    #[inline(always)]
    pub fn pch1sel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Pch1sel::Pch1sel2)
    }
    #[doc = "CH1_OUT is used as a GPO pin. It is controlled by OCTL0.CH1OUT and OCTL0.CH1OE."]
    #[inline(always)]
    pub fn pch1sel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Pch1sel::Pch1sel3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Output functional select for CH0_OUT."]
    #[inline(always)]
    pub fn pch0sel(&self) -> Pch0selR {
        Pch0selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Output functional select for CH0_OUT."]
    #[inline(always)]
    pub fn pch1sel(&self) -> Pch1selR {
        Pch1selR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output functional select for CH0_OUT."]
    #[inline(always)]
    pub fn pch0sel(&mut self) -> Pch0selW<'_, SaphAoselSpec> {
        Pch0selW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Output functional select for CH0_OUT."]
    #[inline(always)]
    pub fn pch1sel(&mut self) -> Pch1selW<'_, SaphAoselSpec> {
        Pch1selW::new(self, 2)
    }
}
#[doc = "Physical Interface Output Function Select\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aosel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aosel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAoselSpec;
impl crate::RegisterSpec for SaphAoselSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aosel::R`](R) reader structure"]
impl crate::Readable for SaphAoselSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_aosel::W`](W) writer structure"]
impl crate::Writable for SaphAoselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AOSEL to value 0"]
impl crate::Resettable for SaphAoselSpec {}
