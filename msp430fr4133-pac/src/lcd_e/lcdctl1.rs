#[doc = "Register `LCDCTL1` reader"]
pub type R = crate::R<Lcdctl1Spec>;
#[doc = "Register `LCDCTL1` writer"]
pub type W = crate::W<Lcdctl1Spec>;
#[doc = "Field `LCDFRMIFG` reader - LCD_E LCD frame interrupt flag"]
pub type LcdfrmifgR = crate::BitReader;
#[doc = "Field `LCDFRMIFG` writer - LCD_E LCD frame interrupt flag"]
pub type LcdfrmifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDBLKOFFIFG` reader - LCD_E LCD blinking off interrupt flag"]
pub type LcdblkoffifgR = crate::BitReader;
#[doc = "Field `LCDBLKOFFIFG` writer - LCD_E LCD blinking off interrupt flag"]
pub type LcdblkoffifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDBLKONIFG` reader - LCD_E LCD blinking on interrupt flag"]
pub type LcdblkonifgR = crate::BitReader;
#[doc = "Field `LCDBLKONIFG` writer - LCD_E LCD blinking on interrupt flag"]
pub type LcdblkonifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDFRMIE` reader - LCD_E LCD frame interrupt enable"]
pub type LcdfrmieR = crate::BitReader;
#[doc = "Field `LCDFRMIE` writer - LCD_E LCD frame interrupt enable"]
pub type LcdfrmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDBLKOFFIE` reader - LCD_E LCD blinking off interrupt flag"]
pub type LcdblkoffieR = crate::BitReader;
#[doc = "Field `LCDBLKOFFIE` writer - LCD_E LCD blinking off interrupt flag"]
pub type LcdblkoffieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDBLKONIE` reader - LCD_E LCD blinking on interrupt flag"]
pub type LcdblkonieR = crate::BitReader;
#[doc = "Field `LCDBLKONIE` writer - LCD_E LCD blinking on interrupt flag"]
pub type LcdblkonieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCD_E LCD frame interrupt flag"]
    #[inline(always)]
    pub fn lcdfrmifg(&self) -> LcdfrmifgR {
        LcdfrmifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD_E LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffifg(&self) -> LcdblkoffifgR {
        LcdblkoffifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD_E LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonifg(&self) -> LcdblkonifgR {
        LcdblkonifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD_E LCD frame interrupt enable"]
    #[inline(always)]
    pub fn lcdfrmie(&self) -> LcdfrmieR {
        LcdfrmieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD_E LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffie(&self) -> LcdblkoffieR {
        LcdblkoffieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD_E LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonie(&self) -> LcdblkonieR {
        LcdblkonieR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD_E LCD frame interrupt flag"]
    #[inline(always)]
    pub fn lcdfrmifg(&mut self) -> LcdfrmifgW<'_, Lcdctl1Spec> {
        LcdfrmifgW::new(self, 0)
    }
    #[doc = "Bit 1 - LCD_E LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffifg(&mut self) -> LcdblkoffifgW<'_, Lcdctl1Spec> {
        LcdblkoffifgW::new(self, 1)
    }
    #[doc = "Bit 2 - LCD_E LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonifg(&mut self) -> LcdblkonifgW<'_, Lcdctl1Spec> {
        LcdblkonifgW::new(self, 2)
    }
    #[doc = "Bit 8 - LCD_E LCD frame interrupt enable"]
    #[inline(always)]
    pub fn lcdfrmie(&mut self) -> LcdfrmieW<'_, Lcdctl1Spec> {
        LcdfrmieW::new(self, 8)
    }
    #[doc = "Bit 9 - LCD_E LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffie(&mut self) -> LcdblkoffieW<'_, Lcdctl1Spec> {
        LcdblkoffieW::new(self, 9)
    }
    #[doc = "Bit 10 - LCD_E LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonie(&mut self) -> LcdblkonieW<'_, Lcdctl1Spec> {
        LcdblkonieW::new(self, 10)
    }
}
#[doc = "LCD_E Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdctl1Spec;
impl crate::RegisterSpec for Lcdctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdctl1::R`](R) reader structure"]
impl crate::Readable for Lcdctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdctl1::W`](W) writer structure"]
impl crate::Writable for Lcdctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDCTL1 to value 0"]
impl crate::Resettable for Lcdctl1Spec {}
