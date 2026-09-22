#[doc = "Register `CSCTL2` reader"]
pub type R = crate::R<Csctl2Spec>;
#[doc = "Register `CSCTL2` writer"]
pub type W = crate::W<Csctl2Spec>;
#[doc = "Field `FLLN0` reader - FLL Multipier Bit : 0"]
pub type Flln0R = crate::BitReader;
#[doc = "Field `FLLN0` writer - FLL Multipier Bit : 0"]
pub type Flln0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN1` reader - FLL Multipier Bit : 1"]
pub type Flln1R = crate::BitReader;
#[doc = "Field `FLLN1` writer - FLL Multipier Bit : 1"]
pub type Flln1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN2` reader - FLL Multipier Bit : 2"]
pub type Flln2R = crate::BitReader;
#[doc = "Field `FLLN2` writer - FLL Multipier Bit : 2"]
pub type Flln2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN3` reader - FLL Multipier Bit : 3"]
pub type Flln3R = crate::BitReader;
#[doc = "Field `FLLN3` writer - FLL Multipier Bit : 3"]
pub type Flln3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN4` reader - FLL Multipier Bit : 4"]
pub type Flln4R = crate::BitReader;
#[doc = "Field `FLLN4` writer - FLL Multipier Bit : 4"]
pub type Flln4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN5` reader - FLL Multipier Bit : 5"]
pub type Flln5R = crate::BitReader;
#[doc = "Field `FLLN5` writer - FLL Multipier Bit : 5"]
pub type Flln5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN6` reader - FLL Multipier Bit : 6"]
pub type Flln6R = crate::BitReader;
#[doc = "Field `FLLN6` writer - FLL Multipier Bit : 6"]
pub type Flln6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN7` reader - FLL Multipier Bit : 7"]
pub type Flln7R = crate::BitReader;
#[doc = "Field `FLLN7` writer - FLL Multipier Bit : 7"]
pub type Flln7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN8` reader - FLL Multipier Bit : 8"]
pub type Flln8R = crate::BitReader;
#[doc = "Field `FLLN8` writer - FLL Multipier Bit : 8"]
pub type Flln8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN9` reader - FLL Multipier Bit : 9"]
pub type Flln9R = crate::BitReader;
#[doc = "Field `FLLN9` writer - FLL Multipier Bit : 9"]
pub type Flln9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Loop Divider Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flld {
    #[doc = "0: Multiply Selected Loop Freq. By 1"]
    Flld0 = 0,
    #[doc = "1: Multiply Selected Loop Freq. By 2"]
    Flld1 = 1,
    #[doc = "2: Multiply Selected Loop Freq. By 4"]
    Flld2 = 2,
    #[doc = "3: Multiply Selected Loop Freq. By 8"]
    Flld3 = 3,
    #[doc = "4: Multiply Selected Loop Freq. By 16"]
    Flld4 = 4,
    #[doc = "5: Multiply Selected Loop Freq. By 32"]
    Flld5 = 5,
    #[doc = "6: Reserved"]
    Flld6 = 6,
    #[doc = "7: Reserved"]
    Flld7 = 7,
}
impl From<Flld> for u8 {
    #[inline(always)]
    fn from(variant: Flld) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flld {
    type Ux = u8;
}
impl crate::IsEnum for Flld {}
#[doc = "Field `FLLD` reader - Loop Divider Bit : 0"]
pub type FlldR = crate::FieldReader<Flld>;
impl FlldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flld {
        match self.bits {
            0 => Flld::Flld0,
            1 => Flld::Flld1,
            2 => Flld::Flld2,
            3 => Flld::Flld3,
            4 => Flld::Flld4,
            5 => Flld::Flld5,
            6 => Flld::Flld6,
            7 => Flld::Flld7,
            _ => unreachable!(),
        }
    }
    #[doc = "Multiply Selected Loop Freq. By 1"]
    #[inline(always)]
    pub fn is_flld_0(&self) -> bool {
        *self == Flld::Flld0
    }
    #[doc = "Multiply Selected Loop Freq. By 2"]
    #[inline(always)]
    pub fn is_flld_1(&self) -> bool {
        *self == Flld::Flld1
    }
    #[doc = "Multiply Selected Loop Freq. By 4"]
    #[inline(always)]
    pub fn is_flld_2(&self) -> bool {
        *self == Flld::Flld2
    }
    #[doc = "Multiply Selected Loop Freq. By 8"]
    #[inline(always)]
    pub fn is_flld_3(&self) -> bool {
        *self == Flld::Flld3
    }
    #[doc = "Multiply Selected Loop Freq. By 16"]
    #[inline(always)]
    pub fn is_flld_4(&self) -> bool {
        *self == Flld::Flld4
    }
    #[doc = "Multiply Selected Loop Freq. By 32"]
    #[inline(always)]
    pub fn is_flld_5(&self) -> bool {
        *self == Flld::Flld5
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_flld_6(&self) -> bool {
        *self == Flld::Flld6
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_flld_7(&self) -> bool {
        *self == Flld::Flld7
    }
}
#[doc = "Field `FLLD` writer - Loop Divider Bit : 0"]
pub type FlldW<'a, REG> = crate::FieldWriter<'a, REG, 3, Flld, crate::Safe>;
impl<'a, REG> FlldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Multiply Selected Loop Freq. By 1"]
    #[inline(always)]
    pub fn flld_0(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::Flld0)
    }
    #[doc = "Multiply Selected Loop Freq. By 2"]
    #[inline(always)]
    pub fn flld_1(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::Flld1)
    }
    #[doc = "Multiply Selected Loop Freq. By 4"]
    #[inline(always)]
    pub fn flld_2(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::Flld2)
    }
    #[doc = "Multiply Selected Loop Freq. By 8"]
    #[inline(always)]
    pub fn flld_3(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::Flld3)
    }
    #[doc = "Multiply Selected Loop Freq. By 16"]
    #[inline(always)]
    pub fn flld_4(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::Flld4)
    }
    #[doc = "Multiply Selected Loop Freq. By 32"]
    #[inline(always)]
    pub fn flld_5(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::Flld5)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn flld_6(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::Flld6)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn flld_7(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::Flld7)
    }
}
impl R {
    #[doc = "Bit 0 - FLL Multipier Bit : 0"]
    #[inline(always)]
    pub fn flln0(&self) -> Flln0R {
        Flln0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLL Multipier Bit : 1"]
    #[inline(always)]
    pub fn flln1(&self) -> Flln1R {
        Flln1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLL Multipier Bit : 2"]
    #[inline(always)]
    pub fn flln2(&self) -> Flln2R {
        Flln2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FLL Multipier Bit : 3"]
    #[inline(always)]
    pub fn flln3(&self) -> Flln3R {
        Flln3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FLL Multipier Bit : 4"]
    #[inline(always)]
    pub fn flln4(&self) -> Flln4R {
        Flln4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FLL Multipier Bit : 5"]
    #[inline(always)]
    pub fn flln5(&self) -> Flln5R {
        Flln5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FLL Multipier Bit : 6"]
    #[inline(always)]
    pub fn flln6(&self) -> Flln6R {
        Flln6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FLL Multipier Bit : 7"]
    #[inline(always)]
    pub fn flln7(&self) -> Flln7R {
        Flln7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLL Multipier Bit : 8"]
    #[inline(always)]
    pub fn flln8(&self) -> Flln8R {
        Flln8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLL Multipier Bit : 9"]
    #[inline(always)]
    pub fn flln9(&self) -> Flln9R {
        Flln9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Loop Divider Bit : 0"]
    #[inline(always)]
    pub fn flld(&self) -> FlldR {
        FlldR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FLL Multipier Bit : 0"]
    #[inline(always)]
    pub fn flln0(&mut self) -> Flln0W<'_, Csctl2Spec> {
        Flln0W::new(self, 0)
    }
    #[doc = "Bit 1 - FLL Multipier Bit : 1"]
    #[inline(always)]
    pub fn flln1(&mut self) -> Flln1W<'_, Csctl2Spec> {
        Flln1W::new(self, 1)
    }
    #[doc = "Bit 2 - FLL Multipier Bit : 2"]
    #[inline(always)]
    pub fn flln2(&mut self) -> Flln2W<'_, Csctl2Spec> {
        Flln2W::new(self, 2)
    }
    #[doc = "Bit 3 - FLL Multipier Bit : 3"]
    #[inline(always)]
    pub fn flln3(&mut self) -> Flln3W<'_, Csctl2Spec> {
        Flln3W::new(self, 3)
    }
    #[doc = "Bit 4 - FLL Multipier Bit : 4"]
    #[inline(always)]
    pub fn flln4(&mut self) -> Flln4W<'_, Csctl2Spec> {
        Flln4W::new(self, 4)
    }
    #[doc = "Bit 5 - FLL Multipier Bit : 5"]
    #[inline(always)]
    pub fn flln5(&mut self) -> Flln5W<'_, Csctl2Spec> {
        Flln5W::new(self, 5)
    }
    #[doc = "Bit 6 - FLL Multipier Bit : 6"]
    #[inline(always)]
    pub fn flln6(&mut self) -> Flln6W<'_, Csctl2Spec> {
        Flln6W::new(self, 6)
    }
    #[doc = "Bit 7 - FLL Multipier Bit : 7"]
    #[inline(always)]
    pub fn flln7(&mut self) -> Flln7W<'_, Csctl2Spec> {
        Flln7W::new(self, 7)
    }
    #[doc = "Bit 8 - FLL Multipier Bit : 8"]
    #[inline(always)]
    pub fn flln8(&mut self) -> Flln8W<'_, Csctl2Spec> {
        Flln8W::new(self, 8)
    }
    #[doc = "Bit 9 - FLL Multipier Bit : 9"]
    #[inline(always)]
    pub fn flln9(&mut self) -> Flln9W<'_, Csctl2Spec> {
        Flln9W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Loop Divider Bit : 0"]
    #[inline(always)]
    pub fn flld(&mut self) -> FlldW<'_, Csctl2Spec> {
        FlldW::new(self, 12)
    }
}
#[doc = "CS Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl2Spec;
impl crate::RegisterSpec for Csctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl2::R`](R) reader structure"]
impl crate::Readable for Csctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl2::W`](W) writer structure"]
impl crate::Writable for Csctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL2 to value 0"]
impl crate::Resettable for Csctl2Spec {}
