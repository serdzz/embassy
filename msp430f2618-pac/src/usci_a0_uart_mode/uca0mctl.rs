#[doc = "Register `UCA0MCTL` reader"]
pub type R = crate::R<Uca0mctlSpec>;
#[doc = "Register `UCA0MCTL` writer"]
pub type W = crate::W<Uca0mctlSpec>;
#[doc = "Field `UCOS16` reader - USCI 16-times Oversampling enable"]
pub type Ucos16R = crate::BitReader;
#[doc = "Field `UCOS16` writer - USCI 16-times Oversampling enable"]
pub type Ucos16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USCI Second Stage Modulation Select 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucbrs {
    #[doc = "0: USCI Second Stage Modulation: 0"]
    Ucbrs0 = 0,
    #[doc = "1: USCI Second Stage Modulation: 1"]
    Ucbrs1 = 1,
    #[doc = "2: USCI Second Stage Modulation: 2"]
    Ucbrs2 = 2,
    #[doc = "3: USCI Second Stage Modulation: 3"]
    Ucbrs3 = 3,
    #[doc = "4: USCI Second Stage Modulation: 4"]
    Ucbrs4 = 4,
    #[doc = "5: USCI Second Stage Modulation: 5"]
    Ucbrs5 = 5,
    #[doc = "6: USCI Second Stage Modulation: 6"]
    Ucbrs6 = 6,
    #[doc = "7: USCI Second Stage Modulation: 7"]
    Ucbrs7 = 7,
}
impl From<Ucbrs> for u8 {
    #[inline(always)]
    fn from(variant: Ucbrs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucbrs {
    type Ux = u8;
}
impl crate::IsEnum for Ucbrs {}
#[doc = "Field `UCBRS` reader - USCI Second Stage Modulation Select 2"]
pub type UcbrsR = crate::FieldReader<Ucbrs>;
impl UcbrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucbrs {
        match self.bits {
            0 => Ucbrs::Ucbrs0,
            1 => Ucbrs::Ucbrs1,
            2 => Ucbrs::Ucbrs2,
            3 => Ucbrs::Ucbrs3,
            4 => Ucbrs::Ucbrs4,
            5 => Ucbrs::Ucbrs5,
            6 => Ucbrs::Ucbrs6,
            7 => Ucbrs::Ucbrs7,
            _ => unreachable!(),
        }
    }
    #[doc = "USCI Second Stage Modulation: 0"]
    #[inline(always)]
    pub fn is_ucbrs_0(&self) -> bool {
        *self == Ucbrs::Ucbrs0
    }
    #[doc = "USCI Second Stage Modulation: 1"]
    #[inline(always)]
    pub fn is_ucbrs_1(&self) -> bool {
        *self == Ucbrs::Ucbrs1
    }
    #[doc = "USCI Second Stage Modulation: 2"]
    #[inline(always)]
    pub fn is_ucbrs_2(&self) -> bool {
        *self == Ucbrs::Ucbrs2
    }
    #[doc = "USCI Second Stage Modulation: 3"]
    #[inline(always)]
    pub fn is_ucbrs_3(&self) -> bool {
        *self == Ucbrs::Ucbrs3
    }
    #[doc = "USCI Second Stage Modulation: 4"]
    #[inline(always)]
    pub fn is_ucbrs_4(&self) -> bool {
        *self == Ucbrs::Ucbrs4
    }
    #[doc = "USCI Second Stage Modulation: 5"]
    #[inline(always)]
    pub fn is_ucbrs_5(&self) -> bool {
        *self == Ucbrs::Ucbrs5
    }
    #[doc = "USCI Second Stage Modulation: 6"]
    #[inline(always)]
    pub fn is_ucbrs_6(&self) -> bool {
        *self == Ucbrs::Ucbrs6
    }
    #[doc = "USCI Second Stage Modulation: 7"]
    #[inline(always)]
    pub fn is_ucbrs_7(&self) -> bool {
        *self == Ucbrs::Ucbrs7
    }
}
#[doc = "Field `UCBRS` writer - USCI Second Stage Modulation Select 2"]
pub type UcbrsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ucbrs, crate::Safe>;
impl<'a, REG> UcbrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USCI Second Stage Modulation: 0"]
    #[inline(always)]
    pub fn ucbrs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrs::Ucbrs0)
    }
    #[doc = "USCI Second Stage Modulation: 1"]
    #[inline(always)]
    pub fn ucbrs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrs::Ucbrs1)
    }
    #[doc = "USCI Second Stage Modulation: 2"]
    #[inline(always)]
    pub fn ucbrs_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrs::Ucbrs2)
    }
    #[doc = "USCI Second Stage Modulation: 3"]
    #[inline(always)]
    pub fn ucbrs_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrs::Ucbrs3)
    }
    #[doc = "USCI Second Stage Modulation: 4"]
    #[inline(always)]
    pub fn ucbrs_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrs::Ucbrs4)
    }
    #[doc = "USCI Second Stage Modulation: 5"]
    #[inline(always)]
    pub fn ucbrs_5(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrs::Ucbrs5)
    }
    #[doc = "USCI Second Stage Modulation: 6"]
    #[inline(always)]
    pub fn ucbrs_6(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrs::Ucbrs6)
    }
    #[doc = "USCI Second Stage Modulation: 7"]
    #[inline(always)]
    pub fn ucbrs_7(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrs::Ucbrs7)
    }
}
#[doc = "USCI First Stage Modulation Select 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucbrf {
    #[doc = "0: USCI First Stage Modulation: 0"]
    Ucbrf0 = 0,
    #[doc = "1: USCI First Stage Modulation: 1"]
    Ucbrf1 = 1,
    #[doc = "2: USCI First Stage Modulation: 2"]
    Ucbrf2 = 2,
    #[doc = "3: USCI First Stage Modulation: 3"]
    Ucbrf3 = 3,
    #[doc = "4: USCI First Stage Modulation: 4"]
    Ucbrf4 = 4,
    #[doc = "5: USCI First Stage Modulation: 5"]
    Ucbrf5 = 5,
    #[doc = "6: USCI First Stage Modulation: 6"]
    Ucbrf6 = 6,
    #[doc = "7: USCI First Stage Modulation: 7"]
    Ucbrf7 = 7,
    #[doc = "8: USCI First Stage Modulation: 8"]
    Ucbrf8 = 8,
    #[doc = "9: USCI First Stage Modulation: 9"]
    Ucbrf9 = 9,
    #[doc = "10: USCI First Stage Modulation: A"]
    Ucbrf10 = 10,
    #[doc = "11: USCI First Stage Modulation: B"]
    Ucbrf11 = 11,
    #[doc = "12: USCI First Stage Modulation: C"]
    Ucbrf12 = 12,
    #[doc = "13: USCI First Stage Modulation: D"]
    Ucbrf13 = 13,
    #[doc = "14: USCI First Stage Modulation: E"]
    Ucbrf14 = 14,
    #[doc = "15: USCI First Stage Modulation: F"]
    Ucbrf15 = 15,
}
impl From<Ucbrf> for u8 {
    #[inline(always)]
    fn from(variant: Ucbrf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucbrf {
    type Ux = u8;
}
impl crate::IsEnum for Ucbrf {}
#[doc = "Field `UCBRF` reader - USCI First Stage Modulation Select 3"]
pub type UcbrfR = crate::FieldReader<Ucbrf>;
impl UcbrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucbrf {
        match self.bits {
            0 => Ucbrf::Ucbrf0,
            1 => Ucbrf::Ucbrf1,
            2 => Ucbrf::Ucbrf2,
            3 => Ucbrf::Ucbrf3,
            4 => Ucbrf::Ucbrf4,
            5 => Ucbrf::Ucbrf5,
            6 => Ucbrf::Ucbrf6,
            7 => Ucbrf::Ucbrf7,
            8 => Ucbrf::Ucbrf8,
            9 => Ucbrf::Ucbrf9,
            10 => Ucbrf::Ucbrf10,
            11 => Ucbrf::Ucbrf11,
            12 => Ucbrf::Ucbrf12,
            13 => Ucbrf::Ucbrf13,
            14 => Ucbrf::Ucbrf14,
            15 => Ucbrf::Ucbrf15,
            _ => unreachable!(),
        }
    }
    #[doc = "USCI First Stage Modulation: 0"]
    #[inline(always)]
    pub fn is_ucbrf_0(&self) -> bool {
        *self == Ucbrf::Ucbrf0
    }
    #[doc = "USCI First Stage Modulation: 1"]
    #[inline(always)]
    pub fn is_ucbrf_1(&self) -> bool {
        *self == Ucbrf::Ucbrf1
    }
    #[doc = "USCI First Stage Modulation: 2"]
    #[inline(always)]
    pub fn is_ucbrf_2(&self) -> bool {
        *self == Ucbrf::Ucbrf2
    }
    #[doc = "USCI First Stage Modulation: 3"]
    #[inline(always)]
    pub fn is_ucbrf_3(&self) -> bool {
        *self == Ucbrf::Ucbrf3
    }
    #[doc = "USCI First Stage Modulation: 4"]
    #[inline(always)]
    pub fn is_ucbrf_4(&self) -> bool {
        *self == Ucbrf::Ucbrf4
    }
    #[doc = "USCI First Stage Modulation: 5"]
    #[inline(always)]
    pub fn is_ucbrf_5(&self) -> bool {
        *self == Ucbrf::Ucbrf5
    }
    #[doc = "USCI First Stage Modulation: 6"]
    #[inline(always)]
    pub fn is_ucbrf_6(&self) -> bool {
        *self == Ucbrf::Ucbrf6
    }
    #[doc = "USCI First Stage Modulation: 7"]
    #[inline(always)]
    pub fn is_ucbrf_7(&self) -> bool {
        *self == Ucbrf::Ucbrf7
    }
    #[doc = "USCI First Stage Modulation: 8"]
    #[inline(always)]
    pub fn is_ucbrf_8(&self) -> bool {
        *self == Ucbrf::Ucbrf8
    }
    #[doc = "USCI First Stage Modulation: 9"]
    #[inline(always)]
    pub fn is_ucbrf_9(&self) -> bool {
        *self == Ucbrf::Ucbrf9
    }
    #[doc = "USCI First Stage Modulation: A"]
    #[inline(always)]
    pub fn is_ucbrf_10(&self) -> bool {
        *self == Ucbrf::Ucbrf10
    }
    #[doc = "USCI First Stage Modulation: B"]
    #[inline(always)]
    pub fn is_ucbrf_11(&self) -> bool {
        *self == Ucbrf::Ucbrf11
    }
    #[doc = "USCI First Stage Modulation: C"]
    #[inline(always)]
    pub fn is_ucbrf_12(&self) -> bool {
        *self == Ucbrf::Ucbrf12
    }
    #[doc = "USCI First Stage Modulation: D"]
    #[inline(always)]
    pub fn is_ucbrf_13(&self) -> bool {
        *self == Ucbrf::Ucbrf13
    }
    #[doc = "USCI First Stage Modulation: E"]
    #[inline(always)]
    pub fn is_ucbrf_14(&self) -> bool {
        *self == Ucbrf::Ucbrf14
    }
    #[doc = "USCI First Stage Modulation: F"]
    #[inline(always)]
    pub fn is_ucbrf_15(&self) -> bool {
        *self == Ucbrf::Ucbrf15
    }
}
#[doc = "Field `UCBRF` writer - USCI First Stage Modulation Select 3"]
pub type UcbrfW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ucbrf, crate::Safe>;
impl<'a, REG> UcbrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USCI First Stage Modulation: 0"]
    #[inline(always)]
    pub fn ucbrf_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf0)
    }
    #[doc = "USCI First Stage Modulation: 1"]
    #[inline(always)]
    pub fn ucbrf_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf1)
    }
    #[doc = "USCI First Stage Modulation: 2"]
    #[inline(always)]
    pub fn ucbrf_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf2)
    }
    #[doc = "USCI First Stage Modulation: 3"]
    #[inline(always)]
    pub fn ucbrf_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf3)
    }
    #[doc = "USCI First Stage Modulation: 4"]
    #[inline(always)]
    pub fn ucbrf_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf4)
    }
    #[doc = "USCI First Stage Modulation: 5"]
    #[inline(always)]
    pub fn ucbrf_5(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf5)
    }
    #[doc = "USCI First Stage Modulation: 6"]
    #[inline(always)]
    pub fn ucbrf_6(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf6)
    }
    #[doc = "USCI First Stage Modulation: 7"]
    #[inline(always)]
    pub fn ucbrf_7(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf7)
    }
    #[doc = "USCI First Stage Modulation: 8"]
    #[inline(always)]
    pub fn ucbrf_8(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf8)
    }
    #[doc = "USCI First Stage Modulation: 9"]
    #[inline(always)]
    pub fn ucbrf_9(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf9)
    }
    #[doc = "USCI First Stage Modulation: A"]
    #[inline(always)]
    pub fn ucbrf_10(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf10)
    }
    #[doc = "USCI First Stage Modulation: B"]
    #[inline(always)]
    pub fn ucbrf_11(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf11)
    }
    #[doc = "USCI First Stage Modulation: C"]
    #[inline(always)]
    pub fn ucbrf_12(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf12)
    }
    #[doc = "USCI First Stage Modulation: D"]
    #[inline(always)]
    pub fn ucbrf_13(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf13)
    }
    #[doc = "USCI First Stage Modulation: E"]
    #[inline(always)]
    pub fn ucbrf_14(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf14)
    }
    #[doc = "USCI First Stage Modulation: F"]
    #[inline(always)]
    pub fn ucbrf_15(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrf::Ucbrf15)
    }
}
impl R {
    #[doc = "Bit 0 - USCI 16-times Oversampling enable"]
    #[inline(always)]
    pub fn ucos16(&self) -> Ucos16R {
        Ucos16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - USCI Second Stage Modulation Select 2"]
    #[inline(always)]
    pub fn ucbrs(&self) -> UcbrsR {
        UcbrsR::new((self.bits >> 1) & 7)
    }
    #[doc = "Bits 4:7 - USCI First Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrf(&self) -> UcbrfR {
        UcbrfR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bit 0 - USCI 16-times Oversampling enable"]
    #[inline(always)]
    pub fn ucos16(&mut self) -> Ucos16W<'_, Uca0mctlSpec> {
        Ucos16W::new(self, 0)
    }
    #[doc = "Bits 1:3 - USCI Second Stage Modulation Select 2"]
    #[inline(always)]
    pub fn ucbrs(&mut self) -> UcbrsW<'_, Uca0mctlSpec> {
        UcbrsW::new(self, 1)
    }
    #[doc = "Bits 4:7 - USCI First Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrf(&mut self) -> UcbrfW<'_, Uca0mctlSpec> {
        UcbrfW::new(self, 4)
    }
}
#[doc = "USCI A0 Modulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0mctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0mctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0mctlSpec;
impl crate::RegisterSpec for Uca0mctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0mctl::R`](R) reader structure"]
impl crate::Readable for Uca0mctlSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0mctl::W`](W) writer structure"]
impl crate::Writable for Uca0mctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0MCTL to value 0"]
impl crate::Resettable for Uca0mctlSpec {}
