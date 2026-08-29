#[doc = "Register `CSCTL2` reader"]
pub type R = crate::R<Csctl2Spec>;
#[doc = "Register `CSCTL2` writer"]
pub type W = crate::W<Csctl2Spec>;
#[doc = "Selects the MCLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selm {
    #[doc = "0: LFXTCLK when LFXT available, otherwise VLOCLK"]
    Lfxtclk = 0,
    #[doc = "1: VLOCLK"]
    Vloclk = 1,
    #[doc = "2: LFMODCLK"]
    Lfmodclk = 2,
    #[doc = "3: DCOCLK"]
    Dcoclk = 3,
    #[doc = "4: MODCLK"]
    Modclk = 4,
    #[doc = "5: HFXTCLK when HFXT available, otherwise DCOCLK"]
    Hfxtclk = 5,
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
#[doc = "Field `SELM` reader - Selects the MCLK source"]
pub type SelmR = crate::FieldReader<Selm>;
impl SelmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Selm> {
        match self.bits {
            0 => Some(Selm::Lfxtclk),
            1 => Some(Selm::Vloclk),
            2 => Some(Selm::Lfmodclk),
            3 => Some(Selm::Dcoclk),
            4 => Some(Selm::Modclk),
            5 => Some(Selm::Hfxtclk),
            _ => None,
        }
    }
    #[doc = "LFXTCLK when LFXT available, otherwise VLOCLK"]
    #[inline(always)]
    pub fn is_lfxtclk(&self) -> bool {
        *self == Selm::Lfxtclk
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == Selm::Vloclk
    }
    #[doc = "LFMODCLK"]
    #[inline(always)]
    pub fn is_lfmodclk(&self) -> bool {
        *self == Selm::Lfmodclk
    }
    #[doc = "DCOCLK"]
    #[inline(always)]
    pub fn is_dcoclk(&self) -> bool {
        *self == Selm::Dcoclk
    }
    #[doc = "MODCLK"]
    #[inline(always)]
    pub fn is_modclk(&self) -> bool {
        *self == Selm::Modclk
    }
    #[doc = "HFXTCLK when HFXT available, otherwise DCOCLK"]
    #[inline(always)]
    pub fn is_hfxtclk(&self) -> bool {
        *self == Selm::Hfxtclk
    }
}
#[doc = "Field `SELM` writer - Selects the MCLK source"]
pub type SelmW<'a, REG> = crate::FieldWriter<'a, REG, 3, Selm>;
impl<'a, REG> SelmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFXTCLK when LFXT available, otherwise VLOCLK"]
    #[inline(always)]
    pub fn lfxtclk(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Lfxtclk)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Vloclk)
    }
    #[doc = "LFMODCLK"]
    #[inline(always)]
    pub fn lfmodclk(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Lfmodclk)
    }
    #[doc = "DCOCLK"]
    #[inline(always)]
    pub fn dcoclk(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Dcoclk)
    }
    #[doc = "MODCLK"]
    #[inline(always)]
    pub fn modclk(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Modclk)
    }
    #[doc = "HFXTCLK when HFXT available, otherwise DCOCLK"]
    #[inline(always)]
    pub fn hfxtclk(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Hfxtclk)
    }
}
#[doc = "Selects the SMCLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sels {
    #[doc = "0: LFXTCLK when LFXT available, otherwise VLOCLK."]
    Lfxtclk = 0,
    #[doc = "1: VLOCLK"]
    Vloclk = 1,
    #[doc = "2: LFMODCLK"]
    Lfmodclk = 2,
    #[doc = "3: DCOCLK"]
    Dcoclk = 3,
    #[doc = "4: MODCLK"]
    Modclk = 4,
    #[doc = "5: HFXTCLK when HFXT available, otherwise DCOCLK."]
    Hfxtclk = 5,
}
impl From<Sels> for u8 {
    #[inline(always)]
    fn from(variant: Sels) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sels {
    type Ux = u8;
}
impl crate::IsEnum for Sels {}
#[doc = "Field `SELS` reader - Selects the SMCLK source"]
pub type SelsR = crate::FieldReader<Sels>;
impl SelsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sels> {
        match self.bits {
            0 => Some(Sels::Lfxtclk),
            1 => Some(Sels::Vloclk),
            2 => Some(Sels::Lfmodclk),
            3 => Some(Sels::Dcoclk),
            4 => Some(Sels::Modclk),
            5 => Some(Sels::Hfxtclk),
            _ => None,
        }
    }
    #[doc = "LFXTCLK when LFXT available, otherwise VLOCLK."]
    #[inline(always)]
    pub fn is_lfxtclk(&self) -> bool {
        *self == Sels::Lfxtclk
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == Sels::Vloclk
    }
    #[doc = "LFMODCLK"]
    #[inline(always)]
    pub fn is_lfmodclk(&self) -> bool {
        *self == Sels::Lfmodclk
    }
    #[doc = "DCOCLK"]
    #[inline(always)]
    pub fn is_dcoclk(&self) -> bool {
        *self == Sels::Dcoclk
    }
    #[doc = "MODCLK"]
    #[inline(always)]
    pub fn is_modclk(&self) -> bool {
        *self == Sels::Modclk
    }
    #[doc = "HFXTCLK when HFXT available, otherwise DCOCLK."]
    #[inline(always)]
    pub fn is_hfxtclk(&self) -> bool {
        *self == Sels::Hfxtclk
    }
}
#[doc = "Field `SELS` writer - Selects the SMCLK source"]
pub type SelsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sels>;
impl<'a, REG> SelsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFXTCLK when LFXT available, otherwise VLOCLK."]
    #[inline(always)]
    pub fn lfxtclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::Lfxtclk)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::Vloclk)
    }
    #[doc = "LFMODCLK"]
    #[inline(always)]
    pub fn lfmodclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::Lfmodclk)
    }
    #[doc = "DCOCLK"]
    #[inline(always)]
    pub fn dcoclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::Dcoclk)
    }
    #[doc = "MODCLK"]
    #[inline(always)]
    pub fn modclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::Modclk)
    }
    #[doc = "HFXTCLK when HFXT available, otherwise DCOCLK."]
    #[inline(always)]
    pub fn hfxtclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::Hfxtclk)
    }
}
#[doc = "Selects the ACLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sela {
    #[doc = "0: LFXTCLK when LFXT available, otherwise VLOCLK."]
    Lfxtclk = 0,
    #[doc = "1: VLOCLK"]
    Vloclk = 1,
    #[doc = "2: LFMODCLK"]
    Lfmodclk = 2,
}
impl From<Sela> for u8 {
    #[inline(always)]
    fn from(variant: Sela) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sela {
    type Ux = u8;
}
impl crate::IsEnum for Sela {}
#[doc = "Field `SELA` reader - Selects the ACLK source"]
pub type SelaR = crate::FieldReader<Sela>;
impl SelaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sela> {
        match self.bits {
            0 => Some(Sela::Lfxtclk),
            1 => Some(Sela::Vloclk),
            2 => Some(Sela::Lfmodclk),
            _ => None,
        }
    }
    #[doc = "LFXTCLK when LFXT available, otherwise VLOCLK."]
    #[inline(always)]
    pub fn is_lfxtclk(&self) -> bool {
        *self == Sela::Lfxtclk
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == Sela::Vloclk
    }
    #[doc = "LFMODCLK"]
    #[inline(always)]
    pub fn is_lfmodclk(&self) -> bool {
        *self == Sela::Lfmodclk
    }
}
#[doc = "Field `SELA` writer - Selects the ACLK source"]
pub type SelaW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sela>;
impl<'a, REG> SelaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFXTCLK when LFXT available, otherwise VLOCLK."]
    #[inline(always)]
    pub fn lfxtclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Lfxtclk)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Vloclk)
    }
    #[doc = "LFMODCLK"]
    #[inline(always)]
    pub fn lfmodclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Lfmodclk)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects the MCLK source"]
    #[inline(always)]
    pub fn selm(&self) -> SelmR {
        SelmR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Selects the SMCLK source"]
    #[inline(always)]
    pub fn sels(&self) -> SelsR {
        SelsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Selects the ACLK source"]
    #[inline(always)]
    pub fn sela(&self) -> SelaR {
        SelaR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects the MCLK source"]
    #[inline(always)]
    pub fn selm(&mut self) -> SelmW<'_, Csctl2Spec> {
        SelmW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Selects the SMCLK source"]
    #[inline(always)]
    pub fn sels(&mut self) -> SelsW<'_, Csctl2Spec> {
        SelsW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Selects the ACLK source"]
    #[inline(always)]
    pub fn sela(&mut self) -> SelaW<'_, Csctl2Spec> {
        SelaW::new(self, 8)
    }
}
#[doc = "Clock System Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
