#[doc = "Register `FCTL2` reader"]
pub type R = crate::R<Fctl2Spec>;
#[doc = "Register `FCTL2` writer"]
pub type W = crate::W<Fctl2Spec>;
#[doc = "Field `FN0` reader - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
pub type Fn0R = crate::BitReader;
#[doc = "Field `FN0` writer - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
pub type Fn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FN1` reader - 32*FN5 + 16*FN4 + 8*FN3 + 4*FN2 + 2*FN1 + FN0 + 1"]
pub type Fn1R = crate::BitReader;
#[doc = "Field `FN1` writer - 32*FN5 + 16*FN4 + 8*FN3 + 4*FN2 + 2*FN1 + FN0 + 1"]
pub type Fn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FN2` reader - FN2"]
pub type Fn2R = crate::BitReader;
#[doc = "Field `FN2` writer - FN2"]
pub type Fn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FN3` reader - FN3"]
pub type Fn3R = crate::BitReader;
#[doc = "Field `FN3` writer - FN3"]
pub type Fn3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FN4` reader - FN4"]
pub type Fn4R = crate::BitReader;
#[doc = "Field `FN4` writer - FN4"]
pub type Fn4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FN5` reader - FN5"]
pub type Fn5R = crate::BitReader;
#[doc = "Field `FN5` writer - FN5"]
pub type Fn5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Flash clock select 0 */ /* to distinguish from USART SSELx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fssel {
    #[doc = "0: Flash clock select: 0 - ACLK"]
    Fssel0 = 0,
    #[doc = "1: Flash clock select: 1 - MCLK"]
    Fssel1 = 1,
    #[doc = "2: Flash clock select: 2 - SMCLK"]
    Fssel2 = 2,
    #[doc = "3: Flash clock select: 3 - SMCLK"]
    Fssel3 = 3,
}
impl From<Fssel> for u8 {
    #[inline(always)]
    fn from(variant: Fssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fssel {
    type Ux = u8;
}
impl crate::IsEnum for Fssel {}
#[doc = "Field `FSSEL` reader - Flash clock select 0 */ /* to distinguish from USART SSELx"]
pub type FsselR = crate::FieldReader<Fssel>;
impl FsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fssel {
        match self.bits {
            0 => Fssel::Fssel0,
            1 => Fssel::Fssel1,
            2 => Fssel::Fssel2,
            3 => Fssel::Fssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "Flash clock select: 0 - ACLK"]
    #[inline(always)]
    pub fn is_fssel_0(&self) -> bool {
        *self == Fssel::Fssel0
    }
    #[doc = "Flash clock select: 1 - MCLK"]
    #[inline(always)]
    pub fn is_fssel_1(&self) -> bool {
        *self == Fssel::Fssel1
    }
    #[doc = "Flash clock select: 2 - SMCLK"]
    #[inline(always)]
    pub fn is_fssel_2(&self) -> bool {
        *self == Fssel::Fssel2
    }
    #[doc = "Flash clock select: 3 - SMCLK"]
    #[inline(always)]
    pub fn is_fssel_3(&self) -> bool {
        *self == Fssel::Fssel3
    }
}
#[doc = "Field `FSSEL` writer - Flash clock select 0 */ /* to distinguish from USART SSELx"]
pub type FsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fssel, crate::Safe>;
impl<'a, REG> FsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flash clock select: 0 - ACLK"]
    #[inline(always)]
    pub fn fssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fssel::Fssel0)
    }
    #[doc = "Flash clock select: 1 - MCLK"]
    #[inline(always)]
    pub fn fssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fssel::Fssel1)
    }
    #[doc = "Flash clock select: 2 - SMCLK"]
    #[inline(always)]
    pub fn fssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Fssel::Fssel2)
    }
    #[doc = "Flash clock select: 3 - SMCLK"]
    #[inline(always)]
    pub fn fssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Fssel::Fssel3)
    }
}
impl R {
    #[doc = "Bit 0 - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
    #[inline(always)]
    pub fn fn0(&self) -> Fn0R {
        Fn0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 32*FN5 + 16*FN4 + 8*FN3 + 4*FN2 + 2*FN1 + FN0 + 1"]
    #[inline(always)]
    pub fn fn1(&self) -> Fn1R {
        Fn1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FN2"]
    #[inline(always)]
    pub fn fn2(&self) -> Fn2R {
        Fn2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FN3"]
    #[inline(always)]
    pub fn fn3(&self) -> Fn3R {
        Fn3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FN4"]
    #[inline(always)]
    pub fn fn4(&self) -> Fn4R {
        Fn4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FN5"]
    #[inline(always)]
    pub fn fn5(&self) -> Fn5R {
        Fn5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Flash clock select 0 */ /* to distinguish from USART SSELx"]
    #[inline(always)]
    pub fn fssel(&self) -> FsselR {
        FsselR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
    #[inline(always)]
    pub fn fn0(&mut self) -> Fn0W<'_, Fctl2Spec> {
        Fn0W::new(self, 0)
    }
    #[doc = "Bit 1 - 32*FN5 + 16*FN4 + 8*FN3 + 4*FN2 + 2*FN1 + FN0 + 1"]
    #[inline(always)]
    pub fn fn1(&mut self) -> Fn1W<'_, Fctl2Spec> {
        Fn1W::new(self, 1)
    }
    #[doc = "Bit 2 - FN2"]
    #[inline(always)]
    pub fn fn2(&mut self) -> Fn2W<'_, Fctl2Spec> {
        Fn2W::new(self, 2)
    }
    #[doc = "Bit 3 - FN3"]
    #[inline(always)]
    pub fn fn3(&mut self) -> Fn3W<'_, Fctl2Spec> {
        Fn3W::new(self, 3)
    }
    #[doc = "Bit 4 - FN4"]
    #[inline(always)]
    pub fn fn4(&mut self) -> Fn4W<'_, Fctl2Spec> {
        Fn4W::new(self, 4)
    }
    #[doc = "Bit 5 - FN5"]
    #[inline(always)]
    pub fn fn5(&mut self) -> Fn5W<'_, Fctl2Spec> {
        Fn5W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Flash clock select 0 */ /* to distinguish from USART SSELx"]
    #[inline(always)]
    pub fn fssel(&mut self) -> FsselW<'_, Fctl2Spec> {
        FsselW::new(self, 6)
    }
}
#[doc = "FLASH Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fctl2Spec;
impl crate::RegisterSpec for Fctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fctl2::R`](R) reader structure"]
impl crate::Readable for Fctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`fctl2::W`](W) writer structure"]
impl crate::Writable for Fctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCTL2 to value 0"]
impl crate::Resettable for Fctl2Spec {}
