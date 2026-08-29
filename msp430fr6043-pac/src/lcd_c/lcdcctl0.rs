#[doc = "Register `LCDCCTL0` reader"]
pub type R = crate::R<Lcdcctl0Spec>;
#[doc = "Register `LCDCCTL0` writer"]
pub type W = crate::W<Lcdcctl0Spec>;
#[doc = "LCD on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdon {
    #[doc = "0: LCD_C module off"]
    Off = 0,
    #[doc = "1: LCD_C module on"]
    On = 1,
}
impl From<Lcdon> for bool {
    #[inline(always)]
    fn from(variant: Lcdon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDON` reader - LCD on"]
pub type LcdonR = crate::BitReader<Lcdon>;
impl LcdonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdon {
        match self.bits {
            false => Lcdon::Off,
            true => Lcdon::On,
        }
    }
    #[doc = "LCD_C module off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Lcdon::Off
    }
    #[doc = "LCD_C module on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Lcdon::On
    }
}
#[doc = "Field `LCDON` writer - LCD on"]
pub type LcdonW<'a, REG> = crate::BitWriter<'a, REG, Lcdon>;
impl<'a, REG> LcdonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD_C module off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdon::Off)
    }
    #[doc = "LCD_C module on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdon::On)
    }
}
#[doc = "LCD low-power waveform\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdlp {
    #[doc = "0: Standard LCD waveforms on segment and common lines selected"]
    Lcdlp0 = 0,
    #[doc = "1: Low-power LCD waveforms on segment and common lines selected"]
    Lcdlp1 = 1,
}
impl From<Lcdlp> for bool {
    #[inline(always)]
    fn from(variant: Lcdlp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDLP` reader - LCD low-power waveform"]
pub type LcdlpR = crate::BitReader<Lcdlp>;
impl LcdlpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdlp {
        match self.bits {
            false => Lcdlp::Lcdlp0,
            true => Lcdlp::Lcdlp1,
        }
    }
    #[doc = "Standard LCD waveforms on segment and common lines selected"]
    #[inline(always)]
    pub fn is_lcdlp_0(&self) -> bool {
        *self == Lcdlp::Lcdlp0
    }
    #[doc = "Low-power LCD waveforms on segment and common lines selected"]
    #[inline(always)]
    pub fn is_lcdlp_1(&self) -> bool {
        *self == Lcdlp::Lcdlp1
    }
}
#[doc = "Field `LCDLP` writer - LCD low-power waveform"]
pub type LcdlpW<'a, REG> = crate::BitWriter<'a, REG, Lcdlp>;
impl<'a, REG> LcdlpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard LCD waveforms on segment and common lines selected"]
    #[inline(always)]
    pub fn lcdlp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdlp::Lcdlp0)
    }
    #[doc = "Low-power LCD waveforms on segment and common lines selected"]
    #[inline(always)]
    pub fn lcdlp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdlp::Lcdlp1)
    }
}
#[doc = "LCD segments on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdson {
    #[doc = "0: All LCD segments are off"]
    Off = 0,
    #[doc = "1: All LCD segments are enabled and on or off according to their corresponding memory location"]
    On = 1,
}
impl From<Lcdson> for bool {
    #[inline(always)]
    fn from(variant: Lcdson) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDSON` reader - LCD segments on"]
pub type LcdsonR = crate::BitReader<Lcdson>;
impl LcdsonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdson {
        match self.bits {
            false => Lcdson::Off,
            true => Lcdson::On,
        }
    }
    #[doc = "All LCD segments are off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Lcdson::Off
    }
    #[doc = "All LCD segments are enabled and on or off according to their corresponding memory location"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Lcdson::On
    }
}
#[doc = "Field `LCDSON` writer - LCD segments on"]
pub type LcdsonW<'a, REG> = crate::BitWriter<'a, REG, Lcdson>;
impl<'a, REG> LcdsonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All LCD segments are off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdson::Off)
    }
    #[doc = "All LCD segments are enabled and on or off according to their corresponding memory location"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdson::On)
    }
}
#[doc = "LCD mux rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcdmx {
    #[doc = "0: Static"]
    Static = 0,
    #[doc = "1: 2-mux"]
    _2mux = 1,
    #[doc = "2: 3-mux"]
    _3mux = 2,
    #[doc = "3: 4-mux"]
    _4mux = 3,
    #[doc = "4: 5-mux"]
    _5mux = 4,
    #[doc = "5: 6-mux"]
    _6mux = 5,
    #[doc = "6: 7-mux"]
    _7mux = 6,
    #[doc = "7: 8-mux"]
    _8mux = 7,
}
impl From<Lcdmx> for u8 {
    #[inline(always)]
    fn from(variant: Lcdmx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcdmx {
    type Ux = u8;
}
impl crate::IsEnum for Lcdmx {}
#[doc = "Field `LCDMX` reader - LCD mux rate"]
pub type LcdmxR = crate::FieldReader<Lcdmx>;
impl LcdmxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdmx {
        match self.bits {
            0 => Lcdmx::Static,
            1 => Lcdmx::_2mux,
            2 => Lcdmx::_3mux,
            3 => Lcdmx::_4mux,
            4 => Lcdmx::_5mux,
            5 => Lcdmx::_6mux,
            6 => Lcdmx::_7mux,
            7 => Lcdmx::_8mux,
            _ => unreachable!(),
        }
    }
    #[doc = "Static"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == Lcdmx::Static
    }
    #[doc = "2-mux"]
    #[inline(always)]
    pub fn is_2mux(&self) -> bool {
        *self == Lcdmx::_2mux
    }
    #[doc = "3-mux"]
    #[inline(always)]
    pub fn is_3mux(&self) -> bool {
        *self == Lcdmx::_3mux
    }
    #[doc = "4-mux"]
    #[inline(always)]
    pub fn is_4mux(&self) -> bool {
        *self == Lcdmx::_4mux
    }
    #[doc = "5-mux"]
    #[inline(always)]
    pub fn is_5mux(&self) -> bool {
        *self == Lcdmx::_5mux
    }
    #[doc = "6-mux"]
    #[inline(always)]
    pub fn is_6mux(&self) -> bool {
        *self == Lcdmx::_6mux
    }
    #[doc = "7-mux"]
    #[inline(always)]
    pub fn is_7mux(&self) -> bool {
        *self == Lcdmx::_7mux
    }
    #[doc = "8-mux"]
    #[inline(always)]
    pub fn is_8mux(&self) -> bool {
        *self == Lcdmx::_8mux
    }
}
#[doc = "Field `LCDMX` writer - LCD mux rate"]
pub type LcdmxW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lcdmx, crate::Safe>;
impl<'a, REG> LcdmxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Static"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmx::Static)
    }
    #[doc = "2-mux"]
    #[inline(always)]
    pub fn _2mux(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmx::_2mux)
    }
    #[doc = "3-mux"]
    #[inline(always)]
    pub fn _3mux(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmx::_3mux)
    }
    #[doc = "4-mux"]
    #[inline(always)]
    pub fn _4mux(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmx::_4mux)
    }
    #[doc = "5-mux"]
    #[inline(always)]
    pub fn _5mux(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmx::_5mux)
    }
    #[doc = "6-mux"]
    #[inline(always)]
    pub fn _6mux(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmx::_6mux)
    }
    #[doc = "7-mux"]
    #[inline(always)]
    pub fn _7mux(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmx::_7mux)
    }
    #[doc = "8-mux"]
    #[inline(always)]
    pub fn _8mux(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdmx::_8mux)
    }
}
#[doc = "Clock source select for LCD and blinking frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdssel {
    #[doc = "0: ACLK (30 kHz to 40 kHz)"]
    Aclk = 0,
    #[doc = "1: VLOCLK"]
    Vloclk = 1,
}
impl From<Lcdssel> for bool {
    #[inline(always)]
    fn from(variant: Lcdssel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDSSEL` reader - Clock source select for LCD and blinking frequency"]
pub type LcdsselR = crate::BitReader<Lcdssel>;
impl LcdsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdssel {
        match self.bits {
            false => Lcdssel::Aclk,
            true => Lcdssel::Vloclk,
        }
    }
    #[doc = "ACLK (30 kHz to 40 kHz)"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == Lcdssel::Aclk
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == Lcdssel::Vloclk
    }
}
#[doc = "Field `LCDSSEL` writer - Clock source select for LCD and blinking frequency"]
pub type LcdsselW<'a, REG> = crate::BitWriter<'a, REG, Lcdssel>;
impl<'a, REG> LcdsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ACLK (30 kHz to 40 kHz)"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdssel::Aclk)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdssel::Vloclk)
    }
}
#[doc = "LCD frequency pre-scaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcdpre {
    #[doc = "0: Divide by 1"]
    _1 = 0,
    #[doc = "1: Divide by 2"]
    _2 = 1,
    #[doc = "2: Divide by 4"]
    _4 = 2,
    #[doc = "3: Divide by 8"]
    _8 = 3,
    #[doc = "4: Divide by 16"]
    _16 = 4,
    #[doc = "5: Divide by 32"]
    _32 = 5,
}
impl From<Lcdpre> for u8 {
    #[inline(always)]
    fn from(variant: Lcdpre) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcdpre {
    type Ux = u8;
}
impl crate::IsEnum for Lcdpre {}
#[doc = "Field `LCDPRE` reader - LCD frequency pre-scaler"]
pub type LcdpreR = crate::FieldReader<Lcdpre>;
impl LcdpreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lcdpre> {
        match self.bits {
            0 => Some(Lcdpre::_1),
            1 => Some(Lcdpre::_2),
            2 => Some(Lcdpre::_4),
            3 => Some(Lcdpre::_8),
            4 => Some(Lcdpre::_16),
            5 => Some(Lcdpre::_32),
            _ => None,
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lcdpre::_1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Lcdpre::_2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Lcdpre::_4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Lcdpre::_8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Lcdpre::_16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Lcdpre::_32
    }
}
#[doc = "Field `LCDPRE` writer - LCD frequency pre-scaler"]
pub type LcdpreW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lcdpre>;
impl<'a, REG> LcdpreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdpre::_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdpre::_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdpre::_4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdpre::_8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdpre::_16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdpre::_32)
    }
}
#[doc = "LCD frequency divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcddiv {
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
    #[doc = "8: Divide by 9"]
    _9 = 8,
    #[doc = "9: Divide by 10"]
    _10 = 9,
    #[doc = "10: Divide by 11"]
    _11 = 10,
    #[doc = "11: Divide by 12"]
    _12 = 11,
    #[doc = "12: Divide by 13"]
    _13 = 12,
    #[doc = "13: Divide by 14"]
    _14 = 13,
    #[doc = "14: Divide by 15"]
    _15 = 14,
    #[doc = "15: Divide by 16"]
    _16 = 15,
    #[doc = "16: Divide by 17"]
    _17 = 16,
    #[doc = "17: Divide by 18"]
    _18 = 17,
    #[doc = "18: Divide by 19"]
    _19 = 18,
    #[doc = "19: Divide by 20"]
    _20 = 19,
    #[doc = "20: Divide by 21"]
    _21 = 20,
    #[doc = "21: Divide by 22"]
    _22 = 21,
    #[doc = "22: Divide by 23"]
    _23 = 22,
    #[doc = "23: Divide by 24"]
    _24 = 23,
    #[doc = "24: Divide by 25"]
    _25 = 24,
    #[doc = "25: Divide by 26"]
    _26 = 25,
    #[doc = "26: Divide by 27"]
    _27 = 26,
    #[doc = "27: Divide by 28"]
    _28 = 27,
    #[doc = "28: Divide by 29"]
    _29 = 28,
    #[doc = "29: Divide by 30"]
    _30 = 29,
    #[doc = "30: Divide by 31"]
    _31 = 30,
    #[doc = "31: Divide by 32"]
    _32 = 31,
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
#[doc = "Field `LCDDIV` reader - LCD frequency divider"]
pub type LcddivR = crate::FieldReader<Lcddiv>;
impl LcddivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcddiv {
        match self.bits {
            0 => Lcddiv::_1,
            1 => Lcddiv::_2,
            2 => Lcddiv::_3,
            3 => Lcddiv::_4,
            4 => Lcddiv::_5,
            5 => Lcddiv::_6,
            6 => Lcddiv::_7,
            7 => Lcddiv::_8,
            8 => Lcddiv::_9,
            9 => Lcddiv::_10,
            10 => Lcddiv::_11,
            11 => Lcddiv::_12,
            12 => Lcddiv::_13,
            13 => Lcddiv::_14,
            14 => Lcddiv::_15,
            15 => Lcddiv::_16,
            16 => Lcddiv::_17,
            17 => Lcddiv::_18,
            18 => Lcddiv::_19,
            19 => Lcddiv::_20,
            20 => Lcddiv::_21,
            21 => Lcddiv::_22,
            22 => Lcddiv::_23,
            23 => Lcddiv::_24,
            24 => Lcddiv::_25,
            25 => Lcddiv::_26,
            26 => Lcddiv::_27,
            27 => Lcddiv::_28,
            28 => Lcddiv::_29,
            29 => Lcddiv::_30,
            30 => Lcddiv::_31,
            31 => Lcddiv::_32,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lcddiv::_1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Lcddiv::_2
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == Lcddiv::_3
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Lcddiv::_4
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == Lcddiv::_5
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == Lcddiv::_6
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == Lcddiv::_7
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Lcddiv::_8
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        *self == Lcddiv::_9
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Lcddiv::_10
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Lcddiv::_11
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        *self == Lcddiv::_12
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        *self == Lcddiv::_13
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        *self == Lcddiv::_14
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        *self == Lcddiv::_15
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Lcddiv::_16
    }
    #[doc = "Divide by 17"]
    #[inline(always)]
    pub fn is_17(&self) -> bool {
        *self == Lcddiv::_17
    }
    #[doc = "Divide by 18"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        *self == Lcddiv::_18
    }
    #[doc = "Divide by 19"]
    #[inline(always)]
    pub fn is_19(&self) -> bool {
        *self == Lcddiv::_19
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == Lcddiv::_20
    }
    #[doc = "Divide by 21"]
    #[inline(always)]
    pub fn is_21(&self) -> bool {
        *self == Lcddiv::_21
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn is_22(&self) -> bool {
        *self == Lcddiv::_22
    }
    #[doc = "Divide by 23"]
    #[inline(always)]
    pub fn is_23(&self) -> bool {
        *self == Lcddiv::_23
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == Lcddiv::_24
    }
    #[doc = "Divide by 25"]
    #[inline(always)]
    pub fn is_25(&self) -> bool {
        *self == Lcddiv::_25
    }
    #[doc = "Divide by 26"]
    #[inline(always)]
    pub fn is_26(&self) -> bool {
        *self == Lcddiv::_26
    }
    #[doc = "Divide by 27"]
    #[inline(always)]
    pub fn is_27(&self) -> bool {
        *self == Lcddiv::_27
    }
    #[doc = "Divide by 28"]
    #[inline(always)]
    pub fn is_28(&self) -> bool {
        *self == Lcddiv::_28
    }
    #[doc = "Divide by 29"]
    #[inline(always)]
    pub fn is_29(&self) -> bool {
        *self == Lcddiv::_29
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn is_30(&self) -> bool {
        *self == Lcddiv::_30
    }
    #[doc = "Divide by 31"]
    #[inline(always)]
    pub fn is_31(&self) -> bool {
        *self == Lcddiv::_31
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Lcddiv::_32
    }
}
#[doc = "Field `LCDDIV` writer - LCD frequency divider"]
pub type LcddivW<'a, REG> = crate::FieldWriter<'a, REG, 5, Lcddiv, crate::Safe>;
impl<'a, REG> LcddivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_8)
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_9)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_10)
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_11)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_12)
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_13)
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_14)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_15)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_16)
    }
    #[doc = "Divide by 17"]
    #[inline(always)]
    pub fn _17(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_17)
    }
    #[doc = "Divide by 18"]
    #[inline(always)]
    pub fn _18(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_18)
    }
    #[doc = "Divide by 19"]
    #[inline(always)]
    pub fn _19(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_19)
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_20)
    }
    #[doc = "Divide by 21"]
    #[inline(always)]
    pub fn _21(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_21)
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn _22(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_22)
    }
    #[doc = "Divide by 23"]
    #[inline(always)]
    pub fn _23(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_23)
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_24)
    }
    #[doc = "Divide by 25"]
    #[inline(always)]
    pub fn _25(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_25)
    }
    #[doc = "Divide by 26"]
    #[inline(always)]
    pub fn _26(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_26)
    }
    #[doc = "Divide by 27"]
    #[inline(always)]
    pub fn _27(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_27)
    }
    #[doc = "Divide by 28"]
    #[inline(always)]
    pub fn _28(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_28)
    }
    #[doc = "Divide by 29"]
    #[inline(always)]
    pub fn _29(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_29)
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn _30(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_30)
    }
    #[doc = "Divide by 31"]
    #[inline(always)]
    pub fn _31(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_31)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddiv::_32)
    }
}
impl R {
    #[doc = "Bit 0 - LCD on"]
    #[inline(always)]
    pub fn lcdon(&self) -> LcdonR {
        LcdonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD low-power waveform"]
    #[inline(always)]
    pub fn lcdlp(&self) -> LcdlpR {
        LcdlpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD segments on"]
    #[inline(always)]
    pub fn lcdson(&self) -> LcdsonR {
        LcdsonR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - LCD mux rate"]
    #[inline(always)]
    pub fn lcdmx(&self) -> LcdmxR {
        LcdmxR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - Clock source select for LCD and blinking frequency"]
    #[inline(always)]
    pub fn lcdssel(&self) -> LcdsselR {
        LcdsselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - LCD frequency pre-scaler"]
    #[inline(always)]
    pub fn lcdpre(&self) -> LcdpreR {
        LcdpreR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - LCD frequency divider"]
    #[inline(always)]
    pub fn lcddiv(&self) -> LcddivR {
        LcddivR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LCD on"]
    #[inline(always)]
    pub fn lcdon(&mut self) -> LcdonW<'_, Lcdcctl0Spec> {
        LcdonW::new(self, 0)
    }
    #[doc = "Bit 1 - LCD low-power waveform"]
    #[inline(always)]
    pub fn lcdlp(&mut self) -> LcdlpW<'_, Lcdcctl0Spec> {
        LcdlpW::new(self, 1)
    }
    #[doc = "Bit 2 - LCD segments on"]
    #[inline(always)]
    pub fn lcdson(&mut self) -> LcdsonW<'_, Lcdcctl0Spec> {
        LcdsonW::new(self, 2)
    }
    #[doc = "Bits 3:5 - LCD mux rate"]
    #[inline(always)]
    pub fn lcdmx(&mut self) -> LcdmxW<'_, Lcdcctl0Spec> {
        LcdmxW::new(self, 3)
    }
    #[doc = "Bit 7 - Clock source select for LCD and blinking frequency"]
    #[inline(always)]
    pub fn lcdssel(&mut self) -> LcdsselW<'_, Lcdcctl0Spec> {
        LcdsselW::new(self, 7)
    }
    #[doc = "Bits 8:10 - LCD frequency pre-scaler"]
    #[inline(always)]
    pub fn lcdpre(&mut self) -> LcdpreW<'_, Lcdcctl0Spec> {
        LcdpreW::new(self, 8)
    }
    #[doc = "Bits 11:15 - LCD frequency divider"]
    #[inline(always)]
    pub fn lcddiv(&mut self) -> LcddivW<'_, Lcdcctl0Spec> {
        LcddivW::new(self, 11)
    }
}
#[doc = "LCD_C control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdcctl0Spec;
impl crate::RegisterSpec for Lcdcctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdcctl0::R`](R) reader structure"]
impl crate::Readable for Lcdcctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdcctl0::W`](W) writer structure"]
impl crate::Writable for Lcdcctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDCCTL0 to value 0"]
impl crate::Resettable for Lcdcctl0Spec {}
