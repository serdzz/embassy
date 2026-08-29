#[doc = "Register `TA3EX0` reader"]
pub type R = crate::R<Ta3ex0Spec>;
#[doc = "Register `TA3EX0` writer"]
pub type W = crate::W<Ta3ex0Spec>;
#[doc = "Input divider expansion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Taidex {
    #[doc = "0: Divide by 1"]
    _1 = 0,
    #[doc = "1: Divide by 2"]
    _2 = 1,
    #[doc = "2: Divide by 3"]
    _3 = 2,
    #[doc = "3: Divide by 4"]
    _4 = 3,
    #[doc = "4: Divide by 5"]
    _5 = 4,
    #[doc = "5: Divide by 6"]
    _6 = 5,
    #[doc = "6: Divide by 7"]
    _7 = 6,
    #[doc = "7: Divide by 8"]
    _8 = 7,
}
impl From<Taidex> for u8 {
    #[inline(always)]
    fn from(variant: Taidex) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Taidex {
    type Ux = u8;
}
impl crate::IsEnum for Taidex {}
#[doc = "Field `TAIDEX` reader - Input divider expansion"]
pub type TaidexR = crate::FieldReader<Taidex>;
impl TaidexR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Taidex {
        match self.bits {
            0 => Taidex::_1,
            1 => Taidex::_2,
            2 => Taidex::_3,
            3 => Taidex::_4,
            4 => Taidex::_5,
            5 => Taidex::_6,
            6 => Taidex::_7,
            7 => Taidex::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Taidex::_1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Taidex::_2
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == Taidex::_3
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Taidex::_4
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == Taidex::_5
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == Taidex::_6
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == Taidex::_7
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Taidex::_8
    }
}
#[doc = "Field `TAIDEX` writer - Input divider expansion"]
pub type TaidexW<'a, REG> = crate::FieldWriter<'a, REG, 3, Taidex, crate::Safe>;
impl<'a, REG> TaidexW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::_8)
    }
}
impl R {
    #[doc = "Bits 0:2 - Input divider expansion"]
    #[inline(always)]
    pub fn taidex(&self) -> TaidexR {
        TaidexR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input divider expansion"]
    #[inline(always)]
    pub fn taidex(&mut self) -> TaidexW<'_, Ta3ex0Spec> {
        TaidexW::new(self, 0)
    }
}
#[doc = "TimerAx Expansion 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta3ex0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta3ex0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta3ex0Spec;
impl crate::RegisterSpec for Ta3ex0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta3ex0::R`](R) reader structure"]
impl crate::Readable for Ta3ex0Spec {}
#[doc = "`write(|w| ..)` method takes [`ta3ex0::W`](W) writer structure"]
impl crate::Writable for Ta3ex0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA3EX0 to value 0"]
impl crate::Resettable for Ta3ex0Spec {}
