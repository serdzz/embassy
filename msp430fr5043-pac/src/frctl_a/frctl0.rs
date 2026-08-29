#[doc = "Register `FRCTL0` reader"]
pub type R = crate::R<Frctl0Spec>;
#[doc = "Register `FRCTL0` writer"]
pub type W = crate::W<Frctl0Spec>;
#[doc = "Write Protection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wprot {
    #[doc = "0: Disable Write Protection. Write to FRAM memory is allowed."]
    Wprot0 = 0,
    #[doc = "1: Enable Write Protection. Write to FRAM memory is not allowed. In case a write access is attempted, the WPIFG (Write Protection Flag) bit will be set."]
    Wprot1 = 1,
}
impl From<Wprot> for bool {
    #[inline(always)]
    fn from(variant: Wprot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPROT` reader - Write Protection Enable"]
pub type WprotR = crate::BitReader<Wprot>;
impl WprotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wprot {
        match self.bits {
            false => Wprot::Wprot0,
            true => Wprot::Wprot1,
        }
    }
    #[doc = "Disable Write Protection. Write to FRAM memory is allowed."]
    #[inline(always)]
    pub fn is_wprot_0(&self) -> bool {
        *self == Wprot::Wprot0
    }
    #[doc = "Enable Write Protection. Write to FRAM memory is not allowed. In case a write access is attempted, the WPIFG (Write Protection Flag) bit will be set."]
    #[inline(always)]
    pub fn is_wprot_1(&self) -> bool {
        *self == Wprot::Wprot1
    }
}
#[doc = "Field `WPROT` writer - Write Protection Enable"]
pub type WprotW<'a, REG> = crate::BitWriter<'a, REG, Wprot>;
impl<'a, REG> WprotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Write Protection. Write to FRAM memory is allowed."]
    #[inline(always)]
    pub fn wprot_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wprot::Wprot0)
    }
    #[doc = "Enable Write Protection. Write to FRAM memory is not allowed. In case a write access is attempted, the WPIFG (Write Protection Flag) bit will be set."]
    #[inline(always)]
    pub fn wprot_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wprot::Wprot1)
    }
}
#[doc = "Enable automatic Wait State Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Auto {
    #[doc = "0: User Wait State Mode. The NWAITS\\[3:0\\] is used for the FRAM wait state."]
    Auto0 = 0,
    #[doc = "1: Auto mode. The NWAITS\\[3:0\\] is ignored. Wait states are generated automatically by the internal FRAM controller state machine."]
    Auto1 = 1,
}
impl From<Auto> for bool {
    #[inline(always)]
    fn from(variant: Auto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTO` reader - Enable automatic Wait State Mode"]
pub type AutoR = crate::BitReader<Auto>;
impl AutoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Auto {
        match self.bits {
            false => Auto::Auto0,
            true => Auto::Auto1,
        }
    }
    #[doc = "User Wait State Mode. The NWAITS\\[3:0\\] is used for the FRAM wait state."]
    #[inline(always)]
    pub fn is_auto_0(&self) -> bool {
        *self == Auto::Auto0
    }
    #[doc = "Auto mode. The NWAITS\\[3:0\\] is ignored. Wait states are generated automatically by the internal FRAM controller state machine."]
    #[inline(always)]
    pub fn is_auto_1(&self) -> bool {
        *self == Auto::Auto1
    }
}
#[doc = "Field `AUTO` writer - Enable automatic Wait State Mode"]
pub type AutoW<'a, REG> = crate::BitWriter<'a, REG, Auto>;
impl<'a, REG> AutoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "User Wait State Mode. The NWAITS\\[3:0\\] is used for the FRAM wait state."]
    #[inline(always)]
    pub fn auto_0(self) -> &'a mut crate::W<REG> {
        self.variant(Auto::Auto0)
    }
    #[doc = "Auto mode. The NWAITS\\[3:0\\] is ignored. Wait states are generated automatically by the internal FRAM controller state machine."]
    #[inline(always)]
    pub fn auto_1(self) -> &'a mut crate::W<REG> {
        self.variant(Auto::Auto1)
    }
}
#[doc = "Wait state numbers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nwaits {
    #[doc = "0: FRAM wait states: 0"]
    Nwaits0 = 0,
    #[doc = "1: FRAM wait states: 1"]
    Nwaits1 = 1,
    #[doc = "2: FRAM wait states: 2"]
    Nwaits2 = 2,
    #[doc = "3: FRAM wait states: 3"]
    Nwaits3 = 3,
    #[doc = "4: FRAM wait states: 4"]
    Nwaits4 = 4,
    #[doc = "5: FRAM wait states: 5"]
    Nwaits5 = 5,
    #[doc = "6: FRAM wait states: 6"]
    Nwaits6 = 6,
    #[doc = "7: FRAM wait states: 7"]
    Nwaits7 = 7,
    #[doc = "8: FRAM wait states: 8"]
    Nwaits8 = 8,
    #[doc = "9: FRAM wait states: 9"]
    Nwaits9 = 9,
    #[doc = "10: FRAM wait states: 10"]
    Nwaits10 = 10,
    #[doc = "11: FRAM wait states: 11"]
    Nwaits11 = 11,
    #[doc = "12: FRAM wait states: 12"]
    Nwaits12 = 12,
    #[doc = "13: FRAM wait states: 13"]
    Nwaits13 = 13,
    #[doc = "14: FRAM wait states: 14"]
    Nwaits14 = 14,
    #[doc = "15: FRAM wait states: 15"]
    Nwaits15 = 15,
}
impl From<Nwaits> for u8 {
    #[inline(always)]
    fn from(variant: Nwaits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nwaits {
    type Ux = u8;
}
impl crate::IsEnum for Nwaits {}
#[doc = "Field `NWAITS` reader - Wait state numbers"]
pub type NwaitsR = crate::FieldReader<Nwaits>;
impl NwaitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nwaits {
        match self.bits {
            0 => Nwaits::Nwaits0,
            1 => Nwaits::Nwaits1,
            2 => Nwaits::Nwaits2,
            3 => Nwaits::Nwaits3,
            4 => Nwaits::Nwaits4,
            5 => Nwaits::Nwaits5,
            6 => Nwaits::Nwaits6,
            7 => Nwaits::Nwaits7,
            8 => Nwaits::Nwaits8,
            9 => Nwaits::Nwaits9,
            10 => Nwaits::Nwaits10,
            11 => Nwaits::Nwaits11,
            12 => Nwaits::Nwaits12,
            13 => Nwaits::Nwaits13,
            14 => Nwaits::Nwaits14,
            15 => Nwaits::Nwaits15,
            _ => unreachable!(),
        }
    }
    #[doc = "FRAM wait states: 0"]
    #[inline(always)]
    pub fn is_nwaits_0(&self) -> bool {
        *self == Nwaits::Nwaits0
    }
    #[doc = "FRAM wait states: 1"]
    #[inline(always)]
    pub fn is_nwaits_1(&self) -> bool {
        *self == Nwaits::Nwaits1
    }
    #[doc = "FRAM wait states: 2"]
    #[inline(always)]
    pub fn is_nwaits_2(&self) -> bool {
        *self == Nwaits::Nwaits2
    }
    #[doc = "FRAM wait states: 3"]
    #[inline(always)]
    pub fn is_nwaits_3(&self) -> bool {
        *self == Nwaits::Nwaits3
    }
    #[doc = "FRAM wait states: 4"]
    #[inline(always)]
    pub fn is_nwaits_4(&self) -> bool {
        *self == Nwaits::Nwaits4
    }
    #[doc = "FRAM wait states: 5"]
    #[inline(always)]
    pub fn is_nwaits_5(&self) -> bool {
        *self == Nwaits::Nwaits5
    }
    #[doc = "FRAM wait states: 6"]
    #[inline(always)]
    pub fn is_nwaits_6(&self) -> bool {
        *self == Nwaits::Nwaits6
    }
    #[doc = "FRAM wait states: 7"]
    #[inline(always)]
    pub fn is_nwaits_7(&self) -> bool {
        *self == Nwaits::Nwaits7
    }
    #[doc = "FRAM wait states: 8"]
    #[inline(always)]
    pub fn is_nwaits_8(&self) -> bool {
        *self == Nwaits::Nwaits8
    }
    #[doc = "FRAM wait states: 9"]
    #[inline(always)]
    pub fn is_nwaits_9(&self) -> bool {
        *self == Nwaits::Nwaits9
    }
    #[doc = "FRAM wait states: 10"]
    #[inline(always)]
    pub fn is_nwaits_10(&self) -> bool {
        *self == Nwaits::Nwaits10
    }
    #[doc = "FRAM wait states: 11"]
    #[inline(always)]
    pub fn is_nwaits_11(&self) -> bool {
        *self == Nwaits::Nwaits11
    }
    #[doc = "FRAM wait states: 12"]
    #[inline(always)]
    pub fn is_nwaits_12(&self) -> bool {
        *self == Nwaits::Nwaits12
    }
    #[doc = "FRAM wait states: 13"]
    #[inline(always)]
    pub fn is_nwaits_13(&self) -> bool {
        *self == Nwaits::Nwaits13
    }
    #[doc = "FRAM wait states: 14"]
    #[inline(always)]
    pub fn is_nwaits_14(&self) -> bool {
        *self == Nwaits::Nwaits14
    }
    #[doc = "FRAM wait states: 15"]
    #[inline(always)]
    pub fn is_nwaits_15(&self) -> bool {
        *self == Nwaits::Nwaits15
    }
}
#[doc = "Field `NWAITS` writer - Wait state numbers"]
pub type NwaitsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Nwaits, crate::Safe>;
impl<'a, REG> NwaitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FRAM wait states: 0"]
    #[inline(always)]
    pub fn nwaits_0(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits0)
    }
    #[doc = "FRAM wait states: 1"]
    #[inline(always)]
    pub fn nwaits_1(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits1)
    }
    #[doc = "FRAM wait states: 2"]
    #[inline(always)]
    pub fn nwaits_2(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits2)
    }
    #[doc = "FRAM wait states: 3"]
    #[inline(always)]
    pub fn nwaits_3(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits3)
    }
    #[doc = "FRAM wait states: 4"]
    #[inline(always)]
    pub fn nwaits_4(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits4)
    }
    #[doc = "FRAM wait states: 5"]
    #[inline(always)]
    pub fn nwaits_5(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits5)
    }
    #[doc = "FRAM wait states: 6"]
    #[inline(always)]
    pub fn nwaits_6(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits6)
    }
    #[doc = "FRAM wait states: 7"]
    #[inline(always)]
    pub fn nwaits_7(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits7)
    }
    #[doc = "FRAM wait states: 8"]
    #[inline(always)]
    pub fn nwaits_8(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits8)
    }
    #[doc = "FRAM wait states: 9"]
    #[inline(always)]
    pub fn nwaits_9(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits9)
    }
    #[doc = "FRAM wait states: 10"]
    #[inline(always)]
    pub fn nwaits_10(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits10)
    }
    #[doc = "FRAM wait states: 11"]
    #[inline(always)]
    pub fn nwaits_11(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits11)
    }
    #[doc = "FRAM wait states: 12"]
    #[inline(always)]
    pub fn nwaits_12(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits12)
    }
    #[doc = "FRAM wait states: 13"]
    #[inline(always)]
    pub fn nwaits_13(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits13)
    }
    #[doc = "FRAM wait states: 14"]
    #[inline(always)]
    pub fn nwaits_14(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits14)
    }
    #[doc = "FRAM wait states: 15"]
    #[inline(always)]
    pub fn nwaits_15(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits15)
    }
}
#[doc = "Field `FRCTLPW` reader - FRCTLPW password"]
pub type FrctlpwR = crate::FieldReader;
#[doc = "Field `FRCTLPW` writer - FRCTLPW password"]
pub type FrctlpwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wprot(&self) -> WprotR {
        WprotR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Enable automatic Wait State Mode"]
    #[inline(always)]
    pub fn auto(&self) -> AutoR {
        AutoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Wait state numbers"]
    #[inline(always)]
    pub fn nwaits(&self) -> NwaitsR {
        NwaitsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - FRCTLPW password"]
    #[inline(always)]
    pub fn frctlpw(&self) -> FrctlpwR {
        FrctlpwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wprot(&mut self) -> WprotW<'_, Frctl0Spec> {
        WprotW::new(self, 0)
    }
    #[doc = "Bit 3 - Enable automatic Wait State Mode"]
    #[inline(always)]
    pub fn auto(&mut self) -> AutoW<'_, Frctl0Spec> {
        AutoW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Wait state numbers"]
    #[inline(always)]
    pub fn nwaits(&mut self) -> NwaitsW<'_, Frctl0Spec> {
        NwaitsW::new(self, 4)
    }
    #[doc = "Bits 8:15 - FRCTLPW password"]
    #[inline(always)]
    pub fn frctlpw(&mut self) -> FrctlpwW<'_, Frctl0Spec> {
        FrctlpwW::new(self, 8)
    }
}
#[doc = "FRAM Controller A Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`frctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Frctl0Spec;
impl crate::RegisterSpec for Frctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`frctl0::R`](R) reader structure"]
impl crate::Readable for Frctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`frctl0::W`](W) writer structure"]
impl crate::Writable for Frctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRCTL0 to value 0"]
impl crate::Resettable for Frctl0Spec {}
