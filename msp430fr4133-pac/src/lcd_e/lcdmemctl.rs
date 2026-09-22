#[doc = "Register `LCDMEMCTL` reader"]
pub type R = crate::R<LcdmemctlSpec>;
#[doc = "Register `LCDMEMCTL` writer"]
pub type W = crate::W<LcdmemctlSpec>;
#[doc = "Field `LCDDISP` reader - LCD_E LCD memory registers for display"]
pub type LcddispR = crate::BitReader;
#[doc = "Field `LCDDISP` writer - LCD_E LCD memory registers for display"]
pub type LcddispW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCLRM` reader - LCD_E Clear LCD memory"]
pub type LcdclrmR = crate::BitReader;
#[doc = "Field `LCDCLRM` writer - LCD_E Clear LCD memory"]
pub type LcdclrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCLRBM` reader - LCD_E Clear LCD blinking memory"]
pub type LcdclrbmR = crate::BitReader;
#[doc = "Field `LCDCLRBM` writer - LCD_E Clear LCD blinking memory"]
pub type LcdclrbmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCD_E LCD memory registers for display"]
    #[inline(always)]
    pub fn lcddisp(&self) -> LcddispR {
        LcddispR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD_E Clear LCD memory"]
    #[inline(always)]
    pub fn lcdclrm(&self) -> LcdclrmR {
        LcdclrmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD_E Clear LCD blinking memory"]
    #[inline(always)]
    pub fn lcdclrbm(&self) -> LcdclrbmR {
        LcdclrbmR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD_E LCD memory registers for display"]
    #[inline(always)]
    pub fn lcddisp(&mut self) -> LcddispW<'_, LcdmemctlSpec> {
        LcddispW::new(self, 0)
    }
    #[doc = "Bit 1 - LCD_E Clear LCD memory"]
    #[inline(always)]
    pub fn lcdclrm(&mut self) -> LcdclrmW<'_, LcdmemctlSpec> {
        LcdclrmW::new(self, 1)
    }
    #[doc = "Bit 2 - LCD_E Clear LCD blinking memory"]
    #[inline(always)]
    pub fn lcdclrbm(&mut self) -> LcdclrbmW<'_, LcdmemctlSpec> {
        LcdclrbmW::new(self, 2)
    }
}
#[doc = "LCD_E memory control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdmemctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdmemctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdmemctlSpec;
impl crate::RegisterSpec for LcdmemctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdmemctl::R`](R) reader structure"]
impl crate::Readable for LcdmemctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdmemctl::W`](W) writer structure"]
impl crate::Writable for LcdmemctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDMEMCTL to value 0"]
impl crate::Resettable for LcdmemctlSpec {}
