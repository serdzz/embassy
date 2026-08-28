#[doc = "Register `BCSCTL3` reader"]
pub type R = crate::R<Bcsctl3Spec>;
#[doc = "Register `BCSCTL3` writer"]
pub type W = crate::W<Bcsctl3Spec>;
#[doc = "Field `LFXT1OF` reader - Low/high Frequency Oscillator Fault Flag"]
pub type Lfxt1ofR = crate::BitReader;
#[doc = "Field `LFXT1OF` writer - Low/high Frequency Oscillator Fault Flag"]
pub type Lfxt1ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT2OF` reader - High frequency oscillator 2 fault flag"]
pub type Xt2ofR = crate::BitReader;
#[doc = "Field `XT2OF` writer - High frequency oscillator 2 fault flag"]
pub type Xt2ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "XIN/XOUT Cap 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xcap {
    #[doc = "0: XIN/XOUT Cap : 0 pF"]
    Xcap0 = 0,
    #[doc = "1: XIN/XOUT Cap : 6 pF"]
    Xcap1 = 1,
    #[doc = "2: XIN/XOUT Cap : 10 pF"]
    Xcap2 = 2,
    #[doc = "3: XIN/XOUT Cap : 12.5 pF"]
    Xcap3 = 3,
}
impl From<Xcap> for u8 {
    #[inline(always)]
    fn from(variant: Xcap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xcap {
    type Ux = u8;
}
impl crate::IsEnum for Xcap {}
#[doc = "Field `XCAP` reader - XIN/XOUT Cap 0"]
pub type XcapR = crate::FieldReader<Xcap>;
impl XcapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xcap {
        match self.bits {
            0 => Xcap::Xcap0,
            1 => Xcap::Xcap1,
            2 => Xcap::Xcap2,
            3 => Xcap::Xcap3,
            _ => unreachable!(),
        }
    }
    #[doc = "XIN/XOUT Cap : 0 pF"]
    #[inline(always)]
    pub fn is_xcap_0(&self) -> bool {
        *self == Xcap::Xcap0
    }
    #[doc = "XIN/XOUT Cap : 6 pF"]
    #[inline(always)]
    pub fn is_xcap_1(&self) -> bool {
        *self == Xcap::Xcap1
    }
    #[doc = "XIN/XOUT Cap : 10 pF"]
    #[inline(always)]
    pub fn is_xcap_2(&self) -> bool {
        *self == Xcap::Xcap2
    }
    #[doc = "XIN/XOUT Cap : 12.5 pF"]
    #[inline(always)]
    pub fn is_xcap_3(&self) -> bool {
        *self == Xcap::Xcap3
    }
}
#[doc = "Field `XCAP` writer - XIN/XOUT Cap 0"]
pub type XcapW<'a, REG> = crate::FieldWriter<'a, REG, 2, Xcap, crate::Safe>;
impl<'a, REG> XcapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XIN/XOUT Cap : 0 pF"]
    #[inline(always)]
    pub fn xcap_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xcap::Xcap0)
    }
    #[doc = "XIN/XOUT Cap : 6 pF"]
    #[inline(always)]
    pub fn xcap_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xcap::Xcap1)
    }
    #[doc = "XIN/XOUT Cap : 10 pF"]
    #[inline(always)]
    pub fn xcap_2(self) -> &'a mut crate::W<REG> {
        self.variant(Xcap::Xcap2)
    }
    #[doc = "XIN/XOUT Cap : 12.5 pF"]
    #[inline(always)]
    pub fn xcap_3(self) -> &'a mut crate::W<REG> {
        self.variant(Xcap::Xcap3)
    }
}
#[doc = "Mode 0 for LFXT1 (XTS = 0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfxt1s {
    #[doc = "0: Mode 0 for LFXT1 : Normal operation"]
    Lfxt1s0 = 0,
    #[doc = "1: Mode 1 for LFXT1 : Reserved"]
    Lfxt1s1 = 1,
    #[doc = "2: Mode 2 for LFXT1 : VLO"]
    Lfxt1s2 = 2,
    #[doc = "3: Mode 3 for LFXT1 : Digital input signal"]
    Lfxt1s3 = 3,
}
impl From<Lfxt1s> for u8 {
    #[inline(always)]
    fn from(variant: Lfxt1s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfxt1s {
    type Ux = u8;
}
impl crate::IsEnum for Lfxt1s {}
#[doc = "Field `LFXT1S` reader - Mode 0 for LFXT1 (XTS = 0)"]
pub type Lfxt1sR = crate::FieldReader<Lfxt1s>;
impl Lfxt1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfxt1s {
        match self.bits {
            0 => Lfxt1s::Lfxt1s0,
            1 => Lfxt1s::Lfxt1s1,
            2 => Lfxt1s::Lfxt1s2,
            3 => Lfxt1s::Lfxt1s3,
            _ => unreachable!(),
        }
    }
    #[doc = "Mode 0 for LFXT1 : Normal operation"]
    #[inline(always)]
    pub fn is_lfxt1s_0(&self) -> bool {
        *self == Lfxt1s::Lfxt1s0
    }
    #[doc = "Mode 1 for LFXT1 : Reserved"]
    #[inline(always)]
    pub fn is_lfxt1s_1(&self) -> bool {
        *self == Lfxt1s::Lfxt1s1
    }
    #[doc = "Mode 2 for LFXT1 : VLO"]
    #[inline(always)]
    pub fn is_lfxt1s_2(&self) -> bool {
        *self == Lfxt1s::Lfxt1s2
    }
    #[doc = "Mode 3 for LFXT1 : Digital input signal"]
    #[inline(always)]
    pub fn is_lfxt1s_3(&self) -> bool {
        *self == Lfxt1s::Lfxt1s3
    }
}
#[doc = "Field `LFXT1S` writer - Mode 0 for LFXT1 (XTS = 0)"]
pub type Lfxt1sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lfxt1s, crate::Safe>;
impl<'a, REG> Lfxt1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mode 0 for LFXT1 : Normal operation"]
    #[inline(always)]
    pub fn lfxt1s_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxt1s::Lfxt1s0)
    }
    #[doc = "Mode 1 for LFXT1 : Reserved"]
    #[inline(always)]
    pub fn lfxt1s_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxt1s::Lfxt1s1)
    }
    #[doc = "Mode 2 for LFXT1 : VLO"]
    #[inline(always)]
    pub fn lfxt1s_2(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxt1s::Lfxt1s2)
    }
    #[doc = "Mode 3 for LFXT1 : Digital input signal"]
    #[inline(always)]
    pub fn lfxt1s_3(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxt1s::Lfxt1s3)
    }
}
#[doc = "Mode 0 for XT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xt2s {
    #[doc = "0: Mode 0 for XT2 : 0.4 - 1 MHz"]
    Xt2s0 = 0,
    #[doc = "1: Mode 1 for XT2 : 1 - 4 MHz"]
    Xt2s1 = 1,
    #[doc = "2: Mode 2 for XT2 : 2 - 16 MHz"]
    Xt2s2 = 2,
    #[doc = "3: Mode 3 for XT2 : Digital input signal"]
    Xt2s3 = 3,
}
impl From<Xt2s> for u8 {
    #[inline(always)]
    fn from(variant: Xt2s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xt2s {
    type Ux = u8;
}
impl crate::IsEnum for Xt2s {}
#[doc = "Field `XT2S` reader - Mode 0 for XT2"]
pub type Xt2sR = crate::FieldReader<Xt2s>;
impl Xt2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xt2s {
        match self.bits {
            0 => Xt2s::Xt2s0,
            1 => Xt2s::Xt2s1,
            2 => Xt2s::Xt2s2,
            3 => Xt2s::Xt2s3,
            _ => unreachable!(),
        }
    }
    #[doc = "Mode 0 for XT2 : 0.4 - 1 MHz"]
    #[inline(always)]
    pub fn is_xt2s_0(&self) -> bool {
        *self == Xt2s::Xt2s0
    }
    #[doc = "Mode 1 for XT2 : 1 - 4 MHz"]
    #[inline(always)]
    pub fn is_xt2s_1(&self) -> bool {
        *self == Xt2s::Xt2s1
    }
    #[doc = "Mode 2 for XT2 : 2 - 16 MHz"]
    #[inline(always)]
    pub fn is_xt2s_2(&self) -> bool {
        *self == Xt2s::Xt2s2
    }
    #[doc = "Mode 3 for XT2 : Digital input signal"]
    #[inline(always)]
    pub fn is_xt2s_3(&self) -> bool {
        *self == Xt2s::Xt2s3
    }
}
#[doc = "Field `XT2S` writer - Mode 0 for XT2"]
pub type Xt2sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Xt2s, crate::Safe>;
impl<'a, REG> Xt2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mode 0 for XT2 : 0.4 - 1 MHz"]
    #[inline(always)]
    pub fn xt2s_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xt2s::Xt2s0)
    }
    #[doc = "Mode 1 for XT2 : 1 - 4 MHz"]
    #[inline(always)]
    pub fn xt2s_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xt2s::Xt2s1)
    }
    #[doc = "Mode 2 for XT2 : 2 - 16 MHz"]
    #[inline(always)]
    pub fn xt2s_2(self) -> &'a mut crate::W<REG> {
        self.variant(Xt2s::Xt2s2)
    }
    #[doc = "Mode 3 for XT2 : Digital input signal"]
    #[inline(always)]
    pub fn xt2s_3(self) -> &'a mut crate::W<REG> {
        self.variant(Xt2s::Xt2s3)
    }
}
impl R {
    #[doc = "Bit 0 - Low/high Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn lfxt1of(&self) -> Lfxt1ofR {
        Lfxt1ofR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High frequency oscillator 2 fault flag"]
    #[inline(always)]
    pub fn xt2of(&self) -> Xt2ofR {
        Xt2ofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - XIN/XOUT Cap 0"]
    #[inline(always)]
    pub fn xcap(&self) -> XcapR {
        XcapR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Mode 0 for LFXT1 (XTS = 0)"]
    #[inline(always)]
    pub fn lfxt1s(&self) -> Lfxt1sR {
        Lfxt1sR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Mode 0 for XT2"]
    #[inline(always)]
    pub fn xt2s(&self) -> Xt2sR {
        Xt2sR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Low/high Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn lfxt1of(&mut self) -> Lfxt1ofW<'_, Bcsctl3Spec> {
        Lfxt1ofW::new(self, 0)
    }
    #[doc = "Bit 1 - High frequency oscillator 2 fault flag"]
    #[inline(always)]
    pub fn xt2of(&mut self) -> Xt2ofW<'_, Bcsctl3Spec> {
        Xt2ofW::new(self, 1)
    }
    #[doc = "Bits 2:3 - XIN/XOUT Cap 0"]
    #[inline(always)]
    pub fn xcap(&mut self) -> XcapW<'_, Bcsctl3Spec> {
        XcapW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Mode 0 for LFXT1 (XTS = 0)"]
    #[inline(always)]
    pub fn lfxt1s(&mut self) -> Lfxt1sW<'_, Bcsctl3Spec> {
        Lfxt1sW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Mode 0 for XT2"]
    #[inline(always)]
    pub fn xt2s(&mut self) -> Xt2sW<'_, Bcsctl3Spec> {
        Xt2sW::new(self, 6)
    }
}
#[doc = "Basic Clock System Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bcsctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcsctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcsctl3Spec;
impl crate::RegisterSpec for Bcsctl3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcsctl3::R`](R) reader structure"]
impl crate::Readable for Bcsctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`bcsctl3::W`](W) writer structure"]
impl crate::Writable for Bcsctl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCSCTL3 to value 0"]
impl crate::Resettable for Bcsctl3Spec {}
