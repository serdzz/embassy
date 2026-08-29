#[doc = "Register `SDHSCTL6` reader"]
pub type R = crate::R<Sdhsctl6Spec>;
#[doc = "Register `SDHSCTL6` writer"]
pub type W = crate::W<Sdhsctl6Spec>;
#[doc = "Field `PGA_GAIN` reader - PGA Gain Control bits"]
pub type PgaGainR = crate::FieldReader;
#[doc = "Field `PGA_GAIN` writer - PGA Gain Control bits"]
pub type PgaGainW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - PGA Gain Control bits"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PgaGainR {
        PgaGainR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PGA Gain Control bits"]
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PgaGainW<'_, Sdhsctl6Spec> {
        PgaGainW::new(self, 0)
    }
}
#[doc = "SDHS Control Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdhsctl6Spec;
impl crate::RegisterSpec for Sdhsctl6Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsctl6::R`](R) reader structure"]
impl crate::Readable for Sdhsctl6Spec {}
#[doc = "`write(|w| ..)` method takes [`sdhsctl6::W`](W) writer structure"]
impl crate::Writable for Sdhsctl6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSCTL6 to value 0"]
impl crate::Resettable for Sdhsctl6Spec {}
