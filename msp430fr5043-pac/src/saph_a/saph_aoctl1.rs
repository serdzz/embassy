#[doc = "Register `SAPH_AOCTL1` reader"]
pub type R = crate::R<SaphAoctl1Spec>;
#[doc = "Register `SAPH_AOCTL1` writer"]
pub type W = crate::W<SaphAoctl1Spec>;
#[doc = "CH0 termination switch (SWG0) enable. When OSEL.PCH0SEL =0, this bit controls the SWG0 switch. 0 = SWG0 is off. 1 = SWG0 is on. The CH0_OUT is disabled and connected to PVSS via SWG0 switch. No need to change OCTL0.CH0OE status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0term {
    #[doc = "0: CH0 Output is defined by CH0OUT and CH0OE"]
    Ch0term0 = 0,
    #[doc = "1: CH0 Output is set low with termination strength"]
    Ch0term1 = 1,
}
impl From<Ch0term> for bool {
    #[inline(always)]
    fn from(variant: Ch0term) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0TERM` reader - CH0 termination switch (SWG0) enable. When OSEL.PCH0SEL =0, this bit controls the SWG0 switch. 0 = SWG0 is off. 1 = SWG0 is on. The CH0_OUT is disabled and connected to PVSS via SWG0 switch. No need to change OCTL0.CH0OE status."]
pub type Ch0termR = crate::BitReader<Ch0term>;
impl Ch0termR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0term {
        match self.bits {
            false => Ch0term::Ch0term0,
            true => Ch0term::Ch0term1,
        }
    }
    #[doc = "CH0 Output is defined by CH0OUT and CH0OE"]
    #[inline(always)]
    pub fn is_ch0term_0(&self) -> bool {
        *self == Ch0term::Ch0term0
    }
    #[doc = "CH0 Output is set low with termination strength"]
    #[inline(always)]
    pub fn is_ch0term_1(&self) -> bool {
        *self == Ch0term::Ch0term1
    }
}
#[doc = "Field `CH0TERM` writer - CH0 termination switch (SWG0) enable. When OSEL.PCH0SEL =0, this bit controls the SWG0 switch. 0 = SWG0 is off. 1 = SWG0 is on. The CH0_OUT is disabled and connected to PVSS via SWG0 switch. No need to change OCTL0.CH0OE status."]
pub type Ch0termW<'a, REG> = crate::BitWriter<'a, REG, Ch0term>;
impl<'a, REG> Ch0termW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CH0 Output is defined by CH0OUT and CH0OE"]
    #[inline(always)]
    pub fn ch0term_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0term::Ch0term0)
    }
    #[doc = "CH0 Output is set low with termination strength"]
    #[inline(always)]
    pub fn ch0term_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0term::Ch0term1)
    }
}
#[doc = "CH1 termination switch (SWG1) enable. When OSEL.PCH1SEL =0, this bit controls the SWG1 switch. 0 = SWG1 is off. 1 = SWG1 is on. The CH1_OUT is disabled and connected to PVSS via SWG1 switch. No need to change OCTL0.CH1OE status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1term {
    #[doc = "0: CH1 Output is defined by CH1OUT and CH1OE"]
    Ch1term0 = 0,
    #[doc = "1: CH1 Output is set low with termination strength"]
    Ch1term1 = 1,
}
impl From<Ch1term> for bool {
    #[inline(always)]
    fn from(variant: Ch1term) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1TERM` reader - CH1 termination switch (SWG1) enable. When OSEL.PCH1SEL =0, this bit controls the SWG1 switch. 0 = SWG1 is off. 1 = SWG1 is on. The CH1_OUT is disabled and connected to PVSS via SWG1 switch. No need to change OCTL0.CH1OE status."]
pub type Ch1termR = crate::BitReader<Ch1term>;
impl Ch1termR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1term {
        match self.bits {
            false => Ch1term::Ch1term0,
            true => Ch1term::Ch1term1,
        }
    }
    #[doc = "CH1 Output is defined by CH1OUT and CH1OE"]
    #[inline(always)]
    pub fn is_ch1term_0(&self) -> bool {
        *self == Ch1term::Ch1term0
    }
    #[doc = "CH1 Output is set low with termination strength"]
    #[inline(always)]
    pub fn is_ch1term_1(&self) -> bool {
        *self == Ch1term::Ch1term1
    }
}
#[doc = "Field `CH1TERM` writer - CH1 termination switch (SWG1) enable. When OSEL.PCH1SEL =0, this bit controls the SWG1 switch. 0 = SWG1 is off. 1 = SWG1 is on. The CH1_OUT is disabled and connected to PVSS via SWG1 switch. No need to change OCTL0.CH1OE status."]
pub type Ch1termW<'a, REG> = crate::BitWriter<'a, REG, Ch1term>;
impl<'a, REG> Ch1termW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CH1 Output is defined by CH1OUT and CH1OE"]
    #[inline(always)]
    pub fn ch1term_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1term::Ch1term0)
    }
    #[doc = "CH1 Output is set low with termination strength"]
    #[inline(always)]
    pub fn ch1term_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1term::Ch1term1)
    }
}
#[doc = "DRV0 (output driver on the CH0_OUT) full strength enable. 0 = The DRV0 output impedance is deterimed by CH0PUT and CH0PDT registers. 1 = The DRV0 has lowest output impedance.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0fp {
    #[doc = "0: Ch0 Output is set to normal strength"]
    Ch0fp0 = 0,
    #[doc = "1: Ch0 Output is set to maximum strength"]
    Ch0fp1 = 1,
}
impl From<Ch0fp> for bool {
    #[inline(always)]
    fn from(variant: Ch0fp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0FP` reader - DRV0 (output driver on the CH0_OUT) full strength enable. 0 = The DRV0 output impedance is deterimed by CH0PUT and CH0PDT registers. 1 = The DRV0 has lowest output impedance."]
pub type Ch0fpR = crate::BitReader<Ch0fp>;
impl Ch0fpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0fp {
        match self.bits {
            false => Ch0fp::Ch0fp0,
            true => Ch0fp::Ch0fp1,
        }
    }
    #[doc = "Ch0 Output is set to normal strength"]
    #[inline(always)]
    pub fn is_ch0fp_0(&self) -> bool {
        *self == Ch0fp::Ch0fp0
    }
    #[doc = "Ch0 Output is set to maximum strength"]
    #[inline(always)]
    pub fn is_ch0fp_1(&self) -> bool {
        *self == Ch0fp::Ch0fp1
    }
}
#[doc = "Field `CH0FP` writer - DRV0 (output driver on the CH0_OUT) full strength enable. 0 = The DRV0 output impedance is deterimed by CH0PUT and CH0PDT registers. 1 = The DRV0 has lowest output impedance."]
pub type Ch0fpW<'a, REG> = crate::BitWriter<'a, REG, Ch0fp>;
impl<'a, REG> Ch0fpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ch0 Output is set to normal strength"]
    #[inline(always)]
    pub fn ch0fp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0fp::Ch0fp0)
    }
    #[doc = "Ch0 Output is set to maximum strength"]
    #[inline(always)]
    pub fn ch0fp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0fp::Ch0fp1)
    }
}
#[doc = "DRV1 (output driver on the CH1_OUT) full strength enable. 0 = The DRV1 output impedance is deterimed by CH1PUT and CH1PDT registers. 1 = The DRV1 has lowest output impedance.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1fp {
    #[doc = "0: Ch1 Output is set to normal strength"]
    Ch1fp0 = 0,
    #[doc = "1: Ch1 Output is set to maximum strength"]
    Ch1fp1 = 1,
}
impl From<Ch1fp> for bool {
    #[inline(always)]
    fn from(variant: Ch1fp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1FP` reader - DRV1 (output driver on the CH1_OUT) full strength enable. 0 = The DRV1 output impedance is deterimed by CH1PUT and CH1PDT registers. 1 = The DRV1 has lowest output impedance."]
pub type Ch1fpR = crate::BitReader<Ch1fp>;
impl Ch1fpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1fp {
        match self.bits {
            false => Ch1fp::Ch1fp0,
            true => Ch1fp::Ch1fp1,
        }
    }
    #[doc = "Ch1 Output is set to normal strength"]
    #[inline(always)]
    pub fn is_ch1fp_0(&self) -> bool {
        *self == Ch1fp::Ch1fp0
    }
    #[doc = "Ch1 Output is set to maximum strength"]
    #[inline(always)]
    pub fn is_ch1fp_1(&self) -> bool {
        *self == Ch1fp::Ch1fp1
    }
}
#[doc = "Field `CH1FP` writer - DRV1 (output driver on the CH1_OUT) full strength enable. 0 = The DRV1 output impedance is deterimed by CH1PUT and CH1PDT registers. 1 = The DRV1 has lowest output impedance."]
pub type Ch1fpW<'a, REG> = crate::BitWriter<'a, REG, Ch1fp>;
impl<'a, REG> Ch1fpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ch1 Output is set to normal strength"]
    #[inline(always)]
    pub fn ch1fp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1fp::Ch1fp0)
    }
    #[doc = "Ch1 Output is set to maximum strength"]
    #[inline(always)]
    pub fn ch1fp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1fp::Ch1fp1)
    }
}
impl R {
    #[doc = "Bit 0 - CH0 termination switch (SWG0) enable. When OSEL.PCH0SEL =0, this bit controls the SWG0 switch. 0 = SWG0 is off. 1 = SWG0 is on. The CH0_OUT is disabled and connected to PVSS via SWG0 switch. No need to change OCTL0.CH0OE status."]
    #[inline(always)]
    pub fn ch0term(&self) -> Ch0termR {
        Ch0termR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 termination switch (SWG1) enable. When OSEL.PCH1SEL =0, this bit controls the SWG1 switch. 0 = SWG1 is off. 1 = SWG1 is on. The CH1_OUT is disabled and connected to PVSS via SWG1 switch. No need to change OCTL0.CH1OE status."]
    #[inline(always)]
    pub fn ch1term(&self) -> Ch1termR {
        Ch1termR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - DRV0 (output driver on the CH0_OUT) full strength enable. 0 = The DRV0 output impedance is deterimed by CH0PUT and CH0PDT registers. 1 = The DRV0 has lowest output impedance."]
    #[inline(always)]
    pub fn ch0fp(&self) -> Ch0fpR {
        Ch0fpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DRV1 (output driver on the CH1_OUT) full strength enable. 0 = The DRV1 output impedance is deterimed by CH1PUT and CH1PDT registers. 1 = The DRV1 has lowest output impedance."]
    #[inline(always)]
    pub fn ch1fp(&self) -> Ch1fpR {
        Ch1fpR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0 termination switch (SWG0) enable. When OSEL.PCH0SEL =0, this bit controls the SWG0 switch. 0 = SWG0 is off. 1 = SWG0 is on. The CH0_OUT is disabled and connected to PVSS via SWG0 switch. No need to change OCTL0.CH0OE status."]
    #[inline(always)]
    pub fn ch0term(&mut self) -> Ch0termW<'_, SaphAoctl1Spec> {
        Ch0termW::new(self, 0)
    }
    #[doc = "Bit 1 - CH1 termination switch (SWG1) enable. When OSEL.PCH1SEL =0, this bit controls the SWG1 switch. 0 = SWG1 is off. 1 = SWG1 is on. The CH1_OUT is disabled and connected to PVSS via SWG1 switch. No need to change OCTL0.CH1OE status."]
    #[inline(always)]
    pub fn ch1term(&mut self) -> Ch1termW<'_, SaphAoctl1Spec> {
        Ch1termW::new(self, 1)
    }
    #[doc = "Bit 8 - DRV0 (output driver on the CH0_OUT) full strength enable. 0 = The DRV0 output impedance is deterimed by CH0PUT and CH0PDT registers. 1 = The DRV0 has lowest output impedance."]
    #[inline(always)]
    pub fn ch0fp(&mut self) -> Ch0fpW<'_, SaphAoctl1Spec> {
        Ch0fpW::new(self, 8)
    }
    #[doc = "Bit 9 - DRV1 (output driver on the CH1_OUT) full strength enable. 0 = The DRV1 output impedance is deterimed by CH1PUT and CH1PDT registers. 1 = The DRV1 has lowest output impedance."]
    #[inline(always)]
    pub fn ch1fp(&mut self) -> Ch1fpW<'_, SaphAoctl1Spec> {
        Ch1fpW::new(self, 9)
    }
}
#[doc = "Physical Interface Output Control #1\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_aoctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_aoctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAoctl1Spec;
impl crate::RegisterSpec for SaphAoctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_aoctl1::R`](R) reader structure"]
impl crate::Readable for SaphAoctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`saph_aoctl1::W`](W) writer structure"]
impl crate::Writable for SaphAoctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AOCTL1 to value 0"]
impl crate::Resettable for SaphAoctl1Spec {}
