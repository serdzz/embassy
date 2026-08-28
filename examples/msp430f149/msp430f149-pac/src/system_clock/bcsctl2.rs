#[doc = "Register `BCSCTL2` reader"]
pub type R = crate::R<Bcsctl2Spec>;
#[doc = "Register `BCSCTL2` writer"]
pub type W = crate::W<Bcsctl2Spec>;
#[doc = "Field `DCOR` reader - Enable External Resistor : 1"]
pub type DcorR = crate::BitReader;
#[doc = "Field `DCOR` writer - Enable External Resistor : 1"]
pub type DcorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SMCLK Divider 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divs {
    #[doc = "0: SMCLK Divider 0: /1"]
    Divs0 = 0,
    #[doc = "1: SMCLK Divider 1: /2"]
    Divs1 = 1,
    #[doc = "2: SMCLK Divider 2: /4"]
    Divs2 = 2,
    #[doc = "3: SMCLK Divider 3: /8"]
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
#[doc = "Field `DIVS` reader - SMCLK Divider 0"]
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
    #[doc = "SMCLK Divider 0: /1"]
    #[inline(always)]
    pub fn is_divs_0(&self) -> bool {
        *self == Divs::Divs0
    }
    #[doc = "SMCLK Divider 1: /2"]
    #[inline(always)]
    pub fn is_divs_1(&self) -> bool {
        *self == Divs::Divs1
    }
    #[doc = "SMCLK Divider 2: /4"]
    #[inline(always)]
    pub fn is_divs_2(&self) -> bool {
        *self == Divs::Divs2
    }
    #[doc = "SMCLK Divider 3: /8"]
    #[inline(always)]
    pub fn is_divs_3(&self) -> bool {
        *self == Divs::Divs3
    }
}
#[doc = "Field `DIVS` writer - SMCLK Divider 0"]
pub type DivsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Divs, crate::Safe>;
impl<'a, REG> DivsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SMCLK Divider 0: /1"]
    #[inline(always)]
    pub fn divs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs0)
    }
    #[doc = "SMCLK Divider 1: /2"]
    #[inline(always)]
    pub fn divs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs1)
    }
    #[doc = "SMCLK Divider 2: /4"]
    #[inline(always)]
    pub fn divs_2(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs2)
    }
    #[doc = "SMCLK Divider 3: /8"]
    #[inline(always)]
    pub fn divs_3(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs3)
    }
}
#[doc = "Field `SELS` reader - SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK"]
pub type SelsR = crate::BitReader;
#[doc = "Field `SELS` writer - SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK"]
pub type SelsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MCLK Divider 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divm {
    #[doc = "0: MCLK Divider 0: /1"]
    Divm0 = 0,
    #[doc = "1: MCLK Divider 1: /2"]
    Divm1 = 1,
    #[doc = "2: MCLK Divider 2: /4"]
    Divm2 = 2,
    #[doc = "3: MCLK Divider 3: /8"]
    Divm3 = 3,
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
#[doc = "Field `DIVM` reader - MCLK Divider 0"]
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
            _ => unreachable!(),
        }
    }
    #[doc = "MCLK Divider 0: /1"]
    #[inline(always)]
    pub fn is_divm_0(&self) -> bool {
        *self == Divm::Divm0
    }
    #[doc = "MCLK Divider 1: /2"]
    #[inline(always)]
    pub fn is_divm_1(&self) -> bool {
        *self == Divm::Divm1
    }
    #[doc = "MCLK Divider 2: /4"]
    #[inline(always)]
    pub fn is_divm_2(&self) -> bool {
        *self == Divm::Divm2
    }
    #[doc = "MCLK Divider 3: /8"]
    #[inline(always)]
    pub fn is_divm_3(&self) -> bool {
        *self == Divm::Divm3
    }
}
#[doc = "Field `DIVM` writer - MCLK Divider 0"]
pub type DivmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Divm, crate::Safe>;
impl<'a, REG> DivmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCLK Divider 0: /1"]
    #[inline(always)]
    pub fn divm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm0)
    }
    #[doc = "MCLK Divider 1: /2"]
    #[inline(always)]
    pub fn divm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm1)
    }
    #[doc = "MCLK Divider 2: /4"]
    #[inline(always)]
    pub fn divm_2(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm2)
    }
    #[doc = "MCLK Divider 3: /8"]
    #[inline(always)]
    pub fn divm_3(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm3)
    }
}
#[doc = "MCLK Source Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selm {
    #[doc = "0: MCLK Source Select 0: DCOCLK"]
    Selm0 = 0,
    #[doc = "1: MCLK Source Select 1: DCOCLK"]
    Selm1 = 1,
    #[doc = "2: MCLK Source Select 2: XT2CLK/LFXTCLK"]
    Selm2 = 2,
    #[doc = "3: MCLK Source Select 3: LFXTCLK"]
    Selm3 = 3,
}
impl From<Selm> for u8 {
    #[inline(always)]
    fn from(variant: Selm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selm {
    type Ux = u8;
}
impl crate::IsEnum for Selm {}
#[doc = "Field `SELM` reader - MCLK Source Select 0"]
pub type SelmR = crate::FieldReader<Selm>;
impl SelmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selm {
        match self.bits {
            0 => Selm::Selm0,
            1 => Selm::Selm1,
            2 => Selm::Selm2,
            3 => Selm::Selm3,
            _ => unreachable!(),
        }
    }
    #[doc = "MCLK Source Select 0: DCOCLK"]
    #[inline(always)]
    pub fn is_selm_0(&self) -> bool {
        *self == Selm::Selm0
    }
    #[doc = "MCLK Source Select 1: DCOCLK"]
    #[inline(always)]
    pub fn is_selm_1(&self) -> bool {
        *self == Selm::Selm1
    }
    #[doc = "MCLK Source Select 2: XT2CLK/LFXTCLK"]
    #[inline(always)]
    pub fn is_selm_2(&self) -> bool {
        *self == Selm::Selm2
    }
    #[doc = "MCLK Source Select 3: LFXTCLK"]
    #[inline(always)]
    pub fn is_selm_3(&self) -> bool {
        *self == Selm::Selm3
    }
}
#[doc = "Field `SELM` writer - MCLK Source Select 0"]
pub type SelmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Selm, crate::Safe>;
impl<'a, REG> SelmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCLK Source Select 0: DCOCLK"]
    #[inline(always)]
    pub fn selm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Selm0)
    }
    #[doc = "MCLK Source Select 1: DCOCLK"]
    #[inline(always)]
    pub fn selm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Selm1)
    }
    #[doc = "MCLK Source Select 2: XT2CLK/LFXTCLK"]
    #[inline(always)]
    pub fn selm_2(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Selm2)
    }
    #[doc = "MCLK Source Select 3: LFXTCLK"]
    #[inline(always)]
    pub fn selm_3(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Selm3)
    }
}
impl R {
    #[doc = "Bit 0 - Enable External Resistor : 1"]
    #[inline(always)]
    pub fn dcor(&self) -> DcorR {
        DcorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - SMCLK Divider 0"]
    #[inline(always)]
    pub fn divs(&self) -> DivsR {
        DivsR::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK"]
    #[inline(always)]
    pub fn sels(&self) -> SelsR {
        SelsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - MCLK Divider 0"]
    #[inline(always)]
    pub fn divm(&self) -> DivmR {
        DivmR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - MCLK Source Select 0"]
    #[inline(always)]
    pub fn selm(&self) -> SelmR {
        SelmR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Enable External Resistor : 1"]
    #[inline(always)]
    pub fn dcor(&mut self) -> DcorW<'_, Bcsctl2Spec> {
        DcorW::new(self, 0)
    }
    #[doc = "Bits 1:2 - SMCLK Divider 0"]
    #[inline(always)]
    pub fn divs(&mut self) -> DivsW<'_, Bcsctl2Spec> {
        DivsW::new(self, 1)
    }
    #[doc = "Bit 3 - SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK"]
    #[inline(always)]
    pub fn sels(&mut self) -> SelsW<'_, Bcsctl2Spec> {
        SelsW::new(self, 3)
    }
    #[doc = "Bits 4:5 - MCLK Divider 0"]
    #[inline(always)]
    pub fn divm(&mut self) -> DivmW<'_, Bcsctl2Spec> {
        DivmW::new(self, 4)
    }
    #[doc = "Bits 6:7 - MCLK Source Select 0"]
    #[inline(always)]
    pub fn selm(&mut self) -> SelmW<'_, Bcsctl2Spec> {
        SelmW::new(self, 6)
    }
}
#[doc = "Basic Clock System Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bcsctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcsctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcsctl2Spec;
impl crate::RegisterSpec for Bcsctl2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcsctl2::R`](R) reader structure"]
impl crate::Readable for Bcsctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`bcsctl2::W`](W) writer structure"]
impl crate::Writable for Bcsctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCSCTL2 to value 0"]
impl crate::Resettable for Bcsctl2Spec {}
