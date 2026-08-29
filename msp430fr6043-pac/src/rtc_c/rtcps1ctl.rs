#[doc = "Register `RTCPS1CTL` reader"]
pub type R = crate::R<Rtcps1ctlSpec>;
#[doc = "Register `RTCPS1CTL` writer"]
pub type W = crate::W<Rtcps1ctlSpec>;
#[doc = "Prescale timer 1 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt1psifg {
    #[doc = "0: No time event occurred"]
    Rt1psifg0 = 0,
    #[doc = "1: Time event occurred"]
    Rt1psifg1 = 1,
}
impl From<Rt1psifg> for bool {
    #[inline(always)]
    fn from(variant: Rt1psifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT1PSIFG` reader - Prescale timer 1 interrupt flag"]
pub type Rt1psifgR = crate::BitReader<Rt1psifg>;
impl Rt1psifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt1psifg {
        match self.bits {
            false => Rt1psifg::Rt1psifg0,
            true => Rt1psifg::Rt1psifg1,
        }
    }
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn is_rt1psifg_0(&self) -> bool {
        *self == Rt1psifg::Rt1psifg0
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn is_rt1psifg_1(&self) -> bool {
        *self == Rt1psifg::Rt1psifg1
    }
}
#[doc = "Field `RT1PSIFG` writer - Prescale timer 1 interrupt flag"]
pub type Rt1psifgW<'a, REG> = crate::BitWriter<'a, REG, Rt1psifg>;
impl<'a, REG> Rt1psifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn rt1psifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psifg::Rt1psifg0)
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn rt1psifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psifg::Rt1psifg1)
    }
}
#[doc = "Prescale timer 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt1psie {
    #[doc = "0: Interrupt not enabled"]
    Disable = 0,
    #[doc = "1: Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    Enable = 1,
}
impl From<Rt1psie> for bool {
    #[inline(always)]
    fn from(variant: Rt1psie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT1PSIE` reader - Prescale timer 1 interrupt enable"]
pub type Rt1psieR = crate::BitReader<Rt1psie>;
impl Rt1psieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt1psie {
        match self.bits {
            false => Rt1psie::Disable,
            true => Rt1psie::Enable,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rt1psie::Disable
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rt1psie::Enable
    }
}
#[doc = "Field `RT1PSIE` writer - Prescale timer 1 interrupt enable"]
pub type Rt1psieW<'a, REG> = crate::BitWriter<'a, REG, Rt1psie>;
impl<'a, REG> Rt1psieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psie::Disable)
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psie::Enable)
    }
}
#[doc = "Prescale timer 1 interrupt interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rt1ip {
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
impl From<Rt1ip> for u8 {
    #[inline(always)]
    fn from(variant: Rt1ip) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rt1ip {
    type Ux = u8;
}
impl crate::IsEnum for Rt1ip {}
#[doc = "Field `RT1IP` reader - Prescale timer 1 interrupt interval"]
pub type Rt1ipR = crate::FieldReader<Rt1ip>;
impl Rt1ipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt1ip {
        match self.bits {
            0 => Rt1ip::_2,
            1 => Rt1ip::_4,
            2 => Rt1ip::_8,
            3 => Rt1ip::_16,
            4 => Rt1ip::_32,
            5 => Rt1ip::_64,
            6 => Rt1ip::_128,
            7 => Rt1ip::_256,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Rt1ip::_2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Rt1ip::_4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Rt1ip::_8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Rt1ip::_16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Rt1ip::_32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Rt1ip::_64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Rt1ip::_128
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Rt1ip::_256
    }
}
#[doc = "Field `RT1IP` writer - Prescale timer 1 interrupt interval"]
pub type Rt1ipW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rt1ip, crate::Safe>;
impl<'a, REG> Rt1ipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::_4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::_8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::_16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::_32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::_64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::_128)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::_256)
    }
}
#[doc = "Prescale timer 1 hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt1pshold {
    #[doc = "0: RT1PS is operational"]
    Rt1pshold0 = 0,
    #[doc = "1: RT1PS is held"]
    Rt1pshold1 = 1,
}
impl From<Rt1pshold> for bool {
    #[inline(always)]
    fn from(variant: Rt1pshold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT1PSHOLD` reader - Prescale timer 1 hold"]
pub type Rt1psholdR = crate::BitReader<Rt1pshold>;
impl Rt1psholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt1pshold {
        match self.bits {
            false => Rt1pshold::Rt1pshold0,
            true => Rt1pshold::Rt1pshold1,
        }
    }
    #[doc = "RT1PS is operational"]
    #[inline(always)]
    pub fn is_rt1pshold_0(&self) -> bool {
        *self == Rt1pshold::Rt1pshold0
    }
    #[doc = "RT1PS is held"]
    #[inline(always)]
    pub fn is_rt1pshold_1(&self) -> bool {
        *self == Rt1pshold::Rt1pshold1
    }
}
#[doc = "Field `RT1PSHOLD` writer - Prescale timer 1 hold"]
pub type Rt1psholdW<'a, REG> = crate::BitWriter<'a, REG, Rt1pshold>;
impl<'a, REG> Rt1psholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RT1PS is operational"]
    #[inline(always)]
    pub fn rt1pshold_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1pshold::Rt1pshold0)
    }
    #[doc = "RT1PS is held"]
    #[inline(always)]
    pub fn rt1pshold_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1pshold::Rt1pshold1)
    }
}
#[doc = "Prescale timer 1 clock divide\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rt1psdiv {
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
impl From<Rt1psdiv> for u8 {
    #[inline(always)]
    fn from(variant: Rt1psdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rt1psdiv {
    type Ux = u8;
}
impl crate::IsEnum for Rt1psdiv {}
#[doc = "Field `RT1PSDIV` reader - Prescale timer 1 clock divide"]
pub type Rt1psdivR = crate::FieldReader<Rt1psdiv>;
impl Rt1psdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt1psdiv {
        match self.bits {
            0 => Rt1psdiv::_2,
            1 => Rt1psdiv::_4,
            2 => Rt1psdiv::_8,
            3 => Rt1psdiv::_16,
            4 => Rt1psdiv::_32,
            5 => Rt1psdiv::_64,
            6 => Rt1psdiv::_128,
            7 => Rt1psdiv::_256,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Rt1psdiv::_2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Rt1psdiv::_4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Rt1psdiv::_8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Rt1psdiv::_16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Rt1psdiv::_32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Rt1psdiv::_64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Rt1psdiv::_128
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Rt1psdiv::_256
    }
}
#[doc = "Field `RT1PSDIV` writer - Prescale timer 1 clock divide"]
pub type Rt1psdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rt1psdiv, crate::Safe>;
impl<'a, REG> Rt1psdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psdiv::_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psdiv::_4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psdiv::_8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psdiv::_16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psdiv::_32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psdiv::_64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psdiv::_128)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psdiv::_256)
    }
}
#[doc = "Prescale timer 1 clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rt1ssel {
    #[doc = "0: 32-kHz crystal oscillator clock"]
    Rt1ssel0 = 0,
    #[doc = "1: 32-kHz crystal oscillator clock"]
    Rt1ssel1 = 1,
    #[doc = "2: Output from RT0PS"]
    Rt0ps = 2,
    #[doc = "3: Output from RT0PS"]
    Rt0ps3 = 3,
}
impl From<Rt1ssel> for u8 {
    #[inline(always)]
    fn from(variant: Rt1ssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rt1ssel {
    type Ux = u8;
}
impl crate::IsEnum for Rt1ssel {}
#[doc = "Field `RT1SSEL` reader - Prescale timer 1 clock source select"]
pub type Rt1sselR = crate::FieldReader<Rt1ssel>;
impl Rt1sselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt1ssel {
        match self.bits {
            0 => Rt1ssel::Rt1ssel0,
            1 => Rt1ssel::Rt1ssel1,
            2 => Rt1ssel::Rt0ps,
            3 => Rt1ssel::Rt0ps3,
            _ => unreachable!(),
        }
    }
    #[doc = "32-kHz crystal oscillator clock"]
    #[inline(always)]
    pub fn is_rt1ssel_0(&self) -> bool {
        *self == Rt1ssel::Rt1ssel0
    }
    #[doc = "32-kHz crystal oscillator clock"]
    #[inline(always)]
    pub fn is_rt1ssel_1(&self) -> bool {
        *self == Rt1ssel::Rt1ssel1
    }
    #[doc = "Output from RT0PS"]
    #[inline(always)]
    pub fn is_rt0ps(&self) -> bool {
        *self == Rt1ssel::Rt0ps
    }
    #[doc = "Output from RT0PS"]
    #[inline(always)]
    pub fn is_rt0ps_3(&self) -> bool {
        *self == Rt1ssel::Rt0ps3
    }
}
#[doc = "Field `RT1SSEL` writer - Prescale timer 1 clock source select"]
pub type Rt1sselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rt1ssel, crate::Safe>;
impl<'a, REG> Rt1sselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32-kHz crystal oscillator clock"]
    #[inline(always)]
    pub fn rt1ssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ssel::Rt1ssel0)
    }
    #[doc = "32-kHz crystal oscillator clock"]
    #[inline(always)]
    pub fn rt1ssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ssel::Rt1ssel1)
    }
    #[doc = "Output from RT0PS"]
    #[inline(always)]
    pub fn rt0ps(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ssel::Rt0ps)
    }
    #[doc = "Output from RT0PS"]
    #[inline(always)]
    pub fn rt0ps_3(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ssel::Rt0ps3)
    }
}
impl R {
    #[doc = "Bit 0 - Prescale timer 1 interrupt flag"]
    #[inline(always)]
    pub fn rt1psifg(&self) -> Rt1psifgR {
        Rt1psifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Prescale timer 1 interrupt enable"]
    #[inline(always)]
    pub fn rt1psie(&self) -> Rt1psieR {
        Rt1psieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Prescale timer 1 interrupt interval"]
    #[inline(always)]
    pub fn rt1ip(&self) -> Rt1ipR {
        Rt1ipR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 8 - Prescale timer 1 hold"]
    #[inline(always)]
    pub fn rt1pshold(&self) -> Rt1psholdR {
        Rt1psholdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Prescale timer 1 clock divide"]
    #[inline(always)]
    pub fn rt1psdiv(&self) -> Rt1psdivR {
        Rt1psdivR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - Prescale timer 1 clock source select"]
    #[inline(always)]
    pub fn rt1ssel(&self) -> Rt1sselR {
        Rt1sselR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Prescale timer 1 interrupt flag"]
    #[inline(always)]
    pub fn rt1psifg(&mut self) -> Rt1psifgW<'_, Rtcps1ctlSpec> {
        Rt1psifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Prescale timer 1 interrupt enable"]
    #[inline(always)]
    pub fn rt1psie(&mut self) -> Rt1psieW<'_, Rtcps1ctlSpec> {
        Rt1psieW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Prescale timer 1 interrupt interval"]
    #[inline(always)]
    pub fn rt1ip(&mut self) -> Rt1ipW<'_, Rtcps1ctlSpec> {
        Rt1ipW::new(self, 2)
    }
    #[doc = "Bit 8 - Prescale timer 1 hold"]
    #[inline(always)]
    pub fn rt1pshold(&mut self) -> Rt1psholdW<'_, Rtcps1ctlSpec> {
        Rt1psholdW::new(self, 8)
    }
    #[doc = "Bits 11:13 - Prescale timer 1 clock divide"]
    #[inline(always)]
    pub fn rt1psdiv(&mut self) -> Rt1psdivW<'_, Rtcps1ctlSpec> {
        Rt1psdivW::new(self, 11)
    }
    #[doc = "Bits 14:15 - Prescale timer 1 clock source select"]
    #[inline(always)]
    pub fn rt1ssel(&mut self) -> Rt1sselW<'_, Rtcps1ctlSpec> {
        Rt1sselW::new(self, 14)
    }
}
#[doc = "Real-Time Clock Prescale Timer 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcps1ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcps1ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtcps1ctlSpec;
impl crate::RegisterSpec for Rtcps1ctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcps1ctl::R`](R) reader structure"]
impl crate::Readable for Rtcps1ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcps1ctl::W`](W) writer structure"]
impl crate::Writable for Rtcps1ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCPS1CTL to value 0"]
impl crate::Resettable for Rtcps1ctlSpec {}
