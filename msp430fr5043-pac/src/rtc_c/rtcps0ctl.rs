#[doc = "Register `RTCPS0CTL` reader"]
pub type R = crate::R<Rtcps0ctlSpec>;
#[doc = "Register `RTCPS0CTL` writer"]
pub type W = crate::W<Rtcps0ctlSpec>;
#[doc = "Prescale timer 0 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt0psifg {
    #[doc = "0: No time event occurred"]
    Rt0psifg0 = 0,
    #[doc = "1: Time event occurred"]
    Rt0psifg1 = 1,
}
impl From<Rt0psifg> for bool {
    #[inline(always)]
    fn from(variant: Rt0psifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT0PSIFG` reader - Prescale timer 0 interrupt flag"]
pub type Rt0psifgR = crate::BitReader<Rt0psifg>;
impl Rt0psifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt0psifg {
        match self.bits {
            false => Rt0psifg::Rt0psifg0,
            true => Rt0psifg::Rt0psifg1,
        }
    }
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn is_rt0psifg_0(&self) -> bool {
        *self == Rt0psifg::Rt0psifg0
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn is_rt0psifg_1(&self) -> bool {
        *self == Rt0psifg::Rt0psifg1
    }
}
#[doc = "Field `RT0PSIFG` writer - Prescale timer 0 interrupt flag"]
pub type Rt0psifgW<'a, REG> = crate::BitWriter<'a, REG, Rt0psifg>;
impl<'a, REG> Rt0psifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn rt0psifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psifg::Rt0psifg0)
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn rt0psifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psifg::Rt0psifg1)
    }
}
#[doc = "Prescale timer 0 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt0psie {
    #[doc = "0: Interrupt not enabled"]
    Disable = 0,
    #[doc = "1: Interrupt enabled"]
    Enable = 1,
}
impl From<Rt0psie> for bool {
    #[inline(always)]
    fn from(variant: Rt0psie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT0PSIE` reader - Prescale timer 0 interrupt enable"]
pub type Rt0psieR = crate::BitReader<Rt0psie>;
impl Rt0psieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt0psie {
        match self.bits {
            false => Rt0psie::Disable,
            true => Rt0psie::Enable,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rt0psie::Disable
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rt0psie::Enable
    }
}
#[doc = "Field `RT0PSIE` writer - Prescale timer 0 interrupt enable"]
pub type Rt0psieW<'a, REG> = crate::BitWriter<'a, REG, Rt0psie>;
impl<'a, REG> Rt0psieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psie::Disable)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psie::Enable)
    }
}
#[doc = "Prescale timer 0 interrupt interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rt0ip {
    #[doc = "0: Divide by 2"]
    _2 = 0,
    #[doc = "1: Divide by 4"]
    _4 = 1,
    #[doc = "2: Divide by 8"]
    _8 = 2,
    #[doc = "3: Divide by 16"]
    _16 = 3,
    #[doc = "4: Divide by 32"]
    _32 = 4,
    #[doc = "5: Divide by 64"]
    _64 = 5,
    #[doc = "6: Divide by 128"]
    _128 = 6,
    #[doc = "7: Divide by 256"]
    _256 = 7,
}
impl From<Rt0ip> for u8 {
    #[inline(always)]
    fn from(variant: Rt0ip) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rt0ip {
    type Ux = u8;
}
impl crate::IsEnum for Rt0ip {}
#[doc = "Field `RT0IP` reader - Prescale timer 0 interrupt interval"]
pub type Rt0ipR = crate::FieldReader<Rt0ip>;
impl Rt0ipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt0ip {
        match self.bits {
            0 => Rt0ip::_2,
            1 => Rt0ip::_4,
            2 => Rt0ip::_8,
            3 => Rt0ip::_16,
            4 => Rt0ip::_32,
            5 => Rt0ip::_64,
            6 => Rt0ip::_128,
            7 => Rt0ip::_256,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Rt0ip::_2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Rt0ip::_4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Rt0ip::_8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Rt0ip::_16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Rt0ip::_32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Rt0ip::_64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Rt0ip::_128
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Rt0ip::_256
    }
}
#[doc = "Field `RT0IP` writer - Prescale timer 0 interrupt interval"]
pub type Rt0ipW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rt0ip, crate::Safe>;
impl<'a, REG> Rt0ipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::_4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::_8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::_16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::_32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::_64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::_128)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::_256)
    }
}
#[doc = "Prescale timer 0 hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt0pshold {
    #[doc = "0: RT0PS is operational"]
    Rt0pshold0 = 0,
    #[doc = "1: RT0PS is held"]
    Rt0pshold1 = 1,
}
impl From<Rt0pshold> for bool {
    #[inline(always)]
    fn from(variant: Rt0pshold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT0PSHOLD` reader - Prescale timer 0 hold"]
pub type Rt0psholdR = crate::BitReader<Rt0pshold>;
impl Rt0psholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt0pshold {
        match self.bits {
            false => Rt0pshold::Rt0pshold0,
            true => Rt0pshold::Rt0pshold1,
        }
    }
    #[doc = "RT0PS is operational"]
    #[inline(always)]
    pub fn is_rt0pshold_0(&self) -> bool {
        *self == Rt0pshold::Rt0pshold0
    }
    #[doc = "RT0PS is held"]
    #[inline(always)]
    pub fn is_rt0pshold_1(&self) -> bool {
        *self == Rt0pshold::Rt0pshold1
    }
}
#[doc = "Field `RT0PSHOLD` writer - Prescale timer 0 hold"]
pub type Rt0psholdW<'a, REG> = crate::BitWriter<'a, REG, Rt0pshold>;
impl<'a, REG> Rt0psholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RT0PS is operational"]
    #[inline(always)]
    pub fn rt0pshold_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0pshold::Rt0pshold0)
    }
    #[doc = "RT0PS is held"]
    #[inline(always)]
    pub fn rt0pshold_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0pshold::Rt0pshold1)
    }
}
#[doc = "Prescale timer 0 clock divide\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rt0psdiv {
    #[doc = "0: Divide by 2"]
    _2 = 0,
    #[doc = "1: Divide by 4"]
    _4 = 1,
    #[doc = "2: Divide by 8"]
    _8 = 2,
    #[doc = "3: Divide by 16"]
    _16 = 3,
    #[doc = "4: Divide by 32"]
    _32 = 4,
    #[doc = "5: Divide by 64"]
    _64 = 5,
    #[doc = "6: Divide by 128"]
    _128 = 6,
    #[doc = "7: Divide by 256"]
    _256 = 7,
}
impl From<Rt0psdiv> for u8 {
    #[inline(always)]
    fn from(variant: Rt0psdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rt0psdiv {
    type Ux = u8;
}
impl crate::IsEnum for Rt0psdiv {}
#[doc = "Field `RT0PSDIV` reader - Prescale timer 0 clock divide"]
pub type Rt0psdivR = crate::FieldReader<Rt0psdiv>;
impl Rt0psdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt0psdiv {
        match self.bits {
            0 => Rt0psdiv::_2,
            1 => Rt0psdiv::_4,
            2 => Rt0psdiv::_8,
            3 => Rt0psdiv::_16,
            4 => Rt0psdiv::_32,
            5 => Rt0psdiv::_64,
            6 => Rt0psdiv::_128,
            7 => Rt0psdiv::_256,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Rt0psdiv::_2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Rt0psdiv::_4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Rt0psdiv::_8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Rt0psdiv::_16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Rt0psdiv::_32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Rt0psdiv::_64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Rt0psdiv::_128
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Rt0psdiv::_256
    }
}
#[doc = "Field `RT0PSDIV` writer - Prescale timer 0 clock divide"]
pub type Rt0psdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rt0psdiv, crate::Safe>;
impl<'a, REG> Rt0psdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psdiv::_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psdiv::_4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psdiv::_8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psdiv::_16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psdiv::_32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psdiv::_64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psdiv::_128)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psdiv::_256)
    }
}
impl R {
    #[doc = "Bit 0 - Prescale timer 0 interrupt flag"]
    #[inline(always)]
    pub fn rt0psifg(&self) -> Rt0psifgR {
        Rt0psifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Prescale timer 0 interrupt enable"]
    #[inline(always)]
    pub fn rt0psie(&self) -> Rt0psieR {
        Rt0psieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Prescale timer 0 interrupt interval"]
    #[inline(always)]
    pub fn rt0ip(&self) -> Rt0ipR {
        Rt0ipR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 8 - Prescale timer 0 hold"]
    #[inline(always)]
    pub fn rt0pshold(&self) -> Rt0psholdR {
        Rt0psholdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Prescale timer 0 clock divide"]
    #[inline(always)]
    pub fn rt0psdiv(&self) -> Rt0psdivR {
        Rt0psdivR::new(((self.bits >> 11) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Prescale timer 0 interrupt flag"]
    #[inline(always)]
    pub fn rt0psifg(&mut self) -> Rt0psifgW<'_, Rtcps0ctlSpec> {
        Rt0psifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Prescale timer 0 interrupt enable"]
    #[inline(always)]
    pub fn rt0psie(&mut self) -> Rt0psieW<'_, Rtcps0ctlSpec> {
        Rt0psieW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Prescale timer 0 interrupt interval"]
    #[inline(always)]
    pub fn rt0ip(&mut self) -> Rt0ipW<'_, Rtcps0ctlSpec> {
        Rt0ipW::new(self, 2)
    }
    #[doc = "Bit 8 - Prescale timer 0 hold"]
    #[inline(always)]
    pub fn rt0pshold(&mut self) -> Rt0psholdW<'_, Rtcps0ctlSpec> {
        Rt0psholdW::new(self, 8)
    }
    #[doc = "Bits 11:13 - Prescale timer 0 clock divide"]
    #[inline(always)]
    pub fn rt0psdiv(&mut self) -> Rt0psdivW<'_, Rtcps0ctlSpec> {
        Rt0psdivW::new(self, 11)
    }
}
#[doc = "Real-Time Clock Prescale Timer 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcps0ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcps0ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtcps0ctlSpec;
impl crate::RegisterSpec for Rtcps0ctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcps0ctl::R`](R) reader structure"]
impl crate::Readable for Rtcps0ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcps0ctl::W`](W) writer structure"]
impl crate::Writable for Rtcps0ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCPS0CTL to value 0"]
impl crate::Resettable for Rtcps0ctlSpec {}
