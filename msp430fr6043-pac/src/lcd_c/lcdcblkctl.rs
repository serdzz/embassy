#[doc = "Register `LCDCBLKCTL` reader"]
pub type R = crate::R<LcdcblkctlSpec>;
#[doc = "Register `LCDCBLKCTL` writer"]
pub type W = crate::W<LcdcblkctlSpec>;
#[doc = "Blinking mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcdblkmod {
    #[doc = "0: Blinking disabled"]
    Lcdblkmod0 = 0,
    #[doc = "1: Blinking of individual segments as enabled in blinking memory register LCDBMx. In mux mode 5 blinking is disabled."]
    Lcdblkmod1 = 1,
    #[doc = "2: Blinking of all segments"]
    Lcdblkmod2 = 2,
    #[doc = "3: Switching between display contents as stored in LCDMx and LCDBMx memory registers. In mux mode 5 blinking is disabled."]
    Lcdblkmod3 = 3,
}
impl From<Lcdblkmod> for u8 {
    #[inline(always)]
    fn from(variant: Lcdblkmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcdblkmod {
    type Ux = u8;
}
impl crate::IsEnum for Lcdblkmod {}
#[doc = "Field `LCDBLKMOD` reader - Blinking mode"]
pub type LcdblkmodR = crate::FieldReader<Lcdblkmod>;
impl LcdblkmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdblkmod {
        match self.bits {
            0 => Lcdblkmod::Lcdblkmod0,
            1 => Lcdblkmod::Lcdblkmod1,
            2 => Lcdblkmod::Lcdblkmod2,
            3 => Lcdblkmod::Lcdblkmod3,
            _ => unreachable!(),
        }
    }
    #[doc = "Blinking disabled"]
    #[inline(always)]
    pub fn is_lcdblkmod_0(&self) -> bool {
        *self == Lcdblkmod::Lcdblkmod0
    }
    #[doc = "Blinking of individual segments as enabled in blinking memory register LCDBMx. In mux mode 5 blinking is disabled."]
    #[inline(always)]
    pub fn is_lcdblkmod_1(&self) -> bool {
        *self == Lcdblkmod::Lcdblkmod1
    }
    #[doc = "Blinking of all segments"]
    #[inline(always)]
    pub fn is_lcdblkmod_2(&self) -> bool {
        *self == Lcdblkmod::Lcdblkmod2
    }
    #[doc = "Switching between display contents as stored in LCDMx and LCDBMx memory registers. In mux mode 5 blinking is disabled."]
    #[inline(always)]
    pub fn is_lcdblkmod_3(&self) -> bool {
        *self == Lcdblkmod::Lcdblkmod3
    }
}
#[doc = "Field `LCDBLKMOD` writer - Blinking mode"]
pub type LcdblkmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lcdblkmod, crate::Safe>;
impl<'a, REG> LcdblkmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Blinking disabled"]
    #[inline(always)]
    pub fn lcdblkmod_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkmod::Lcdblkmod0)
    }
    #[doc = "Blinking of individual segments as enabled in blinking memory register LCDBMx. In mux mode 5 blinking is disabled."]
    #[inline(always)]
    pub fn lcdblkmod_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkmod::Lcdblkmod1)
    }
    #[doc = "Blinking of all segments"]
    #[inline(always)]
    pub fn lcdblkmod_2(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkmod::Lcdblkmod2)
    }
    #[doc = "Switching between display contents as stored in LCDMx and LCDBMx memory registers. In mux mode 5 blinking is disabled."]
    #[inline(always)]
    pub fn lcdblkmod_3(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkmod::Lcdblkmod3)
    }
}
#[doc = "Clock pre-scaler for blinking frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcdblkpre {
    #[doc = "0: Divide by 512"]
    _512 = 0,
    #[doc = "1: Divide by 1024"]
    _1024 = 1,
    #[doc = "2: Divide by 2048"]
    _2048 = 2,
    #[doc = "3: Divide by 4096"]
    _4096 = 3,
    #[doc = "4: Divide by 8162"]
    _8162 = 4,
    #[doc = "5: Divide by 16384"]
    _16384 = 5,
    #[doc = "6: Divide by 32768"]
    _32768 = 6,
    #[doc = "7: Divide by 65536"]
    _65536 = 7,
}
impl From<Lcdblkpre> for u8 {
    #[inline(always)]
    fn from(variant: Lcdblkpre) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcdblkpre {
    type Ux = u8;
}
impl crate::IsEnum for Lcdblkpre {}
#[doc = "Field `LCDBLKPRE` reader - Clock pre-scaler for blinking frequency"]
pub type LcdblkpreR = crate::FieldReader<Lcdblkpre>;
impl LcdblkpreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdblkpre {
        match self.bits {
            0 => Lcdblkpre::_512,
            1 => Lcdblkpre::_1024,
            2 => Lcdblkpre::_2048,
            3 => Lcdblkpre::_4096,
            4 => Lcdblkpre::_8162,
            5 => Lcdblkpre::_16384,
            6 => Lcdblkpre::_32768,
            7 => Lcdblkpre::_65536,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 512"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == Lcdblkpre::_512
    }
    #[doc = "Divide by 1024"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == Lcdblkpre::_1024
    }
    #[doc = "Divide by 2048"]
    #[inline(always)]
    pub fn is_2048(&self) -> bool {
        *self == Lcdblkpre::_2048
    }
    #[doc = "Divide by 4096"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        *self == Lcdblkpre::_4096
    }
    #[doc = "Divide by 8162"]
    #[inline(always)]
    pub fn is_8162(&self) -> bool {
        *self == Lcdblkpre::_8162
    }
    #[doc = "Divide by 16384"]
    #[inline(always)]
    pub fn is_16384(&self) -> bool {
        *self == Lcdblkpre::_16384
    }
    #[doc = "Divide by 32768"]
    #[inline(always)]
    pub fn is_32768(&self) -> bool {
        *self == Lcdblkpre::_32768
    }
    #[doc = "Divide by 65536"]
    #[inline(always)]
    pub fn is_65536(&self) -> bool {
        *self == Lcdblkpre::_65536
    }
}
#[doc = "Field `LCDBLKPRE` writer - Clock pre-scaler for blinking frequency"]
pub type LcdblkpreW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lcdblkpre, crate::Safe>;
impl<'a, REG> LcdblkpreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 512"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::_512)
    }
    #[doc = "Divide by 1024"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::_1024)
    }
    #[doc = "Divide by 2048"]
    #[inline(always)]
    pub fn _2048(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::_2048)
    }
    #[doc = "Divide by 4096"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::_4096)
    }
    #[doc = "Divide by 8162"]
    #[inline(always)]
    pub fn _8162(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::_8162)
    }
    #[doc = "Divide by 16384"]
    #[inline(always)]
    pub fn _16384(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::_16384)
    }
    #[doc = "Divide by 32768"]
    #[inline(always)]
    pub fn _32768(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::_32768)
    }
    #[doc = "Divide by 65536"]
    #[inline(always)]
    pub fn _65536(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::_65536)
    }
}
#[doc = "Clock divider for blinking frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcdblkdiv {
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
}
impl From<Lcdblkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Lcdblkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcdblkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Lcdblkdiv {}
#[doc = "Field `LCDBLKDIV` reader - Clock divider for blinking frequency"]
pub type LcdblkdivR = crate::FieldReader<Lcdblkdiv>;
impl LcdblkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdblkdiv {
        match self.bits {
            0 => Lcdblkdiv::_1,
            1 => Lcdblkdiv::_2,
            2 => Lcdblkdiv::_3,
            3 => Lcdblkdiv::_4,
            4 => Lcdblkdiv::_5,
            5 => Lcdblkdiv::_6,
            6 => Lcdblkdiv::_7,
            7 => Lcdblkdiv::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lcdblkdiv::_1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Lcdblkdiv::_2
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == Lcdblkdiv::_3
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Lcdblkdiv::_4
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == Lcdblkdiv::_5
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == Lcdblkdiv::_6
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == Lcdblkdiv::_7
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Lcdblkdiv::_8
    }
}
#[doc = "Field `LCDBLKDIV` writer - Clock divider for blinking frequency"]
pub type LcdblkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lcdblkdiv, crate::Safe>;
impl<'a, REG> LcdblkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkdiv::_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkdiv::_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkdiv::_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkdiv::_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkdiv::_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkdiv::_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkdiv::_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkdiv::_8)
    }
}
impl R {
    #[doc = "Bits 0:1 - Blinking mode"]
    #[inline(always)]
    pub fn lcdblkmod(&self) -> LcdblkmodR {
        LcdblkmodR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Clock pre-scaler for blinking frequency"]
    #[inline(always)]
    pub fn lcdblkpre(&self) -> LcdblkpreR {
        LcdblkpreR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - Clock divider for blinking frequency"]
    #[inline(always)]
    pub fn lcdblkdiv(&self) -> LcdblkdivR {
        LcdblkdivR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Blinking mode"]
    #[inline(always)]
    pub fn lcdblkmod(&mut self) -> LcdblkmodW<'_, LcdcblkctlSpec> {
        LcdblkmodW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Clock pre-scaler for blinking frequency"]
    #[inline(always)]
    pub fn lcdblkpre(&mut self) -> LcdblkpreW<'_, LcdcblkctlSpec> {
        LcdblkpreW::new(self, 2)
    }
    #[doc = "Bits 5:7 - Clock divider for blinking frequency"]
    #[inline(always)]
    pub fn lcdblkdiv(&mut self) -> LcdblkdivW<'_, LcdcblkctlSpec> {
        LcdblkdivW::new(self, 5)
    }
}
#[doc = "LCD_C blinking control\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcblkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcblkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcblkctlSpec;
impl crate::RegisterSpec for LcdcblkctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdcblkctl::R`](R) reader structure"]
impl crate::Readable for LcdcblkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdcblkctl::W`](W) writer structure"]
impl crate::Writable for LcdcblkctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDCBLKCTL to value 0"]
impl crate::Resettable for LcdcblkctlSpec {}
