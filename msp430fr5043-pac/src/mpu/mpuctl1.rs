#[doc = "Register `MPUCTL1` reader"]
pub type R = crate::R<Mpuctl1Spec>;
#[doc = "Register `MPUCTL1` writer"]
pub type W = crate::W<Mpuctl1Spec>;
#[doc = "Main Memory Segment 1 Violation Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuseg1ifg {
    #[doc = "0: No interrupt pending"]
    Mpuseg1ifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Mpuseg1ifg1 = 1,
}
impl From<Mpuseg1ifg> for bool {
    #[inline(always)]
    fn from(variant: Mpuseg1ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEG1IFG` reader - Main Memory Segment 1 Violation Interrupt Flag"]
pub type Mpuseg1ifgR = crate::BitReader<Mpuseg1ifg>;
impl Mpuseg1ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuseg1ifg {
        match self.bits {
            false => Mpuseg1ifg::Mpuseg1ifg0,
            true => Mpuseg1ifg::Mpuseg1ifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_mpuseg1ifg_0(&self) -> bool {
        *self == Mpuseg1ifg::Mpuseg1ifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_mpuseg1ifg_1(&self) -> bool {
        *self == Mpuseg1ifg::Mpuseg1ifg1
    }
}
#[doc = "Field `MPUSEG1IFG` writer - Main Memory Segment 1 Violation Interrupt Flag"]
pub type Mpuseg1ifgW<'a, REG> = crate::BitWriter<'a, REG, Mpuseg1ifg>;
impl<'a, REG> Mpuseg1ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn mpuseg1ifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg1ifg::Mpuseg1ifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn mpuseg1ifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg1ifg::Mpuseg1ifg1)
    }
}
#[doc = "Main Memory Segment 2 Violation Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuseg2ifg {
    #[doc = "0: No interrupt pending"]
    Mpuseg2ifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Mpuseg2ifg1 = 1,
}
impl From<Mpuseg2ifg> for bool {
    #[inline(always)]
    fn from(variant: Mpuseg2ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEG2IFG` reader - Main Memory Segment 2 Violation Interrupt Flag"]
pub type Mpuseg2ifgR = crate::BitReader<Mpuseg2ifg>;
impl Mpuseg2ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuseg2ifg {
        match self.bits {
            false => Mpuseg2ifg::Mpuseg2ifg0,
            true => Mpuseg2ifg::Mpuseg2ifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_mpuseg2ifg_0(&self) -> bool {
        *self == Mpuseg2ifg::Mpuseg2ifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_mpuseg2ifg_1(&self) -> bool {
        *self == Mpuseg2ifg::Mpuseg2ifg1
    }
}
#[doc = "Field `MPUSEG2IFG` writer - Main Memory Segment 2 Violation Interrupt Flag"]
pub type Mpuseg2ifgW<'a, REG> = crate::BitWriter<'a, REG, Mpuseg2ifg>;
impl<'a, REG> Mpuseg2ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn mpuseg2ifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg2ifg::Mpuseg2ifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn mpuseg2ifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg2ifg::Mpuseg2ifg1)
    }
}
#[doc = "Main Memory Segment 3 Violation Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuseg3ifg {
    #[doc = "0: No interrupt pending"]
    Mpuseg1ifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Mpuseg1ifg1 = 1,
}
impl From<Mpuseg3ifg> for bool {
    #[inline(always)]
    fn from(variant: Mpuseg3ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEG3IFG` reader - Main Memory Segment 3 Violation Interrupt Flag"]
pub type Mpuseg3ifgR = crate::BitReader<Mpuseg3ifg>;
impl Mpuseg3ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuseg3ifg {
        match self.bits {
            false => Mpuseg3ifg::Mpuseg1ifg0,
            true => Mpuseg3ifg::Mpuseg1ifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_mpuseg1ifg_0(&self) -> bool {
        *self == Mpuseg3ifg::Mpuseg1ifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_mpuseg1ifg_1(&self) -> bool {
        *self == Mpuseg3ifg::Mpuseg1ifg1
    }
}
#[doc = "Field `MPUSEG3IFG` writer - Main Memory Segment 3 Violation Interrupt Flag"]
pub type Mpuseg3ifgW<'a, REG> = crate::BitWriter<'a, REG, Mpuseg3ifg>;
impl<'a, REG> Mpuseg3ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn mpuseg1ifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg3ifg::Mpuseg1ifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn mpuseg1ifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuseg3ifg::Mpuseg1ifg1)
    }
}
#[doc = "User Information Memory Violation Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpusegiifg {
    #[doc = "0: No interrupt pending"]
    Mpusegiifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Mpusegiifg1 = 1,
}
impl From<Mpusegiifg> for bool {
    #[inline(always)]
    fn from(variant: Mpusegiifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEGIIFG` reader - User Information Memory Violation Interrupt Flag"]
pub type MpusegiifgR = crate::BitReader<Mpusegiifg>;
impl MpusegiifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpusegiifg {
        match self.bits {
            false => Mpusegiifg::Mpusegiifg0,
            true => Mpusegiifg::Mpusegiifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_mpusegiifg_0(&self) -> bool {
        *self == Mpusegiifg::Mpusegiifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_mpusegiifg_1(&self) -> bool {
        *self == Mpusegiifg::Mpusegiifg1
    }
}
#[doc = "Field `MPUSEGIIFG` writer - User Information Memory Violation Interrupt Flag"]
pub type MpusegiifgW<'a, REG> = crate::BitWriter<'a, REG, Mpusegiifg>;
impl<'a, REG> MpusegiifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn mpusegiifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpusegiifg::Mpusegiifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn mpusegiifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpusegiifg::Mpusegiifg1)
    }
}
#[doc = "IP Encapsulation Access Violation Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpusegipifg {
    #[doc = "0: No interrupt pending"]
    Mpuseg1ifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Mpuseg1ifg1 = 1,
}
impl From<Mpusegipifg> for bool {
    #[inline(always)]
    fn from(variant: Mpusegipifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUSEGIPIFG` reader - IP Encapsulation Access Violation Interrupt Flag"]
pub type MpusegipifgR = crate::BitReader<Mpusegipifg>;
impl MpusegipifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpusegipifg {
        match self.bits {
            false => Mpusegipifg::Mpuseg1ifg0,
            true => Mpusegipifg::Mpuseg1ifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_mpuseg1ifg_0(&self) -> bool {
        *self == Mpusegipifg::Mpuseg1ifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_mpuseg1ifg_1(&self) -> bool {
        *self == Mpusegipifg::Mpuseg1ifg1
    }
}
#[doc = "Field `MPUSEGIPIFG` writer - IP Encapsulation Access Violation Interrupt Flag"]
pub type MpusegipifgW<'a, REG> = crate::BitWriter<'a, REG, Mpusegipifg>;
impl<'a, REG> MpusegipifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn mpuseg1ifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpusegipifg::Mpuseg1ifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn mpuseg1ifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpusegipifg::Mpuseg1ifg1)
    }
}
impl R {
    #[doc = "Bit 0 - Main Memory Segment 1 Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpuseg1ifg(&self) -> Mpuseg1ifgR {
        Mpuseg1ifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Main Memory Segment 2 Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpuseg2ifg(&self) -> Mpuseg2ifgR {
        Mpuseg2ifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Main Memory Segment 3 Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpuseg3ifg(&self) -> Mpuseg3ifgR {
        Mpuseg3ifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - User Information Memory Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpusegiifg(&self) -> MpusegiifgR {
        MpusegiifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IP Encapsulation Access Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpusegipifg(&self) -> MpusegipifgR {
        MpusegipifgR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main Memory Segment 1 Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpuseg1ifg(&mut self) -> Mpuseg1ifgW<'_, Mpuctl1Spec> {
        Mpuseg1ifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Main Memory Segment 2 Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpuseg2ifg(&mut self) -> Mpuseg2ifgW<'_, Mpuctl1Spec> {
        Mpuseg2ifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Main Memory Segment 3 Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpuseg3ifg(&mut self) -> Mpuseg3ifgW<'_, Mpuctl1Spec> {
        Mpuseg3ifgW::new(self, 2)
    }
    #[doc = "Bit 3 - User Information Memory Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpusegiifg(&mut self) -> MpusegiifgW<'_, Mpuctl1Spec> {
        MpusegiifgW::new(self, 3)
    }
    #[doc = "Bit 4 - IP Encapsulation Access Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpusegipifg(&mut self) -> MpusegipifgW<'_, Mpuctl1Spec> {
        MpusegipifgW::new(self, 4)
    }
}
#[doc = "Memory Protection Unit Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mpuctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpuctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpuctl1Spec;
impl crate::RegisterSpec for Mpuctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpuctl1::R`](R) reader structure"]
impl crate::Readable for Mpuctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`mpuctl1::W`](W) writer structure"]
impl crate::Writable for Mpuctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPUCTL1 to value 0"]
impl crate::Resettable for Mpuctl1Spec {}
