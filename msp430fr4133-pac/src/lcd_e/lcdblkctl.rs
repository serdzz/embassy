#[doc = "Register `LCDBLKCTL` reader"]
pub type R = crate::R<LcdblkctlSpec>;
#[doc = "Register `LCDBLKCTL` writer"]
pub type W = crate::W<LcdblkctlSpec>;
#[doc = "LCD_E Blinking mode Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcdblkmod {
    #[doc = "0: LCD_E Blinking mode: Off"]
    Lcdblkmod0 = 0,
    #[doc = "1: LCD_E Blinking mode: Individual"]
    Lcdblkmod1 = 1,
    #[doc = "2: LCD_E Blinking mode: All"]
    Lcdblkmod2 = 2,
    #[doc = "3: LCD_E Blinking mode: Switching"]
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
#[doc = "Field `LCDBLKMOD` reader - LCD_E Blinking mode Bit: 0"]
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
    #[doc = "LCD_E Blinking mode: Off"]
    #[inline(always)]
    pub fn is_lcdblkmod_0(&self) -> bool {
        *self == Lcdblkmod::Lcdblkmod0
    }
    #[doc = "LCD_E Blinking mode: Individual"]
    #[inline(always)]
    pub fn is_lcdblkmod_1(&self) -> bool {
        *self == Lcdblkmod::Lcdblkmod1
    }
    #[doc = "LCD_E Blinking mode: All"]
    #[inline(always)]
    pub fn is_lcdblkmod_2(&self) -> bool {
        *self == Lcdblkmod::Lcdblkmod2
    }
    #[doc = "LCD_E Blinking mode: Switching"]
    #[inline(always)]
    pub fn is_lcdblkmod_3(&self) -> bool {
        *self == Lcdblkmod::Lcdblkmod3
    }
}
#[doc = "Field `LCDBLKMOD` writer - LCD_E Blinking mode Bit: 0"]
pub type LcdblkmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lcdblkmod, crate::Safe>;
impl<'a, REG> LcdblkmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LCD_E Blinking mode: Off"]
    #[inline(always)]
    pub fn lcdblkmod_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkmod::Lcdblkmod0)
    }
    #[doc = "LCD_E Blinking mode: Individual"]
    #[inline(always)]
    pub fn lcdblkmod_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkmod::Lcdblkmod1)
    }
    #[doc = "LCD_E Blinking mode: All"]
    #[inline(always)]
    pub fn lcdblkmod_2(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkmod::Lcdblkmod2)
    }
    #[doc = "LCD_E Blinking mode: Switching"]
    #[inline(always)]
    pub fn lcdblkmod_3(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkmod::Lcdblkmod3)
    }
}
#[doc = "LCD_E Clock pre-scaler for blinking frequency Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcdblkpre {
    #[doc = "0: LCD_E Clock pre-scaler for blinking frequency: 0"]
    Lcdblkpre0 = 0,
    #[doc = "1: LCD_E Clock pre-scaler for blinking frequency: 1"]
    Lcdblkpre1 = 1,
    #[doc = "2: LCD_E Clock pre-scaler for blinking frequency: 2"]
    Lcdblkpre2 = 2,
    #[doc = "3: LCD_E Clock pre-scaler for blinking frequency: 3"]
    Lcdblkpre3 = 3,
    #[doc = "4: LCD_E Clock pre-scaler for blinking frequency: 4"]
    Lcdblkpre4 = 4,
    #[doc = "5: LCD_E Clock pre-scaler for blinking frequency: 5"]
    Lcdblkpre5 = 5,
    #[doc = "6: LCD_E Clock pre-scaler for blinking frequency: 6"]
    Lcdblkpre6 = 6,
    #[doc = "7: LCD_E Clock pre-scaler for blinking frequency: 7"]
    Lcdblkpre7 = 7,
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
#[doc = "Field `LCDBLKPRE` reader - LCD_E Clock pre-scaler for blinking frequency Bit: 0"]
pub type LcdblkpreR = crate::FieldReader<Lcdblkpre>;
impl LcdblkpreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdblkpre {
        match self.bits {
            0 => Lcdblkpre::Lcdblkpre0,
            1 => Lcdblkpre::Lcdblkpre1,
            2 => Lcdblkpre::Lcdblkpre2,
            3 => Lcdblkpre::Lcdblkpre3,
            4 => Lcdblkpre::Lcdblkpre4,
            5 => Lcdblkpre::Lcdblkpre5,
            6 => Lcdblkpre::Lcdblkpre6,
            7 => Lcdblkpre::Lcdblkpre7,
            _ => unreachable!(),
        }
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 0"]
    #[inline(always)]
    pub fn is_lcdblkpre_0(&self) -> bool {
        *self == Lcdblkpre::Lcdblkpre0
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 1"]
    #[inline(always)]
    pub fn is_lcdblkpre_1(&self) -> bool {
        *self == Lcdblkpre::Lcdblkpre1
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 2"]
    #[inline(always)]
    pub fn is_lcdblkpre_2(&self) -> bool {
        *self == Lcdblkpre::Lcdblkpre2
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 3"]
    #[inline(always)]
    pub fn is_lcdblkpre_3(&self) -> bool {
        *self == Lcdblkpre::Lcdblkpre3
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 4"]
    #[inline(always)]
    pub fn is_lcdblkpre_4(&self) -> bool {
        *self == Lcdblkpre::Lcdblkpre4
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 5"]
    #[inline(always)]
    pub fn is_lcdblkpre_5(&self) -> bool {
        *self == Lcdblkpre::Lcdblkpre5
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 6"]
    #[inline(always)]
    pub fn is_lcdblkpre_6(&self) -> bool {
        *self == Lcdblkpre::Lcdblkpre6
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 7"]
    #[inline(always)]
    pub fn is_lcdblkpre_7(&self) -> bool {
        *self == Lcdblkpre::Lcdblkpre7
    }
}
#[doc = "Field `LCDBLKPRE` writer - LCD_E Clock pre-scaler for blinking frequency Bit: 0"]
pub type LcdblkpreW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lcdblkpre, crate::Safe>;
impl<'a, REG> LcdblkpreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 0"]
    #[inline(always)]
    pub fn lcdblkpre_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::Lcdblkpre0)
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 1"]
    #[inline(always)]
    pub fn lcdblkpre_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::Lcdblkpre1)
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 2"]
    #[inline(always)]
    pub fn lcdblkpre_2(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::Lcdblkpre2)
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 3"]
    #[inline(always)]
    pub fn lcdblkpre_3(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::Lcdblkpre3)
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 4"]
    #[inline(always)]
    pub fn lcdblkpre_4(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::Lcdblkpre4)
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 5"]
    #[inline(always)]
    pub fn lcdblkpre_5(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::Lcdblkpre5)
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 6"]
    #[inline(always)]
    pub fn lcdblkpre_6(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::Lcdblkpre6)
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 7"]
    #[inline(always)]
    pub fn lcdblkpre_7(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdblkpre::Lcdblkpre7)
    }
}
impl R {
    #[doc = "Bits 0:1 - LCD_E Blinking mode Bit: 0"]
    #[inline(always)]
    pub fn lcdblkmod(&self) -> LcdblkmodR {
        LcdblkmodR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - LCD_E Clock pre-scaler for blinking frequency Bit: 0"]
    #[inline(always)]
    pub fn lcdblkpre(&self) -> LcdblkpreR {
        LcdblkpreR::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LCD_E Blinking mode Bit: 0"]
    #[inline(always)]
    pub fn lcdblkmod(&mut self) -> LcdblkmodW<'_, LcdblkctlSpec> {
        LcdblkmodW::new(self, 0)
    }
    #[doc = "Bits 2:4 - LCD_E Clock pre-scaler for blinking frequency Bit: 0"]
    #[inline(always)]
    pub fn lcdblkpre(&mut self) -> LcdblkpreW<'_, LcdblkctlSpec> {
        LcdblkpreW::new(self, 2)
    }
}
#[doc = "LCD_E blinking control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdblkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdblkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdblkctlSpec;
impl crate::RegisterSpec for LcdblkctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdblkctl::R`](R) reader structure"]
impl crate::Readable for LcdblkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdblkctl::W`](W) writer structure"]
impl crate::Writable for LcdblkctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDBLKCTL to value 0"]
impl crate::Resettable for LcdblkctlSpec {}
