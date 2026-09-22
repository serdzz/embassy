#[doc = "Register `UCA0MCTLW` reader"]
pub type R = crate::R<Uca0mctlwSpec>;
#[doc = "Register `UCA0MCTLW` writer"]
pub type W = crate::W<Uca0mctlwSpec>;
#[doc = "Field `UCOS16` reader - USCI 16-times Oversampling enable"]
pub type Ucos16R = crate::BitReader;
#[doc = "Field `UCOS16` writer - USCI 16-times Oversampling enable"]
pub type Ucos16W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `UCBRS0` reader - USCI Second Stage Modulation Select 0"]
pub type Ucbrs0R = crate::BitReader;
#[doc = "Field `UCBRS0` writer - USCI Second Stage Modulation Select 0"]
pub type Ucbrs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBRS1` reader - USCI Second Stage Modulation Select 1"]
pub type Ucbrs1R = crate::BitReader;
#[doc = "Field `UCBRS1` writer - USCI Second Stage Modulation Select 1"]
pub type Ucbrs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBRS2` reader - USCI Second Stage Modulation Select 2"]
pub type Ucbrs2R = crate::BitReader;
#[doc = "Field `UCBRS2` writer - USCI Second Stage Modulation Select 2"]
pub type Ucbrs2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBRS3` reader - USCI Second Stage Modulation Select 3"]
pub type Ucbrs3R = crate::BitReader;
#[doc = "Field `UCBRS3` writer - USCI Second Stage Modulation Select 3"]
pub type Ucbrs3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBRS4` reader - USCI Second Stage Modulation Select 4"]
pub type Ucbrs4R = crate::BitReader;
#[doc = "Field `UCBRS4` writer - USCI Second Stage Modulation Select 4"]
pub type Ucbrs4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBRS5` reader - USCI Second Stage Modulation Select 5"]
pub type Ucbrs5R = crate::BitReader;
#[doc = "Field `UCBRS5` writer - USCI Second Stage Modulation Select 5"]
pub type Ucbrs5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBRS6` reader - USCI Second Stage Modulation Select 6"]
pub type Ucbrs6R = crate::BitReader;
#[doc = "Field `UCBRS6` writer - USCI Second Stage Modulation Select 6"]
pub type Ucbrs6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBRS7` reader - USCI Second Stage Modulation Select 7"]
pub type Ucbrs7R = crate::BitReader;
#[doc = "Field `UCBRS7` writer - USCI Second Stage Modulation Select 7"]
pub type Ucbrs7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USCI 16-times Oversampling enable"]
    #[inline(always)]
    pub fn ucos16(&self) -> Ucos16R {
        Ucos16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - USCI First Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrf(&self) -> UcbrfR {
        UcbrfR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - USCI Second Stage Modulation Select 0"]
    #[inline(always)]
    pub fn ucbrs0(&self) -> Ucbrs0R {
        Ucbrs0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USCI Second Stage Modulation Select 1"]
    #[inline(always)]
    pub fn ucbrs1(&self) -> Ucbrs1R {
        Ucbrs1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USCI Second Stage Modulation Select 2"]
    #[inline(always)]
    pub fn ucbrs2(&self) -> Ucbrs2R {
        Ucbrs2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USCI Second Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrs3(&self) -> Ucbrs3R {
        Ucbrs3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USCI Second Stage Modulation Select 4"]
    #[inline(always)]
    pub fn ucbrs4(&self) -> Ucbrs4R {
        Ucbrs4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USCI Second Stage Modulation Select 5"]
    #[inline(always)]
    pub fn ucbrs5(&self) -> Ucbrs5R {
        Ucbrs5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USCI Second Stage Modulation Select 6"]
    #[inline(always)]
    pub fn ucbrs6(&self) -> Ucbrs6R {
        Ucbrs6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - USCI Second Stage Modulation Select 7"]
    #[inline(always)]
    pub fn ucbrs7(&self) -> Ucbrs7R {
        Ucbrs7R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI 16-times Oversampling enable"]
    #[inline(always)]
    pub fn ucos16(&mut self) -> Ucos16W<'_, Uca0mctlwSpec> {
        Ucos16W::new(self, 0)
    }
    #[doc = "Bits 4:7 - USCI First Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrf(&mut self) -> UcbrfW<'_, Uca0mctlwSpec> {
        UcbrfW::new(self, 4)
    }
    #[doc = "Bit 8 - USCI Second Stage Modulation Select 0"]
    #[inline(always)]
    pub fn ucbrs0(&mut self) -> Ucbrs0W<'_, Uca0mctlwSpec> {
        Ucbrs0W::new(self, 8)
    }
    #[doc = "Bit 9 - USCI Second Stage Modulation Select 1"]
    #[inline(always)]
    pub fn ucbrs1(&mut self) -> Ucbrs1W<'_, Uca0mctlwSpec> {
        Ucbrs1W::new(self, 9)
    }
    #[doc = "Bit 10 - USCI Second Stage Modulation Select 2"]
    #[inline(always)]
    pub fn ucbrs2(&mut self) -> Ucbrs2W<'_, Uca0mctlwSpec> {
        Ucbrs2W::new(self, 10)
    }
    #[doc = "Bit 11 - USCI Second Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrs3(&mut self) -> Ucbrs3W<'_, Uca0mctlwSpec> {
        Ucbrs3W::new(self, 11)
    }
    #[doc = "Bit 12 - USCI Second Stage Modulation Select 4"]
    #[inline(always)]
    pub fn ucbrs4(&mut self) -> Ucbrs4W<'_, Uca0mctlwSpec> {
        Ucbrs4W::new(self, 12)
    }
    #[doc = "Bit 13 - USCI Second Stage Modulation Select 5"]
    #[inline(always)]
    pub fn ucbrs5(&mut self) -> Ucbrs5W<'_, Uca0mctlwSpec> {
        Ucbrs5W::new(self, 13)
    }
    #[doc = "Bit 14 - USCI Second Stage Modulation Select 6"]
    #[inline(always)]
    pub fn ucbrs6(&mut self) -> Ucbrs6W<'_, Uca0mctlwSpec> {
        Ucbrs6W::new(self, 14)
    }
    #[doc = "Bit 15 - USCI Second Stage Modulation Select 7"]
    #[inline(always)]
    pub fn ucbrs7(&mut self) -> Ucbrs7W<'_, Uca0mctlwSpec> {
        Ucbrs7W::new(self, 15)
    }
}
#[doc = "USCI A0 Modulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0mctlw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0mctlw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0mctlwSpec;
impl crate::RegisterSpec for Uca0mctlwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca0mctlw::R`](R) reader structure"]
impl crate::Readable for Uca0mctlwSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0mctlw::W`](W) writer structure"]
impl crate::Writable for Uca0mctlwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0MCTLW to value 0"]
impl crate::Resettable for Uca0mctlwSpec {}
