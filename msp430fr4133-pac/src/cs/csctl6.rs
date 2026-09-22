#[doc = "Register `CSCTL6` reader"]
pub type R = crate::R<Csctl6Spec>;
#[doc = "Register `CSCTL6` writer"]
pub type W = crate::W<Csctl6Spec>;
#[doc = "Field `XT1AUTOOFF` reader - XT1 automatic off enable"]
pub type Xt1autooffR = crate::BitReader;
#[doc = "Field `XT1AUTOOFF` writer - XT1 automatic off enable"]
pub type Xt1autooffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT1AGCOFF` reader - XT1 Automatic Gain Control (AGC) disable"]
pub type Xt1agcoffR = crate::BitReader;
#[doc = "Field `XT1AGCOFF` writer - XT1 Automatic Gain Control (AGC) disable"]
pub type Xt1agcoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT1BYPASS` reader - XT1 bypass mode : 0: internal 1:sourced from external pin"]
pub type Xt1bypassR = crate::BitReader;
#[doc = "Field `XT1BYPASS` writer - XT1 bypass mode : 0: internal 1:sourced from external pin"]
pub type Xt1bypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTS` reader - 1: Selects high-freq. oscillator"]
pub type XtsR = crate::BitReader;
#[doc = "Field `XTS` writer - 1: Selects high-freq. oscillator"]
pub type XtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "XT1 Drive Level mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xt1drive {
    #[doc = "0: XT1 Drive Level mode: 0"]
    Xt1drive0 = 0,
    #[doc = "1: XT1 Drive Level mode: 1"]
    Xt1drive1 = 1,
    #[doc = "2: XT1 Drive Level mode: 2"]
    Xt1drive2 = 2,
    #[doc = "3: XT1 Drive Level mode: 3"]
    Xt1drive3 = 3,
}
impl From<Xt1drive> for u8 {
    #[inline(always)]
    fn from(variant: Xt1drive) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xt1drive {
    type Ux = u8;
}
impl crate::IsEnum for Xt1drive {}
#[doc = "Field `XT1DRIVE` reader - XT1 Drive Level mode Bit 0"]
pub type Xt1driveR = crate::FieldReader<Xt1drive>;
impl Xt1driveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xt1drive {
        match self.bits {
            0 => Xt1drive::Xt1drive0,
            1 => Xt1drive::Xt1drive1,
            2 => Xt1drive::Xt1drive2,
            3 => Xt1drive::Xt1drive3,
            _ => unreachable!(),
        }
    }
    #[doc = "XT1 Drive Level mode: 0"]
    #[inline(always)]
    pub fn is_xt1drive_0(&self) -> bool {
        *self == Xt1drive::Xt1drive0
    }
    #[doc = "XT1 Drive Level mode: 1"]
    #[inline(always)]
    pub fn is_xt1drive_1(&self) -> bool {
        *self == Xt1drive::Xt1drive1
    }
    #[doc = "XT1 Drive Level mode: 2"]
    #[inline(always)]
    pub fn is_xt1drive_2(&self) -> bool {
        *self == Xt1drive::Xt1drive2
    }
    #[doc = "XT1 Drive Level mode: 3"]
    #[inline(always)]
    pub fn is_xt1drive_3(&self) -> bool {
        *self == Xt1drive::Xt1drive3
    }
}
#[doc = "Field `XT1DRIVE` writer - XT1 Drive Level mode Bit 0"]
pub type Xt1driveW<'a, REG> = crate::FieldWriter<'a, REG, 2, Xt1drive, crate::Safe>;
impl<'a, REG> Xt1driveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XT1 Drive Level mode: 0"]
    #[inline(always)]
    pub fn xt1drive_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1drive::Xt1drive0)
    }
    #[doc = "XT1 Drive Level mode: 1"]
    #[inline(always)]
    pub fn xt1drive_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1drive::Xt1drive1)
    }
    #[doc = "XT1 Drive Level mode: 2"]
    #[inline(always)]
    pub fn xt1drive_2(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1drive::Xt1drive2)
    }
    #[doc = "XT1 Drive Level mode: 3"]
    #[inline(always)]
    pub fn xt1drive_3(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1drive::Xt1drive3)
    }
}
impl R {
    #[doc = "Bit 0 - XT1 automatic off enable"]
    #[inline(always)]
    pub fn xt1autooff(&self) -> Xt1autooffR {
        Xt1autooffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XT1 Automatic Gain Control (AGC) disable"]
    #[inline(always)]
    pub fn xt1agcoff(&self) -> Xt1agcoffR {
        Xt1agcoffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - XT1 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn xt1bypass(&self) -> Xt1bypassR {
        Xt1bypassR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Selects high-freq. oscillator"]
    #[inline(always)]
    pub fn xts(&self) -> XtsR {
        XtsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - XT1 Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn xt1drive(&self) -> Xt1driveR {
        Xt1driveR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - XT1 automatic off enable"]
    #[inline(always)]
    pub fn xt1autooff(&mut self) -> Xt1autooffW<'_, Csctl6Spec> {
        Xt1autooffW::new(self, 0)
    }
    #[doc = "Bit 1 - XT1 Automatic Gain Control (AGC) disable"]
    #[inline(always)]
    pub fn xt1agcoff(&mut self) -> Xt1agcoffW<'_, Csctl6Spec> {
        Xt1agcoffW::new(self, 1)
    }
    #[doc = "Bit 4 - XT1 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn xt1bypass(&mut self) -> Xt1bypassW<'_, Csctl6Spec> {
        Xt1bypassW::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Selects high-freq. oscillator"]
    #[inline(always)]
    pub fn xts(&mut self) -> XtsW<'_, Csctl6Spec> {
        XtsW::new(self, 5)
    }
    #[doc = "Bits 6:7 - XT1 Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn xt1drive(&mut self) -> Xt1driveW<'_, Csctl6Spec> {
        Xt1driveW::new(self, 6)
    }
}
#[doc = "CS Control Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl6Spec;
impl crate::RegisterSpec for Csctl6Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl6::R`](R) reader structure"]
impl crate::Readable for Csctl6Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl6::W`](W) writer structure"]
impl crate::Writable for Csctl6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL6 to value 0"]
impl crate::Resettable for Csctl6Spec {}
