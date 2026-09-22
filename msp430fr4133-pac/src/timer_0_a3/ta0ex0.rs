#[doc = "Register `TA0EX0` reader"]
pub type R = crate::R<Ta0ex0Spec>;
#[doc = "Register `TA0EX0` writer"]
pub type W = crate::W<Ta0ex0Spec>;
#[doc = "Timer A Input divider expansion Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Taidex {
    #[doc = "0: Timer A Input divider expansion : /1"]
    Taidex0 = 0,
    #[doc = "1: Timer A Input divider expansion : /2"]
    Taidex1 = 1,
    #[doc = "2: Timer A Input divider expansion : /3"]
    Taidex2 = 2,
    #[doc = "3: Timer A Input divider expansion : /4"]
    Taidex3 = 3,
    #[doc = "4: Timer A Input divider expansion : /5"]
    Taidex4 = 4,
    #[doc = "5: Timer A Input divider expansion : /6"]
    Taidex5 = 5,
    #[doc = "6: Timer A Input divider expansion : /7"]
    Taidex6 = 6,
    #[doc = "7: Timer A Input divider expansion : /8"]
    Taidex7 = 7,
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
#[doc = "Field `TAIDEX` reader - Timer A Input divider expansion Bit: 0"]
pub type TaidexR = crate::FieldReader<Taidex>;
impl TaidexR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Taidex {
        match self.bits {
            0 => Taidex::Taidex0,
            1 => Taidex::Taidex1,
            2 => Taidex::Taidex2,
            3 => Taidex::Taidex3,
            4 => Taidex::Taidex4,
            5 => Taidex::Taidex5,
            6 => Taidex::Taidex6,
            7 => Taidex::Taidex7,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer A Input divider expansion : /1"]
    #[inline(always)]
    pub fn is_taidex_0(&self) -> bool {
        *self == Taidex::Taidex0
    }
    #[doc = "Timer A Input divider expansion : /2"]
    #[inline(always)]
    pub fn is_taidex_1(&self) -> bool {
        *self == Taidex::Taidex1
    }
    #[doc = "Timer A Input divider expansion : /3"]
    #[inline(always)]
    pub fn is_taidex_2(&self) -> bool {
        *self == Taidex::Taidex2
    }
    #[doc = "Timer A Input divider expansion : /4"]
    #[inline(always)]
    pub fn is_taidex_3(&self) -> bool {
        *self == Taidex::Taidex3
    }
    #[doc = "Timer A Input divider expansion : /5"]
    #[inline(always)]
    pub fn is_taidex_4(&self) -> bool {
        *self == Taidex::Taidex4
    }
    #[doc = "Timer A Input divider expansion : /6"]
    #[inline(always)]
    pub fn is_taidex_5(&self) -> bool {
        *self == Taidex::Taidex5
    }
    #[doc = "Timer A Input divider expansion : /7"]
    #[inline(always)]
    pub fn is_taidex_6(&self) -> bool {
        *self == Taidex::Taidex6
    }
    #[doc = "Timer A Input divider expansion : /8"]
    #[inline(always)]
    pub fn is_taidex_7(&self) -> bool {
        *self == Taidex::Taidex7
    }
}
#[doc = "Field `TAIDEX` writer - Timer A Input divider expansion Bit: 0"]
pub type TaidexW<'a, REG> = crate::FieldWriter<'a, REG, 3, Taidex, crate::Safe>;
impl<'a, REG> TaidexW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer A Input divider expansion : /1"]
    #[inline(always)]
    pub fn taidex_0(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex0)
    }
    #[doc = "Timer A Input divider expansion : /2"]
    #[inline(always)]
    pub fn taidex_1(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex1)
    }
    #[doc = "Timer A Input divider expansion : /3"]
    #[inline(always)]
    pub fn taidex_2(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex2)
    }
    #[doc = "Timer A Input divider expansion : /4"]
    #[inline(always)]
    pub fn taidex_3(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex3)
    }
    #[doc = "Timer A Input divider expansion : /5"]
    #[inline(always)]
    pub fn taidex_4(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex4)
    }
    #[doc = "Timer A Input divider expansion : /6"]
    #[inline(always)]
    pub fn taidex_5(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex5)
    }
    #[doc = "Timer A Input divider expansion : /7"]
    #[inline(always)]
    pub fn taidex_6(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex6)
    }
    #[doc = "Timer A Input divider expansion : /8"]
    #[inline(always)]
    pub fn taidex_7(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Timer A Input divider expansion Bit: 0"]
    #[inline(always)]
    pub fn taidex(&self) -> TaidexR {
        TaidexR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timer A Input divider expansion Bit: 0"]
    #[inline(always)]
    pub fn taidex(&mut self) -> TaidexW<'_, Ta0ex0Spec> {
        TaidexW::new(self, 0)
    }
}
#[doc = "Timer0_A3 Expansion Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ta0ex0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta0ex0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta0ex0Spec;
impl crate::RegisterSpec for Ta0ex0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta0ex0::R`](R) reader structure"]
impl crate::Readable for Ta0ex0Spec {}
#[doc = "`write(|w| ..)` method takes [`ta0ex0::W`](W) writer structure"]
impl crate::Writable for Ta0ex0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA0EX0 to value 0"]
impl crate::Resettable for Ta0ex0Spec {}
