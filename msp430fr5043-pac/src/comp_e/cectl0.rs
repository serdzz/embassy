#[doc = "Register `CECTL0` reader"]
pub type R = crate::R<Cectl0Spec>;
#[doc = "Register `CECTL0` writer"]
pub type W = crate::W<Cectl0Spec>;
#[doc = "Channel input selected for the V+ terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ceipsel {
    #[doc = "0: Channel 0 selected"]
    Ceipsel0 = 0,
    #[doc = "1: Channel 1 selected"]
    Ceipsel1 = 1,
    #[doc = "2: Channel 2 selected"]
    Ceipsel2 = 2,
    #[doc = "3: Channel 3 selected"]
    Ceipsel3 = 3,
    #[doc = "4: Channel 4 selected"]
    Ceipsel4 = 4,
    #[doc = "5: Channel 5 selected"]
    Ceipsel5 = 5,
    #[doc = "6: Channel 6 selected"]
    Ceipsel6 = 6,
    #[doc = "7: Channel 7 selected"]
    Ceipsel7 = 7,
    #[doc = "8: Channel 8 selected"]
    Ceipsel8 = 8,
    #[doc = "9: Channel 9 selected"]
    Ceipsel9 = 9,
    #[doc = "10: Channel 10 selected"]
    Ceipsel10 = 10,
    #[doc = "11: Channel 11 selected"]
    Ceipsel11 = 11,
    #[doc = "12: Channel 12 selected"]
    Ceipsel12 = 12,
    #[doc = "13: Channel 13 selected"]
    Ceipsel13 = 13,
    #[doc = "14: Channel 14 selected"]
    Ceipsel14 = 14,
    #[doc = "15: Channel 15 selected"]
    Ceipsel15 = 15,
}
impl From<Ceipsel> for u8 {
    #[inline(always)]
    fn from(variant: Ceipsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ceipsel {
    type Ux = u8;
}
impl crate::IsEnum for Ceipsel {}
#[doc = "Field `CEIPSEL` reader - Channel input selected for the V+ terminal"]
pub type CeipselR = crate::FieldReader<Ceipsel>;
impl CeipselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceipsel {
        match self.bits {
            0 => Ceipsel::Ceipsel0,
            1 => Ceipsel::Ceipsel1,
            2 => Ceipsel::Ceipsel2,
            3 => Ceipsel::Ceipsel3,
            4 => Ceipsel::Ceipsel4,
            5 => Ceipsel::Ceipsel5,
            6 => Ceipsel::Ceipsel6,
            7 => Ceipsel::Ceipsel7,
            8 => Ceipsel::Ceipsel8,
            9 => Ceipsel::Ceipsel9,
            10 => Ceipsel::Ceipsel10,
            11 => Ceipsel::Ceipsel11,
            12 => Ceipsel::Ceipsel12,
            13 => Ceipsel::Ceipsel13,
            14 => Ceipsel::Ceipsel14,
            15 => Ceipsel::Ceipsel15,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 0 selected"]
    #[inline(always)]
    pub fn is_ceipsel_0(&self) -> bool {
        *self == Ceipsel::Ceipsel0
    }
    #[doc = "Channel 1 selected"]
    #[inline(always)]
    pub fn is_ceipsel_1(&self) -> bool {
        *self == Ceipsel::Ceipsel1
    }
    #[doc = "Channel 2 selected"]
    #[inline(always)]
    pub fn is_ceipsel_2(&self) -> bool {
        *self == Ceipsel::Ceipsel2
    }
    #[doc = "Channel 3 selected"]
    #[inline(always)]
    pub fn is_ceipsel_3(&self) -> bool {
        *self == Ceipsel::Ceipsel3
    }
    #[doc = "Channel 4 selected"]
    #[inline(always)]
    pub fn is_ceipsel_4(&self) -> bool {
        *self == Ceipsel::Ceipsel4
    }
    #[doc = "Channel 5 selected"]
    #[inline(always)]
    pub fn is_ceipsel_5(&self) -> bool {
        *self == Ceipsel::Ceipsel5
    }
    #[doc = "Channel 6 selected"]
    #[inline(always)]
    pub fn is_ceipsel_6(&self) -> bool {
        *self == Ceipsel::Ceipsel6
    }
    #[doc = "Channel 7 selected"]
    #[inline(always)]
    pub fn is_ceipsel_7(&self) -> bool {
        *self == Ceipsel::Ceipsel7
    }
    #[doc = "Channel 8 selected"]
    #[inline(always)]
    pub fn is_ceipsel_8(&self) -> bool {
        *self == Ceipsel::Ceipsel8
    }
    #[doc = "Channel 9 selected"]
    #[inline(always)]
    pub fn is_ceipsel_9(&self) -> bool {
        *self == Ceipsel::Ceipsel9
    }
    #[doc = "Channel 10 selected"]
    #[inline(always)]
    pub fn is_ceipsel_10(&self) -> bool {
        *self == Ceipsel::Ceipsel10
    }
    #[doc = "Channel 11 selected"]
    #[inline(always)]
    pub fn is_ceipsel_11(&self) -> bool {
        *self == Ceipsel::Ceipsel11
    }
    #[doc = "Channel 12 selected"]
    #[inline(always)]
    pub fn is_ceipsel_12(&self) -> bool {
        *self == Ceipsel::Ceipsel12
    }
    #[doc = "Channel 13 selected"]
    #[inline(always)]
    pub fn is_ceipsel_13(&self) -> bool {
        *self == Ceipsel::Ceipsel13
    }
    #[doc = "Channel 14 selected"]
    #[inline(always)]
    pub fn is_ceipsel_14(&self) -> bool {
        *self == Ceipsel::Ceipsel14
    }
    #[doc = "Channel 15 selected"]
    #[inline(always)]
    pub fn is_ceipsel_15(&self) -> bool {
        *self == Ceipsel::Ceipsel15
    }
}
#[doc = "Field `CEIPSEL` writer - Channel input selected for the V+ terminal"]
pub type CeipselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ceipsel, crate::Safe>;
impl<'a, REG> CeipselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 selected"]
    #[inline(always)]
    pub fn ceipsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel0)
    }
    #[doc = "Channel 1 selected"]
    #[inline(always)]
    pub fn ceipsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel1)
    }
    #[doc = "Channel 2 selected"]
    #[inline(always)]
    pub fn ceipsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel2)
    }
    #[doc = "Channel 3 selected"]
    #[inline(always)]
    pub fn ceipsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel3)
    }
    #[doc = "Channel 4 selected"]
    #[inline(always)]
    pub fn ceipsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel4)
    }
    #[doc = "Channel 5 selected"]
    #[inline(always)]
    pub fn ceipsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel5)
    }
    #[doc = "Channel 6 selected"]
    #[inline(always)]
    pub fn ceipsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel6)
    }
    #[doc = "Channel 7 selected"]
    #[inline(always)]
    pub fn ceipsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel7)
    }
    #[doc = "Channel 8 selected"]
    #[inline(always)]
    pub fn ceipsel_8(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel8)
    }
    #[doc = "Channel 9 selected"]
    #[inline(always)]
    pub fn ceipsel_9(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel9)
    }
    #[doc = "Channel 10 selected"]
    #[inline(always)]
    pub fn ceipsel_10(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel10)
    }
    #[doc = "Channel 11 selected"]
    #[inline(always)]
    pub fn ceipsel_11(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel11)
    }
    #[doc = "Channel 12 selected"]
    #[inline(always)]
    pub fn ceipsel_12(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel12)
    }
    #[doc = "Channel 13 selected"]
    #[inline(always)]
    pub fn ceipsel_13(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel13)
    }
    #[doc = "Channel 14 selected"]
    #[inline(always)]
    pub fn ceipsel_14(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel14)
    }
    #[doc = "Channel 15 selected"]
    #[inline(always)]
    pub fn ceipsel_15(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipsel::Ceipsel15)
    }
}
#[doc = "Channel input enable for the V+ terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceipen {
    #[doc = "0: Selected analog input channel for V+ terminal is disabled"]
    Disable = 0,
    #[doc = "1: Selected analog input channel for V+ terminal is enabled"]
    Enable = 1,
}
impl From<Ceipen> for bool {
    #[inline(always)]
    fn from(variant: Ceipen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIPEN` reader - Channel input enable for the V+ terminal"]
pub type CeipenR = crate::BitReader<Ceipen>;
impl CeipenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceipen {
        match self.bits {
            false => Ceipen::Disable,
            true => Ceipen::Enable,
        }
    }
    #[doc = "Selected analog input channel for V+ terminal is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ceipen::Disable
    }
    #[doc = "Selected analog input channel for V+ terminal is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ceipen::Enable
    }
}
#[doc = "Field `CEIPEN` writer - Channel input enable for the V+ terminal"]
pub type CeipenW<'a, REG> = crate::BitWriter<'a, REG, Ceipen>;
impl<'a, REG> CeipenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selected analog input channel for V+ terminal is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipen::Disable)
    }
    #[doc = "Selected analog input channel for V+ terminal is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ceipen::Enable)
    }
}
#[doc = "Channel input selected for the - terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ceimsel {
    #[doc = "0: Channel 0 selected"]
    Ceimsel0 = 0,
    #[doc = "1: Channel 1 selected"]
    Ceimsel1 = 1,
    #[doc = "2: Channel 2 selected"]
    Ceimsel2 = 2,
    #[doc = "3: Channel 3 selected"]
    Ceimsel3 = 3,
    #[doc = "4: Channel 4 selected"]
    Ceimsel4 = 4,
    #[doc = "5: Channel 5 selected"]
    Ceimsel5 = 5,
    #[doc = "6: Channel 6 selected"]
    Ceimsel6 = 6,
    #[doc = "7: Channel 7 selected"]
    Ceimsel7 = 7,
    #[doc = "8: Channel 8 selected"]
    Ceimsel8 = 8,
    #[doc = "9: Channel 9 selected"]
    Ceimsel9 = 9,
    #[doc = "10: Channel 10 selected"]
    Ceimsel10 = 10,
    #[doc = "11: Channel 11 selected"]
    Ceimsel11 = 11,
    #[doc = "12: Channel 12 selected"]
    Ceimsel12 = 12,
    #[doc = "13: Channel 13 selected"]
    Ceimsel13 = 13,
    #[doc = "14: Channel 14 selected"]
    Ceimsel14 = 14,
    #[doc = "15: Channel 15 selected"]
    Ceimsel15 = 15,
}
impl From<Ceimsel> for u8 {
    #[inline(always)]
    fn from(variant: Ceimsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ceimsel {
    type Ux = u8;
}
impl crate::IsEnum for Ceimsel {}
#[doc = "Field `CEIMSEL` reader - Channel input selected for the - terminal"]
pub type CeimselR = crate::FieldReader<Ceimsel>;
impl CeimselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceimsel {
        match self.bits {
            0 => Ceimsel::Ceimsel0,
            1 => Ceimsel::Ceimsel1,
            2 => Ceimsel::Ceimsel2,
            3 => Ceimsel::Ceimsel3,
            4 => Ceimsel::Ceimsel4,
            5 => Ceimsel::Ceimsel5,
            6 => Ceimsel::Ceimsel6,
            7 => Ceimsel::Ceimsel7,
            8 => Ceimsel::Ceimsel8,
            9 => Ceimsel::Ceimsel9,
            10 => Ceimsel::Ceimsel10,
            11 => Ceimsel::Ceimsel11,
            12 => Ceimsel::Ceimsel12,
            13 => Ceimsel::Ceimsel13,
            14 => Ceimsel::Ceimsel14,
            15 => Ceimsel::Ceimsel15,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 0 selected"]
    #[inline(always)]
    pub fn is_ceimsel_0(&self) -> bool {
        *self == Ceimsel::Ceimsel0
    }
    #[doc = "Channel 1 selected"]
    #[inline(always)]
    pub fn is_ceimsel_1(&self) -> bool {
        *self == Ceimsel::Ceimsel1
    }
    #[doc = "Channel 2 selected"]
    #[inline(always)]
    pub fn is_ceimsel_2(&self) -> bool {
        *self == Ceimsel::Ceimsel2
    }
    #[doc = "Channel 3 selected"]
    #[inline(always)]
    pub fn is_ceimsel_3(&self) -> bool {
        *self == Ceimsel::Ceimsel3
    }
    #[doc = "Channel 4 selected"]
    #[inline(always)]
    pub fn is_ceimsel_4(&self) -> bool {
        *self == Ceimsel::Ceimsel4
    }
    #[doc = "Channel 5 selected"]
    #[inline(always)]
    pub fn is_ceimsel_5(&self) -> bool {
        *self == Ceimsel::Ceimsel5
    }
    #[doc = "Channel 6 selected"]
    #[inline(always)]
    pub fn is_ceimsel_6(&self) -> bool {
        *self == Ceimsel::Ceimsel6
    }
    #[doc = "Channel 7 selected"]
    #[inline(always)]
    pub fn is_ceimsel_7(&self) -> bool {
        *self == Ceimsel::Ceimsel7
    }
    #[doc = "Channel 8 selected"]
    #[inline(always)]
    pub fn is_ceimsel_8(&self) -> bool {
        *self == Ceimsel::Ceimsel8
    }
    #[doc = "Channel 9 selected"]
    #[inline(always)]
    pub fn is_ceimsel_9(&self) -> bool {
        *self == Ceimsel::Ceimsel9
    }
    #[doc = "Channel 10 selected"]
    #[inline(always)]
    pub fn is_ceimsel_10(&self) -> bool {
        *self == Ceimsel::Ceimsel10
    }
    #[doc = "Channel 11 selected"]
    #[inline(always)]
    pub fn is_ceimsel_11(&self) -> bool {
        *self == Ceimsel::Ceimsel11
    }
    #[doc = "Channel 12 selected"]
    #[inline(always)]
    pub fn is_ceimsel_12(&self) -> bool {
        *self == Ceimsel::Ceimsel12
    }
    #[doc = "Channel 13 selected"]
    #[inline(always)]
    pub fn is_ceimsel_13(&self) -> bool {
        *self == Ceimsel::Ceimsel13
    }
    #[doc = "Channel 14 selected"]
    #[inline(always)]
    pub fn is_ceimsel_14(&self) -> bool {
        *self == Ceimsel::Ceimsel14
    }
    #[doc = "Channel 15 selected"]
    #[inline(always)]
    pub fn is_ceimsel_15(&self) -> bool {
        *self == Ceimsel::Ceimsel15
    }
}
#[doc = "Field `CEIMSEL` writer - Channel input selected for the - terminal"]
pub type CeimselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ceimsel, crate::Safe>;
impl<'a, REG> CeimselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 selected"]
    #[inline(always)]
    pub fn ceimsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel0)
    }
    #[doc = "Channel 1 selected"]
    #[inline(always)]
    pub fn ceimsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel1)
    }
    #[doc = "Channel 2 selected"]
    #[inline(always)]
    pub fn ceimsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel2)
    }
    #[doc = "Channel 3 selected"]
    #[inline(always)]
    pub fn ceimsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel3)
    }
    #[doc = "Channel 4 selected"]
    #[inline(always)]
    pub fn ceimsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel4)
    }
    #[doc = "Channel 5 selected"]
    #[inline(always)]
    pub fn ceimsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel5)
    }
    #[doc = "Channel 6 selected"]
    #[inline(always)]
    pub fn ceimsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel6)
    }
    #[doc = "Channel 7 selected"]
    #[inline(always)]
    pub fn ceimsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel7)
    }
    #[doc = "Channel 8 selected"]
    #[inline(always)]
    pub fn ceimsel_8(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel8)
    }
    #[doc = "Channel 9 selected"]
    #[inline(always)]
    pub fn ceimsel_9(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel9)
    }
    #[doc = "Channel 10 selected"]
    #[inline(always)]
    pub fn ceimsel_10(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel10)
    }
    #[doc = "Channel 11 selected"]
    #[inline(always)]
    pub fn ceimsel_11(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel11)
    }
    #[doc = "Channel 12 selected"]
    #[inline(always)]
    pub fn ceimsel_12(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel12)
    }
    #[doc = "Channel 13 selected"]
    #[inline(always)]
    pub fn ceimsel_13(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel13)
    }
    #[doc = "Channel 14 selected"]
    #[inline(always)]
    pub fn ceimsel_14(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel14)
    }
    #[doc = "Channel 15 selected"]
    #[inline(always)]
    pub fn ceimsel_15(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimsel::Ceimsel15)
    }
}
#[doc = "Channel input enable for the - terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceimen {
    #[doc = "0: Selected analog input channel for V- terminal is disabled"]
    Disable = 0,
    #[doc = "1: Selected analog input channel for V- terminal is enabled"]
    Enable = 1,
}
impl From<Ceimen> for bool {
    #[inline(always)]
    fn from(variant: Ceimen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIMEN` reader - Channel input enable for the - terminal"]
pub type CeimenR = crate::BitReader<Ceimen>;
impl CeimenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceimen {
        match self.bits {
            false => Ceimen::Disable,
            true => Ceimen::Enable,
        }
    }
    #[doc = "Selected analog input channel for V- terminal is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ceimen::Disable
    }
    #[doc = "Selected analog input channel for V- terminal is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ceimen::Enable
    }
}
#[doc = "Field `CEIMEN` writer - Channel input enable for the - terminal"]
pub type CeimenW<'a, REG> = crate::BitWriter<'a, REG, Ceimen>;
impl<'a, REG> CeimenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selected analog input channel for V- terminal is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimen::Disable)
    }
    #[doc = "Selected analog input channel for V- terminal is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ceimen::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel input selected for the V+ terminal"]
    #[inline(always)]
    pub fn ceipsel(&self) -> CeipselR {
        CeipselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Channel input enable for the V+ terminal"]
    #[inline(always)]
    pub fn ceipen(&self) -> CeipenR {
        CeipenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Channel input selected for the - terminal"]
    #[inline(always)]
    pub fn ceimsel(&self) -> CeimselR {
        CeimselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Channel input enable for the - terminal"]
    #[inline(always)]
    pub fn ceimen(&self) -> CeimenR {
        CeimenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel input selected for the V+ terminal"]
    #[inline(always)]
    pub fn ceipsel(&mut self) -> CeipselW<'_, Cectl0Spec> {
        CeipselW::new(self, 0)
    }
    #[doc = "Bit 7 - Channel input enable for the V+ terminal"]
    #[inline(always)]
    pub fn ceipen(&mut self) -> CeipenW<'_, Cectl0Spec> {
        CeipenW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Channel input selected for the - terminal"]
    #[inline(always)]
    pub fn ceimsel(&mut self) -> CeimselW<'_, Cectl0Spec> {
        CeimselW::new(self, 8)
    }
    #[doc = "Bit 15 - Channel input enable for the - terminal"]
    #[inline(always)]
    pub fn ceimen(&mut self) -> CeimenW<'_, Cectl0Spec> {
        CeimenW::new(self, 15)
    }
}
#[doc = "Comparator Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cectl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cectl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cectl0Spec;
impl crate::RegisterSpec for Cectl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cectl0::R`](R) reader structure"]
impl crate::Readable for Cectl0Spec {}
#[doc = "`write(|w| ..)` method takes [`cectl0::W`](W) writer structure"]
impl crate::Writable for Cectl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CECTL0 to value 0"]
impl crate::Resettable for Cectl0Spec {}
