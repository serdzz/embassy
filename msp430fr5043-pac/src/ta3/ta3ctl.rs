#[doc = "Register `TA3CTL` reader"]
pub type R = crate::R<Ta3ctlSpec>;
#[doc = "Register `TA3CTL` writer"]
pub type W = crate::W<Ta3ctlSpec>;
#[doc = "TimerA interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Taifg {
    #[doc = "0: No interrupt pending"]
    Taifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Taifg1 = 1,
}
impl From<Taifg> for bool {
    #[inline(always)]
    fn from(variant: Taifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAIFG` reader - TimerA interrupt flag"]
pub type TaifgR = crate::BitReader<Taifg>;
impl TaifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Taifg {
        match self.bits {
            false => Taifg::Taifg0,
            true => Taifg::Taifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_taifg_0(&self) -> bool {
        *self == Taifg::Taifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_taifg_1(&self) -> bool {
        *self == Taifg::Taifg1
    }
}
#[doc = "Field `TAIFG` writer - TimerA interrupt flag"]
pub type TaifgW<'a, REG> = crate::BitWriter<'a, REG, Taifg>;
impl<'a, REG> TaifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn taifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Taifg::Taifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn taifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Taifg::Taifg1)
    }
}
#[doc = "TimerA interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Taie {
    #[doc = "0: Interrupt disabled"]
    Taie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Taie1 = 1,
}
impl From<Taie> for bool {
    #[inline(always)]
    fn from(variant: Taie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAIE` reader - TimerA interrupt enable"]
pub type TaieR = crate::BitReader<Taie>;
impl TaieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Taie {
        match self.bits {
            false => Taie::Taie0,
            true => Taie::Taie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_taie_0(&self) -> bool {
        *self == Taie::Taie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_taie_1(&self) -> bool {
        *self == Taie::Taie1
    }
}
#[doc = "Field `TAIE` writer - TimerA interrupt enable"]
pub type TaieW<'a, REG> = crate::BitWriter<'a, REG, Taie>;
impl<'a, REG> TaieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn taie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Taie::Taie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn taie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Taie::Taie1)
    }
}
#[doc = "Field `TACLR` reader - TimerA clear"]
pub type TaclrR = crate::BitReader;
#[doc = "Field `TACLR` writer - TimerA clear"]
pub type TaclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mc {
    #[doc = "0: Stop mode: Timer is halted"]
    Stop = 0,
    #[doc = "1: Up mode: Timer counts up to TAxCCR0"]
    Up = 1,
    #[doc = "2: Continuous mode: Timer counts up to 0FFFFh"]
    Continuous = 2,
    #[doc = "3: Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    Updown = 3,
}
impl From<Mc> for u8 {
    #[inline(always)]
    fn from(variant: Mc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mc {
    type Ux = u8;
}
impl crate::IsEnum for Mc {}
#[doc = "Field `MC` reader - Mode control"]
pub type McR = crate::FieldReader<Mc>;
impl McR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mc {
        match self.bits {
            0 => Mc::Stop,
            1 => Mc::Up,
            2 => Mc::Continuous,
            3 => Mc::Updown,
            _ => unreachable!(),
        }
    }
    #[doc = "Stop mode: Timer is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Mc::Stop
    }
    #[doc = "Up mode: Timer counts up to TAxCCR0"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Mc::Up
    }
    #[doc = "Continuous mode: Timer counts up to 0FFFFh"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Mc::Continuous
    }
    #[doc = "Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == Mc::Updown
    }
}
#[doc = "Field `MC` writer - Mode control"]
pub type McW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mc, crate::Safe>;
impl<'a, REG> McW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stop mode: Timer is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Stop)
    }
    #[doc = "Up mode: Timer counts up to TAxCCR0"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Up)
    }
    #[doc = "Continuous mode: Timer counts up to 0FFFFh"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Continuous)
    }
    #[doc = "Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    #[inline(always)]
    pub fn updown(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Updown)
    }
}
#[doc = "Input divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Id {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /2"]
    _2 = 1,
    #[doc = "2: /4"]
    _4 = 2,
    #[doc = "3: /8"]
    _8 = 3,
}
impl From<Id> for u8 {
    #[inline(always)]
    fn from(variant: Id) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Id {
    type Ux = u8;
}
impl crate::IsEnum for Id {}
#[doc = "Field `ID` reader - Input divider"]
pub type IdR = crate::FieldReader<Id>;
impl IdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id {
        match self.bits {
            0 => Id::_1,
            1 => Id::_2,
            2 => Id::_4,
            3 => Id::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Id::_1
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Id::_2
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Id::_4
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Id::_8
    }
}
#[doc = "Field `ID` writer - Input divider"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Id, crate::Safe>;
impl<'a, REG> IdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Id::_1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Id::_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Id::_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Id::_8)
    }
}
#[doc = "TimerA clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tassel {
    #[doc = "0: TAxCLK"]
    Taclk = 0,
    #[doc = "1: ACLK"]
    Aclk = 1,
    #[doc = "2: SMCLK"]
    Smclk = 2,
    #[doc = "3: INCLK"]
    Inclk = 3,
}
impl From<Tassel> for u8 {
    #[inline(always)]
    fn from(variant: Tassel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tassel {
    type Ux = u8;
}
impl crate::IsEnum for Tassel {}
#[doc = "Field `TASSEL` reader - TimerA clock source select"]
pub type TasselR = crate::FieldReader<Tassel>;
impl TasselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tassel {
        match self.bits {
            0 => Tassel::Taclk,
            1 => Tassel::Aclk,
            2 => Tassel::Smclk,
            3 => Tassel::Inclk,
            _ => unreachable!(),
        }
    }
    #[doc = "TAxCLK"]
    #[inline(always)]
    pub fn is_taclk(&self) -> bool {
        *self == Tassel::Taclk
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == Tassel::Aclk
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == Tassel::Smclk
    }
    #[doc = "INCLK"]
    #[inline(always)]
    pub fn is_inclk(&self) -> bool {
        *self == Tassel::Inclk
    }
}
#[doc = "Field `TASSEL` writer - TimerA clock source select"]
pub type TasselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tassel, crate::Safe>;
impl<'a, REG> TasselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TAxCLK"]
    #[inline(always)]
    pub fn taclk(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Taclk)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Aclk)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Smclk)
    }
    #[doc = "INCLK"]
    #[inline(always)]
    pub fn inclk(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Inclk)
    }
}
impl R {
    #[doc = "Bit 0 - TimerA interrupt flag"]
    #[inline(always)]
    pub fn taifg(&self) -> TaifgR {
        TaifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TimerA interrupt enable"]
    #[inline(always)]
    pub fn taie(&self) -> TaieR {
        TaieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TimerA clear"]
    #[inline(always)]
    pub fn taclr(&self) -> TaclrR {
        TaclrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    pub fn mc(&self) -> McR {
        McR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TimerA clock source select"]
    #[inline(always)]
    pub fn tassel(&self) -> TasselR {
        TasselR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TimerA interrupt flag"]
    #[inline(always)]
    pub fn taifg(&mut self) -> TaifgW<'_, Ta3ctlSpec> {
        TaifgW::new(self, 0)
    }
    #[doc = "Bit 1 - TimerA interrupt enable"]
    #[inline(always)]
    pub fn taie(&mut self) -> TaieW<'_, Ta3ctlSpec> {
        TaieW::new(self, 1)
    }
    #[doc = "Bit 2 - TimerA clear"]
    #[inline(always)]
    pub fn taclr(&mut self) -> TaclrW<'_, Ta3ctlSpec> {
        TaclrW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    pub fn mc(&mut self) -> McW<'_, Ta3ctlSpec> {
        McW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<'_, Ta3ctlSpec> {
        IdW::new(self, 6)
    }
    #[doc = "Bits 8:9 - TimerA clock source select"]
    #[inline(always)]
    pub fn tassel(&mut self) -> TasselW<'_, Ta3ctlSpec> {
        TasselW::new(self, 8)
    }
}
#[doc = "TimerAx Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta3ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta3ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta3ctlSpec;
impl crate::RegisterSpec for Ta3ctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta3ctl::R`](R) reader structure"]
impl crate::Readable for Ta3ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ta3ctl::W`](W) writer structure"]
impl crate::Writable for Ta3ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA3CTL to value 0"]
impl crate::Resettable for Ta3ctlSpec {}
