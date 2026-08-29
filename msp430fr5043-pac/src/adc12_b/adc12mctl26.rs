#[doc = "Register `ADC12MCTL26` reader"]
pub type R = crate::R<Adc12mctl26Spec>;
#[doc = "Register `ADC12MCTL26` writer"]
pub type W = crate::W<Adc12mctl26Spec>;
#[doc = "Input channel select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc12inch {
    #[doc = "0: If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1"]
    Adc12inch0 = 0,
    #[doc = "1: If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1"]
    Adc12inch1 = 1,
    #[doc = "2: If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3"]
    Adc12inch2 = 2,
    #[doc = "3: If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3"]
    Adc12inch3 = 3,
    #[doc = "4: If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5"]
    Adc12inch4 = 4,
    #[doc = "5: If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5"]
    Adc12inch5 = 5,
    #[doc = "6: If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7"]
    Adc12inch6 = 6,
    #[doc = "7: If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7"]
    Adc12inch7 = 7,
    #[doc = "8: If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9"]
    Adc12inch8 = 8,
    #[doc = "9: If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9"]
    Adc12inch9 = 9,
    #[doc = "10: If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11"]
    Adc12inch10 = 10,
    #[doc = "11: If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11"]
    Adc12inch11 = 11,
    #[doc = "12: If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13"]
    Adc12inch12 = 12,
    #[doc = "13: If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13"]
    Adc12inch13 = 13,
    #[doc = "14: If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15"]
    Adc12inch14 = 14,
    #[doc = "15: If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15"]
    Adc12inch15 = 15,
    #[doc = "16: If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17"]
    Adc12inch16 = 16,
    #[doc = "17: If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17"]
    Adc12inch17 = 17,
    #[doc = "18: If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19"]
    Adc12inch18 = 18,
    #[doc = "19: If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19"]
    Adc12inch19 = 19,
    #[doc = "20: If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21"]
    Adc12inch20 = 20,
    #[doc = "21: If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21"]
    Adc12inch21 = 21,
    #[doc = "22: If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23"]
    Adc12inch22 = 22,
    #[doc = "23: If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23"]
    Adc12inch23 = 23,
    #[doc = "24: If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25"]
    Adc12inch24 = 24,
    #[doc = "25: If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25"]
    Adc12inch25 = 25,
    #[doc = "26: If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27"]
    Adc12inch26 = 26,
    #[doc = "27: If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27"]
    Adc12inch27 = 27,
    #[doc = "28: If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29"]
    Adc12inch28 = 28,
    #[doc = "29: If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29"]
    Adc12inch29 = 29,
    #[doc = "30: If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31"]
    Adc12inch30 = 30,
    #[doc = "31: If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31"]
    Adc12inch31 = 31,
}
impl From<Adc12inch> for u8 {
    #[inline(always)]
    fn from(variant: Adc12inch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc12inch {
    type Ux = u8;
}
impl crate::IsEnum for Adc12inch {}
#[doc = "Field `ADC12INCH` reader - Input channel select"]
pub type Adc12inchR = crate::FieldReader<Adc12inch>;
impl Adc12inchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12inch {
        match self.bits {
            0 => Adc12inch::Adc12inch0,
            1 => Adc12inch::Adc12inch1,
            2 => Adc12inch::Adc12inch2,
            3 => Adc12inch::Adc12inch3,
            4 => Adc12inch::Adc12inch4,
            5 => Adc12inch::Adc12inch5,
            6 => Adc12inch::Adc12inch6,
            7 => Adc12inch::Adc12inch7,
            8 => Adc12inch::Adc12inch8,
            9 => Adc12inch::Adc12inch9,
            10 => Adc12inch::Adc12inch10,
            11 => Adc12inch::Adc12inch11,
            12 => Adc12inch::Adc12inch12,
            13 => Adc12inch::Adc12inch13,
            14 => Adc12inch::Adc12inch14,
            15 => Adc12inch::Adc12inch15,
            16 => Adc12inch::Adc12inch16,
            17 => Adc12inch::Adc12inch17,
            18 => Adc12inch::Adc12inch18,
            19 => Adc12inch::Adc12inch19,
            20 => Adc12inch::Adc12inch20,
            21 => Adc12inch::Adc12inch21,
            22 => Adc12inch::Adc12inch22,
            23 => Adc12inch::Adc12inch23,
            24 => Adc12inch::Adc12inch24,
            25 => Adc12inch::Adc12inch25,
            26 => Adc12inch::Adc12inch26,
            27 => Adc12inch::Adc12inch27,
            28 => Adc12inch::Adc12inch28,
            29 => Adc12inch::Adc12inch29,
            30 => Adc12inch::Adc12inch30,
            31 => Adc12inch::Adc12inch31,
            _ => unreachable!(),
        }
    }
    #[doc = "If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1"]
    #[inline(always)]
    pub fn is_adc12inch_0(&self) -> bool {
        *self == Adc12inch::Adc12inch0
    }
    #[doc = "If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1"]
    #[inline(always)]
    pub fn is_adc12inch_1(&self) -> bool {
        *self == Adc12inch::Adc12inch1
    }
    #[doc = "If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3"]
    #[inline(always)]
    pub fn is_adc12inch_2(&self) -> bool {
        *self == Adc12inch::Adc12inch2
    }
    #[doc = "If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3"]
    #[inline(always)]
    pub fn is_adc12inch_3(&self) -> bool {
        *self == Adc12inch::Adc12inch3
    }
    #[doc = "If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5"]
    #[inline(always)]
    pub fn is_adc12inch_4(&self) -> bool {
        *self == Adc12inch::Adc12inch4
    }
    #[doc = "If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5"]
    #[inline(always)]
    pub fn is_adc12inch_5(&self) -> bool {
        *self == Adc12inch::Adc12inch5
    }
    #[doc = "If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7"]
    #[inline(always)]
    pub fn is_adc12inch_6(&self) -> bool {
        *self == Adc12inch::Adc12inch6
    }
    #[doc = "If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7"]
    #[inline(always)]
    pub fn is_adc12inch_7(&self) -> bool {
        *self == Adc12inch::Adc12inch7
    }
    #[doc = "If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9"]
    #[inline(always)]
    pub fn is_adc12inch_8(&self) -> bool {
        *self == Adc12inch::Adc12inch8
    }
    #[doc = "If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9"]
    #[inline(always)]
    pub fn is_adc12inch_9(&self) -> bool {
        *self == Adc12inch::Adc12inch9
    }
    #[doc = "If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11"]
    #[inline(always)]
    pub fn is_adc12inch_10(&self) -> bool {
        *self == Adc12inch::Adc12inch10
    }
    #[doc = "If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11"]
    #[inline(always)]
    pub fn is_adc12inch_11(&self) -> bool {
        *self == Adc12inch::Adc12inch11
    }
    #[doc = "If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13"]
    #[inline(always)]
    pub fn is_adc12inch_12(&self) -> bool {
        *self == Adc12inch::Adc12inch12
    }
    #[doc = "If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13"]
    #[inline(always)]
    pub fn is_adc12inch_13(&self) -> bool {
        *self == Adc12inch::Adc12inch13
    }
    #[doc = "If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15"]
    #[inline(always)]
    pub fn is_adc12inch_14(&self) -> bool {
        *self == Adc12inch::Adc12inch14
    }
    #[doc = "If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15"]
    #[inline(always)]
    pub fn is_adc12inch_15(&self) -> bool {
        *self == Adc12inch::Adc12inch15
    }
    #[doc = "If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17"]
    #[inline(always)]
    pub fn is_adc12inch_16(&self) -> bool {
        *self == Adc12inch::Adc12inch16
    }
    #[doc = "If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17"]
    #[inline(always)]
    pub fn is_adc12inch_17(&self) -> bool {
        *self == Adc12inch::Adc12inch17
    }
    #[doc = "If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19"]
    #[inline(always)]
    pub fn is_adc12inch_18(&self) -> bool {
        *self == Adc12inch::Adc12inch18
    }
    #[doc = "If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19"]
    #[inline(always)]
    pub fn is_adc12inch_19(&self) -> bool {
        *self == Adc12inch::Adc12inch19
    }
    #[doc = "If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21"]
    #[inline(always)]
    pub fn is_adc12inch_20(&self) -> bool {
        *self == Adc12inch::Adc12inch20
    }
    #[doc = "If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21"]
    #[inline(always)]
    pub fn is_adc12inch_21(&self) -> bool {
        *self == Adc12inch::Adc12inch21
    }
    #[doc = "If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23"]
    #[inline(always)]
    pub fn is_adc12inch_22(&self) -> bool {
        *self == Adc12inch::Adc12inch22
    }
    #[doc = "If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23"]
    #[inline(always)]
    pub fn is_adc12inch_23(&self) -> bool {
        *self == Adc12inch::Adc12inch23
    }
    #[doc = "If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25"]
    #[inline(always)]
    pub fn is_adc12inch_24(&self) -> bool {
        *self == Adc12inch::Adc12inch24
    }
    #[doc = "If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25"]
    #[inline(always)]
    pub fn is_adc12inch_25(&self) -> bool {
        *self == Adc12inch::Adc12inch25
    }
    #[doc = "If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27"]
    #[inline(always)]
    pub fn is_adc12inch_26(&self) -> bool {
        *self == Adc12inch::Adc12inch26
    }
    #[doc = "If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27"]
    #[inline(always)]
    pub fn is_adc12inch_27(&self) -> bool {
        *self == Adc12inch::Adc12inch27
    }
    #[doc = "If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29"]
    #[inline(always)]
    pub fn is_adc12inch_28(&self) -> bool {
        *self == Adc12inch::Adc12inch28
    }
    #[doc = "If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29"]
    #[inline(always)]
    pub fn is_adc12inch_29(&self) -> bool {
        *self == Adc12inch::Adc12inch29
    }
    #[doc = "If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31"]
    #[inline(always)]
    pub fn is_adc12inch_30(&self) -> bool {
        *self == Adc12inch::Adc12inch30
    }
    #[doc = "If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31"]
    #[inline(always)]
    pub fn is_adc12inch_31(&self) -> bool {
        *self == Adc12inch::Adc12inch31
    }
}
#[doc = "Field `ADC12INCH` writer - Input channel select"]
pub type Adc12inchW<'a, REG> = crate::FieldWriter<'a, REG, 5, Adc12inch, crate::Safe>;
impl<'a, REG> Adc12inchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1"]
    #[inline(always)]
    pub fn adc12inch_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch0)
    }
    #[doc = "If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1"]
    #[inline(always)]
    pub fn adc12inch_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch1)
    }
    #[doc = "If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3"]
    #[inline(always)]
    pub fn adc12inch_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch2)
    }
    #[doc = "If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3"]
    #[inline(always)]
    pub fn adc12inch_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch3)
    }
    #[doc = "If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5"]
    #[inline(always)]
    pub fn adc12inch_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch4)
    }
    #[doc = "If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5"]
    #[inline(always)]
    pub fn adc12inch_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch5)
    }
    #[doc = "If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7"]
    #[inline(always)]
    pub fn adc12inch_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch6)
    }
    #[doc = "If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7"]
    #[inline(always)]
    pub fn adc12inch_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch7)
    }
    #[doc = "If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9"]
    #[inline(always)]
    pub fn adc12inch_8(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch8)
    }
    #[doc = "If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9"]
    #[inline(always)]
    pub fn adc12inch_9(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch9)
    }
    #[doc = "If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11"]
    #[inline(always)]
    pub fn adc12inch_10(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch10)
    }
    #[doc = "If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11"]
    #[inline(always)]
    pub fn adc12inch_11(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch11)
    }
    #[doc = "If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13"]
    #[inline(always)]
    pub fn adc12inch_12(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch12)
    }
    #[doc = "If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13"]
    #[inline(always)]
    pub fn adc12inch_13(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch13)
    }
    #[doc = "If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15"]
    #[inline(always)]
    pub fn adc12inch_14(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch14)
    }
    #[doc = "If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15"]
    #[inline(always)]
    pub fn adc12inch_15(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch15)
    }
    #[doc = "If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17"]
    #[inline(always)]
    pub fn adc12inch_16(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch16)
    }
    #[doc = "If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17"]
    #[inline(always)]
    pub fn adc12inch_17(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch17)
    }
    #[doc = "If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19"]
    #[inline(always)]
    pub fn adc12inch_18(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch18)
    }
    #[doc = "If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19"]
    #[inline(always)]
    pub fn adc12inch_19(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch19)
    }
    #[doc = "If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21"]
    #[inline(always)]
    pub fn adc12inch_20(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch20)
    }
    #[doc = "If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21"]
    #[inline(always)]
    pub fn adc12inch_21(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch21)
    }
    #[doc = "If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23"]
    #[inline(always)]
    pub fn adc12inch_22(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch22)
    }
    #[doc = "If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23"]
    #[inline(always)]
    pub fn adc12inch_23(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch23)
    }
    #[doc = "If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25"]
    #[inline(always)]
    pub fn adc12inch_24(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch24)
    }
    #[doc = "If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25"]
    #[inline(always)]
    pub fn adc12inch_25(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch25)
    }
    #[doc = "If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27"]
    #[inline(always)]
    pub fn adc12inch_26(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch26)
    }
    #[doc = "If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27"]
    #[inline(always)]
    pub fn adc12inch_27(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch27)
    }
    #[doc = "If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29"]
    #[inline(always)]
    pub fn adc12inch_28(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch28)
    }
    #[doc = "If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29"]
    #[inline(always)]
    pub fn adc12inch_29(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch29)
    }
    #[doc = "If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31"]
    #[inline(always)]
    pub fn adc12inch_30(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch30)
    }
    #[doc = "If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31"]
    #[inline(always)]
    pub fn adc12inch_31(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12inch::Adc12inch31)
    }
}
#[doc = "End of sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12eos {
    #[doc = "0: Not end of sequence"]
    Adc12eos0 = 0,
    #[doc = "1: End of sequence"]
    Adc12eos1 = 1,
}
impl From<Adc12eos> for bool {
    #[inline(always)]
    fn from(variant: Adc12eos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12EOS` reader - End of sequence"]
pub type Adc12eosR = crate::BitReader<Adc12eos>;
impl Adc12eosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12eos {
        match self.bits {
            false => Adc12eos::Adc12eos0,
            true => Adc12eos::Adc12eos1,
        }
    }
    #[doc = "Not end of sequence"]
    #[inline(always)]
    pub fn is_adc12eos_0(&self) -> bool {
        *self == Adc12eos::Adc12eos0
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn is_adc12eos_1(&self) -> bool {
        *self == Adc12eos::Adc12eos1
    }
}
#[doc = "Field `ADC12EOS` writer - End of sequence"]
pub type Adc12eosW<'a, REG> = crate::BitWriter<'a, REG, Adc12eos>;
impl<'a, REG> Adc12eosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not end of sequence"]
    #[inline(always)]
    pub fn adc12eos_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12eos::Adc12eos0)
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn adc12eos_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12eos::Adc12eos1)
    }
}
#[doc = "reference selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc12vrsel {
    #[doc = "0: VR+ = AVCC, VR- = AVSS"]
    Adc12vrsel0 = 0,
    #[doc = "1: VR+ = VREF buffered, VR- = AVSS"]
    Adc12vrsel1 = 1,
    #[doc = "2: VR+ = VeREF-, VR- = AVSS"]
    Adc12vrsel2 = 2,
    #[doc = "3: VR+ = VeREF+ buffered, VR- = AVSS"]
    Adc12vrsel3 = 3,
    #[doc = "4: VR+ = VeREF+, VR- = AVSS"]
    Adc12vrsel4 = 4,
    #[doc = "5: VR+ = AVCC, VR- = VeREF+ buffered"]
    Adc12vrsel5 = 5,
    #[doc = "6: VR+ = AVCC, VR- = VeREF+"]
    Adc12vrsel6 = 6,
    #[doc = "7: VR+ = VREF buffered, VR- = VeREF+"]
    Adc12vrsel7 = 7,
    #[doc = "8: Reserved"]
    Adc12vrsel8 = 8,
    #[doc = "9: VR+ = AVCC, VR- = VREF buffered"]
    Adc12vrsel9 = 9,
    #[doc = "10: Reserved"]
    Adc12vrsel10 = 10,
    #[doc = "11: VR+ = VeREF+, VR- = VREF buffered"]
    Adc12vrsel11 = 11,
    #[doc = "12: VR+ = AVCC, VR- = VeREF-"]
    Adc12vrsel12 = 12,
    #[doc = "13: VR+ = VREF buffered, VR- = VeREF-"]
    Adc12vrsel13 = 13,
    #[doc = "14: VR+ = VeREF+, VR- = VeREF-"]
    Adc12vrsel14 = 14,
    #[doc = "15: VR+ = VeREF+ buffered, VR- = VeREF-"]
    Adc12vrsel15 = 15,
}
impl From<Adc12vrsel> for u8 {
    #[inline(always)]
    fn from(variant: Adc12vrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc12vrsel {
    type Ux = u8;
}
impl crate::IsEnum for Adc12vrsel {}
#[doc = "Field `ADC12VRSEL` reader - reference selection"]
pub type Adc12vrselR = crate::FieldReader<Adc12vrsel>;
impl Adc12vrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12vrsel {
        match self.bits {
            0 => Adc12vrsel::Adc12vrsel0,
            1 => Adc12vrsel::Adc12vrsel1,
            2 => Adc12vrsel::Adc12vrsel2,
            3 => Adc12vrsel::Adc12vrsel3,
            4 => Adc12vrsel::Adc12vrsel4,
            5 => Adc12vrsel::Adc12vrsel5,
            6 => Adc12vrsel::Adc12vrsel6,
            7 => Adc12vrsel::Adc12vrsel7,
            8 => Adc12vrsel::Adc12vrsel8,
            9 => Adc12vrsel::Adc12vrsel9,
            10 => Adc12vrsel::Adc12vrsel10,
            11 => Adc12vrsel::Adc12vrsel11,
            12 => Adc12vrsel::Adc12vrsel12,
            13 => Adc12vrsel::Adc12vrsel13,
            14 => Adc12vrsel::Adc12vrsel14,
            15 => Adc12vrsel::Adc12vrsel15,
            _ => unreachable!(),
        }
    }
    #[doc = "VR+ = AVCC, VR- = AVSS"]
    #[inline(always)]
    pub fn is_adc12vrsel_0(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel0
    }
    #[doc = "VR+ = VREF buffered, VR- = AVSS"]
    #[inline(always)]
    pub fn is_adc12vrsel_1(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel1
    }
    #[doc = "VR+ = VeREF-, VR- = AVSS"]
    #[inline(always)]
    pub fn is_adc12vrsel_2(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel2
    }
    #[doc = "VR+ = VeREF+ buffered, VR- = AVSS"]
    #[inline(always)]
    pub fn is_adc12vrsel_3(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel3
    }
    #[doc = "VR+ = VeREF+, VR- = AVSS"]
    #[inline(always)]
    pub fn is_adc12vrsel_4(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel4
    }
    #[doc = "VR+ = AVCC, VR- = VeREF+ buffered"]
    #[inline(always)]
    pub fn is_adc12vrsel_5(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel5
    }
    #[doc = "VR+ = AVCC, VR- = VeREF+"]
    #[inline(always)]
    pub fn is_adc12vrsel_6(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel6
    }
    #[doc = "VR+ = VREF buffered, VR- = VeREF+"]
    #[inline(always)]
    pub fn is_adc12vrsel_7(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel7
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adc12vrsel_8(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel8
    }
    #[doc = "VR+ = AVCC, VR- = VREF buffered"]
    #[inline(always)]
    pub fn is_adc12vrsel_9(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel9
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adc12vrsel_10(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel10
    }
    #[doc = "VR+ = VeREF+, VR- = VREF buffered"]
    #[inline(always)]
    pub fn is_adc12vrsel_11(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel11
    }
    #[doc = "VR+ = AVCC, VR- = VeREF-"]
    #[inline(always)]
    pub fn is_adc12vrsel_12(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel12
    }
    #[doc = "VR+ = VREF buffered, VR- = VeREF-"]
    #[inline(always)]
    pub fn is_adc12vrsel_13(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel13
    }
    #[doc = "VR+ = VeREF+, VR- = VeREF-"]
    #[inline(always)]
    pub fn is_adc12vrsel_14(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel14
    }
    #[doc = "VR+ = VeREF+ buffered, VR- = VeREF-"]
    #[inline(always)]
    pub fn is_adc12vrsel_15(&self) -> bool {
        *self == Adc12vrsel::Adc12vrsel15
    }
}
#[doc = "Field `ADC12VRSEL` writer - reference selection"]
pub type Adc12vrselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Adc12vrsel, crate::Safe>;
impl<'a, REG> Adc12vrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VR+ = AVCC, VR- = AVSS"]
    #[inline(always)]
    pub fn adc12vrsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel0)
    }
    #[doc = "VR+ = VREF buffered, VR- = AVSS"]
    #[inline(always)]
    pub fn adc12vrsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel1)
    }
    #[doc = "VR+ = VeREF-, VR- = AVSS"]
    #[inline(always)]
    pub fn adc12vrsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel2)
    }
    #[doc = "VR+ = VeREF+ buffered, VR- = AVSS"]
    #[inline(always)]
    pub fn adc12vrsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel3)
    }
    #[doc = "VR+ = VeREF+, VR- = AVSS"]
    #[inline(always)]
    pub fn adc12vrsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel4)
    }
    #[doc = "VR+ = AVCC, VR- = VeREF+ buffered"]
    #[inline(always)]
    pub fn adc12vrsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel5)
    }
    #[doc = "VR+ = AVCC, VR- = VeREF+"]
    #[inline(always)]
    pub fn adc12vrsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel6)
    }
    #[doc = "VR+ = VREF buffered, VR- = VeREF+"]
    #[inline(always)]
    pub fn adc12vrsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel7)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12vrsel_8(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel8)
    }
    #[doc = "VR+ = AVCC, VR- = VREF buffered"]
    #[inline(always)]
    pub fn adc12vrsel_9(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel9)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12vrsel_10(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel10)
    }
    #[doc = "VR+ = VeREF+, VR- = VREF buffered"]
    #[inline(always)]
    pub fn adc12vrsel_11(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel11)
    }
    #[doc = "VR+ = AVCC, VR- = VeREF-"]
    #[inline(always)]
    pub fn adc12vrsel_12(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel12)
    }
    #[doc = "VR+ = VREF buffered, VR- = VeREF-"]
    #[inline(always)]
    pub fn adc12vrsel_13(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel13)
    }
    #[doc = "VR+ = VeREF+, VR- = VeREF-"]
    #[inline(always)]
    pub fn adc12vrsel_14(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel14)
    }
    #[doc = "VR+ = VeREF+ buffered, VR- = VeREF-"]
    #[inline(always)]
    pub fn adc12vrsel_15(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12vrsel::Adc12vrsel15)
    }
}
#[doc = "Differential mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12dif {
    #[doc = "0: Single-ended mode enabled"]
    Adc12dif0 = 0,
    #[doc = "1: Differential mode enabled"]
    Adc12dif1 = 1,
}
impl From<Adc12dif> for bool {
    #[inline(always)]
    fn from(variant: Adc12dif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12DIF` reader - Differential mode."]
pub type Adc12difR = crate::BitReader<Adc12dif>;
impl Adc12difR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12dif {
        match self.bits {
            false => Adc12dif::Adc12dif0,
            true => Adc12dif::Adc12dif1,
        }
    }
    #[doc = "Single-ended mode enabled"]
    #[inline(always)]
    pub fn is_adc12dif_0(&self) -> bool {
        *self == Adc12dif::Adc12dif0
    }
    #[doc = "Differential mode enabled"]
    #[inline(always)]
    pub fn is_adc12dif_1(&self) -> bool {
        *self == Adc12dif::Adc12dif1
    }
}
#[doc = "Field `ADC12DIF` writer - Differential mode."]
pub type Adc12difW<'a, REG> = crate::BitWriter<'a, REG, Adc12dif>;
impl<'a, REG> Adc12difW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single-ended mode enabled"]
    #[inline(always)]
    pub fn adc12dif_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12dif::Adc12dif0)
    }
    #[doc = "Differential mode enabled"]
    #[inline(always)]
    pub fn adc12dif_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12dif::Adc12dif1)
    }
}
#[doc = "Comparator window enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12winc {
    #[doc = "0: Comparator window disabled"]
    Adc12winc0 = 0,
    #[doc = "1: Comparator window enabled"]
    Adc12winc1 = 1,
}
impl From<Adc12winc> for bool {
    #[inline(always)]
    fn from(variant: Adc12winc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12WINC` reader - Comparator window enable"]
pub type Adc12wincR = crate::BitReader<Adc12winc>;
impl Adc12wincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12winc {
        match self.bits {
            false => Adc12winc::Adc12winc0,
            true => Adc12winc::Adc12winc1,
        }
    }
    #[doc = "Comparator window disabled"]
    #[inline(always)]
    pub fn is_adc12winc_0(&self) -> bool {
        *self == Adc12winc::Adc12winc0
    }
    #[doc = "Comparator window enabled"]
    #[inline(always)]
    pub fn is_adc12winc_1(&self) -> bool {
        *self == Adc12winc::Adc12winc1
    }
}
#[doc = "Field `ADC12WINC` writer - Comparator window enable"]
pub type Adc12wincW<'a, REG> = crate::BitWriter<'a, REG, Adc12winc>;
impl<'a, REG> Adc12wincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator window disabled"]
    #[inline(always)]
    pub fn adc12winc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12winc::Adc12winc0)
    }
    #[doc = "Comparator window enabled"]
    #[inline(always)]
    pub fn adc12winc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12winc::Adc12winc1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adc12inch(&self) -> Adc12inchR {
        Adc12inchR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - End of sequence"]
    #[inline(always)]
    pub fn adc12eos(&self) -> Adc12eosR {
        Adc12eosR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - reference selection"]
    #[inline(always)]
    pub fn adc12vrsel(&self) -> Adc12vrselR {
        Adc12vrselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Differential mode."]
    #[inline(always)]
    pub fn adc12dif(&self) -> Adc12difR {
        Adc12difR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Comparator window enable"]
    #[inline(always)]
    pub fn adc12winc(&self) -> Adc12wincR {
        Adc12wincR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adc12inch(&mut self) -> Adc12inchW<'_, Adc12mctl26Spec> {
        Adc12inchW::new(self, 0)
    }
    #[doc = "Bit 7 - End of sequence"]
    #[inline(always)]
    pub fn adc12eos(&mut self) -> Adc12eosW<'_, Adc12mctl26Spec> {
        Adc12eosW::new(self, 7)
    }
    #[doc = "Bits 8:11 - reference selection"]
    #[inline(always)]
    pub fn adc12vrsel(&mut self) -> Adc12vrselW<'_, Adc12mctl26Spec> {
        Adc12vrselW::new(self, 8)
    }
    #[doc = "Bit 13 - Differential mode."]
    #[inline(always)]
    pub fn adc12dif(&mut self) -> Adc12difW<'_, Adc12mctl26Spec> {
        Adc12difW::new(self, 13)
    }
    #[doc = "Bit 14 - Comparator window enable"]
    #[inline(always)]
    pub fn adc12winc(&mut self) -> Adc12wincW<'_, Adc12mctl26Spec> {
        Adc12wincW::new(self, 14)
    }
}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12mctl26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12mctl26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12mctl26Spec;
impl crate::RegisterSpec for Adc12mctl26Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12mctl26::R`](R) reader structure"]
impl crate::Readable for Adc12mctl26Spec {}
#[doc = "`write(|w| ..)` method takes [`adc12mctl26::W`](W) writer structure"]
impl crate::Writable for Adc12mctl26Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC12MCTL26 to value 0"]
impl crate::Resettable for Adc12mctl26Spec {}
