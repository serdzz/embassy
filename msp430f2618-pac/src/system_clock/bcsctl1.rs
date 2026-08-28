#[doc = "Register `BCSCTL1` reader"]
pub type R = crate::R<Bcsctl1Spec>;
#[doc = "Register `BCSCTL1` writer"]
pub type W = crate::W<Bcsctl1Spec>;
#[doc = "Field `RSEL0` reader - Range Select Bit 0"]
pub type Rsel0R = crate::BitReader;
#[doc = "Field `RSEL0` writer - Range Select Bit 0"]
pub type Rsel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSEL1` reader - Range Select Bit 1"]
pub type Rsel1R = crate::BitReader;
#[doc = "Field `RSEL1` writer - Range Select Bit 1"]
pub type Rsel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSEL2` reader - Range Select Bit 2"]
pub type Rsel2R = crate::BitReader;
#[doc = "Field `RSEL2` writer - Range Select Bit 2"]
pub type Rsel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSEL3` reader - Range Select Bit 3"]
pub type Rsel3R = crate::BitReader;
#[doc = "Field `RSEL3` writer - Range Select Bit 3"]
pub type Rsel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ACLK Divider 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Diva {
    #[doc = "0: ACLK Divider 0: /1"]
    Diva0 = 0,
    #[doc = "1: ACLK Divider 1: /2"]
    Diva1 = 1,
    #[doc = "2: ACLK Divider 2: /4"]
    Diva2 = 2,
    #[doc = "3: ACLK Divider 3: /8"]
    Diva3 = 3,
}
impl From<Diva> for u8 {
    #[inline(always)]
    fn from(variant: Diva) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Diva {
    type Ux = u8;
}
impl crate::IsEnum for Diva {}
#[doc = "Field `DIVA` reader - ACLK Divider 0"]
pub type DivaR = crate::FieldReader<Diva>;
impl DivaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diva {
        match self.bits {
            0 => Diva::Diva0,
            1 => Diva::Diva1,
            2 => Diva::Diva2,
            3 => Diva::Diva3,
            _ => unreachable!(),
        }
    }
    #[doc = "ACLK Divider 0: /1"]
    #[inline(always)]
    pub fn is_diva_0(&self) -> bool {
        *self == Diva::Diva0
    }
    #[doc = "ACLK Divider 1: /2"]
    #[inline(always)]
    pub fn is_diva_1(&self) -> bool {
        *self == Diva::Diva1
    }
    #[doc = "ACLK Divider 2: /4"]
    #[inline(always)]
    pub fn is_diva_2(&self) -> bool {
        *self == Diva::Diva2
    }
    #[doc = "ACLK Divider 3: /8"]
    #[inline(always)]
    pub fn is_diva_3(&self) -> bool {
        *self == Diva::Diva3
    }
}
#[doc = "Field `DIVA` writer - ACLK Divider 0"]
pub type DivaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Diva, crate::Safe>;
impl<'a, REG> DivaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ACLK Divider 0: /1"]
    #[inline(always)]
    pub fn diva_0(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva0)
    }
    #[doc = "ACLK Divider 1: /2"]
    #[inline(always)]
    pub fn diva_1(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva1)
    }
    #[doc = "ACLK Divider 2: /4"]
    #[inline(always)]
    pub fn diva_2(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva2)
    }
    #[doc = "ACLK Divider 3: /8"]
    #[inline(always)]
    pub fn diva_3(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva3)
    }
}
#[doc = "Field `XTS` reader - LFXTCLK 0:Low Freq. / 1: High Freq."]
pub type XtsR = crate::BitReader;
#[doc = "Field `XTS` writer - LFXTCLK 0:Low Freq. / 1: High Freq."]
pub type XtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT2OFF` reader - Enable XT2CLK"]
pub type Xt2offR = crate::BitReader;
#[doc = "Field `XT2OFF` writer - Enable XT2CLK"]
pub type Xt2offW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Range Select Bit 0"]
    #[inline(always)]
    pub fn rsel0(&self) -> Rsel0R {
        Rsel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Range Select Bit 1"]
    #[inline(always)]
    pub fn rsel1(&self) -> Rsel1R {
        Rsel1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Range Select Bit 2"]
    #[inline(always)]
    pub fn rsel2(&self) -> Rsel2R {
        Rsel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Range Select Bit 3"]
    #[inline(always)]
    pub fn rsel3(&self) -> Rsel3R {
        Rsel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - ACLK Divider 0"]
    #[inline(always)]
    pub fn diva(&self) -> DivaR {
        DivaR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - LFXTCLK 0:Low Freq. / 1: High Freq."]
    #[inline(always)]
    pub fn xts(&self) -> XtsR {
        XtsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable XT2CLK"]
    #[inline(always)]
    pub fn xt2off(&self) -> Xt2offR {
        Xt2offR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Range Select Bit 0"]
    #[inline(always)]
    pub fn rsel0(&mut self) -> Rsel0W<'_, Bcsctl1Spec> {
        Rsel0W::new(self, 0)
    }
    #[doc = "Bit 1 - Range Select Bit 1"]
    #[inline(always)]
    pub fn rsel1(&mut self) -> Rsel1W<'_, Bcsctl1Spec> {
        Rsel1W::new(self, 1)
    }
    #[doc = "Bit 2 - Range Select Bit 2"]
    #[inline(always)]
    pub fn rsel2(&mut self) -> Rsel2W<'_, Bcsctl1Spec> {
        Rsel2W::new(self, 2)
    }
    #[doc = "Bit 3 - Range Select Bit 3"]
    #[inline(always)]
    pub fn rsel3(&mut self) -> Rsel3W<'_, Bcsctl1Spec> {
        Rsel3W::new(self, 3)
    }
    #[doc = "Bits 4:5 - ACLK Divider 0"]
    #[inline(always)]
    pub fn diva(&mut self) -> DivaW<'_, Bcsctl1Spec> {
        DivaW::new(self, 4)
    }
    #[doc = "Bit 6 - LFXTCLK 0:Low Freq. / 1: High Freq."]
    #[inline(always)]
    pub fn xts(&mut self) -> XtsW<'_, Bcsctl1Spec> {
        XtsW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable XT2CLK"]
    #[inline(always)]
    pub fn xt2off(&mut self) -> Xt2offW<'_, Bcsctl1Spec> {
        Xt2offW::new(self, 7)
    }
}
#[doc = "Basic Clock System Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bcsctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcsctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcsctl1Spec;
impl crate::RegisterSpec for Bcsctl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcsctl1::R`](R) reader structure"]
impl crate::Readable for Bcsctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`bcsctl1::W`](W) writer structure"]
impl crate::Writable for Bcsctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCSCTL1 to value 0"]
impl crate::Resettable for Bcsctl1Spec {}
