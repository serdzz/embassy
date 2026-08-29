#[doc = "Register `ADC12IV` reader"]
pub type R = crate::R<Adc12ivSpec>;
#[doc = "Register `ADC12IV` writer"]
pub type W = crate::W<Adc12ivSpec>;
#[doc = "interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Adc12iv {
    #[doc = "0: Interrupt Source: No interrupt pending, Interrupt Flag: None"]
    None = 0,
    #[doc = "2: Interrupt Source: ADC12MEMx overflow, Interrupt Flag: ADC12OVIFG, Interrupt Priority: Highest"]
    Adc12ovifg = 2,
    #[doc = "4: Interrupt Source: Conversion time overflow, Interrupt Flag: ADC12TOVIFG"]
    Adc12tovifg = 4,
    #[doc = "6: Interrupt Source: ADC12 window high interrupt flag, Interrupt Flag: ADC12HIIFG"]
    Adc12hiifg = 6,
    #[doc = "8: Interrupt Source: ADC12 window low interrupt flag, Interrupt Flag: ADC12LOIFG"]
    Adc12loifg = 8,
    #[doc = "10: Interrupt Source: ADC12 in-window interrupt flag, Interrupt Flag: ADC12INIFG"]
    Adc12inifg = 10,
    #[doc = "12: Interrupt Source: ADC12MEM0 interrupt flag, Interrupt Flag: ADC12IFG0"]
    Adc12ifg0 = 12,
    #[doc = "14: Interrupt Source: ADC12MEM1 interrupt flag, Interrupt Flag: ADC12IFG1"]
    Adc12ifg1 = 14,
    #[doc = "16: Interrupt Source: ADC12MEM2 interrupt flag, Interrupt Flag: ADC12IFG2"]
    Adc12ifg2 = 16,
    #[doc = "18: Interrupt Source: ADC12MEM3 interrupt flag, Interrupt Flag: ADC12IFG3"]
    Adc12ifg3 = 18,
    #[doc = "20: Interrupt Source: ADC12MEM4 interrupt flag, Interrupt Flag: ADC12IFG4"]
    Adc12ifg4 = 20,
    #[doc = "22: Interrupt Source: ADC12MEM5 interrupt flag, Interrupt Flag: ADC12IFG5"]
    Adc12ifg5 = 22,
    #[doc = "24: Interrupt Source: ADC12MEM6 interrupt flag, Interrupt Flag: ADC12IFG6"]
    Adc12ifg6 = 24,
    #[doc = "26: Interrupt Source: ADC12MEM7 interrupt flag, Interrupt Flag: ADC12IFG7"]
    Adc12ifg7 = 26,
    #[doc = "28: Interrupt Source: ADC12MEM8 interrupt flag, Interrupt Flag: ADC12IFG8"]
    Adc12ifg8 = 28,
    #[doc = "30: Interrupt Source: ADC12MEM9 interrupt flag, Interrupt Flag: ADC12IFG9"]
    Adc12ifg9 = 30,
    #[doc = "32: Interrupt Source: ADC12MEM10 interrupt flag, Interrupt Flag: ADC12IFG10"]
    Adc12ifg10 = 32,
    #[doc = "34: Interrupt Source: ADC12MEM11 interrupt flag, Interrupt Flag: ADC12IFG11"]
    Adc12ifg11 = 34,
    #[doc = "36: Interrupt Source: ADC12MEM12 interrupt flag, Interrupt Flag: ADC12IFG12"]
    Adc12ifg12 = 36,
    #[doc = "38: Interrupt Source: ADC12MEM13 interrupt flag, Interrupt Flag: ADC12IFG13"]
    Adc12ifg13 = 38,
    #[doc = "40: Interrupt Source: ADC12MEM14 interrupt flag, Interrupt Flag: ADC12IFG14"]
    Adc12ifg14 = 40,
    #[doc = "42: Interrupt Source: ADC12MEM15 interrupt flag, Interrupt Flag: ADC12IFG15"]
    Adc12ifg15 = 42,
    #[doc = "44: Interrupt Source: ADC12MEM16 interrupt flag, Interrupt Flag: ADC12IFG16"]
    Adc12ifg16 = 44,
    #[doc = "46: Interrupt Source: ADC12MEM17 interrupt flag, Interrupt Flag: ADC12IFG17"]
    Adc12ifg17 = 46,
    #[doc = "48: Interrupt Source: ADC12MEM18 interrupt flag, Interrupt Flag: ADC12IFG18"]
    Adc12ifg18 = 48,
    #[doc = "50: Interrupt Source: ADC12MEM19 interrupt flag, Interrupt Flag: ADC12IFG19"]
    Adc12ifg19 = 50,
    #[doc = "52: Interrupt Source: ADC12MEM20 interrupt flag, Interrupt Flag: ADC12IFG20"]
    Adc12ifg20 = 52,
    #[doc = "54: Interrupt Source: ADC12MEM21 interrupt flag, Interrupt Flag: ADC12IFG21"]
    Adc12ifg21 = 54,
    #[doc = "56: Interrupt Source: ADC12MEM22 interrupt flag, Interrupt Flag: ADC12IFG22"]
    Adc12ifg22 = 56,
    #[doc = "58: Interrupt Source: ADC12MEM23 interrupt flag, Interrupt Flag: ADC12IFG23"]
    Adc12ifg23 = 58,
    #[doc = "60: Interrupt Source: ADC12MEM24 interrupt flag, Interrupt Flag: ADC12IFG24"]
    Adc12ifg24 = 60,
    #[doc = "62: Interrupt Source: ADC12MEM25 interrupt flag, Interrupt Flag: ADC12IFG25"]
    Adc12ifg25 = 62,
    #[doc = "64: Interrupt Source: ADC12MEM26 interrupt flag, Interrupt Flag: ADC12IFG26"]
    Adc12ifg26 = 64,
    #[doc = "66: Interrupt Source: ADC12MEM27 interrupt flag, Interrupt Flag: ADC12IFG27"]
    Adc12ifg27 = 66,
    #[doc = "68: Interrupt Source: ADC12MEM28 interrupt flag, Interrupt Flag: ADC12IFG28"]
    Adc12ifg28 = 68,
    #[doc = "70: Interrupt Source: ADC12MEM29 interrupt flag, Interrupt Flag: ADC12IFG29"]
    Adc12ifg29 = 70,
    #[doc = "72: Interrupt Source: ADC12MEM30 interrupt flag, Interrupt Flag: ADC12IFG30"]
    Adc12ifg30 = 72,
    #[doc = "74: Interrupt Source: ADC12MEM31 interrupt flag, Interrupt Flag: ADC12IFG31"]
    Adc12ifg31 = 74,
    #[doc = "76: Interrupt Source: ADC12RDYIFG interrupt flag, Interrupt Flag: ADC12RDYIFG"]
    Adc12rdyifg = 76,
}
impl From<Adc12iv> for u16 {
    #[inline(always)]
    fn from(variant: Adc12iv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc12iv {
    type Ux = u16;
}
impl crate::IsEnum for Adc12iv {}
#[doc = "Field `ADC12IV` reader - interrupt vector value"]
pub type Adc12ivR = crate::FieldReader<Adc12iv>;
impl Adc12ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adc12iv> {
        match self.bits {
            0 => Some(Adc12iv::None),
            2 => Some(Adc12iv::Adc12ovifg),
            4 => Some(Adc12iv::Adc12tovifg),
            6 => Some(Adc12iv::Adc12hiifg),
            8 => Some(Adc12iv::Adc12loifg),
            10 => Some(Adc12iv::Adc12inifg),
            12 => Some(Adc12iv::Adc12ifg0),
            14 => Some(Adc12iv::Adc12ifg1),
            16 => Some(Adc12iv::Adc12ifg2),
            18 => Some(Adc12iv::Adc12ifg3),
            20 => Some(Adc12iv::Adc12ifg4),
            22 => Some(Adc12iv::Adc12ifg5),
            24 => Some(Adc12iv::Adc12ifg6),
            26 => Some(Adc12iv::Adc12ifg7),
            28 => Some(Adc12iv::Adc12ifg8),
            30 => Some(Adc12iv::Adc12ifg9),
            32 => Some(Adc12iv::Adc12ifg10),
            34 => Some(Adc12iv::Adc12ifg11),
            36 => Some(Adc12iv::Adc12ifg12),
            38 => Some(Adc12iv::Adc12ifg13),
            40 => Some(Adc12iv::Adc12ifg14),
            42 => Some(Adc12iv::Adc12ifg15),
            44 => Some(Adc12iv::Adc12ifg16),
            46 => Some(Adc12iv::Adc12ifg17),
            48 => Some(Adc12iv::Adc12ifg18),
            50 => Some(Adc12iv::Adc12ifg19),
            52 => Some(Adc12iv::Adc12ifg20),
            54 => Some(Adc12iv::Adc12ifg21),
            56 => Some(Adc12iv::Adc12ifg22),
            58 => Some(Adc12iv::Adc12ifg23),
            60 => Some(Adc12iv::Adc12ifg24),
            62 => Some(Adc12iv::Adc12ifg25),
            64 => Some(Adc12iv::Adc12ifg26),
            66 => Some(Adc12iv::Adc12ifg27),
            68 => Some(Adc12iv::Adc12ifg28),
            70 => Some(Adc12iv::Adc12ifg29),
            72 => Some(Adc12iv::Adc12ifg30),
            74 => Some(Adc12iv::Adc12ifg31),
            76 => Some(Adc12iv::Adc12rdyifg),
            _ => None,
        }
    }
    #[doc = "Interrupt Source: No interrupt pending, Interrupt Flag: None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Adc12iv::None
    }
    #[doc = "Interrupt Source: ADC12MEMx overflow, Interrupt Flag: ADC12OVIFG, Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_adc12ovifg(&self) -> bool {
        *self == Adc12iv::Adc12ovifg
    }
    #[doc = "Interrupt Source: Conversion time overflow, Interrupt Flag: ADC12TOVIFG"]
    #[inline(always)]
    pub fn is_adc12tovifg(&self) -> bool {
        *self == Adc12iv::Adc12tovifg
    }
    #[doc = "Interrupt Source: ADC12 window high interrupt flag, Interrupt Flag: ADC12HIIFG"]
    #[inline(always)]
    pub fn is_adc12hiifg(&self) -> bool {
        *self == Adc12iv::Adc12hiifg
    }
    #[doc = "Interrupt Source: ADC12 window low interrupt flag, Interrupt Flag: ADC12LOIFG"]
    #[inline(always)]
    pub fn is_adc12loifg(&self) -> bool {
        *self == Adc12iv::Adc12loifg
    }
    #[doc = "Interrupt Source: ADC12 in-window interrupt flag, Interrupt Flag: ADC12INIFG"]
    #[inline(always)]
    pub fn is_adc12inifg(&self) -> bool {
        *self == Adc12iv::Adc12inifg
    }
    #[doc = "Interrupt Source: ADC12MEM0 interrupt flag, Interrupt Flag: ADC12IFG0"]
    #[inline(always)]
    pub fn is_adc12ifg0(&self) -> bool {
        *self == Adc12iv::Adc12ifg0
    }
    #[doc = "Interrupt Source: ADC12MEM1 interrupt flag, Interrupt Flag: ADC12IFG1"]
    #[inline(always)]
    pub fn is_adc12ifg1(&self) -> bool {
        *self == Adc12iv::Adc12ifg1
    }
    #[doc = "Interrupt Source: ADC12MEM2 interrupt flag, Interrupt Flag: ADC12IFG2"]
    #[inline(always)]
    pub fn is_adc12ifg2(&self) -> bool {
        *self == Adc12iv::Adc12ifg2
    }
    #[doc = "Interrupt Source: ADC12MEM3 interrupt flag, Interrupt Flag: ADC12IFG3"]
    #[inline(always)]
    pub fn is_adc12ifg3(&self) -> bool {
        *self == Adc12iv::Adc12ifg3
    }
    #[doc = "Interrupt Source: ADC12MEM4 interrupt flag, Interrupt Flag: ADC12IFG4"]
    #[inline(always)]
    pub fn is_adc12ifg4(&self) -> bool {
        *self == Adc12iv::Adc12ifg4
    }
    #[doc = "Interrupt Source: ADC12MEM5 interrupt flag, Interrupt Flag: ADC12IFG5"]
    #[inline(always)]
    pub fn is_adc12ifg5(&self) -> bool {
        *self == Adc12iv::Adc12ifg5
    }
    #[doc = "Interrupt Source: ADC12MEM6 interrupt flag, Interrupt Flag: ADC12IFG6"]
    #[inline(always)]
    pub fn is_adc12ifg6(&self) -> bool {
        *self == Adc12iv::Adc12ifg6
    }
    #[doc = "Interrupt Source: ADC12MEM7 interrupt flag, Interrupt Flag: ADC12IFG7"]
    #[inline(always)]
    pub fn is_adc12ifg7(&self) -> bool {
        *self == Adc12iv::Adc12ifg7
    }
    #[doc = "Interrupt Source: ADC12MEM8 interrupt flag, Interrupt Flag: ADC12IFG8"]
    #[inline(always)]
    pub fn is_adc12ifg8(&self) -> bool {
        *self == Adc12iv::Adc12ifg8
    }
    #[doc = "Interrupt Source: ADC12MEM9 interrupt flag, Interrupt Flag: ADC12IFG9"]
    #[inline(always)]
    pub fn is_adc12ifg9(&self) -> bool {
        *self == Adc12iv::Adc12ifg9
    }
    #[doc = "Interrupt Source: ADC12MEM10 interrupt flag, Interrupt Flag: ADC12IFG10"]
    #[inline(always)]
    pub fn is_adc12ifg10(&self) -> bool {
        *self == Adc12iv::Adc12ifg10
    }
    #[doc = "Interrupt Source: ADC12MEM11 interrupt flag, Interrupt Flag: ADC12IFG11"]
    #[inline(always)]
    pub fn is_adc12ifg11(&self) -> bool {
        *self == Adc12iv::Adc12ifg11
    }
    #[doc = "Interrupt Source: ADC12MEM12 interrupt flag, Interrupt Flag: ADC12IFG12"]
    #[inline(always)]
    pub fn is_adc12ifg12(&self) -> bool {
        *self == Adc12iv::Adc12ifg12
    }
    #[doc = "Interrupt Source: ADC12MEM13 interrupt flag, Interrupt Flag: ADC12IFG13"]
    #[inline(always)]
    pub fn is_adc12ifg13(&self) -> bool {
        *self == Adc12iv::Adc12ifg13
    }
    #[doc = "Interrupt Source: ADC12MEM14 interrupt flag, Interrupt Flag: ADC12IFG14"]
    #[inline(always)]
    pub fn is_adc12ifg14(&self) -> bool {
        *self == Adc12iv::Adc12ifg14
    }
    #[doc = "Interrupt Source: ADC12MEM15 interrupt flag, Interrupt Flag: ADC12IFG15"]
    #[inline(always)]
    pub fn is_adc12ifg15(&self) -> bool {
        *self == Adc12iv::Adc12ifg15
    }
    #[doc = "Interrupt Source: ADC12MEM16 interrupt flag, Interrupt Flag: ADC12IFG16"]
    #[inline(always)]
    pub fn is_adc12ifg16(&self) -> bool {
        *self == Adc12iv::Adc12ifg16
    }
    #[doc = "Interrupt Source: ADC12MEM17 interrupt flag, Interrupt Flag: ADC12IFG17"]
    #[inline(always)]
    pub fn is_adc12ifg17(&self) -> bool {
        *self == Adc12iv::Adc12ifg17
    }
    #[doc = "Interrupt Source: ADC12MEM18 interrupt flag, Interrupt Flag: ADC12IFG18"]
    #[inline(always)]
    pub fn is_adc12ifg18(&self) -> bool {
        *self == Adc12iv::Adc12ifg18
    }
    #[doc = "Interrupt Source: ADC12MEM19 interrupt flag, Interrupt Flag: ADC12IFG19"]
    #[inline(always)]
    pub fn is_adc12ifg19(&self) -> bool {
        *self == Adc12iv::Adc12ifg19
    }
    #[doc = "Interrupt Source: ADC12MEM20 interrupt flag, Interrupt Flag: ADC12IFG20"]
    #[inline(always)]
    pub fn is_adc12ifg20(&self) -> bool {
        *self == Adc12iv::Adc12ifg20
    }
    #[doc = "Interrupt Source: ADC12MEM21 interrupt flag, Interrupt Flag: ADC12IFG21"]
    #[inline(always)]
    pub fn is_adc12ifg21(&self) -> bool {
        *self == Adc12iv::Adc12ifg21
    }
    #[doc = "Interrupt Source: ADC12MEM22 interrupt flag, Interrupt Flag: ADC12IFG22"]
    #[inline(always)]
    pub fn is_adc12ifg22(&self) -> bool {
        *self == Adc12iv::Adc12ifg22
    }
    #[doc = "Interrupt Source: ADC12MEM23 interrupt flag, Interrupt Flag: ADC12IFG23"]
    #[inline(always)]
    pub fn is_adc12ifg23(&self) -> bool {
        *self == Adc12iv::Adc12ifg23
    }
    #[doc = "Interrupt Source: ADC12MEM24 interrupt flag, Interrupt Flag: ADC12IFG24"]
    #[inline(always)]
    pub fn is_adc12ifg24(&self) -> bool {
        *self == Adc12iv::Adc12ifg24
    }
    #[doc = "Interrupt Source: ADC12MEM25 interrupt flag, Interrupt Flag: ADC12IFG25"]
    #[inline(always)]
    pub fn is_adc12ifg25(&self) -> bool {
        *self == Adc12iv::Adc12ifg25
    }
    #[doc = "Interrupt Source: ADC12MEM26 interrupt flag, Interrupt Flag: ADC12IFG26"]
    #[inline(always)]
    pub fn is_adc12ifg26(&self) -> bool {
        *self == Adc12iv::Adc12ifg26
    }
    #[doc = "Interrupt Source: ADC12MEM27 interrupt flag, Interrupt Flag: ADC12IFG27"]
    #[inline(always)]
    pub fn is_adc12ifg27(&self) -> bool {
        *self == Adc12iv::Adc12ifg27
    }
    #[doc = "Interrupt Source: ADC12MEM28 interrupt flag, Interrupt Flag: ADC12IFG28"]
    #[inline(always)]
    pub fn is_adc12ifg28(&self) -> bool {
        *self == Adc12iv::Adc12ifg28
    }
    #[doc = "Interrupt Source: ADC12MEM29 interrupt flag, Interrupt Flag: ADC12IFG29"]
    #[inline(always)]
    pub fn is_adc12ifg29(&self) -> bool {
        *self == Adc12iv::Adc12ifg29
    }
    #[doc = "Interrupt Source: ADC12MEM30 interrupt flag, Interrupt Flag: ADC12IFG30"]
    #[inline(always)]
    pub fn is_adc12ifg30(&self) -> bool {
        *self == Adc12iv::Adc12ifg30
    }
    #[doc = "Interrupt Source: ADC12MEM31 interrupt flag, Interrupt Flag: ADC12IFG31"]
    #[inline(always)]
    pub fn is_adc12ifg31(&self) -> bool {
        *self == Adc12iv::Adc12ifg31
    }
    #[doc = "Interrupt Source: ADC12RDYIFG interrupt flag, Interrupt Flag: ADC12RDYIFG"]
    #[inline(always)]
    pub fn is_adc12rdyifg(&self) -> bool {
        *self == Adc12iv::Adc12rdyifg
    }
}
#[doc = "Field `ADC12IV` writer - interrupt vector value"]
pub type Adc12ivW<'a, REG> = crate::FieldWriter<'a, REG, 16, Adc12iv>;
impl<'a, REG> Adc12ivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Interrupt Source: No interrupt pending, Interrupt Flag: None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::None)
    }
    #[doc = "Interrupt Source: ADC12MEMx overflow, Interrupt Flag: ADC12OVIFG, Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn adc12ovifg(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ovifg)
    }
    #[doc = "Interrupt Source: Conversion time overflow, Interrupt Flag: ADC12TOVIFG"]
    #[inline(always)]
    pub fn adc12tovifg(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12tovifg)
    }
    #[doc = "Interrupt Source: ADC12 window high interrupt flag, Interrupt Flag: ADC12HIIFG"]
    #[inline(always)]
    pub fn adc12hiifg(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12hiifg)
    }
    #[doc = "Interrupt Source: ADC12 window low interrupt flag, Interrupt Flag: ADC12LOIFG"]
    #[inline(always)]
    pub fn adc12loifg(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12loifg)
    }
    #[doc = "Interrupt Source: ADC12 in-window interrupt flag, Interrupt Flag: ADC12INIFG"]
    #[inline(always)]
    pub fn adc12inifg(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12inifg)
    }
    #[doc = "Interrupt Source: ADC12MEM0 interrupt flag, Interrupt Flag: ADC12IFG0"]
    #[inline(always)]
    pub fn adc12ifg0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg0)
    }
    #[doc = "Interrupt Source: ADC12MEM1 interrupt flag, Interrupt Flag: ADC12IFG1"]
    #[inline(always)]
    pub fn adc12ifg1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg1)
    }
    #[doc = "Interrupt Source: ADC12MEM2 interrupt flag, Interrupt Flag: ADC12IFG2"]
    #[inline(always)]
    pub fn adc12ifg2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg2)
    }
    #[doc = "Interrupt Source: ADC12MEM3 interrupt flag, Interrupt Flag: ADC12IFG3"]
    #[inline(always)]
    pub fn adc12ifg3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg3)
    }
    #[doc = "Interrupt Source: ADC12MEM4 interrupt flag, Interrupt Flag: ADC12IFG4"]
    #[inline(always)]
    pub fn adc12ifg4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg4)
    }
    #[doc = "Interrupt Source: ADC12MEM5 interrupt flag, Interrupt Flag: ADC12IFG5"]
    #[inline(always)]
    pub fn adc12ifg5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg5)
    }
    #[doc = "Interrupt Source: ADC12MEM6 interrupt flag, Interrupt Flag: ADC12IFG6"]
    #[inline(always)]
    pub fn adc12ifg6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg6)
    }
    #[doc = "Interrupt Source: ADC12MEM7 interrupt flag, Interrupt Flag: ADC12IFG7"]
    #[inline(always)]
    pub fn adc12ifg7(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg7)
    }
    #[doc = "Interrupt Source: ADC12MEM8 interrupt flag, Interrupt Flag: ADC12IFG8"]
    #[inline(always)]
    pub fn adc12ifg8(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg8)
    }
    #[doc = "Interrupt Source: ADC12MEM9 interrupt flag, Interrupt Flag: ADC12IFG9"]
    #[inline(always)]
    pub fn adc12ifg9(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg9)
    }
    #[doc = "Interrupt Source: ADC12MEM10 interrupt flag, Interrupt Flag: ADC12IFG10"]
    #[inline(always)]
    pub fn adc12ifg10(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg10)
    }
    #[doc = "Interrupt Source: ADC12MEM11 interrupt flag, Interrupt Flag: ADC12IFG11"]
    #[inline(always)]
    pub fn adc12ifg11(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg11)
    }
    #[doc = "Interrupt Source: ADC12MEM12 interrupt flag, Interrupt Flag: ADC12IFG12"]
    #[inline(always)]
    pub fn adc12ifg12(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg12)
    }
    #[doc = "Interrupt Source: ADC12MEM13 interrupt flag, Interrupt Flag: ADC12IFG13"]
    #[inline(always)]
    pub fn adc12ifg13(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg13)
    }
    #[doc = "Interrupt Source: ADC12MEM14 interrupt flag, Interrupt Flag: ADC12IFG14"]
    #[inline(always)]
    pub fn adc12ifg14(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg14)
    }
    #[doc = "Interrupt Source: ADC12MEM15 interrupt flag, Interrupt Flag: ADC12IFG15"]
    #[inline(always)]
    pub fn adc12ifg15(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg15)
    }
    #[doc = "Interrupt Source: ADC12MEM16 interrupt flag, Interrupt Flag: ADC12IFG16"]
    #[inline(always)]
    pub fn adc12ifg16(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg16)
    }
    #[doc = "Interrupt Source: ADC12MEM17 interrupt flag, Interrupt Flag: ADC12IFG17"]
    #[inline(always)]
    pub fn adc12ifg17(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg17)
    }
    #[doc = "Interrupt Source: ADC12MEM18 interrupt flag, Interrupt Flag: ADC12IFG18"]
    #[inline(always)]
    pub fn adc12ifg18(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg18)
    }
    #[doc = "Interrupt Source: ADC12MEM19 interrupt flag, Interrupt Flag: ADC12IFG19"]
    #[inline(always)]
    pub fn adc12ifg19(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg19)
    }
    #[doc = "Interrupt Source: ADC12MEM20 interrupt flag, Interrupt Flag: ADC12IFG20"]
    #[inline(always)]
    pub fn adc12ifg20(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg20)
    }
    #[doc = "Interrupt Source: ADC12MEM21 interrupt flag, Interrupt Flag: ADC12IFG21"]
    #[inline(always)]
    pub fn adc12ifg21(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg21)
    }
    #[doc = "Interrupt Source: ADC12MEM22 interrupt flag, Interrupt Flag: ADC12IFG22"]
    #[inline(always)]
    pub fn adc12ifg22(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg22)
    }
    #[doc = "Interrupt Source: ADC12MEM23 interrupt flag, Interrupt Flag: ADC12IFG23"]
    #[inline(always)]
    pub fn adc12ifg23(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg23)
    }
    #[doc = "Interrupt Source: ADC12MEM24 interrupt flag, Interrupt Flag: ADC12IFG24"]
    #[inline(always)]
    pub fn adc12ifg24(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg24)
    }
    #[doc = "Interrupt Source: ADC12MEM25 interrupt flag, Interrupt Flag: ADC12IFG25"]
    #[inline(always)]
    pub fn adc12ifg25(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg25)
    }
    #[doc = "Interrupt Source: ADC12MEM26 interrupt flag, Interrupt Flag: ADC12IFG26"]
    #[inline(always)]
    pub fn adc12ifg26(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg26)
    }
    #[doc = "Interrupt Source: ADC12MEM27 interrupt flag, Interrupt Flag: ADC12IFG27"]
    #[inline(always)]
    pub fn adc12ifg27(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg27)
    }
    #[doc = "Interrupt Source: ADC12MEM28 interrupt flag, Interrupt Flag: ADC12IFG28"]
    #[inline(always)]
    pub fn adc12ifg28(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg28)
    }
    #[doc = "Interrupt Source: ADC12MEM29 interrupt flag, Interrupt Flag: ADC12IFG29"]
    #[inline(always)]
    pub fn adc12ifg29(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg29)
    }
    #[doc = "Interrupt Source: ADC12MEM30 interrupt flag, Interrupt Flag: ADC12IFG30"]
    #[inline(always)]
    pub fn adc12ifg30(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg30)
    }
    #[doc = "Interrupt Source: ADC12MEM31 interrupt flag, Interrupt Flag: ADC12IFG31"]
    #[inline(always)]
    pub fn adc12ifg31(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12ifg31)
    }
    #[doc = "Interrupt Source: ADC12RDYIFG interrupt flag, Interrupt Flag: ADC12RDYIFG"]
    #[inline(always)]
    pub fn adc12rdyifg(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12iv::Adc12rdyifg)
    }
}
impl R {
    #[doc = "Bits 0:15 - interrupt vector value"]
    #[inline(always)]
    pub fn adc12iv(&self) -> Adc12ivR {
        Adc12ivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - interrupt vector value"]
    #[inline(always)]
    pub fn adc12iv(&mut self) -> Adc12ivW<'_, Adc12ivSpec> {
        Adc12ivW::new(self, 0)
    }
}
#[doc = "ADC12_B Interrupt Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`adc12iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc12iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc12ivSpec;
impl crate::RegisterSpec for Adc12ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12iv::R`](R) reader structure"]
impl crate::Readable for Adc12ivSpec {}
#[doc = "`write(|w| ..)` method takes [`adc12iv::W`](W) writer structure"]
impl crate::Writable for Adc12ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC12IV to value 0"]
impl crate::Resettable for Adc12ivSpec {}
