#[doc = "Register `LCDCTL0` reader"]
pub type R = crate::R<Lcdctl0Spec>;
#[doc = "Register `LCDCTL0` writer"]
pub type W = crate::W<Lcdctl0Spec>;
#[doc = "Field `LCDON` reader - LCD_E LCD On"]
pub type LcdonR = crate::BitReader;
#[doc = "Field `LCDON` writer - LCD_E LCD On"]
pub type LcdonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDLP` reader - LCD_E Low Power Waveform"]
pub type LcdlpR = crate::BitReader;
#[doc = "Field `LCDLP` writer - LCD_E Low Power Waveform"]
pub type LcdlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDSON` reader - LCD_E LCD Segments On"]
pub type LcdsonR = crate::BitReader;
#[doc = "Field `LCDSON` writer - LCD_E LCD Segments On"]
pub type LcdsonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDMX0` reader - LCD_E Mux Rate Bit: 0"]
pub type Lcdmx0R = crate::BitReader;
#[doc = "Field `LCDMX0` writer - LCD_E Mux Rate Bit: 0"]
pub type Lcdmx0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDMX1` reader - LCD_E Mux Rate Bit: 1"]
pub type Lcdmx1R = crate::BitReader;
#[doc = "Field `LCDMX1` writer - LCD_E Mux Rate Bit: 1"]
pub type Lcdmx1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDMX2` reader - LCD_E Mux Rate Bit: 2"]
pub type Lcdmx2R = crate::BitReader;
#[doc = "Field `LCDMX2` writer - LCD_E Mux Rate Bit: 2"]
pub type Lcdmx2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "LCD_E Clock Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcdssel {
    #[doc = "0: LCD_E Clock Select: 0"]
    Lcdssel0 = 0,
    #[doc = "1: LCD_E Clock Select: 1"]
    Lcdssel1 = 1,
    #[doc = "2: LCD_E Clock Select: 2"]
    Lcdssel2 = 2,
    #[doc = "3: LCD_E Clock Select: 3"]
    Lcdssel3 = 3,
}
impl From<Lcdssel> for u8 {
    #[inline(always)]
    fn from(variant: Lcdssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcdssel {
    type Ux = u8;
}
impl crate::IsEnum for Lcdssel {}
#[doc = "Field `LCDSSEL` reader - LCD_E Clock Select Bit: 0"]
pub type LcdsselR = crate::FieldReader<Lcdssel>;
impl LcdsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdssel {
        match self.bits {
            0 => Lcdssel::Lcdssel0,
            1 => Lcdssel::Lcdssel1,
            2 => Lcdssel::Lcdssel2,
            3 => Lcdssel::Lcdssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "LCD_E Clock Select: 0"]
    #[inline(always)]
    pub fn is_lcdssel_0(&self) -> bool {
        *self == Lcdssel::Lcdssel0
    }
    #[doc = "LCD_E Clock Select: 1"]
    #[inline(always)]
    pub fn is_lcdssel_1(&self) -> bool {
        *self == Lcdssel::Lcdssel1
    }
    #[doc = "LCD_E Clock Select: 2"]
    #[inline(always)]
    pub fn is_lcdssel_2(&self) -> bool {
        *self == Lcdssel::Lcdssel2
    }
    #[doc = "LCD_E Clock Select: 3"]
    #[inline(always)]
    pub fn is_lcdssel_3(&self) -> bool {
        *self == Lcdssel::Lcdssel3
    }
}
#[doc = "Field `LCDSSEL` writer - LCD_E Clock Select Bit: 0"]
pub type LcdsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lcdssel, crate::Safe>;
impl<'a, REG> LcdsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LCD_E Clock Select: 0"]
    #[inline(always)]
    pub fn lcdssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdssel::Lcdssel0)
    }
    #[doc = "LCD_E Clock Select: 1"]
    #[inline(always)]
    pub fn lcdssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdssel::Lcdssel1)
    }
    #[doc = "LCD_E Clock Select: 2"]
    #[inline(always)]
    pub fn lcdssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdssel::Lcdssel2)
    }
    #[doc = "LCD_E Clock Select: 3"]
    #[inline(always)]
    pub fn lcdssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdssel::Lcdssel3)
    }
}
#[doc = "LCD_E LCD frequency divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcddiv {
    #[doc = "0: LCD_E LCD frequency divider: /1"]
    Lcddiv0 = 0,
    #[doc = "1: LCD_E LCD frequency divider: /2"]
    Lcddiv1 = 1,
    #[doc = "2: LCD_E LCD frequency divider: /3"]
    Lcddiv2 = 2,
    #[doc = "3: LCD_E LCD frequency divider: /4"]
    Lcddiv3 = 3,
    #[doc = "4: LCD_E LCD frequency divider: /5"]
    Lcddiv4 = 4,
    #[doc = "5: LCD_E LCD frequency divider: /6"]
    Lcddiv5 = 5,
    #[doc = "6: LCD_E LCD frequency divider: /7"]
    Lcddiv6 = 6,
    #[doc = "7: LCD_E LCD frequency divider: /8"]
    Lcddiv7 = 7,
    #[doc = "8: LCD_E LCD frequency divider: /9"]
    Lcddiv8 = 8,
    #[doc = "9: LCD_E LCD frequency divider: /10"]
    Lcddiv9 = 9,
    #[doc = "10: LCD_E LCD frequency divider: /11"]
    Lcddiv10 = 10,
    #[doc = "11: LCD_E LCD frequency divider: /12"]
    Lcddiv11 = 11,
    #[doc = "12: LCD_E LCD frequency divider: /13"]
    Lcddiv12 = 12,
    #[doc = "13: LCD_E LCD frequency divider: /14"]
    Lcddiv13 = 13,
    #[doc = "14: LCD_E LCD frequency divider: /15"]
    Lcddiv14 = 14,
    #[doc = "15: LCD_E LCD frequency divider: /16"]
    Lcddiv15 = 15,
    #[doc = "16: LCD_E LCD frequency divider: /17"]
    Lcddiv16 = 16,
    #[doc = "17: LCD_E LCD frequency divider: /18"]
    Lcddiv17 = 17,
    #[doc = "18: LCD_E LCD frequency divider: /19"]
    Lcddiv18 = 18,
    #[doc = "19: LCD_E LCD frequency divider: /20"]
    Lcddiv19 = 19,
    #[doc = "20: LCD_E LCD frequency divider: /21"]
    Lcddiv20 = 20,
    #[doc = "21: LCD_E LCD frequency divider: /22"]
    Lcddiv21 = 21,
    #[doc = "22: LCD_E LCD frequency divider: /23"]
    Lcddiv22 = 22,
    #[doc = "23: LCD_E LCD frequency divider: /24"]
    Lcddiv23 = 23,
    #[doc = "24: LCD_E LCD frequency divider: /25"]
    Lcddiv24 = 24,
    #[doc = "25: LCD_E LCD frequency divider: /26"]
    Lcddiv25 = 25,
    #[doc = "26: LCD_E LCD frequency divider: /27"]
    Lcddiv26 = 26,
    #[doc = "27: LCD_E LCD frequency divider: /28"]
    Lcddiv27 = 27,
    #[doc = "28: LCD_E LCD frequency divider: /29"]
    Lcddiv28 = 28,
    #[doc = "29: LCD_E LCD frequency divider: /30"]
    Lcddiv29 = 29,
    #[doc = "30: LCD_E LCD frequency divider: /31"]
    Lcddiv30 = 30,
    #[doc = "31: LCD_E LCD frequency divider: /32"]
    Lcddiv31 = 31,
}
impl From<Lcddiv> for u8 {
    #[inline(always)]
    fn from(variant: Lcddiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcddiv {
    type Ux = u8;
}
impl crate::IsEnum for Lcddiv {}
#[doc = "Field `LCDDIV` reader - LCD_E LCD frequency divider Bit: 0"]
pub type LcddivR = crate::FieldReader<Lcddiv>;
impl LcddivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcddiv {
        match self.bits {
            0 => Lcddiv::Lcddiv0,
            1 => Lcddiv::Lcddiv1,
            2 => Lcddiv::Lcddiv2,
            3 => Lcddiv::Lcddiv3,
            4 => Lcddiv::Lcddiv4,
            5 => Lcddiv::Lcddiv5,
            6 => Lcddiv::Lcddiv6,
            7 => Lcddiv::Lcddiv7,
            8 => Lcddiv::Lcddiv8,
            9 => Lcddiv::Lcddiv9,
            10 => Lcddiv::Lcddiv10,
            11 => Lcddiv::Lcddiv11,
            12 => Lcddiv::Lcddiv12,
            13 => Lcddiv::Lcddiv13,
            14 => Lcddiv::Lcddiv14,
            15 => Lcddiv::Lcddiv15,
            16 => Lcddiv::Lcddiv16,
            17 => Lcddiv::Lcddiv17,
            18 => Lcddiv::Lcddiv18,
            19 => Lcddiv::Lcddiv19,
            20 => Lcddiv::Lcddiv20,
            21 => Lcddiv::Lcddiv21,
            22 => Lcddiv::Lcddiv22,
            23 => Lcddiv::Lcddiv23,
            24 => Lcddiv::Lcddiv24,
            25 => Lcddiv::Lcddiv25,
            26 => Lcddiv::Lcddiv26,
            27 => Lcddiv::Lcddiv27,
            28 => Lcddiv::Lcddiv28,
            29 => Lcddiv::Lcddiv29,
            30 => Lcddiv::Lcddiv30,
            31 => Lcddiv::Lcddiv31,
            _ => unreachable!(),
        }
    }
    #[doc = "LCD_E LCD frequency divider: /1"]
    #[inline(always)]
    pub fn is_lcddiv_0(&self) -> bool {
        *self == Lcddiv::Lcddiv0
    }
    #[doc = "LCD_E LCD frequency divider: /2"]
    #[inline(always)]
    pub fn is_lcddiv_1(&self) -> bool {
        *self == Lcddiv::Lcddiv1
    }
    #[doc = "LCD_E LCD frequency divider: /3"]
    #[inline(always)]
    pub fn is_lcddiv_2(&self) -> bool {
        *self == Lcddiv::Lcddiv2
    }
    #[doc = "LCD_E LCD frequency divider: /4"]
    #[inline(always)]
    pub fn is_lcddiv_3(&self) -> bool {
        *self == Lcddiv::Lcddiv3
    }
    #[doc = "LCD_E LCD frequency divider: /5"]
    #[inline(always)]
    pub fn is_lcddiv_4(&self) -> bool {
        *self == Lcddiv::Lcddiv4
    }
    #[doc = "LCD_E LCD frequency divider: /6"]
    #[inline(always)]
    pub fn is_lcddiv_5(&self) -> bool {
        *self == Lcddiv::Lcddiv5
    }
    #[doc = "LCD_E LCD frequency divider: /7"]
    #[inline(always)]
    pub fn is_lcddiv_6(&self) -> bool {
        *self == Lcddiv::Lcddiv6
    }
    #[doc = "LCD_E LCD frequency divider: /8"]
    #[inline(always)]
    pub fn is_lcddiv_7(&self) -> bool {
        *self == Lcddiv::Lcddiv7
    }
    #[doc = "LCD_E LCD frequency divider: /9"]
    #[inline(always)]
    pub fn is_lcddiv_8(&self) -> bool {
        *self == Lcddiv::Lcddiv8
    }
    #[doc = "LCD_E LCD frequency divider: /10"]
    #[inline(always)]
    pub fn is_lcddiv_9(&self) -> bool {
        *self == Lcddiv::Lcddiv9
    }
    #[doc = "LCD_E LCD frequency divider: /11"]
    #[inline(always)]
    pub fn is_lcddiv_10(&self) -> bool {
        *self == Lcddiv::Lcddiv10
    }
    #[doc = "LCD_E LCD frequency divider: /12"]
    #[inline(always)]
    pub fn is_lcddiv_11(&self) -> bool {
        *self == Lcddiv::Lcddiv11
    }
    #[doc = "LCD_E LCD frequency divider: /13"]
    #[inline(always)]
    pub fn is_lcddiv_12(&self) -> bool {
        *self == Lcddiv::Lcddiv12
    }
    #[doc = "LCD_E LCD frequency divider: /14"]
    #[inline(always)]
    pub fn is_lcddiv_13(&self) -> bool {
        *self == Lcddiv::Lcddiv13
    }
    #[doc = "LCD_E LCD frequency divider: /15"]
    #[inline(always)]
    pub fn is_lcddiv_14(&self) -> bool {
        *self == Lcddiv::Lcddiv14
    }
    #[doc = "LCD_E LCD frequency divider: /16"]
    #[inline(always)]
    pub fn is_lcddiv_15(&self) -> bool {
        *self == Lcddiv::Lcddiv15
    }
    #[doc = "LCD_E LCD frequency divider: /17"]
    #[inline(always)]
    pub fn is_lcddiv_16(&self) -> bool {
        *self == Lcddiv::Lcddiv16
    }
    #[doc = "LCD_E LCD frequency divider: /18"]
    #[inline(always)]
    pub fn is_lcddiv_17(&self) -> bool {
        *self == Lcddiv::Lcddiv17
    }
    #[doc = "LCD_E LCD frequency divider: /19"]
    #[inline(always)]
    pub fn is_lcddiv_18(&self) -> bool {
        *self == Lcddiv::Lcddiv18
    }
    #[doc = "LCD_E LCD frequency divider: /20"]
    #[inline(always)]
    pub fn is_lcddiv_19(&self) -> bool {
        *self == Lcddiv::Lcddiv19
    }
    #[doc = "LCD_E LCD frequency divider: /21"]
    #[inline(always)]
    pub fn is_lcddiv_20(&self) -> bool {
        *self == Lcddiv::Lcddiv20
    }
    #[doc = "LCD_E LCD frequency divider: /22"]
    #[inline(always)]
    pub fn is_lcddiv_21(&self) -> bool {
        *self == Lcddiv::Lcddiv21
    }
    #[doc = "LCD_E LCD frequency divider: /23"]
    #[inline(always)]
    pub fn is_lcddiv_22(&self) -> bool {
        *self == Lcddiv::Lcddiv22
    }
    #[doc = "LCD_E LCD frequency divider: /24"]
    #[inline(always)]
    pub fn is_lcddiv_23(&self) -> bool {
        *self == Lcddiv::Lcddiv23
    }
    #[doc = "LCD_E LCD frequency divider: /25"]
    #[inline(always)]
    pub fn is_lcddiv_24(&self) -> bool {
        *self == Lcddiv::Lcddiv24
    }
    #[doc = "LCD_E LCD frequency divider: /26"]
    #[inline(always)]
    pub fn is_lcddiv_25(&self) -> bool {
        *self == Lcddiv::Lcddiv25
    }
    #[doc = "LCD_E LCD frequency divider: /27"]
    #[inline(always)]
    pub fn is_lcddiv_26(&self) -> bool {
        *self == Lcddiv::Lcddiv26
    }
    #[doc = "LCD_E LCD frequency divider: /28"]
    #[inline(always)]
    pub fn is_lcddiv_27(&self) -> bool {
        *self == Lcddiv::Lcddiv27
    }
    #[doc = "LCD_E LCD frequency divider: /29"]
    #[inline(always)]
    pub fn is_lcddiv_28(&self) -> bool {
        *self == Lcddiv::Lcddiv28
    }
    #[doc = "LCD_E LCD frequency divider: /30"]
    #[inline(always)]
    pub fn is_lcddiv_29(&self) -> bool {
        *self == Lcddiv::Lcddiv29
    }
    #[doc = "LCD_E LCD frequency divider: /31"]
    #[inline(always)]
    pub fn is_lcddiv_30(&self) -> bool {
        *self == Lcddiv::Lcddiv30
    }
    #[doc = "LCD_E LCD frequency divider: /32"]
    #[inline(always)]
    pub fn is_lcddiv_31(&self) -> bool {
        *self == Lcddiv::Lcddiv31
    }
}
#[doc = "Field `LCDDIV` writer - LCD_E LCD frequency divider Bit: 0"]
pub type LcddivW<'a, REG> = crate::FieldWriter<'a, REG, 5, Lcddiv, crate::Safe>;
impl<'a, REG> LcddivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LCD_E LCD frequency divider: /1"]
    #[inline(always)]
    pub fn lcddiv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv0)
    }
    #[doc = "LCD_E LCD frequency divider: /2"]
    #[inline(always)]
    pub fn lcddiv_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv1)
    }
    #[doc = "LCD_E LCD frequency divider: /3"]
    #[inline(always)]
    pub fn lcddiv_2(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv2)
    }
    #[doc = "LCD_E LCD frequency divider: /4"]
    #[inline(always)]
    pub fn lcddiv_3(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv3)
    }
    #[doc = "LCD_E LCD frequency divider: /5"]
    #[inline(always)]
    pub fn lcddiv_4(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv4)
    }
    #[doc = "LCD_E LCD frequency divider: /6"]
    #[inline(always)]
    pub fn lcddiv_5(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv5)
    }
    #[doc = "LCD_E LCD frequency divider: /7"]
    #[inline(always)]
    pub fn lcddiv_6(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv6)
    }
    #[doc = "LCD_E LCD frequency divider: /8"]
    #[inline(always)]
    pub fn lcddiv_7(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv7)
    }
    #[doc = "LCD_E LCD frequency divider: /9"]
    #[inline(always)]
    pub fn lcddiv_8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv8)
    }
    #[doc = "LCD_E LCD frequency divider: /10"]
    #[inline(always)]
    pub fn lcddiv_9(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv9)
    }
    #[doc = "LCD_E LCD frequency divider: /11"]
    #[inline(always)]
    pub fn lcddiv_10(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv10)
    }
    #[doc = "LCD_E LCD frequency divider: /12"]
    #[inline(always)]
    pub fn lcddiv_11(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv11)
    }
    #[doc = "LCD_E LCD frequency divider: /13"]
    #[inline(always)]
    pub fn lcddiv_12(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv12)
    }
    #[doc = "LCD_E LCD frequency divider: /14"]
    #[inline(always)]
    pub fn lcddiv_13(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv13)
    }
    #[doc = "LCD_E LCD frequency divider: /15"]
    #[inline(always)]
    pub fn lcddiv_14(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv14)
    }
    #[doc = "LCD_E LCD frequency divider: /16"]
    #[inline(always)]
    pub fn lcddiv_15(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv15)
    }
    #[doc = "LCD_E LCD frequency divider: /17"]
    #[inline(always)]
    pub fn lcddiv_16(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv16)
    }
    #[doc = "LCD_E LCD frequency divider: /18"]
    #[inline(always)]
    pub fn lcddiv_17(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv17)
    }
    #[doc = "LCD_E LCD frequency divider: /19"]
    #[inline(always)]
    pub fn lcddiv_18(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv18)
    }
    #[doc = "LCD_E LCD frequency divider: /20"]
    #[inline(always)]
    pub fn lcddiv_19(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv19)
    }
    #[doc = "LCD_E LCD frequency divider: /21"]
    #[inline(always)]
    pub fn lcddiv_20(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv20)
    }
    #[doc = "LCD_E LCD frequency divider: /22"]
    #[inline(always)]
    pub fn lcddiv_21(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv21)
    }
    #[doc = "LCD_E LCD frequency divider: /23"]
    #[inline(always)]
    pub fn lcddiv_22(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv22)
    }
    #[doc = "LCD_E LCD frequency divider: /24"]
    #[inline(always)]
    pub fn lcddiv_23(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv23)
    }
    #[doc = "LCD_E LCD frequency divider: /25"]
    #[inline(always)]
    pub fn lcddiv_24(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv24)
    }
    #[doc = "LCD_E LCD frequency divider: /26"]
    #[inline(always)]
    pub fn lcddiv_25(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv25)
    }
    #[doc = "LCD_E LCD frequency divider: /27"]
    #[inline(always)]
    pub fn lcddiv_26(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv26)
    }
    #[doc = "LCD_E LCD frequency divider: /28"]
    #[inline(always)]
    pub fn lcddiv_27(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv27)
    }
    #[doc = "LCD_E LCD frequency divider: /29"]
    #[inline(always)]
    pub fn lcddiv_28(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv28)
    }
    #[doc = "LCD_E LCD frequency divider: /30"]
    #[inline(always)]
    pub fn lcddiv_29(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv29)
    }
    #[doc = "LCD_E LCD frequency divider: /31"]
    #[inline(always)]
    pub fn lcddiv_30(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv30)
    }
    #[doc = "LCD_E LCD frequency divider: /32"]
    #[inline(always)]
    pub fn lcddiv_31(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::Lcddiv31)
    }
}
impl R {
    #[doc = "Bit 0 - LCD_E LCD On"]
    #[inline(always)]
    pub fn lcdon(&self) -> LcdonR {
        LcdonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD_E Low Power Waveform"]
    #[inline(always)]
    pub fn lcdlp(&self) -> LcdlpR {
        LcdlpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD_E LCD Segments On"]
    #[inline(always)]
    pub fn lcdson(&self) -> LcdsonR {
        LcdsonR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD_E Mux Rate Bit: 0"]
    #[inline(always)]
    pub fn lcdmx0(&self) -> Lcdmx0R {
        Lcdmx0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD_E Mux Rate Bit: 1"]
    #[inline(always)]
    pub fn lcdmx1(&self) -> Lcdmx1R {
        Lcdmx1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD_E Mux Rate Bit: 2"]
    #[inline(always)]
    pub fn lcdmx2(&self) -> Lcdmx2R {
        Lcdmx2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - LCD_E Clock Select Bit: 0"]
    #[inline(always)]
    pub fn lcdssel(&self) -> LcdsselR {
        LcdsselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 11:15 - LCD_E LCD frequency divider Bit: 0"]
    #[inline(always)]
    pub fn lcddiv(&self) -> LcddivR {
        LcddivR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LCD_E LCD On"]
    #[inline(always)]
    pub fn lcdon(&mut self) -> LcdonW<'_, Lcdctl0Spec> {
        LcdonW::new(self, 0)
    }
    #[doc = "Bit 1 - LCD_E Low Power Waveform"]
    #[inline(always)]
    pub fn lcdlp(&mut self) -> LcdlpW<'_, Lcdctl0Spec> {
        LcdlpW::new(self, 1)
    }
    #[doc = "Bit 2 - LCD_E LCD Segments On"]
    #[inline(always)]
    pub fn lcdson(&mut self) -> LcdsonW<'_, Lcdctl0Spec> {
        LcdsonW::new(self, 2)
    }
    #[doc = "Bit 3 - LCD_E Mux Rate Bit: 0"]
    #[inline(always)]
    pub fn lcdmx0(&mut self) -> Lcdmx0W<'_, Lcdctl0Spec> {
        Lcdmx0W::new(self, 3)
    }
    #[doc = "Bit 4 - LCD_E Mux Rate Bit: 1"]
    #[inline(always)]
    pub fn lcdmx1(&mut self) -> Lcdmx1W<'_, Lcdctl0Spec> {
        Lcdmx1W::new(self, 4)
    }
    #[doc = "Bit 5 - LCD_E Mux Rate Bit: 2"]
    #[inline(always)]
    pub fn lcdmx2(&mut self) -> Lcdmx2W<'_, Lcdctl0Spec> {
        Lcdmx2W::new(self, 5)
    }
    #[doc = "Bits 6:7 - LCD_E Clock Select Bit: 0"]
    #[inline(always)]
    pub fn lcdssel(&mut self) -> LcdsselW<'_, Lcdctl0Spec> {
        LcdsselW::new(self, 6)
    }
    #[doc = "Bits 11:15 - LCD_E LCD frequency divider Bit: 0"]
    #[inline(always)]
    pub fn lcddiv(&mut self) -> LcddivW<'_, Lcdctl0Spec> {
        LcddivW::new(self, 11)
    }
}
#[doc = "LCD_E Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdctl0Spec;
impl crate::RegisterSpec for Lcdctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdctl0::R`](R) reader structure"]
impl crate::Readable for Lcdctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdctl0::W`](W) writer structure"]
impl crate::Writable for Lcdctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDCTL0 to value 0"]
impl crate::Resettable for Lcdctl0Spec {}
