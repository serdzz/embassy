#[doc = "Register `SDHSCTL7` reader"]
pub type R = crate::R<Sdhsctl7Spec>;
#[doc = "Register `SDHSCTL7` writer"]
pub type W = crate::W<Sdhsctl7Spec>;
#[doc = "Field `MODOPTI` reader - SDHS Modulator Optimization bits."]
pub type ModoptiR = crate::FieldReader;
#[doc = "Field `MODOPTI` writer - SDHS Modulator Optimization bits."]
pub type ModoptiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - SDHS Modulator Optimization bits."]
    #[inline(always)]
    pub fn modopti(&self) -> ModoptiR {
        ModoptiR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - SDHS Modulator Optimization bits."]
    #[inline(always)]
    pub fn modopti(&mut self) -> ModoptiW<'_, Sdhsctl7Spec> {
        ModoptiW::new(self, 0)
    }
}
#[doc = "SDHS Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdhsctl7Spec;
impl crate::RegisterSpec for Sdhsctl7Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsctl7::R`](R) reader structure"]
impl crate::Readable for Sdhsctl7Spec {}
#[doc = "`write(|w| ..)` method takes [`sdhsctl7::W`](W) writer structure"]
impl crate::Writable for Sdhsctl7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSCTL7 to value 0"]
impl crate::Resettable for Sdhsctl7Spec {}
