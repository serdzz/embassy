#[doc = "Register `CSCTL1` reader"]
pub type R = crate::R<Csctl1Spec>;
#[doc = "Register `CSCTL1` writer"]
pub type W = crate::W<Csctl1Spec>;
#[doc = "Field `DISMOD` reader - Disable Modulation"]
pub type DismodR = crate::BitReader;
#[doc = "Field `DISMOD` writer - Disable Modulation"]
pub type DismodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DCO frequency range select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcorsel {
    #[doc = "0: DCO frequency range select: 0"]
    Dcorsel0 = 0,
    #[doc = "1: DCO frequency range select: 1"]
    Dcorsel1 = 1,
    #[doc = "2: DCO frequency range select: 2"]
    Dcorsel2 = 2,
    #[doc = "3: DCO frequency range select: 3"]
    Dcorsel3 = 3,
    #[doc = "4: DCO frequency range select: 4"]
    Dcorsel4 = 4,
    #[doc = "5: DCO frequency range select: 5"]
    Dcorsel5 = 5,
    #[doc = "6: DCO frequency range select: 6"]
    Dcorsel6 = 6,
    #[doc = "7: DCO frequency range select: 7"]
    Dcorsel7 = 7,
}
impl From<Dcorsel> for u8 {
    #[inline(always)]
    fn from(variant: Dcorsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dcorsel {
    type Ux = u8;
}
impl crate::IsEnum for Dcorsel {}
#[doc = "Field `DCORSEL` reader - DCO frequency range select Bit: 0"]
pub type DcorselR = crate::FieldReader<Dcorsel>;
impl DcorselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcorsel {
        match self.bits {
            0 => Dcorsel::Dcorsel0,
            1 => Dcorsel::Dcorsel1,
            2 => Dcorsel::Dcorsel2,
            3 => Dcorsel::Dcorsel3,
            4 => Dcorsel::Dcorsel4,
            5 => Dcorsel::Dcorsel5,
            6 => Dcorsel::Dcorsel6,
            7 => Dcorsel::Dcorsel7,
            _ => unreachable!(),
        }
    }
    #[doc = "DCO frequency range select: 0"]
    #[inline(always)]
    pub fn is_dcorsel_0(&self) -> bool {
        *self == Dcorsel::Dcorsel0
    }
    #[doc = "DCO frequency range select: 1"]
    #[inline(always)]
    pub fn is_dcorsel_1(&self) -> bool {
        *self == Dcorsel::Dcorsel1
    }
    #[doc = "DCO frequency range select: 2"]
    #[inline(always)]
    pub fn is_dcorsel_2(&self) -> bool {
        *self == Dcorsel::Dcorsel2
    }
    #[doc = "DCO frequency range select: 3"]
    #[inline(always)]
    pub fn is_dcorsel_3(&self) -> bool {
        *self == Dcorsel::Dcorsel3
    }
    #[doc = "DCO frequency range select: 4"]
    #[inline(always)]
    pub fn is_dcorsel_4(&self) -> bool {
        *self == Dcorsel::Dcorsel4
    }
    #[doc = "DCO frequency range select: 5"]
    #[inline(always)]
    pub fn is_dcorsel_5(&self) -> bool {
        *self == Dcorsel::Dcorsel5
    }
    #[doc = "DCO frequency range select: 6"]
    #[inline(always)]
    pub fn is_dcorsel_6(&self) -> bool {
        *self == Dcorsel::Dcorsel6
    }
    #[doc = "DCO frequency range select: 7"]
    #[inline(always)]
    pub fn is_dcorsel_7(&self) -> bool {
        *self == Dcorsel::Dcorsel7
    }
}
#[doc = "Field `DCORSEL` writer - DCO frequency range select Bit: 0"]
pub type DcorselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dcorsel, crate::Safe>;
impl<'a, REG> DcorselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DCO frequency range select: 0"]
    #[inline(always)]
    pub fn dcorsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel0)
    }
    #[doc = "DCO frequency range select: 1"]
    #[inline(always)]
    pub fn dcorsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel1)
    }
    #[doc = "DCO frequency range select: 2"]
    #[inline(always)]
    pub fn dcorsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel2)
    }
    #[doc = "DCO frequency range select: 3"]
    #[inline(always)]
    pub fn dcorsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel3)
    }
    #[doc = "DCO frequency range select: 4"]
    #[inline(always)]
    pub fn dcorsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel4)
    }
    #[doc = "DCO frequency range select: 5"]
    #[inline(always)]
    pub fn dcorsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel5)
    }
    #[doc = "DCO frequency range select: 6"]
    #[inline(always)]
    pub fn dcorsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel6)
    }
    #[doc = "DCO frequency range select: 7"]
    #[inline(always)]
    pub fn dcorsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel7)
    }
}
#[doc = "DCO frequency trim. Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcoftrim {
    #[doc = "0: DCO frequency trim: 0"]
    Dcoftrim0 = 0,
    #[doc = "1: DCO frequency trim: 1"]
    Dcoftrim1 = 1,
    #[doc = "2: DCO frequency trim: 2"]
    Dcoftrim2 = 2,
    #[doc = "3: DCO frequency trim: 3"]
    Dcoftrim3 = 3,
    #[doc = "4: DCO frequency trim: 4"]
    Dcoftrim4 = 4,
    #[doc = "5: DCO frequency trim: 5"]
    Dcoftrim5 = 5,
    #[doc = "6: DCO frequency trim: 6"]
    Dcoftrim6 = 6,
    #[doc = "7: DCO frequency trim: 7"]
    Dcoftrim7 = 7,
}
impl From<Dcoftrim> for u8 {
    #[inline(always)]
    fn from(variant: Dcoftrim) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dcoftrim {
    type Ux = u8;
}
impl crate::IsEnum for Dcoftrim {}
#[doc = "Field `DCOFTRIM` reader - DCO frequency trim. Bit: 0"]
pub type DcoftrimR = crate::FieldReader<Dcoftrim>;
impl DcoftrimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcoftrim {
        match self.bits {
            0 => Dcoftrim::Dcoftrim0,
            1 => Dcoftrim::Dcoftrim1,
            2 => Dcoftrim::Dcoftrim2,
            3 => Dcoftrim::Dcoftrim3,
            4 => Dcoftrim::Dcoftrim4,
            5 => Dcoftrim::Dcoftrim5,
            6 => Dcoftrim::Dcoftrim6,
            7 => Dcoftrim::Dcoftrim7,
            _ => unreachable!(),
        }
    }
    #[doc = "DCO frequency trim: 0"]
    #[inline(always)]
    pub fn is_dcoftrim_0(&self) -> bool {
        *self == Dcoftrim::Dcoftrim0
    }
    #[doc = "DCO frequency trim: 1"]
    #[inline(always)]
    pub fn is_dcoftrim_1(&self) -> bool {
        *self == Dcoftrim::Dcoftrim1
    }
    #[doc = "DCO frequency trim: 2"]
    #[inline(always)]
    pub fn is_dcoftrim_2(&self) -> bool {
        *self == Dcoftrim::Dcoftrim2
    }
    #[doc = "DCO frequency trim: 3"]
    #[inline(always)]
    pub fn is_dcoftrim_3(&self) -> bool {
        *self == Dcoftrim::Dcoftrim3
    }
    #[doc = "DCO frequency trim: 4"]
    #[inline(always)]
    pub fn is_dcoftrim_4(&self) -> bool {
        *self == Dcoftrim::Dcoftrim4
    }
    #[doc = "DCO frequency trim: 5"]
    #[inline(always)]
    pub fn is_dcoftrim_5(&self) -> bool {
        *self == Dcoftrim::Dcoftrim5
    }
    #[doc = "DCO frequency trim: 6"]
    #[inline(always)]
    pub fn is_dcoftrim_6(&self) -> bool {
        *self == Dcoftrim::Dcoftrim6
    }
    #[doc = "DCO frequency trim: 7"]
    #[inline(always)]
    pub fn is_dcoftrim_7(&self) -> bool {
        *self == Dcoftrim::Dcoftrim7
    }
}
#[doc = "Field `DCOFTRIM` writer - DCO frequency trim. Bit: 0"]
pub type DcoftrimW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dcoftrim, crate::Safe>;
impl<'a, REG> DcoftrimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DCO frequency trim: 0"]
    #[inline(always)]
    pub fn dcoftrim_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcoftrim::Dcoftrim0)
    }
    #[doc = "DCO frequency trim: 1"]
    #[inline(always)]
    pub fn dcoftrim_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcoftrim::Dcoftrim1)
    }
    #[doc = "DCO frequency trim: 2"]
    #[inline(always)]
    pub fn dcoftrim_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dcoftrim::Dcoftrim2)
    }
    #[doc = "DCO frequency trim: 3"]
    #[inline(always)]
    pub fn dcoftrim_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dcoftrim::Dcoftrim3)
    }
    #[doc = "DCO frequency trim: 4"]
    #[inline(always)]
    pub fn dcoftrim_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dcoftrim::Dcoftrim4)
    }
    #[doc = "DCO frequency trim: 5"]
    #[inline(always)]
    pub fn dcoftrim_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dcoftrim::Dcoftrim5)
    }
    #[doc = "DCO frequency trim: 6"]
    #[inline(always)]
    pub fn dcoftrim_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dcoftrim::Dcoftrim6)
    }
    #[doc = "DCO frequency trim: 7"]
    #[inline(always)]
    pub fn dcoftrim_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dcoftrim::Dcoftrim7)
    }
}
#[doc = "Field `DCOFTRIMEN` reader - DCO frequency trim enable"]
pub type DcoftrimenR = crate::BitReader;
#[doc = "Field `DCOFTRIMEN` writer - DCO frequency trim enable"]
pub type DcoftrimenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable Modulation"]
    #[inline(always)]
    pub fn dismod(&self) -> DismodR {
        DismodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - DCO frequency range select Bit: 0"]
    #[inline(always)]
    pub fn dcorsel(&self) -> DcorselR {
        DcorselR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - DCO frequency trim. Bit: 0"]
    #[inline(always)]
    pub fn dcoftrim(&self) -> DcoftrimR {
        DcoftrimR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - DCO frequency trim enable"]
    #[inline(always)]
    pub fn dcoftrimen(&self) -> DcoftrimenR {
        DcoftrimenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Modulation"]
    #[inline(always)]
    pub fn dismod(&mut self) -> DismodW<'_, Csctl1Spec> {
        DismodW::new(self, 0)
    }
    #[doc = "Bits 1:3 - DCO frequency range select Bit: 0"]
    #[inline(always)]
    pub fn dcorsel(&mut self) -> DcorselW<'_, Csctl1Spec> {
        DcorselW::new(self, 1)
    }
    #[doc = "Bits 4:6 - DCO frequency trim. Bit: 0"]
    #[inline(always)]
    pub fn dcoftrim(&mut self) -> DcoftrimW<'_, Csctl1Spec> {
        DcoftrimW::new(self, 4)
    }
    #[doc = "Bit 7 - DCO frequency trim enable"]
    #[inline(always)]
    pub fn dcoftrimen(&mut self) -> DcoftrimenW<'_, Csctl1Spec> {
        DcoftrimenW::new(self, 7)
    }
}
#[doc = "CS Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl1Spec;
impl crate::RegisterSpec for Csctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl1::R`](R) reader structure"]
impl crate::Readable for Csctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl1::W`](W) writer structure"]
impl crate::Writable for Csctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL1 to value 0"]
impl crate::Resettable for Csctl1Spec {}
