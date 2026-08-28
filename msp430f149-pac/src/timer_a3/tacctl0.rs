#[doc = "Register `TACCTL0` reader"]
pub type R = crate::R<Tacctl0Spec>;
#[doc = "Register `TACCTL0` writer"]
pub type W = crate::W<Tacctl0Spec>;
#[doc = "Field `CCIFG` reader - Capture/compare interrupt flag"]
pub type CcifgR = crate::BitReader;
#[doc = "Field `CCIFG` writer - Capture/compare interrupt flag"]
pub type CcifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COV` reader - Capture/compare overflow flag"]
pub type CovR = crate::BitReader;
#[doc = "Field `COV` writer - Capture/compare overflow flag"]
pub type CovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT` reader - PWM Output signal if output mode 0"]
pub type OutR = crate::BitReader;
#[doc = "Field `OUT` writer - PWM Output signal if output mode 0"]
pub type OutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCI` reader - Capture input signal (read)"]
pub type CciR = crate::BitReader;
#[doc = "Field `CCI` writer - Capture input signal (read)"]
pub type CciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCIE` reader - Capture/compare interrupt enable"]
pub type CcieR = crate::BitReader;
#[doc = "Field `CCIE` writer - Capture/compare interrupt enable"]
pub type CcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Output mode 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outmod {
    #[doc = "0: PWM output mode: 0 - output only"]
    Outmod0 = 0,
    #[doc = "1: PWM output mode: 1 - set"]
    Outmod1 = 1,
    #[doc = "2: PWM output mode: 2 - PWM toggle/reset"]
    Outmod2 = 2,
    #[doc = "3: PWM output mode: 3 - PWM set/reset"]
    Outmod3 = 3,
    #[doc = "4: PWM output mode: 4 - toggle"]
    Outmod4 = 4,
    #[doc = "5: PWM output mode: 5 - Reset"]
    Outmod5 = 5,
    #[doc = "6: PWM output mode: 6 - PWM toggle/set"]
    Outmod6 = 6,
    #[doc = "7: PWM output mode: 7 - PWM reset/set"]
    Outmod7 = 7,
}
impl From<Outmod> for u8 {
    #[inline(always)]
    fn from(variant: Outmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outmod {
    type Ux = u8;
}
impl crate::IsEnum for Outmod {}
#[doc = "Field `OUTMOD` reader - Output mode 2"]
pub type OutmodR = crate::FieldReader<Outmod>;
impl OutmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outmod {
        match self.bits {
            0 => Outmod::Outmod0,
            1 => Outmod::Outmod1,
            2 => Outmod::Outmod2,
            3 => Outmod::Outmod3,
            4 => Outmod::Outmod4,
            5 => Outmod::Outmod5,
            6 => Outmod::Outmod6,
            7 => Outmod::Outmod7,
            _ => unreachable!(),
        }
    }
    #[doc = "PWM output mode: 0 - output only"]
    #[inline(always)]
    pub fn is_outmod_0(&self) -> bool {
        *self == Outmod::Outmod0
    }
    #[doc = "PWM output mode: 1 - set"]
    #[inline(always)]
    pub fn is_outmod_1(&self) -> bool {
        *self == Outmod::Outmod1
    }
    #[doc = "PWM output mode: 2 - PWM toggle/reset"]
    #[inline(always)]
    pub fn is_outmod_2(&self) -> bool {
        *self == Outmod::Outmod2
    }
    #[doc = "PWM output mode: 3 - PWM set/reset"]
    #[inline(always)]
    pub fn is_outmod_3(&self) -> bool {
        *self == Outmod::Outmod3
    }
    #[doc = "PWM output mode: 4 - toggle"]
    #[inline(always)]
    pub fn is_outmod_4(&self) -> bool {
        *self == Outmod::Outmod4
    }
    #[doc = "PWM output mode: 5 - Reset"]
    #[inline(always)]
    pub fn is_outmod_5(&self) -> bool {
        *self == Outmod::Outmod5
    }
    #[doc = "PWM output mode: 6 - PWM toggle/set"]
    #[inline(always)]
    pub fn is_outmod_6(&self) -> bool {
        *self == Outmod::Outmod6
    }
    #[doc = "PWM output mode: 7 - PWM reset/set"]
    #[inline(always)]
    pub fn is_outmod_7(&self) -> bool {
        *self == Outmod::Outmod7
    }
}
#[doc = "Field `OUTMOD` writer - Output mode 2"]
pub type OutmodW<'a, REG> = crate::FieldWriter<'a, REG, 3, Outmod, crate::Safe>;
impl<'a, REG> OutmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PWM output mode: 0 - output only"]
    #[inline(always)]
    pub fn outmod_0(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod0)
    }
    #[doc = "PWM output mode: 1 - set"]
    #[inline(always)]
    pub fn outmod_1(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod1)
    }
    #[doc = "PWM output mode: 2 - PWM toggle/reset"]
    #[inline(always)]
    pub fn outmod_2(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod2)
    }
    #[doc = "PWM output mode: 3 - PWM set/reset"]
    #[inline(always)]
    pub fn outmod_3(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod3)
    }
    #[doc = "PWM output mode: 4 - toggle"]
    #[inline(always)]
    pub fn outmod_4(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod4)
    }
    #[doc = "PWM output mode: 5 - Reset"]
    #[inline(always)]
    pub fn outmod_5(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod5)
    }
    #[doc = "PWM output mode: 6 - PWM toggle/set"]
    #[inline(always)]
    pub fn outmod_6(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod6)
    }
    #[doc = "PWM output mode: 7 - PWM reset/set"]
    #[inline(always)]
    pub fn outmod_7(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod7)
    }
}
#[doc = "Field `CAP` reader - Capture mode: 1 /Compare mode : 0"]
pub type CapR = crate::BitReader;
#[doc = "Field `CAP` writer - Capture mode: 1 /Compare mode : 0"]
pub type CapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCCI` reader - Latched capture signal (read)"]
pub type ScciR = crate::BitReader;
#[doc = "Field `SCCI` writer - Latched capture signal (read)"]
pub type ScciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCS` reader - Capture sychronize"]
pub type ScsR = crate::BitReader;
#[doc = "Field `SCS` writer - Capture sychronize"]
pub type ScsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Capture input select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccis {
    #[doc = "0: Capture input select: 0 - CCIxA"]
    Ccis0 = 0,
    #[doc = "1: Capture input select: 1 - CCIxB"]
    Ccis1 = 1,
    #[doc = "2: Capture input select: 2 - GND"]
    Ccis2 = 2,
    #[doc = "3: Capture input select: 3 - Vcc"]
    Ccis3 = 3,
}
impl From<Ccis> for u8 {
    #[inline(always)]
    fn from(variant: Ccis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccis {
    type Ux = u8;
}
impl crate::IsEnum for Ccis {}
#[doc = "Field `CCIS` reader - Capture input select 1"]
pub type CcisR = crate::FieldReader<Ccis>;
impl CcisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccis {
        match self.bits {
            0 => Ccis::Ccis0,
            1 => Ccis::Ccis1,
            2 => Ccis::Ccis2,
            3 => Ccis::Ccis3,
            _ => unreachable!(),
        }
    }
    #[doc = "Capture input select: 0 - CCIxA"]
    #[inline(always)]
    pub fn is_ccis_0(&self) -> bool {
        *self == Ccis::Ccis0
    }
    #[doc = "Capture input select: 1 - CCIxB"]
    #[inline(always)]
    pub fn is_ccis_1(&self) -> bool {
        *self == Ccis::Ccis1
    }
    #[doc = "Capture input select: 2 - GND"]
    #[inline(always)]
    pub fn is_ccis_2(&self) -> bool {
        *self == Ccis::Ccis2
    }
    #[doc = "Capture input select: 3 - Vcc"]
    #[inline(always)]
    pub fn is_ccis_3(&self) -> bool {
        *self == Ccis::Ccis3
    }
}
#[doc = "Field `CCIS` writer - Capture input select 1"]
pub type CcisW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccis, crate::Safe>;
impl<'a, REG> CcisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Capture input select: 0 - CCIxA"]
    #[inline(always)]
    pub fn ccis_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccis::Ccis0)
    }
    #[doc = "Capture input select: 1 - CCIxB"]
    #[inline(always)]
    pub fn ccis_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccis::Ccis1)
    }
    #[doc = "Capture input select: 2 - GND"]
    #[inline(always)]
    pub fn ccis_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccis::Ccis2)
    }
    #[doc = "Capture input select: 3 - Vcc"]
    #[inline(always)]
    pub fn ccis_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ccis::Ccis3)
    }
}
#[doc = "Capture mode 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cm {
    #[doc = "0: Capture mode: 0 - disabled"]
    Cm0 = 0,
    #[doc = "1: Capture mode: 1 - pos. edge"]
    Cm1 = 1,
    #[doc = "2: Capture mode: 1 - neg. edge"]
    Cm2 = 2,
    #[doc = "3: Capture mode: 1 - both edges"]
    Cm3 = 3,
}
impl From<Cm> for u8 {
    #[inline(always)]
    fn from(variant: Cm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cm {
    type Ux = u8;
}
impl crate::IsEnum for Cm {}
#[doc = "Field `CM` reader - Capture mode 1"]
pub type CmR = crate::FieldReader<Cm>;
impl CmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cm {
        match self.bits {
            0 => Cm::Cm0,
            1 => Cm::Cm1,
            2 => Cm::Cm2,
            3 => Cm::Cm3,
            _ => unreachable!(),
        }
    }
    #[doc = "Capture mode: 0 - disabled"]
    #[inline(always)]
    pub fn is_cm_0(&self) -> bool {
        *self == Cm::Cm0
    }
    #[doc = "Capture mode: 1 - pos. edge"]
    #[inline(always)]
    pub fn is_cm_1(&self) -> bool {
        *self == Cm::Cm1
    }
    #[doc = "Capture mode: 1 - neg. edge"]
    #[inline(always)]
    pub fn is_cm_2(&self) -> bool {
        *self == Cm::Cm2
    }
    #[doc = "Capture mode: 1 - both edges"]
    #[inline(always)]
    pub fn is_cm_3(&self) -> bool {
        *self == Cm::Cm3
    }
}
#[doc = "Field `CM` writer - Capture mode 1"]
pub type CmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cm, crate::Safe>;
impl<'a, REG> CmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Capture mode: 0 - disabled"]
    #[inline(always)]
    pub fn cm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Cm0)
    }
    #[doc = "Capture mode: 1 - pos. edge"]
    #[inline(always)]
    pub fn cm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Cm1)
    }
    #[doc = "Capture mode: 1 - neg. edge"]
    #[inline(always)]
    pub fn cm_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Cm2)
    }
    #[doc = "Capture mode: 1 - both edges"]
    #[inline(always)]
    pub fn cm_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Cm3)
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ccifg(&self) -> CcifgR {
        CcifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare overflow flag"]
    #[inline(always)]
    pub fn cov(&self) -> CovR {
        CovR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM Output signal if output mode 0"]
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture input signal (read)"]
    #[inline(always)]
    pub fn cci(&self) -> CciR {
        CciR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CcieR {
        CcieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Output mode 2"]
    #[inline(always)]
    pub fn outmod(&self) -> OutmodR {
        OutmodR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Capture mode: 1 /Compare mode : 0"]
    #[inline(always)]
    pub fn cap(&self) -> CapR {
        CapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Latched capture signal (read)"]
    #[inline(always)]
    pub fn scci(&self) -> ScciR {
        ScciR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture sychronize"]
    #[inline(always)]
    pub fn scs(&self) -> ScsR {
        ScsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Capture input select 1"]
    #[inline(always)]
    pub fn ccis(&self) -> CcisR {
        CcisR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Capture mode 1"]
    #[inline(always)]
    pub fn cm(&self) -> CmR {
        CmR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ccifg(&mut self) -> CcifgW<'_, Tacctl0Spec> {
        CcifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare overflow flag"]
    #[inline(always)]
    pub fn cov(&mut self) -> CovW<'_, Tacctl0Spec> {
        CovW::new(self, 1)
    }
    #[doc = "Bit 2 - PWM Output signal if output mode 0"]
    #[inline(always)]
    pub fn out(&mut self) -> OutW<'_, Tacctl0Spec> {
        OutW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture input signal (read)"]
    #[inline(always)]
    pub fn cci(&mut self) -> CciW<'_, Tacctl0Spec> {
        CciW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ccie(&mut self) -> CcieW<'_, Tacctl0Spec> {
        CcieW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Output mode 2"]
    #[inline(always)]
    pub fn outmod(&mut self) -> OutmodW<'_, Tacctl0Spec> {
        OutmodW::new(self, 5)
    }
    #[doc = "Bit 8 - Capture mode: 1 /Compare mode : 0"]
    #[inline(always)]
    pub fn cap(&mut self) -> CapW<'_, Tacctl0Spec> {
        CapW::new(self, 8)
    }
    #[doc = "Bit 10 - Latched capture signal (read)"]
    #[inline(always)]
    pub fn scci(&mut self) -> ScciW<'_, Tacctl0Spec> {
        ScciW::new(self, 10)
    }
    #[doc = "Bit 11 - Capture sychronize"]
    #[inline(always)]
    pub fn scs(&mut self) -> ScsW<'_, Tacctl0Spec> {
        ScsW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Capture input select 1"]
    #[inline(always)]
    pub fn ccis(&mut self) -> CcisW<'_, Tacctl0Spec> {
        CcisW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Capture mode 1"]
    #[inline(always)]
    pub fn cm(&mut self) -> CmW<'_, Tacctl0Spec> {
        CmW::new(self, 14)
    }
}
#[doc = "Timer A Capture/Compare Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tacctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tacctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tacctl0Spec;
impl crate::RegisterSpec for Tacctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tacctl0::R`](R) reader structure"]
impl crate::Readable for Tacctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`tacctl0::W`](W) writer structure"]
impl crate::Writable for Tacctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TACCTL0 to value 0"]
impl crate::Resettable for Tacctl0Spec {}
