#[doc = "Register `MPUCTL0` reader"]
pub type R = crate::R<Mpuctl0Spec>;
#[doc = "Register `MPUCTL0` writer"]
pub type W = crate::W<Mpuctl0Spec>;
#[doc = "MPU Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuena {
    #[doc = "0: Disabled"]
    Disable = 0,
    #[doc = "1: Enabled"]
    Enable = 1,
}
impl From<Mpuena> for bool {
    #[inline(always)]
    fn from(variant: Mpuena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUENA` reader - MPU Enable"]
pub type MpuenaR = crate::BitReader<Mpuena>;
impl MpuenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuena {
        match self.bits {
            false => Mpuena::Disable,
            true => Mpuena::Enable,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpuena::Disable
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpuena::Enable
    }
}
#[doc = "Field `MPUENA` writer - MPU Enable"]
pub type MpuenaW<'a, REG> = crate::BitWriter<'a, REG, Mpuena>;
impl<'a, REG> MpuenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuena::Disable)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuena::Enable)
    }
}
#[doc = "MPU Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpulock {
    #[doc = "0: Open"]
    Open = 0,
    #[doc = "1: Locked"]
    Lock = 1,
}
impl From<Mpulock> for bool {
    #[inline(always)]
    fn from(variant: Mpulock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPULOCK` reader - MPU Lock"]
pub type MpulockR = crate::BitReader<Mpulock>;
impl MpulockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpulock {
        match self.bits {
            false => Mpulock::Open,
            true => Mpulock::Lock,
        }
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Mpulock::Open
    }
    #[doc = "Locked"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == Mpulock::Lock
    }
}
#[doc = "Field `MPULOCK` writer - MPU Lock"]
pub type MpulockW<'a, REG> = crate::BitWriter<'a, REG, Mpulock>;
impl<'a, REG> MpulockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Open"]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Mpulock::Open)
    }
    #[doc = "Locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(Mpulock::Lock)
    }
}
#[doc = "Enable NMI Event if a Segment violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpusegie {
    #[doc = "0: Segment violation interrupt disabled"]
    Disable = 0,
    #[doc = "1: Segment violation interrupt enabled"]
    Enable = 1,
}
impl From<Mpusegie> for bool {
    #[inline(always)]
    fn from(variant: Mpusegie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEGIE` reader - Enable NMI Event if a Segment violation"]
pub type MpusegieR = crate::BitReader<Mpusegie>;
impl MpusegieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpusegie {
        match self.bits {
            false => Mpusegie::Disable,
            true => Mpusegie::Enable,
        }
    }
    #[doc = "Segment violation interrupt disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpusegie::Disable
    }
    #[doc = "Segment violation interrupt enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpusegie::Enable
    }
}
#[doc = "Field `MPUSEGIE` writer - Enable NMI Event if a Segment violation"]
pub type MpusegieW<'a, REG> = crate::BitWriter<'a, REG, Mpusegie>;
impl<'a, REG> MpusegieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Segment violation interrupt disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpusegie::Disable)
    }
    #[doc = "Segment violation interrupt enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpusegie::Enable)
    }
}
#[doc = "Field `MPUPW` reader - MPU Password"]
pub type MpupwR = crate::FieldReader;
#[doc = "Field `MPUPW` writer - MPU Password"]
pub type MpupwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - MPU Enable"]
    #[inline(always)]
    pub fn mpuena(&self) -> MpuenaR {
        MpuenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MPU Lock"]
    #[inline(always)]
    pub fn mpulock(&self) -> MpulockR {
        MpulockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable NMI Event if a Segment violation"]
    #[inline(always)]
    pub fn mpusegie(&self) -> MpusegieR {
        MpusegieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - MPU Password"]
    #[inline(always)]
    pub fn mpupw(&self) -> MpupwR {
        MpupwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MPU Enable"]
    #[inline(always)]
    pub fn mpuena(&mut self) -> MpuenaW<'_, Mpuctl0Spec> {
        MpuenaW::new(self, 0)
    }
    #[doc = "Bit 1 - MPU Lock"]
    #[inline(always)]
    pub fn mpulock(&mut self) -> MpulockW<'_, Mpuctl0Spec> {
        MpulockW::new(self, 1)
    }
    #[doc = "Bit 4 - Enable NMI Event if a Segment violation"]
    #[inline(always)]
    pub fn mpusegie(&mut self) -> MpusegieW<'_, Mpuctl0Spec> {
        MpusegieW::new(self, 4)
    }
    #[doc = "Bits 8:15 - MPU Password"]
    #[inline(always)]
    pub fn mpupw(&mut self) -> MpupwW<'_, Mpuctl0Spec> {
        MpupwW::new(self, 8)
    }
}
#[doc = "Memory Protection Unit Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mpuctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpuctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpuctl0Spec;
impl crate::RegisterSpec for Mpuctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpuctl0::R`](R) reader structure"]
impl crate::Readable for Mpuctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`mpuctl0::W`](W) writer structure"]
impl crate::Writable for Mpuctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPUCTL0 to value 0"]
impl crate::Resettable for Mpuctl0Spec {}
