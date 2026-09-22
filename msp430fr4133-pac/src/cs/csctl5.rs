#[doc = "Register `CSCTL5` reader"]
pub type R = crate::R<Csctl5Spec>;
#[doc = "Register `CSCTL5` writer"]
pub type W = crate::W<Csctl5Spec>;
#[doc = "MCLK Divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divm {
    #[doc = "0: MCLK Source Divider 0"]
    Divm0 = 0,
    #[doc = "1: MCLK Source Divider 1"]
    Divm1 = 1,
    #[doc = "2: MCLK Source Divider 2"]
    Divm2 = 2,
    #[doc = "3: MCLK Source Divider 3"]
    Divm3 = 3,
    #[doc = "4: MCLK Source Divider 4"]
    Divm4 = 4,
    #[doc = "5: MCLK Source Divider 5"]
    Divm5 = 5,
    #[doc = "6: MCLK Source Divider 6"]
    Divm6 = 6,
    #[doc = "7: MCLK Source Divider 7"]
    Divm7 = 7,
}
impl From<Divm> for u8 {
    #[inline(always)]
    fn from(variant: Divm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divm {
    type Ux = u8;
}
impl crate::IsEnum for Divm {}
#[doc = "Field `DIVM` reader - MCLK Divider Bit: 0"]
pub type DivmR = crate::FieldReader<Divm>;
impl DivmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Divm {
        match self.bits {
            0 => Divm::Divm0,
            1 => Divm::Divm1,
            2 => Divm::Divm2,
            3 => Divm::Divm3,
            4 => Divm::Divm4,
            5 => Divm::Divm5,
            6 => Divm::Divm6,
            7 => Divm::Divm7,
            _ => unreachable!(),
        }
    }
    #[doc = "MCLK Source Divider 0"]
    #[inline(always)]
    pub fn is_divm_0(&self) -> bool {
        *self == Divm::Divm0
    }
    #[doc = "MCLK Source Divider 1"]
    #[inline(always)]
    pub fn is_divm_1(&self) -> bool {
        *self == Divm::Divm1
    }
    #[doc = "MCLK Source Divider 2"]
    #[inline(always)]
    pub fn is_divm_2(&self) -> bool {
        *self == Divm::Divm2
    }
    #[doc = "MCLK Source Divider 3"]
    #[inline(always)]
    pub fn is_divm_3(&self) -> bool {
        *self == Divm::Divm3
    }
    #[doc = "MCLK Source Divider 4"]
    #[inline(always)]
    pub fn is_divm_4(&self) -> bool {
        *self == Divm::Divm4
    }
    #[doc = "MCLK Source Divider 5"]
    #[inline(always)]
    pub fn is_divm_5(&self) -> bool {
        *self == Divm::Divm5
    }
    #[doc = "MCLK Source Divider 6"]
    #[inline(always)]
    pub fn is_divm_6(&self) -> bool {
        *self == Divm::Divm6
    }
    #[doc = "MCLK Source Divider 7"]
    #[inline(always)]
    pub fn is_divm_7(&self) -> bool {
        *self == Divm::Divm7
    }
}
#[doc = "Field `DIVM` writer - MCLK Divider Bit: 0"]
pub type DivmW<'a, REG> = crate::FieldWriter<'a, REG, 3, Divm, crate::Safe>;
impl<'a, REG> DivmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCLK Source Divider 0"]
    #[inline(always)]
    pub fn divm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm0)
    }
    #[doc = "MCLK Source Divider 1"]
    #[inline(always)]
    pub fn divm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm1)
    }
    #[doc = "MCLK Source Divider 2"]
    #[inline(always)]
    pub fn divm_2(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm2)
    }
    #[doc = "MCLK Source Divider 3"]
    #[inline(always)]
    pub fn divm_3(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm3)
    }
    #[doc = "MCLK Source Divider 4"]
    #[inline(always)]
    pub fn divm_4(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm4)
    }
    #[doc = "MCLK Source Divider 5"]
    #[inline(always)]
    pub fn divm_5(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm5)
    }
    #[doc = "MCLK Source Divider 6"]
    #[inline(always)]
    pub fn divm_6(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm6)
    }
    #[doc = "MCLK Source Divider 7"]
    #[inline(always)]
    pub fn divm_7(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm7)
    }
}
#[doc = "SMCLK Divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divs {
    #[doc = "0: SMCLK Source Divider 0"]
    Divs0 = 0,
    #[doc = "1: SMCLK Source Divider 1"]
    Divs1 = 1,
    #[doc = "2: SMCLK Source Divider 2"]
    Divs2 = 2,
    #[doc = "3: SMCLK Source Divider 3"]
    Divs3 = 3,
}
impl From<Divs> for u8 {
    #[inline(always)]
    fn from(variant: Divs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divs {
    type Ux = u8;
}
impl crate::IsEnum for Divs {}
#[doc = "Field `DIVS` reader - SMCLK Divider Bit: 0"]
pub type DivsR = crate::FieldReader<Divs>;
impl DivsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Divs {
        match self.bits {
            0 => Divs::Divs0,
            1 => Divs::Divs1,
            2 => Divs::Divs2,
            3 => Divs::Divs3,
            _ => unreachable!(),
        }
    }
    #[doc = "SMCLK Source Divider 0"]
    #[inline(always)]
    pub fn is_divs_0(&self) -> bool {
        *self == Divs::Divs0
    }
    #[doc = "SMCLK Source Divider 1"]
    #[inline(always)]
    pub fn is_divs_1(&self) -> bool {
        *self == Divs::Divs1
    }
    #[doc = "SMCLK Source Divider 2"]
    #[inline(always)]
    pub fn is_divs_2(&self) -> bool {
        *self == Divs::Divs2
    }
    #[doc = "SMCLK Source Divider 3"]
    #[inline(always)]
    pub fn is_divs_3(&self) -> bool {
        *self == Divs::Divs3
    }
}
#[doc = "Field `DIVS` writer - SMCLK Divider Bit: 0"]
pub type DivsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Divs, crate::Safe>;
impl<'a, REG> DivsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SMCLK Source Divider 0"]
    #[inline(always)]
    pub fn divs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs0)
    }
    #[doc = "SMCLK Source Divider 1"]
    #[inline(always)]
    pub fn divs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs1)
    }
    #[doc = "SMCLK Source Divider 2"]
    #[inline(always)]
    pub fn divs_2(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs2)
    }
    #[doc = "SMCLK Source Divider 3"]
    #[inline(always)]
    pub fn divs_3(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs3)
    }
}
#[doc = "Field `SMCLKOFF` reader - SMCLK off"]
pub type SmclkoffR = crate::BitReader;
#[doc = "Field `SMCLKOFF` writer - SMCLK off"]
pub type SmclkoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLOAUTOOFF` reader - VLO automatic off enable"]
pub type VloautooffR = crate::BitReader;
#[doc = "Field `VLOAUTOOFF` writer - VLO automatic off enable"]
pub type VloautooffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - MCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divm(&self) -> DivmR {
        DivmR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - SMCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divs(&self) -> DivsR {
        DivsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - SMCLK off"]
    #[inline(always)]
    pub fn smclkoff(&self) -> SmclkoffR {
        SmclkoffR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - VLO automatic off enable"]
    #[inline(always)]
    pub fn vloautooff(&self) -> VloautooffR {
        VloautooffR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divm(&mut self) -> DivmW<'_, Csctl5Spec> {
        DivmW::new(self, 0)
    }
    #[doc = "Bits 4:5 - SMCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divs(&mut self) -> DivsW<'_, Csctl5Spec> {
        DivsW::new(self, 4)
    }
    #[doc = "Bit 8 - SMCLK off"]
    #[inline(always)]
    pub fn smclkoff(&mut self) -> SmclkoffW<'_, Csctl5Spec> {
        SmclkoffW::new(self, 8)
    }
    #[doc = "Bit 12 - VLO automatic off enable"]
    #[inline(always)]
    pub fn vloautooff(&mut self) -> VloautooffW<'_, Csctl5Spec> {
        VloautooffW::new(self, 12)
    }
}
#[doc = "CS Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl5Spec;
impl crate::RegisterSpec for Csctl5Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl5::R`](R) reader structure"]
impl crate::Readable for Csctl5Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl5::W`](W) writer structure"]
impl crate::Writable for Csctl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL5 to value 0"]
impl crate::Resettable for Csctl5Spec {}
