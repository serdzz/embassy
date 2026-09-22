#[doc = "Register `CSCTL4` reader"]
pub type R = crate::R<Csctl4Spec>;
#[doc = "Register `CSCTL4` writer"]
pub type W = crate::W<Csctl4Spec>;
#[doc = "MCLK and SMCLK Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selms {
    #[doc = "0: MCLK and SMCLK Source Select 0"]
    Selms0 = 0,
    #[doc = "1: MCLK and SMCLK Source Select 1"]
    Selms1 = 1,
    #[doc = "2: MCLK and SMCLK Source Select 2"]
    Selms2 = 2,
    #[doc = "3: MCLK and SMCLK Source Select 3"]
    Selms3 = 3,
    #[doc = "4: MCLK and SMCLK Source Select 4"]
    Selms4 = 4,
    #[doc = "5: MCLK and SMCLK Source Select 5"]
    Selms5 = 5,
    #[doc = "6: MCLK and SMCLK Source Select 6"]
    Selms6 = 6,
    #[doc = "7: MCLK and SMCLK Source Select 7"]
    Selms7 = 7,
}
impl From<Selms> for u8 {
    #[inline(always)]
    fn from(variant: Selms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selms {
    type Ux = u8;
}
impl crate::IsEnum for Selms {}
#[doc = "Field `SELMS` reader - MCLK and SMCLK Source Select Bit: 0"]
pub type SelmsR = crate::FieldReader<Selms>;
impl SelmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selms {
        match self.bits {
            0 => Selms::Selms0,
            1 => Selms::Selms1,
            2 => Selms::Selms2,
            3 => Selms::Selms3,
            4 => Selms::Selms4,
            5 => Selms::Selms5,
            6 => Selms::Selms6,
            7 => Selms::Selms7,
            _ => unreachable!(),
        }
    }
    #[doc = "MCLK and SMCLK Source Select 0"]
    #[inline(always)]
    pub fn is_selms_0(&self) -> bool {
        *self == Selms::Selms0
    }
    #[doc = "MCLK and SMCLK Source Select 1"]
    #[inline(always)]
    pub fn is_selms_1(&self) -> bool {
        *self == Selms::Selms1
    }
    #[doc = "MCLK and SMCLK Source Select 2"]
    #[inline(always)]
    pub fn is_selms_2(&self) -> bool {
        *self == Selms::Selms2
    }
    #[doc = "MCLK and SMCLK Source Select 3"]
    #[inline(always)]
    pub fn is_selms_3(&self) -> bool {
        *self == Selms::Selms3
    }
    #[doc = "MCLK and SMCLK Source Select 4"]
    #[inline(always)]
    pub fn is_selms_4(&self) -> bool {
        *self == Selms::Selms4
    }
    #[doc = "MCLK and SMCLK Source Select 5"]
    #[inline(always)]
    pub fn is_selms_5(&self) -> bool {
        *self == Selms::Selms5
    }
    #[doc = "MCLK and SMCLK Source Select 6"]
    #[inline(always)]
    pub fn is_selms_6(&self) -> bool {
        *self == Selms::Selms6
    }
    #[doc = "MCLK and SMCLK Source Select 7"]
    #[inline(always)]
    pub fn is_selms_7(&self) -> bool {
        *self == Selms::Selms7
    }
}
#[doc = "Field `SELMS` writer - MCLK and SMCLK Source Select Bit: 0"]
pub type SelmsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Selms, crate::Safe>;
impl<'a, REG> SelmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCLK and SMCLK Source Select 0"]
    #[inline(always)]
    pub fn selms_0(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Selms0)
    }
    #[doc = "MCLK and SMCLK Source Select 1"]
    #[inline(always)]
    pub fn selms_1(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Selms1)
    }
    #[doc = "MCLK and SMCLK Source Select 2"]
    #[inline(always)]
    pub fn selms_2(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Selms2)
    }
    #[doc = "MCLK and SMCLK Source Select 3"]
    #[inline(always)]
    pub fn selms_3(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Selms3)
    }
    #[doc = "MCLK and SMCLK Source Select 4"]
    #[inline(always)]
    pub fn selms_4(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Selms4)
    }
    #[doc = "MCLK and SMCLK Source Select 5"]
    #[inline(always)]
    pub fn selms_5(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Selms5)
    }
    #[doc = "MCLK and SMCLK Source Select 6"]
    #[inline(always)]
    pub fn selms_6(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Selms6)
    }
    #[doc = "MCLK and SMCLK Source Select 7"]
    #[inline(always)]
    pub fn selms_7(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Selms7)
    }
}
#[doc = "Field `SELA` reader - ACLK Source Select Bit: 0"]
pub type SelaR = crate::BitReader;
#[doc = "Field `SELA` writer - ACLK Source Select Bit: 0"]
pub type SelaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - MCLK and SMCLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn selms(&self) -> SelmsR {
        SelmsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - ACLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn sela(&self) -> SelaR {
        SelaR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK and SMCLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn selms(&mut self) -> SelmsW<'_, Csctl4Spec> {
        SelmsW::new(self, 0)
    }
    #[doc = "Bit 8 - ACLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn sela(&mut self) -> SelaW<'_, Csctl4Spec> {
        SelaW::new(self, 8)
    }
}
#[doc = "CS Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl4Spec;
impl crate::RegisterSpec for Csctl4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl4::R`](R) reader structure"]
impl crate::Readable for Csctl4Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl4::W`](W) writer structure"]
impl crate::Writable for Csctl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL4 to value 0"]
impl crate::Resettable for Csctl4Spec {}
